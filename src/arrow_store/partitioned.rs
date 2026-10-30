//! Snapshots partitioned by the day the games were played, `year=YYYY/month=MM/day=DD`, so that
//! the new replays are added to the days they belong to instead of regenerating the snapshot.
//!
//! Each day directory is a small snapshot with all the snapshot files and its own
//! [`SnapshotMetadata`], the snapshot root has the [`SnapshotMetadata`] of all the days. The
//! directories can be read with globs, i.e. polars `scan_ipc("ipcs/year=*/month=*/day=*/details.arrow")`.
//!
//! The replays already in the snapshot are found from the details of every day, by file name and
//! by sha256 so that a replay copied to another directory is not added twice.

use super::*;
use arrow::array::AsArray;
use arrow::datatypes::UInt64Type;
use arrow::ipc::reader::FileReader;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// The files of every day partition, the details first, then the [`EVENT_ARROW_FNAMES`].
pub const PARTITION_ARROW_FNAMES: [&str; 7] = [
    DETAILS_ARROW_FNAME,
    STATS_ARROW_FNAME,
    UPGRADES_ARROW_FNAME,
    UNIT_BORN_ARROW_FNAME,
    UNIT_DIED_ARROW_FNAME,
    CMD_TARGET_POINT_ARROW_FNAME,
    CMD_TARGET_UNIT_ARROW_FNAME,
];

/// The glob of a snapshot file in all the day partitions, relative to the snapshot root.
pub fn partition_glob(file_name: &str) -> String {
    format!("year=*/month=*/day=*/{file_name}")
}

/// The directory of a day, relative to the snapshot root.
pub fn partition_dir(date: chrono::NaiveDate) -> PathBuf {
    use chrono::Datelike;
    PathBuf::from(format!(
        "year={:04}/month={:02}/day={:02}",
        date.year(),
        date.month(),
        date.day()
    ))
}

/// The sub directories of `dir` named `{key}=...`, sorted.
fn key_dirs(dir: &Path, key: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let prefix = format!("{key}=");
    let mut dirs = vec![];
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let is_key_dir = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with(&prefix));
        if is_key_dir && path.is_dir() {
            dirs.push(path);
        }
    }
    dirs.sort();
    Ok(dirs)
}

/// The day partitions of the snapshot, sorted by date.
pub fn list_partitions(root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut days = vec![];
    if !root.is_dir() {
        return Ok(days);
    }
    for year in key_dirs(root, "year")? {
        for month in key_dirs(&year, "month")? {
            days.append(&mut key_dirs(&month, "day")?);
        }
    }
    Ok(days)
}

/// Whether the snapshot was written before the partitions, with the files at its root.
pub fn has_flat_layout(root: &Path) -> bool {
    root.join(DETAILS_ARROW_FNAME).is_file()
}

/// Deletes the snapshot files and the day partitions, other files in the directory are kept.
pub fn remove_snapshot(root: &Path) -> Result<(), std::io::Error> {
    for name in PARTITION_ARROW_FNAMES
        .iter()
        .chain(std::iter::once(&SNAPSHOT_METADATA_FNAME))
    {
        let path = root.join(name);
        if path.is_file() {
            std::fs::remove_file(path)?;
        }
    }
    if root.is_dir() {
        for year in key_dirs(root, "year")? {
            std::fs::remove_dir_all(year)?;
        }
    }
    Ok(())
}

/// Whether the day was written by this version of s2protocol, the days without metadata were not
/// completed.
pub fn is_current_partition(day: &Path) -> bool {
    SnapshotMetadata::read_from_dir(day)
        .is_ok_and(|metadata| metadata.s2protocol_version == S2PROTOCOL_VERSION)
}

/// Removes the days written by another version of s2protocol, so that their replays are parsed
/// again by this version, the replays of the other days are not parsed again. Returns the days
/// removed.
pub fn remove_outdated_partitions(root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let outdated: Vec<PathBuf> = list_partitions(root)?
        .into_iter()
        .filter(|day| !is_current_partition(day))
        .collect();
    for day in &outdated {
        std::fs::remove_dir_all(day)?;
    }
    Ok(outdated)
}

/// The replays already in the snapshot.
#[derive(Debug, Default, Clone)]
pub struct SnapshotIndex {
    pub file_names: HashSet<String>,
    pub sha256s: HashSet<String>,
    /// The highest replay id, the new replays get the ids that follow it.
    pub max_ext_fs_id: Option<u64>,
}

impl SnapshotIndex {
    /// Reads the replays from the details of every day partition.
    pub fn read(root: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut index = Self::default();
        for day in list_partitions(root)? {
            let path = day.join(DETAILS_ARROW_FNAME);
            if !path.is_file() {
                continue;
            }
            let reader = FileReader::try_new(std::fs::File::open(&path)?, None)?;
            for batch in reader {
                let batch = batch?;
                let column = |name: &str| {
                    batch
                        .column_by_name(name)
                        .ok_or_else(|| format!("{} has no column {name}", path.display()))
                };
                let ids = column("ext_fs_id")?
                    .as_primitive_opt::<UInt64Type>()
                    .ok_or("ext_fs_id is not a u64")?;
                index.max_ext_fs_id = ids.iter().flatten().chain(index.max_ext_fs_id).max();
                for name in ["ext_fs_file_name", "ext_fs_sha256"] {
                    let values = column(name)?
                        .as_string_opt::<i32>()
                        .ok_or_else(|| format!("{name} is not a string"))?;
                    let set = if name == "ext_fs_file_name" {
                        &mut index.file_names
                    } else {
                        &mut index.sha256s
                    };
                    set.extend(values.iter().flatten().map(str::to_string));
                }
            }
        }
        Ok(index)
    }

    /// The id of the first new replay.
    pub fn next_ext_fs_id(&self) -> u64 {
        self.max_ext_fs_id.map(|id| id + 1).unwrap_or_default()
    }
}

/// The details files of the snapshot, of every day partition or the one at the root of the
/// snapshots written before the partitions.
fn details_files(root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    if has_flat_layout(root) {
        return Ok(vec![root.join(DETAILS_ARROW_FNAME)]);
    }
    Ok(list_partitions(root)?
        .into_iter()
        .map(|day| day.join(DETAILS_ARROW_FNAME))
        .filter(|path| path.is_file())
        .collect())
}

/// The map caches the replays of the snapshot refer to, each one once.
pub fn snapshot_cache_handles(
    root: &Path,
) -> Result<Vec<crate::cache_handles::CacheHandle>, Box<dyn std::error::Error>> {
    let mut handles = BTreeMap::new();
    for path in details_files(root)? {
        let reader = FileReader::try_new(std::fs::File::open(&path)?, None)?;
        for batch in reader {
            let batch = batch?;
            let column = |name: &str| {
                batch
                    .column_by_name(name)
                    .and_then(|column| column.as_string_opt::<i32>())
                    .ok_or_else(|| format!("{} has no string column {name}", path.display()))
            };
            let regions = column("cache_handle_region")?;
            let extensions = column("cache_handle_extension")?;
            let cache_handles = column("cache_handles")?;
            for row in 0..batch.num_rows() {
                // A cache is the same in every region, it is downloaded once.
                for handle in cache_handles.value(row).split(',') {
                    if handle.is_empty() || handles.contains_key(handle) {
                        continue;
                    }
                    handles.insert(
                        handle.to_string(),
                        crate::cache_handles::CacheHandle::new(
                            regions.value(row),
                            extensions.value(row),
                            handle,
                        ),
                    );
                }
            }
        }
    }
    Ok(handles.into_values().collect())
}

/// Downloads the map caches of the snapshot that are not in `destination` yet, i.e. the ones
/// that failed while the replays were added. The snapshot is not changed.
pub async fn download_snapshot_caches(
    root: &Path,
    destination: &str,
) -> Result<crate::cache_handles::CacheDownloads, Box<dyn std::error::Error>> {
    let handles = snapshot_cache_handles(root)?;
    Ok(crate::cache_handles::download_caches(&handles, destination).await)
}

/// The file being written next to the final one, renamed once the day is complete.
/// The days written at the same time, each open day has 7 files open, the default limit of open
/// files is 256 on macOS and 1024 on Linux, the replays being read are open files too.
const MAX_OPEN_DAYS: usize = 24;

fn tmp_path(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(".tmp");
    PathBuf::from(name)
}

/// Copies the batches of an existing file of the day into the new one, returns the rows copied.
fn copy_existing_batches(
    path: &Path,
    schema: &Schema,
    writer: &ArrowMutexWriter,
) -> Result<usize, Box<dyn std::error::Error>> {
    if !path.is_file() {
        return Ok(0);
    }
    let reader = FileReader::try_new(std::fs::File::open(path)?, None)?;
    if reader.schema().fields() != schema.fields() {
        return Err(format!(
            "{} was written with a different schema, recreate the snapshot",
            path.display()
        )
        .into());
    }
    let mut writer = writer.lock().map_err(|_| "Lock error")?;
    let mut rows = 0;
    for batch in reader {
        let batch = batch?;
        rows += batch.num_rows();
        writer.write(&batch)?;
    }
    Ok(rows)
}

/// The files of a day being written, opened when the first replay of the day is processed.
struct DayWriter {
    dir: PathBuf,
    /// The writers of the [`PARTITION_ARROW_FNAMES`].
    writers: Vec<ArrowMutexWriter>,
    /// The rows written to each of the [`PARTITION_ARROW_FNAMES`], copied and new.
    counts: [AtomicUsize; 7],
}

impl DayWriter {
    fn open(
        dir: PathBuf,
        details: Vec<PlayerLobbyDetailsFlatRow>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        std::fs::create_dir_all(&dir)?;
        let schemas = std::iter::once(PlayerLobbyDetailsFlatRow::schema()).chain(event_schemas());
        let mut writers = vec![];
        let counts: [AtomicUsize; 7] = Default::default();
        for ((name, schema), count) in PARTITION_ARROW_FNAMES.iter().zip(schemas).zip(&counts) {
            let path = dir.join(name);
            let writer = open_arrow_mutex_writer(tmp_path(&path), schema.clone())?;
            count.fetch_add(
                copy_existing_batches(&path, &schema, &writer)?,
                Ordering::Relaxed,
            );
            writers.push(writer);
        }
        counts[0].fetch_add(
            write_rows(&writers[0], details.try_into_arrow()),
            Ordering::Relaxed,
        );
        Ok(Self {
            dir,
            writers,
            counts,
        })
    }

    fn write_events(&self, rows: ReplayEventRows) {
        for (count, rows) in self.counts[1..].iter().zip(rows.write(&self.writers[1..])) {
            count.fetch_add(rows, Ordering::Relaxed);
        }
    }

    /// Closes the files, replaces the previous ones and writes the metadata of the day.
    fn finish(&self, new_replays: usize) -> Result<SnapshotMetadata, Box<dyn std::error::Error>> {
        let init_time = std::time::Instant::now();
        for writer in &self.writers {
            writer.lock().map_err(|_| "Lock error")?.finish()?;
        }
        for name in PARTITION_ARROW_FNAMES {
            let path = self.dir.join(name);
            std::fs::rename(tmp_path(&path), path)?;
        }
        let previous_replays = SnapshotMetadata::read_from_dir(&self.dir)
            .map(|metadata| metadata.replays)
            .unwrap_or_default();
        let metadata = SnapshotMetadata {
            s2protocol_version: S2PROTOCOL_VERSION.to_string(),
            created_at: chrono::Utc::now(),
            elapsed_ms: u64::try_from(init_time.elapsed().as_millis()).unwrap_or(u64::MAX),
            replays: previous_replays + new_replays,
            files: PARTITION_ARROW_FNAMES
                .iter()
                .zip(&self.counts)
                .map(|(name, count)| snapshot_file(&self.dir, name, count.load(Ordering::Relaxed)))
                .collect(),
            partitions: 0,
            added: new_replays,
        };
        metadata.write_to_dir(&self.dir)?;
        Ok(metadata)
    }

    /// Removes the files of a day that failed, the previous files are kept.
    fn discard(&self) {
        for name in PARTITION_ARROW_FNAMES {
            let _ = std::fs::remove_file(tmp_path(&self.dir.join(name)));
        }
    }
}

/// The writer of a day, opened by the first replay of the day and closed by the last one.
enum DayState {
    Pending(Vec<PlayerLobbyDetailsFlatRow>),
    Open(Arc<DayWriter>),
    /// The files are closed, a finished writer still holds them open until it is dropped.
    Closed,
    Failed(String),
}

/// A day with new replays.
struct DaySlot {
    dir: PathBuf,
    state: Mutex<DayState>,
    new_replays: usize,
    /// The replays of the day not yet written, the last one finishes the day.
    remaining: AtomicUsize,
}

impl DaySlot {
    fn writer(&self) -> Result<Arc<DayWriter>, String> {
        let mut state = self.state.lock().unwrap();
        if let DayState::Pending(details) = &mut *state {
            let details = std::mem::take(details);
            *state = match DayWriter::open(self.dir.clone(), details) {
                Ok(writer) => DayState::Open(Arc::new(writer)),
                Err(err) => {
                    for name in PARTITION_ARROW_FNAMES {
                        let _ = std::fs::remove_file(tmp_path(&self.dir.join(name)));
                    }
                    DayState::Failed(err.to_string())
                }
            };
        }
        match &*state {
            DayState::Open(writer) => Ok(writer.clone()),
            DayState::Failed(err) => Err(format!("{}: {err}", self.dir.display())),
            DayState::Pending(_) | DayState::Closed => {
                Err(format!("{}: the day is not open", self.dir.display()))
            }
        }
    }

    /// Takes the writer so that its files are closed once it is finished.
    fn close(&self) -> Option<Arc<DayWriter>> {
        let mut state = self.state.lock().unwrap();
        match std::mem::replace(&mut *state, DayState::Closed) {
            DayState::Open(writer) => Some(writer),
            other => {
                *state = other;
                None
            }
        }
    }
}

/// Writes the new replays into their day partitions, the days that already exist keep their
/// replays. Returns the metadata of every day written.
pub fn write_partitions(
    root: &Path,
    sources: &[SC2ReplayBasicData],
    details: Vec<PlayerLobbyDetailsFlatRow>,
    versioned_abilities: &MultiVersionedBalanceUnits,
    progress: Option<&SnapshotProgressSender>,
) -> Result<Vec<SnapshotMetadata>, Box<dyn std::error::Error>> {
    let day_of = |source: &SC2ReplayBasicData| source.details.ext_datetime.date();
    let mut sources: Vec<&SC2ReplayBasicData> = sources.iter().collect();
    // The replays of a day are processed together so that few days are open at the same time.
    sources.sort_by_key(|source| day_of(source));
    let mut replays_per_day: BTreeMap<chrono::NaiveDate, usize> = BTreeMap::new();
    let mut day_of_replay = HashMap::new();
    for source in &sources {
        *replays_per_day.entry(day_of(source)).or_default() += 1;
        day_of_replay.insert(source.details.ext_fs_id, day_of(source));
    }
    let mut details_per_day: HashMap<chrono::NaiveDate, Vec<PlayerLobbyDetailsFlatRow>> =
        HashMap::new();
    for row in details {
        if let Some(day) = day_of_replay.get(&row.ext_fs_id) {
            details_per_day.entry(*day).or_default().push(row);
        }
    }
    let days: HashMap<chrono::NaiveDate, DaySlot> = replays_per_day
        .into_iter()
        .map(|(day, replays)| {
            let slot = DaySlot {
                dir: root.join(partition_dir(day)),
                state: Mutex::new(DayState::Pending(
                    details_per_day.remove(&day).unwrap_or_default(),
                )),
                new_replays: replays,
                remaining: AtomicUsize::new(replays),
            };
            (day, slot)
        })
        .collect();

    let processed = AtomicUsize::new(0);
    let total = sources.len();
    // Send an update for every percent of the replays processed.
    let progress_every = (total / 100).max(1);
    let written: Mutex<Vec<SnapshotMetadata>> = Mutex::new(vec![]);
    let errors: Mutex<Vec<String>> = Mutex::new(vec![]);
    // The days are written a few at a time, every open day has a file open per IPC file.
    let mut batches: Vec<Vec<&SC2ReplayBasicData>> = vec![];
    let mut batch_days = 0;
    for (idx, source) in sources.iter().enumerate() {
        let new_day = idx == 0 || day_of(sources[idx - 1]) != day_of(source);
        if batches.is_empty() || (new_day && batch_days == MAX_OPEN_DAYS) {
            batches.push(vec![]);
            batch_days = 0;
        }
        batch_days += new_day as usize;
        batches.last_mut().unwrap().push(source);
    }
    for batch in batches {
        batch.par_iter().for_each(|source| {
            let slot = &days[&day_of(source)];
            match slot.writer() {
                Ok(writer) => {
                    if let Some(rows) = ReplayEventRows::collect(source, versioned_abilities) {
                        writer.write_events(rows);
                    }
                }
                Err(err) => errors.lock().unwrap().push(err),
            }
            if slot.remaining.fetch_sub(1, Ordering::AcqRel) == 1
                && let Some(writer) = slot.close()
            {
                match writer.finish(slot.new_replays) {
                    Ok(metadata) => written.lock().unwrap().push(metadata),
                    Err(err) => {
                        writer.discard();
                        errors
                            .lock()
                            .unwrap()
                            .push(format!("{}: {err}", slot.dir.display()));
                    }
                }
            }
            let done = processed.fetch_add(1, Ordering::Relaxed) + 1;
            if done.is_multiple_of(progress_every) || done == total {
                send_progress(progress, SnapshotProgress::ReplaysProcessed { done, total });
            }
        });
    }
    let mut errors = errors.into_inner().unwrap();
    if !errors.is_empty() {
        errors.sort();
        errors.dedup();
        return Err(errors.join(", ").into());
    }
    Ok(written.into_inner().unwrap())
}

/// The metadata of the whole snapshot, from the metadata of every day.
pub fn root_metadata(
    root: &Path,
    added: usize,
    elapsed_ms: u64,
) -> Result<SnapshotMetadata, Box<dyn std::error::Error>> {
    let days = list_partitions(root)?;
    let mut files: Vec<SnapshotFile> = PARTITION_ARROW_FNAMES
        .iter()
        .map(|name| SnapshotFile {
            name: name.to_string(),
            ..Default::default()
        })
        .collect();
    let mut replays = 0;
    for day in &days {
        let metadata = SnapshotMetadata::read_from_dir(day)
            .map_err(|err| format!("{}: {err}", day.join(SNAPSHOT_METADATA_FNAME).display()))?;
        replays += metadata.replays;
        for day_file in metadata.files {
            if let Some(file) = files.iter_mut().find(|file| file.name == day_file.name) {
                file.bytes += day_file.bytes;
                file.records += day_file.records;
            }
        }
    }
    Ok(SnapshotMetadata {
        s2protocol_version: S2PROTOCOL_VERSION.to_string(),
        created_at: chrono::Utc::now(),
        elapsed_ms,
        replays,
        files,
        partitions: days.len(),
        added,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_removes_the_days_of_other_versions() {
        let root = std::env::temp_dir().join(format!("s2protocol-outdated-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let metadata = |version: &str| SnapshotMetadata {
            s2protocol_version: version.to_string(),
            created_at: chrono::Utc::now(),
            elapsed_ms: 0,
            replays: 1,
            files: vec![],
            partitions: 0,
            added: 1,
        };
        let day = |date: &str| root.join(partition_dir(date.parse().unwrap()));
        for (date, version) in [
            ("2026-01-01", Some(S2PROTOCOL_VERSION)),
            ("2026-01-02", Some("3.5.10")),
            ("2026-01-03", None),
        ] {
            std::fs::create_dir_all(day(date)).unwrap();
            if let Some(version) = version {
                metadata(version).write_to_dir(&day(date)).unwrap();
            }
        }
        let removed = remove_outdated_partitions(&root).unwrap();
        assert_eq!(removed, vec![day("2026-01-02"), day("2026-01-03")]);
        assert_eq!(list_partitions(&root).unwrap(), vec![day("2026-01-01")]);
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn it_names_the_day_partitions() {
        let date = chrono::NaiveDate::from_ymd_opt(2024, 3, 7).unwrap();
        assert_eq!(
            partition_dir(date),
            PathBuf::from("year=2024/month=03/day=07")
        );
        assert_eq!(
            partition_glob(DETAILS_ARROW_FNAME),
            "year=*/month=*/day=*/details.arrow"
        );
    }

    #[test]
    fn it_lists_and_removes_the_partitions() {
        let root =
            std::env::temp_dir().join(format!("s2protocol-partitions-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for day in ["year=2024/month=03/day=07", "year=2023/month=12/day=31"] {
            std::fs::create_dir_all(root.join(day)).unwrap();
        }
        std::fs::create_dir_all(root.join("other")).unwrap();
        std::fs::write(root.join(DETAILS_ARROW_FNAME), b"").unwrap();
        assert!(has_flat_layout(&root));
        assert_eq!(
            list_partitions(&root).unwrap(),
            vec![
                root.join("year=2023/month=12/day=31"),
                root.join("year=2024/month=03/day=07")
            ]
        );
        remove_snapshot(&root).unwrap();
        assert!(list_partitions(&root).unwrap().is_empty());
        assert!(!has_flat_layout(&root));
        // Other files of the directory are kept.
        assert!(root.join("other").is_dir());
        std::fs::remove_dir_all(&root).unwrap();
    }
}
