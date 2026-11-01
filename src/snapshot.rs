//! The progress of a snapshot generation and the metadata stored with it.
//!
//! These types do not depend on arrow so that consumers that only display the progress or read
//! the metadata, i.e. a WASM frontend, can use them.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The file inside the snapshot directory that contains the [`SnapshotMetadata`].
pub const SNAPSHOT_METADATA_FNAME: &str = "metadata.json";

/// The version of this crate, stored in the snapshot metadata.
pub const S2PROTOCOL_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The updates sent while a snapshot is generated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SnapshotProgress {
    /// The files matching the replay extension under the source directory.
    FilesFound { count: usize },
    /// The replays with valid init data, these are the ones processed.
    ValidReplays { count: usize },
    /// The replays skipped because they are already in the snapshot, or are copies of another
    /// replay, i.e. the same replay in two directories.
    AlreadyInSnapshot { count: usize },
    /// A step of the snapshot generation started, `index` starts at 1.
    Step {
        index: usize,
        total: usize,
        name: String,
    },
    /// The replays processed so far in the current step.
    ReplaysProcessed { done: usize, total: usize },
    /// A file of the snapshot has been written.
    FileWritten(SnapshotFile),
    /// The snapshot has been generated.
    Done { elapsed_ms: u64 },
}

/// A file of the snapshot.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SnapshotFile {
    /// The file name inside the snapshot directory.
    pub name: String,
    /// The size of the file.
    pub bytes: u64,
    /// The rows written to the file.
    pub records: usize,
}

/// Describes how a snapshot was generated, used to check that the snapshot is compatible with
/// the version of the crate reading it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    /// The version of s2protocol that generated the snapshot.
    pub s2protocol_version: String,
    /// When the snapshot generation finished.
    pub created_at: DateTime<Utc>,
    /// The time it took to generate the snapshot.
    pub elapsed_ms: u64,
    /// The replays in the snapshot.
    pub replays: usize,
    /// The files of the snapshot, for a partitioned snapshot the totals of all the days.
    pub files: Vec<SnapshotFile>,
    /// The day partitions, `year=YYYY/month=MM/day=DD`, 0 for the metadata of a day or for a
    /// snapshot written before the partitions, with the files at its root.
    #[serde(default)]
    pub partitions: usize,
    /// The replays added by the last generation, the others were already in the snapshot.
    #[serde(default)]
    pub added: usize,
}

impl SnapshotMetadata {
    /// Reads the metadata from a snapshot directory.
    pub fn read_from_dir(dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(dir.join(SNAPSHOT_METADATA_FNAME))?;
        Ok(serde_json::from_reader(file)?)
    }

    /// Writes the metadata to a snapshot directory.
    pub fn write_to_dir(&self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(dir.join(SNAPSHOT_METADATA_FNAME))?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    /// Whether the snapshot was generated with the same major and minor version of this crate,
    /// the patch versions do not change the snapshot schemas.
    pub fn is_compatible(&self) -> bool {
        let major_minor = |version: &str| {
            version
                .split('.')
                .take(2)
                .map(str::to_string)
                .collect::<Vec<_>>()
        };
        major_minor(&self.s2protocol_version) == major_minor(S2PROTOCOL_VERSION)
    }
}

/// Receives the [`SnapshotProgress`] updates, it can be called from parallel iterators.
#[cfg(feature = "tokio")]
pub type SnapshotProgressSender = tokio::sync::mpsc::UnboundedSender<SnapshotProgress>;

/// Sends an update if there is a receiver, a closed receiver does not stop the snapshot.
#[cfg(feature = "tokio")]
pub fn send_progress(sender: Option<&SnapshotProgressSender>, progress: SnapshotProgress) {
    if let Some(sender) = sender {
        let _ = sender.send(progress);
    }
}

/// The file in the snapshot directory while a generation runs. It is locked while the generation
/// runs and removed when it ends, so that a reader tells a running generation from one that was
/// interrupted, i.e. the application was closed: the lock dies with the process, the file stays.
pub const SNAPSHOT_IN_PROGRESS_FNAME: &str = "in_progress.json";

/// The generation of a snapshot that is running, the content of [`SNAPSHOT_IN_PROGRESS_FNAME`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotInProgress {
    pub started_at: DateTime<Utc>,
    pub s2protocol_version: String,
    /// The process writing the snapshot.
    pub pid: u32,
}

/// Marks the snapshot directory while a generation runs, see [`SNAPSHOT_IN_PROGRESS_FNAME`]: the
/// file is written and locked when the guard starts, and removed when the guard is dropped. A
/// second generation of the same directory fails to start while one runs.
#[derive(Debug)]
pub struct SnapshotInProgressGuard {
    path: PathBuf,
    /// Keeps the lock for the life of the guard.
    _file: std::fs::File,
}

impl SnapshotInProgressGuard {
    pub fn start(dir: &Path) -> std::io::Result<Self> {
        use std::io::Write;
        let path = dir.join(SNAPSHOT_IN_PROGRESS_FNAME);
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&path)?;
        file.try_lock().map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::WouldBlock,
                format!(
                    "Another generation of the snapshot is running, {} is locked: {err}",
                    path.display()
                ),
            )
        })?;
        let info = SnapshotInProgress {
            started_at: Utc::now(),
            s2protocol_version: S2PROTOCOL_VERSION.to_string(),
            pid: std::process::id(),
        };
        file.set_len(0)?;
        file.write_all(serde_json::to_string_pretty(&info)?.as_bytes())?;
        file.sync_all()?;
        Ok(Self { path, _file: file })
    }
}

impl Drop for SnapshotInProgressGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// What is at a snapshot directory, see [`snapshot_state`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SnapshotState {
    /// No snapshot.
    Missing,
    /// A generation is running.
    InProgress(SnapshotInProgress),
    /// A generation did not finish, the snapshot must be generated again from scratch.
    Interrupted { reason: String },
    /// The snapshot is whole. It may be from another version, see
    /// [`SnapshotMetadata::is_compatible`].
    Ready(SnapshotMetadata),
}

impl SnapshotState {
    /// What a user should know, None when the snapshot is ready.
    pub fn message(&self) -> Option<String> {
        match self {
            SnapshotState::Missing => Some("There is no snapshot, run Optimize".to_string()),
            SnapshotState::InProgress(info) => Some(format!(
                "The snapshot is being created since {} (process {}), the statistics are \
                 available when it finishes",
                info.started_at.format("%Y-%m-%d %H:%M UTC"),
                info.pid
            )),
            SnapshotState::Interrupted { reason } => Some(format!(
                "The snapshot is incomplete, {reason}: recreate it with Optimize"
            )),
            SnapshotState::Ready(_) => None,
        }
    }
}

/// The `key=value` directories of a directory.
fn key_dirs(dir: &Path, key: &str) -> Vec<PathBuf> {
    let prefix = format!("{key}=");
    let Ok(entries) = std::fs::read_dir(dir) else {
        return vec![];
    };
    let mut dirs: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(&prefix))
        })
        .collect();
    dirs.sort();
    dirs
}

/// The day partitions of a snapshot, `year=YYYY/month=MM/day=DD`.
pub fn day_partitions(dir: &Path) -> Vec<PathBuf> {
    key_dirs(dir, "year")
        .iter()
        .flat_map(|year| key_dirs(year, "month"))
        .flat_map(|month| key_dirs(&month, "day"))
        .collect()
}

/// Whether the snapshot at the directory is ready, being generated, or was interrupted: a locked
/// [`SNAPSHOT_IN_PROGRESS_FNAME`] is a running generation and an unlocked one an interrupted
/// generation; a day partition without its metadata, with files still being written, or written
/// after the metadata of the root is also an interrupted generation, the root metadata is written
/// last.
pub fn snapshot_state(dir: &Path) -> SnapshotState {
    let marker = dir.join(SNAPSHOT_IN_PROGRESS_FNAME);
    if marker.is_file() {
        let info: Option<SnapshotInProgress> = std::fs::File::open(&marker)
            .ok()
            .and_then(|file| serde_json::from_reader(file).ok());
        let locked = std::fs::File::open(&marker)
            .map(|file| file.try_lock_shared().is_err())
            .unwrap_or(false);
        return match (locked, info) {
            (true, Some(info)) => SnapshotState::InProgress(info),
            (true, None) => SnapshotState::InProgress(SnapshotInProgress {
                started_at: Utc::now(),
                s2protocol_version: String::new(),
                pid: 0,
            }),
            (false, Some(info)) => SnapshotState::Interrupted {
                reason: format!(
                    "the generation started at {} (process {}) did not finish",
                    info.started_at.format("%Y-%m-%d %H:%M UTC"),
                    info.pid
                ),
            },
            (false, None) => SnapshotState::Interrupted {
                reason: "a generation did not finish".to_string(),
            },
        };
    }
    let days = day_partitions(dir);
    let metadata = match SnapshotMetadata::read_from_dir(dir) {
        Ok(metadata) => metadata,
        Err(_) if days.is_empty() => return SnapshotState::Missing,
        Err(_) => {
            return SnapshotState::Interrupted {
                reason: format!(
                    "{} days are written but the snapshot has no {SNAPSHOT_METADATA_FNAME}",
                    days.len()
                ),
            };
        }
    };
    for day in &days {
        let name = day.strip_prefix(dir).unwrap_or(day).display().to_string();
        let being_written = std::fs::read_dir(day).ok().is_some_and(|entries| {
            entries
                .flatten()
                .any(|entry| entry.path().extension().is_some_and(|ext| ext == "tmp"))
        });
        if being_written {
            return SnapshotState::Interrupted {
                reason: format!("the files of {name} were being written"),
            };
        }
        match SnapshotMetadata::read_from_dir(day) {
            Ok(day_metadata) if day_metadata.created_at > metadata.created_at => {
                return SnapshotState::Interrupted {
                    reason: format!("{name} was written after the snapshot metadata"),
                };
            }
            Ok(_) => {}
            Err(_) => {
                return SnapshotState::Interrupted {
                    reason: format!("{name} has no {SNAPSHOT_METADATA_FNAME}"),
                };
            }
        }
    }
    SnapshotState::Ready(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metadata(replays: usize) -> SnapshotMetadata {
        SnapshotMetadata {
            s2protocol_version: S2PROTOCOL_VERSION.to_string(),
            created_at: Utc::now(),
            elapsed_ms: 0,
            replays,
            files: vec![],
            partitions: 0,
            added: 0,
        }
    }

    #[test]
    fn it_tells_a_running_generation_from_an_interrupted_one() {
        let dir =
            std::env::temp_dir().join(format!("swarmy-snapshot-state-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        assert_eq!(snapshot_state(&dir), SnapshotState::Missing);
        {
            let _guard = SnapshotInProgressGuard::start(&dir).unwrap();
            assert!(
                matches!(snapshot_state(&dir), SnapshotState::InProgress(info) if info.pid == std::process::id())
            );
            // A second generation of the same directory cannot start.
            assert!(SnapshotInProgressGuard::start(&dir).is_err());
        }
        assert_eq!(snapshot_state(&dir), SnapshotState::Missing);
        // The marker of a process that died is not locked.
        std::fs::write(
            dir.join(SNAPSHOT_IN_PROGRESS_FNAME),
            serde_json::to_string(&SnapshotInProgress {
                started_at: Utc::now(),
                s2protocol_version: S2PROTOCOL_VERSION.to_string(),
                pid: 1,
            })
            .unwrap(),
        )
        .unwrap();
        assert!(
            matches!(snapshot_state(&dir), SnapshotState::Interrupted { reason } if reason.contains("process 1"))
        );
        std::fs::remove_file(dir.join(SNAPSHOT_IN_PROGRESS_FNAME)).unwrap();
        // A whole snapshot, then a day written after its metadata, then a day being written.
        let day = dir.join("year=2026/month=10/day=31");
        std::fs::create_dir_all(&day).unwrap();
        metadata(2).write_to_dir(&day).unwrap();
        assert!(
            matches!(snapshot_state(&dir), SnapshotState::Interrupted { reason } if reason.contains("no metadata.json"))
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
        metadata(2).write_to_dir(&dir).unwrap();
        assert!(
            matches!(snapshot_state(&dir), SnapshotState::Ready(metadata) if metadata.replays == 2)
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
        metadata(3).write_to_dir(&day).unwrap();
        assert!(
            matches!(snapshot_state(&dir), SnapshotState::Interrupted { reason } if reason.contains("after the snapshot metadata"))
        );
        metadata(3).write_to_dir(&dir).unwrap();
        std::fs::write(day.join("stats.arrow.tmp"), b"").unwrap();
        assert!(
            matches!(snapshot_state(&dir), SnapshotState::Interrupted { reason } if reason.contains("being written"))
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn it_checks_the_snapshot_compatibility() {
        let mut metadata = SnapshotMetadata {
            s2protocol_version: S2PROTOCOL_VERSION.to_string(),
            created_at: Utc::now(),
            elapsed_ms: 0,
            replays: 0,
            files: vec![],
            partitions: 0,
            added: 0,
        };
        assert!(metadata.is_compatible());
        metadata.s2protocol_version = "0.0.1".to_string();
        assert!(!metadata.is_compatible());
    }

    #[test]
    fn it_serializes_the_progress_with_its_kind() {
        let json = serde_json::to_string(&SnapshotProgress::FilesFound { count: 3 }).unwrap();
        assert_eq!(json, r#"{"kind":"files_found","count":3}"#);
        let json = serde_json::to_string(&SnapshotProgress::FileWritten(SnapshotFile {
            name: "stats.arrow".to_string(),
            bytes: 10,
            records: 2,
        }))
        .unwrap();
        assert_eq!(
            json,
            r#"{"kind":"file_written","name":"stats.arrow","bytes":10,"records":2}"#
        );
    }
}
