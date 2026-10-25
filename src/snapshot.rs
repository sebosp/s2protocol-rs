//! The progress of a snapshot generation and the metadata stored with it.
//!
//! These types do not depend on arrow so that consumers that only display the progress or read
//! the metadata, i.e. a WASM frontend, can use them.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

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
    /// The replays skipped because they are already in the snapshot.
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

#[cfg(test)]
mod tests {
    use super::*;

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
