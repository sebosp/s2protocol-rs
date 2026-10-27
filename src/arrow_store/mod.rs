//! Arrow Specific handling of data.

#[cfg(feature = "dep_arrow")]
use arrow::{array::Array, array::ArrayRef, datatypes::Schema, record_batch::RecordBatch};
#[cfg(feature = "dep_arrow")]
use rayon::prelude::*;

use crate::basic_replay_data::SC2ReplayBasicData;
use crate::cache_handles::populate_map_info_digest_from_caches;
use crate::get_matching_files;

use crate::details::{PlayerLobbyDetails, PlayerLobbyDetailsFlatRow};
use crate::game_events::{
    CmdTargetPointEventFlatRow, CmdTargetUnitEventFlatRow, MultiVersionedBalanceUnits,
};
use crate::snapshot::*;
use crate::tracker_events::{
    PlayerStatsFlatRow, UnitBornEventFlatRow, UnitDiedEventFlatRow, UpgradeEventFlatRow,
};
use crate::*;

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
pub mod ipc_writer;
use ipc_writer::*;
pub mod rows;
pub use rows::TryIntoArrow;
pub mod partitioned;
use partitioned::*;

pub const DETAILS_ARROW_FNAME: &'static str = "details.arrow";
pub const STATS_ARROW_FNAME: &'static str = "stats.arrow";
pub const UPGRADES_ARROW_FNAME: &'static str = "upgrades.arrow";
pub const UNIT_BORN_ARROW_FNAME: &'static str = "unit_born.arrow";
pub const UNIT_DIED_ARROW_FNAME: &'static str = "unit_died.arrow";
pub const CMD_TARGET_POINT_ARROW_FNAME: &'static str = "cmd_target_point.arrow";
pub const CMD_TARGET_UNIT_ARROW_FNAME: &'static str = "cmd_target_unit.arrow";

/// The steps of [`ArrowIpcTypes::handle_arrow_ipc_cmd_with_progress`].
const SNAPSHOT_STEPS: usize = 5;

/// The size and records of a file written to the snapshot directory.
fn snapshot_file(output: &std::path::Path, name: &str, records: usize) -> SnapshotFile {
    SnapshotFile {
        name: name.to_string(),
        bytes: std::fs::metadata(output.join(name))
            .map(|meta| meta.len())
            .unwrap_or_default(),
        records,
    }
}

/// The event files of a snapshot, in the order of [`ReplayEventRows::write`].
pub const EVENT_ARROW_FNAMES: [&str; 6] = [
    STATS_ARROW_FNAME,
    UPGRADES_ARROW_FNAME,
    UNIT_BORN_ARROW_FNAME,
    UNIT_DIED_ARROW_FNAME,
    CMD_TARGET_POINT_ARROW_FNAME,
    CMD_TARGET_UNIT_ARROW_FNAME,
];

/// The schemas of the [`EVENT_ARROW_FNAMES`].
pub fn event_schemas() -> [Schema; 6] {
    [
        PlayerStatsFlatRow::schema(),
        UpgradeEventFlatRow::schema(),
        UnitBornEventFlatRow::schema(),
        UnitDiedEventFlatRow::schema(),
        CmdTargetPointEventFlatRow::schema(),
        CmdTargetUnitEventFlatRow::schema(),
    ]
}

/// The rows of the event files of a single replay.
#[derive(Debug, Default)]
pub struct ReplayEventRows {
    pub stats: Vec<PlayerStatsFlatRow>,
    pub upgrades: Vec<UpgradeEventFlatRow>,
    pub unit_born: Vec<UnitBornEventFlatRow>,
    pub unit_died: Vec<UnitDiedEventFlatRow>,
    pub target_points: Vec<CmdTargetPointEventFlatRow>,
    pub target_units: Vec<CmdTargetUnitEventFlatRow>,
}

/// Writes the rows converted to arrow to the writer, logging the errors, returns the rows written.
fn write_rows(
    writer: &ArrowMutexWriter,
    rows: Result<ArrayRef, arrow::error::ArrowError>,
) -> usize {
    match rows {
        Ok(array) if array.is_empty() => 0,
        Ok(array) => write_to_arrow_mutex_writer(writer, array).unwrap_or(0),
        Err(err) => {
            tracing::error!("Error converting rows to arrow: {err:?}");
            0
        }
    }
}

impl ReplayEventRows {
    /// Reads the events of the replay, None if the replay events cannot be read.
    pub fn collect(
        source: &SC2ReplayBasicData,
        versioned_abilities: &MultiVersionedBalanceUnits,
    ) -> Option<Self> {
        let event_iterator = SC2EventIterator::new(source, versioned_abilities).ok()?;

        let ext_fs_id = event_iterator.sc2_state.details.ext_fs_id;
        let (
            stat_rows,
            upgrade_rows,
            unit_born_rows,
            unit_died_rows,
            target_point_rows,
            target_unit_rows,
        ) = event_iterator.into_iter().fold(
            (vec![], vec![], vec![], vec![], vec![], vec![]),
            |(
                mut stat_rows,
                mut upgrade_rows,
                mut unit_born_rows,
                mut unit_died_rows,
                mut target_point_rows,
                mut target_unit_rows,
            ),
             event_item| {
                match event_item.event_type {
                    SC2EventType::Tracker {
                        tracker_loop,
                        event,
                    } => match event {
                        ReplayTrackerEvent::PlayerStats(event) => {
                            stat_rows.push(PlayerStatsFlatRow::new(event, tracker_loop, ext_fs_id));
                        }
                        ReplayTrackerEvent::UnitBorn(event) => {
                            if let Some(new_row) = UnitBornEventFlatRow::from_unit_born(
                                event,
                                tracker_loop,
                                ext_fs_id,
                                event_item.change_hint,
                            ) {
                                unit_born_rows.push(new_row);
                            }
                        }
                        ReplayTrackerEvent::UnitDone(event) => {
                            if let Some(new_row) = UnitBornEventFlatRow::from_unit_done(
                                event,
                                tracker_loop,
                                ext_fs_id,
                                event_item.change_hint,
                            ) {
                                unit_born_rows.push(new_row);
                            }
                        }
                        ReplayTrackerEvent::UnitTypeChange(event) => match event_item.change_hint {
                            UnitChangeHint::None => {}
                            change_hint => {
                                if let Some(new_row) = UnitBornEventFlatRow::from_unit_type_change(
                                    event,
                                    tracker_loop,
                                    ext_fs_id,
                                    change_hint,
                                ) {
                                    unit_born_rows.push(new_row);
                                }
                            }
                        },
                        ReplayTrackerEvent::UnitDied(event) => {
                            if let Some(new_row) = UnitDiedEventFlatRow::new(
                                ext_fs_id,
                                event,
                                tracker_loop,
                                event_item.change_hint,
                            ) {
                                unit_died_rows.push(new_row);
                            }
                        }
                        ReplayTrackerEvent::Upgrade(event) => {
                            upgrade_rows.push(UpgradeEventFlatRow::new(
                                event,
                                tracker_loop,
                                ext_fs_id,
                            ));
                        }
                        _ => {}
                    },
                    SC2EventType::Game {
                        event: game_events::ReplayGameEvent::Cmd(event),
                        game_loop,
                        user_id,
                        player_name,
                    } => match event.m_data {
                        game_events::GameSCmdData::TargetPoint(_) => {
                            target_point_rows.append(
                                &mut game_events::CmdTargetPointEventFlatRow::new(
                                    ext_fs_id,
                                    event,
                                    game_loop,
                                    user_id,
                                    player_name,
                                    event_item.change_hint,
                                ),
                            );
                        }
                        game_events::GameSCmdData::TargetUnit(_) => {
                            target_unit_rows.append(
                                &mut game_events::CmdTargetUnitEventFlatRow::new(
                                    ext_fs_id,
                                    event,
                                    game_loop,
                                    user_id,
                                    player_name,
                                    event_item.change_hint,
                                ),
                            );
                        }
                        _ => {}
                    },
                    _ => {}
                };
                (
                    stat_rows,
                    upgrade_rows,
                    unit_born_rows,
                    unit_died_rows,
                    target_point_rows,
                    target_unit_rows,
                )
            },
        );
        Some(Self {
            stats: stat_rows,
            upgrades: upgrade_rows,
            unit_born: unit_born_rows,
            unit_died: unit_died_rows,
            target_points: target_point_rows,
            target_units: target_unit_rows,
        })
    }

    /// Writes the rows to the writers of the [`EVENT_ARROW_FNAMES`], returns the rows written
    /// to each.
    pub fn write(self, writers: &[ArrowMutexWriter]) -> [usize; 6] {
        [
            write_rows(&writers[0], self.stats.try_into_arrow()),
            write_rows(&writers[1], self.upgrades.try_into_arrow()),
            write_rows(&writers[2], self.unit_born.try_into_arrow()),
            write_rows(&writers[3], self.unit_died.try_into_arrow()),
            write_rows(&writers[4], self.target_points.try_into_arrow()),
            write_rows(&writers[5], self.target_units.try_into_arrow()),
        ]
    }
}

/// The details rows of the replays, with the map info digest from the map caches, it downloads
/// the caches that are missing.
pub async fn details_rows(
    sources: &[SC2ReplayBasicData],
    cache_path: &str,
) -> Vec<PlayerLobbyDetailsFlatRow> {
    // Identify from the cache handle bundles where the map info is located and get its sha256 digest for uniqueness.
    let cache_handle_to_map_info_digest =
        populate_map_info_digest_from_caches(sources, cache_path.to_string()).await;

    sources
        .par_iter()
        .map(|source| std::convert::Into::<Vec<PlayerLobbyDetails>>::into(source))
        .flatten()
        .map(|mut detail| {
            for cache_id in &detail.cache_handles {
                if let Some(Some(map_info_digest)) = cache_handle_to_map_info_digest.get(cache_id) {
                    detail.map_info_sha256 = map_info_digest.to_owned();
                    break;
                }
            }
            detail
        })
        .map(|d| d.into())
        .collect()
}

///  Create a subcommand that handles the max depth and max files to process
#[derive(Debug, Clone)]
pub struct WriteArrowIpcProps {
    /// Reads these many  files recursing, these files may or may not be valid.
    pub scan_max_files: usize,
    /// The maximum number of files to process
    pub process_max_files: usize,
    /// The maximum directory depth to traverse
    pub traverse_max_depth: usize,
    /// The minimum protocol version
    pub min_version: Option<u32>,
    /// The maximum protocol version
    pub max_version: Option<u32>,
    /// Deletes the snapshot and generates it from scratch, otherwise only the replays that are
    /// not in the snapshot yet are added.
    pub recreate: bool,
}

/// The supported Arrow IPC types
#[derive(Debug, Clone)]
pub enum ArrowIpcTypes {
    /// Writes the [`crate::init_data::UserInitDataFlatRow`] flat row to an Arrow IPC file
    UserInitData,
    /// Writes the [`crate::details::PlayerLobbyDetailsFlatRow`] flat row to an Arrow IPC file
    Details,
    /// Writes the [`crate::tracker_events::PlayerStatsEvent`] to an Arrow IPC file
    Stats,
    /// Writes the [`crate::tracker_events::UpgradeEvent`] to an Arrow IPC file
    Upgrades,
    /// Writes the [`crate::tracker_events::UnitBornEvent`] to an Arrow IPC file
    UnitBorn,
    /// Writes the [`crate::tracker_events::UnitDiedEvent`] to an Arrow IPC file
    UnitDied,
    /// Writes the [`crate::message_events::MessageEvent`] to an Arrow IPC file
    MessageEvents,
    /// Writes the [`crate::game_events::CmdTargetPointEventFlatRow`] to an Arrow IPC file
    CmdTargetPoint,
    /// Writes the [`crate::game_events::CmdTargetUnitEventFlatRow`] to an Arrow IPC file
    CmdTargetUnit,
    /// Writes all the implemented flat row types to Arrow IPC files inside the output directory
    All,
}

impl ArrowIpcTypes {
    /// Returns the schema for the chosen output type
    pub fn schema(&self) -> Schema {
        match self {
            Self::UserInitData => UserInitDataFlatRow::schema(),
            Self::Details => PlayerLobbyDetailsFlatRow::schema(),
            Self::Stats => PlayerStatsFlatRow::schema(),
            Self::Upgrades => UpgradeEventFlatRow::schema(),
            Self::UnitBorn => UnitBornEventFlatRow::schema(),
            Self::UnitDied => UnitDiedEventFlatRow::schema(),
            Self::CmdTargetPoint => CmdTargetPointEventFlatRow::schema(),
            Self::CmdTargetUnit => CmdTargetUnitEventFlatRow::schema(),
            _ => unimplemented!(),
        }
    }

    /// Writes a snapshot of the replay collection.
    /// A snapshot is a collection of generated files that work together.
    /// The consistency of the files is not yet implemented.
    /// But the files should have been generated at around the same time.
    /// If one file lags behind, it may be from an incomplete data generation.
    /// i.e. this function is called but it errors in the middle and no retries/fixes are done.
    /// Two things todo:
    /// First delete all the files in the snapshot directory.
    /// Add a snashopt generation timestamp and when reads are done, they are checked for
    /// very basic timestamp write consistency.
    #[tracing::instrument(level = "debug")]
    pub async fn handle_write_snapshot(
        sources: &[SC2ReplayBasicData],
        output: PathBuf,
        unit_abilities: &MultiVersionedBalanceUnits,
        disable_parallel_scans: bool,
        cache_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let init_time = std::time::Instant::now();
        if !output.is_dir() {
            panic!("Output must be a directory for types 'all'");
        }
        // output must be a directory, for this directory we will create the following files:
        // details.arrow
        // stats.arrow
        // upgrades.arrow
        // unit_born.arrow
        // unit_died.arrow
        // cmd_target_point.arrow
        // cmd_target_unit.arrow
        Self::Details
            .handle_details_ipc_cmd(sources, output.join(DETAILS_ARROW_FNAME), cache_path)
            .await?;
        println!(
            "handle_write_snapshot {}: Total time: {:?}",
            DETAILS_ARROW_FNAME,
            init_time.elapsed()
        );
        Self::Stats.handle_tracker_events(
            sources,
            output.join(STATS_ARROW_FNAME),
            unit_abilities,
            disable_parallel_scans,
        )?;
        println!(
            "handle_write_snapshot {}: Total time: {:?}",
            STATS_ARROW_FNAME,
            init_time.elapsed()
        );
        Self::Upgrades.handle_tracker_events(
            sources,
            output.join(UPGRADES_ARROW_FNAME),
            unit_abilities,
            disable_parallel_scans,
        )?;
        println!(
            "handle_write_snapshot {}: Total time: {:?}",
            UPGRADES_ARROW_FNAME,
            init_time.elapsed()
        );
        Self::UnitBorn.handle_tracker_events(
            sources,
            output.join(UNIT_BORN_ARROW_FNAME),
            unit_abilities,
            disable_parallel_scans,
        )?;
        println!(
            "handle_write_snapshot {}: Total time: {:?}",
            UNIT_BORN_ARROW_FNAME,
            init_time.elapsed()
        );
        Self::UnitDied.handle_tracker_events(
            sources,
            output.join(UNIT_DIED_ARROW_FNAME),
            unit_abilities,
            disable_parallel_scans,
        )?;
        println!(
            "handle_write_snapshot {}: Total time: {:?}",
            UNIT_DIED_ARROW_FNAME,
            init_time.elapsed()
        );
        Self::CmdTargetPoint.handle_game_events(
            sources,
            output.join(CMD_TARGET_POINT_ARROW_FNAME),
            unit_abilities,
            disable_parallel_scans,
        )?;
        println!(
            "handle_write_snapshot {}: Total time: {:?}",
            CMD_TARGET_POINT_ARROW_FNAME,
            init_time.elapsed()
        );
        Self::CmdTargetUnit.handle_game_events(
            sources,
            output.join("cmd_target_unit.arrow"),
            unit_abilities,
            disable_parallel_scans,
        )?;
        println!(
            "handle_write_snapshot: {}: Total time: {:?}",
            CMD_TARGET_UNIT_ARROW_FNAME,
            init_time.elapsed()
        );
        Ok(())
    }

    /// Creates a new Arrow IPC file with the tracker events data
    /// This seems to be small enough to not need to be chunked and is done in parallel
    /// This requires 1.5GB of RAM for 3600 files, so maybe not good for real players.
    #[tracing::instrument(level = "debug")]
    pub fn handle_tracker_events(
        &self,
        sources: &[SC2ReplayBasicData],
        output: PathBuf,
        versioned_abilities: &MultiVersionedBalanceUnits,
        disable_parallel_scans: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let init_time = std::time::Instant::now();
        tracing::info!("Processing TrackerEvents IPC write request: {:?}", self);
        let writer = open_arrow_mutex_writer(output, self.schema())?;

        let total_records = if disable_parallel_scans {
            sources
                .iter()
                .filter_map(|source| {
                    let event_iterator =
                        SC2EventIterator::new(&source, versioned_abilities).ok()?;
                    match self {
                        Self::Stats => {
                            let batch = event_iterator.collect_into_player_stats_flat_rows();
                            batch.try_into_arrow().ok()
                        }
                        Self::Upgrades => {
                            let batch = event_iterator.collect_into_upgrades_flat_rows();
                            batch.try_into_arrow().ok()
                        }
                        Self::UnitBorn => {
                            let batch = event_iterator.collect_into_unit_born_flat_rows();
                            batch.try_into_arrow().ok()
                        }
                        Self::UnitDied => {
                            let batch = event_iterator.collect_into_unit_died_flat_rows();
                            batch.try_into_arrow().ok()
                        }
                        _ => unimplemented!(),
                    }
                })
                .collect::<Vec<ArrayRef>>()
                .into_iter()
                .filter_map(|array_ref| {
                    if array_ref.len() == 0 {
                        None
                    } else {
                        write_to_arrow_mutex_writer(&writer, array_ref)
                    }
                })
                .sum::<usize>()
        } else {
            sources
                .par_iter()
                .filter_map(|source| {
                    let event_iterator =
                        SC2EventIterator::new(&source, versioned_abilities).ok()?;
                    match self {
                        Self::Stats => {
                            let batch = event_iterator.collect_into_player_stats_flat_rows();
                            batch.try_into_arrow().ok()
                        }
                        Self::Upgrades => {
                            let batch = event_iterator.collect_into_upgrades_flat_rows();
                            batch.try_into_arrow().ok()
                        }
                        Self::UnitBorn => {
                            let batch = event_iterator.collect_into_unit_born_flat_rows();
                            batch.try_into_arrow().ok()
                        }
                        Self::UnitDied => {
                            let batch = event_iterator.collect_into_unit_died_flat_rows();
                            batch.try_into_arrow().ok()
                        }
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<ArrayRef>>()
                .into_iter()
                .filter_map(|array_ref| {
                    if array_ref.len() == 0 {
                        None
                    } else {
                        write_to_arrow_mutex_writer(&writer, array_ref)
                    }
                })
                .sum::<usize>()
        };
        tracing::info!("Loaded {} records", total_records);
        println!(
            "handle_tracker_events: {}. records Total time: {:?}",
            total_records,
            init_time.elapsed(),
        );
        let res = close_arrow_mutex_writer(writer);
        println!(
            "handle_tracker_events: {}. records Total time: {:?}. after close mutex",
            total_records,
            init_time.elapsed(),
        );
        res
    }

    /// Creates a new Arrow IPC file with the tracker events data
    /// This seems to be small enough to not need to be chunked and is done in parallel
    /// This requires 1.5GB of RAM for 3600 files, so maybe not good for real players.
    #[tracing::instrument(level = "debug")]
    pub async fn write_all_tracker_events(
        sources: &[SC2ReplayBasicData],
        output: PathBuf,
        versioned_abilities: &MultiVersionedBalanceUnits,
        disable_parallel_scans: bool,
        cache_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Self::write_all_tracker_events_with_progress(
            sources,
            output,
            versioned_abilities,
            disable_parallel_scans,
            cache_path,
            None,
        )
        .await?;
        Ok(())
    }

    /// Same as [`Self::write_all_tracker_events`], sending the progress to `progress`.
    /// Returns the files written with their sizes and records.
    #[tracing::instrument(level = "debug", skip(progress))]
    pub async fn write_all_tracker_events_with_progress(
        sources: &[SC2ReplayBasicData],
        output: PathBuf,
        versioned_abilities: &MultiVersionedBalanceUnits,
        disable_parallel_scans: bool,
        cache_path: &str,
        progress: Option<&SnapshotProgressSender>,
    ) -> Result<Vec<SnapshotFile>, Box<dyn std::error::Error>> {
        let init_time = std::time::Instant::now();
        if !output.is_dir() {
            return Err(format!("Output {} must be a directory", output.display()).into());
        }
        tracing::info!("Processing TrackerEvents IPC write all request");
        send_progress(
            progress,
            SnapshotProgress::Step {
                index: 3,
                total: SNAPSHOT_STEPS,
                name: format!("Reading the map caches and writing {DETAILS_ARROW_FNAME}"),
            },
        );
        let details_records = Self::Details
            .write_details_ipc(sources, output.join(DETAILS_ARROW_FNAME), cache_path)
            .await?;
        let mut files = vec![snapshot_file(&output, DETAILS_ARROW_FNAME, details_records)];
        send_progress(progress, SnapshotProgress::FileWritten(files[0].clone()));
        println!(
            "handle_write_snapshot {}: Total time: {:?}",
            DETAILS_ARROW_FNAME,
            init_time.elapsed()
        );
        send_progress(
            progress,
            SnapshotProgress::Step {
                index: 4,
                total: SNAPSHOT_STEPS,
                name: "Reading the replay events and writing the event files".to_string(),
            },
        );
        let processed = AtomicUsize::new(0);
        let total_sources = sources.len();
        // Send an update for every percent of the replays processed.
        let progress_every = (total_sources / 100).max(1);
        let counts: [AtomicUsize; 6] = Default::default();
        let event_writers = EVENT_ARROW_FNAMES
            .into_iter()
            .zip(event_schemas())
            .map(|(name, schema)| open_arrow_mutex_writer(output.join(name), schema))
            .collect::<Result<Vec<_>, _>>()?;

        let total_records = sources
            .par_iter()
            .filter_map(|source| {
                let done = processed.fetch_add(1, Ordering::Relaxed) + 1;
                if done % progress_every == 0 || done == total_sources {
                    send_progress(
                        progress,
                        SnapshotProgress::ReplaysProcessed {
                            done,
                            total: total_sources,
                        },
                    );
                }
                let rows = ReplayEventRows::collect(source, versioned_abilities)?;
                let file_counts = rows.write(&event_writers);
                for (count, file_count) in counts.iter().zip(file_counts) {
                    count.fetch_add(file_count, Ordering::Relaxed);
                }
                Some(file_counts.iter().sum::<usize>())
            })
            .sum::<usize>();
        for ((name, writer), count) in EVENT_ARROW_FNAMES.iter().zip(event_writers).zip(&counts) {
            close_arrow_mutex_writer(writer)?;
            let file = snapshot_file(&output, name, count.load(Ordering::Relaxed));
            send_progress(progress, SnapshotProgress::FileWritten(file.clone()));
            files.push(file);
        }

        println!(
            "write_all_tracker_events completed after {:?} Total records: {}",
            init_time.elapsed(),
            total_records
        );
        Ok(files)
    }

    /// Creates a new Arrow IPC file with the game events data
    /// This requires 1.5GB of RAM for 3600 files, so maybe not good for real players.
    #[tracing::instrument(level = "debug")]
    pub fn handle_game_events(
        &self,
        sources: &[SC2ReplayBasicData],
        output: PathBuf,
        versioned_abilities: &MultiVersionedBalanceUnits,
        disable_parallel_scans: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let init_time = std::time::Instant::now();
        tracing::info!("Processing GameEvents IPC write request: {:?}", self);
        let writer = open_arrow_mutex_writer(output, self.schema())?;

        let total_records = if disable_parallel_scans {
            sources
                .iter()
                .filter_map(|source| {
                    let event_iterator =
                        SC2EventIterator::new(&source, versioned_abilities).ok()?;
                    match self {
                        Self::CmdTargetPoint => {
                            let batch =
                                event_iterator.collect_into_game_cmd_target_points_flat_rows();
                            batch.try_into_arrow().ok()
                        }
                        Self::CmdTargetUnit => {
                            let batch =
                                event_iterator.collect_into_game_cmd_target_units_flat_rows();
                            batch.try_into_arrow().ok()
                        }
                        e => unimplemented!("{:?}", e),
                    }
                })
                .collect::<Vec<ArrayRef>>()
                .into_iter()
                .filter_map(|array_ref| {
                    if array_ref.len() == 0 {
                        None
                    } else {
                        write_to_arrow_mutex_writer(&writer, array_ref)
                    }
                })
                .sum::<usize>()
        } else {
            sources
                .par_iter()
                .filter_map(|source| {
                    let event_iterator =
                        SC2EventIterator::new(&source, versioned_abilities).ok()?;
                    match self {
                        Self::CmdTargetPoint => {
                            let batch =
                                event_iterator.collect_into_game_cmd_target_points_flat_rows();
                            batch.try_into_arrow().ok()
                        }
                        Self::CmdTargetUnit => {
                            let batch =
                                event_iterator.collect_into_game_cmd_target_units_flat_rows();
                            batch.try_into_arrow().ok()
                        }
                        e => unimplemented!("{:?}", e),
                    }
                })
                .collect::<Vec<ArrayRef>>()
                .into_iter()
                .filter_map(|array_ref| {
                    if array_ref.len() == 0 {
                        None
                    } else {
                        write_to_arrow_mutex_writer(&writer, array_ref)
                    }
                })
                .sum::<usize>()
        };
        tracing::info!("Loaded {} records", total_records);
        println!("handle_game_events: Total time: {:?}", init_time.elapsed());
        close_arrow_mutex_writer(writer)
    }

    /// Creates a new Arrow IPC file with the details data
    #[tracing::instrument(level = "debug")]
    pub fn handle_read_once_write_all(
        &self,
        sources: &[SC2ReplayBasicData],
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Processing Read Once Write All IPC request");
        // process the sources in parallel consuming into the batch variable

        let details_flaw_rows: Vec<PlayerLobbyDetailsFlatRow> = sources
            .iter()
            .flat_map(|source| {
                std::convert::Into::<Vec<PlayerLobbyDetails>>::into(source)
                    .into_iter()
                    .map(|d| d.into())
                    .collect::<Vec<PlayerLobbyDetailsFlatRow>>()
            })
            .collect();
        let res: ArrayRef = details_flaw_rows.try_into_arrow()?;
        let chunk: RecordBatch = res
            .as_any()
            .downcast_ref::<arrow::array::StructArray>()
            .unwrap()
            .into();

        write_batches(output, Self::Details.schema(), chunk)?;
        Ok(())
    }
    /// Creates a new Arrow IPC file with the details data
    #[tracing::instrument(level = "debug")]
    pub async fn handle_details_ipc_cmd(
        &self,
        sources: &[SC2ReplayBasicData],
        output: PathBuf,
        cache_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.write_details_ipc(sources, output, cache_path).await?;
        Ok(())
    }

    /// Writes the details Arrow IPC file, returns the records written.
    async fn write_details_ipc(
        &self,
        sources: &[SC2ReplayBasicData],
        output: PathBuf,
        cache_path: &str,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        tracing::info!("Processing Details IPC write request");

        let details_flaw_rows = details_rows(sources, cache_path).await;
        let records = details_flaw_rows.len();
        let res: ArrayRef = details_flaw_rows.try_into_arrow()?;
        let chunk: RecordBatch = res
            .as_any()
            .downcast_ref::<arrow::array::StructArray>()
            .unwrap()
            .into();

        write_batches(output, Self::Details.schema(), chunk)?;
        Ok(records)
    }

    /// Handles the Arrow IPC command variants
    #[tracing::instrument(level = "debug")]
    pub async fn handle_arrow_ipc_cmd(
        source: PathBuf,
        output: PathBuf,
        cmd: &WriteArrowIpcProps,
        unit_abilities: &MultiVersionedBalanceUnits,
        disable_parallel_scans: bool,
        cache_path: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Self::handle_arrow_ipc_cmd_with_progress(
            source,
            output,
            cmd,
            unit_abilities,
            disable_parallel_scans,
            cache_path,
            None,
        )
        .await?;
        Ok(())
    }

    /// Same as [`Self::handle_arrow_ipc_cmd`], sending the progress of the snapshot generation
    /// to `progress`.
    ///
    /// The snapshot is partitioned by day, see [`partitioned`], only the replays that are not in
    /// the snapshot yet are processed, unless [`WriteArrowIpcProps::recreate`] is set. A snapshot
    /// written before the partitions, with the files at its root, is regenerated.
    /// Writes the [`SnapshotMetadata`] of every day changed and of the whole snapshot at its
    /// root, and returns the latter.
    #[tracing::instrument(level = "debug", skip(progress))]
    pub async fn handle_arrow_ipc_cmd_with_progress(
        source: PathBuf,
        output: PathBuf,
        cmd: &WriteArrowIpcProps,
        unit_abilities: &MultiVersionedBalanceUnits,
        disable_parallel_scans: bool,
        cache_path: String,
        progress: Option<SnapshotProgressSender>,
    ) -> Result<SnapshotMetadata, Box<dyn std::error::Error>> {
        let init_time = std::time::Instant::now();
        let progress = progress.as_ref();
        println!(
            "Processing Arrow write request with scan_max_files: {}, traverse_max_depth: {}, process_max_files: {}, min_version: {:?}, max_version: {:?}, recreate: {}",
            cmd.scan_max_files,
            cmd.traverse_max_depth,
            cmd.process_max_files,
            cmd.min_version,
            cmd.max_version,
            cmd.recreate,
        );
        if !output.is_dir() {
            return Err(format!("Output {} must be a directory", output.display()).into());
        }
        if cmd.recreate || has_flat_layout(&output) {
            println!("Removing the snapshot at {}", output.display());
            remove_snapshot(&output)?;
        }
        send_progress(
            progress,
            SnapshotProgress::Step {
                index: 1,
                total: SNAPSHOT_STEPS,
                name: format!("Locating the replays in {}", source.display()),
            },
        );
        let sources = get_matching_files(source, cmd.scan_max_files, cmd.traverse_max_depth)?;
        println!("Located {} matching files by extension", sources.len());
        send_progress(
            progress,
            SnapshotProgress::FilesFound {
                count: sources.len(),
            },
        );
        send_progress(
            progress,
            SnapshotProgress::Step {
                index: 2,
                total: SNAPSHOT_STEPS,
                name: "Reading the replay headers of the replays not in the snapshot".to_string(),
            },
        );
        // The replays parsed by another version of s2protocol are parsed again.
        let outdated = remove_outdated_partitions(&output)?;
        if !outdated.is_empty() {
            println!(
                "Removed {} days written by another version of s2protocol, their replays are \
                 parsed again",
                outdated.len()
            );
        }
        let index = SnapshotIndex::read(&output)?;
        let located = sources.len();
        // The file names are compared first, so the known replays are not read at all.
        let sources: Vec<PathBuf> = sources
            .into_iter()
            .filter(|path| !index.file_names.contains(path.to_str().unwrap_or_default()))
            .collect();
        let already_in_snapshot = located - sources.len();
        let to_read = sources.len();
        let first_id = index.next_ext_fs_id();
        let read_header = |(idx, source): (usize, &PathBuf)| {
            SC2ReplayBasicData::new(source, first_id + u64::try_from(idx).unwrap()).ok()
        };
        let sources: Vec<SC2ReplayBasicData> = if disable_parallel_scans {
            tracing::debug!("Working serially");
            sources.iter().enumerate().filter_map(read_header).collect()
        } else {
            tracing::debug!("Working in parallel");
            sources
                .par_iter()
                .enumerate()
                .filter_map(read_header)
                .collect()
        };
        let unreadable = to_read - sources.len();
        // A replay copied to another directory is in the snapshot with another file name, or is
        // read twice in this run.
        let mut sha256s = index.sha256s;
        let mut copies = 0;
        let sources: Vec<SC2ReplayBasicData> = sources
            .into_iter()
            .filter(|source| {
                let is_new = sha256s.insert(source.init_data.ext_fs_sha256.clone());
                copies += usize::from(!is_new);
                is_new
            })
            .filter(|source| {
                if let Some(min_version) = cmd.min_version
                    && source.init_data.version < min_version
                {
                    return false;
                }
                if let Some(max_version) = cmd.max_version
                    && source.init_data.version > max_version
                {
                    return false;
                }
                true
            })
            .take(cmd.process_max_files)
            .collect();
        println!(
            "{already_in_snapshot} replays are already in the snapshot, {copies} are copies of \
             another replay, {unreadable} could not be read, {} new replays have valid init data, \
             processing...",
            sources.len()
        );
        let already_in_snapshot = already_in_snapshot + copies;
        send_progress(
            progress,
            SnapshotProgress::AlreadyInSnapshot {
                count: already_in_snapshot,
            },
        );
        send_progress(
            progress,
            SnapshotProgress::ValidReplays {
                count: sources.len(),
            },
        );
        if sources.is_empty() && index.max_ext_fs_id.is_none() {
            return Err("No replays with valid init data found".into());
        }
        send_progress(
            progress,
            SnapshotProgress::Step {
                index: 3,
                total: SNAPSHOT_STEPS,
                name: "Reading the map caches and the replay details".to_string(),
            },
        );
        let details = details_rows(&sources, &cache_path).await;
        // The snapshot knows its replays from the details, a replay without player details would
        // be added again, with all its events, every time.
        let with_details: HashSet<u64> = details.iter().map(|row| row.ext_fs_id).collect();
        let (sources, without_details): (Vec<SC2ReplayBasicData>, Vec<SC2ReplayBasicData>) =
            sources
                .into_iter()
                .partition(|source| with_details.contains(&source.details.ext_fs_id));
        if !without_details.is_empty() {
            println!(
                "{} replays are skipped, their players could not be matched to the lobby slots, i.e. {}",
                without_details.len(),
                without_details
                    .iter()
                    .take(3)
                    .map(|source| source.init_data.ext_fs_file_name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        send_progress(
            progress,
            SnapshotProgress::Step {
                index: 4,
                total: SNAPSHOT_STEPS,
                name: "Reading the replay events and writing the days they were played".to_string(),
            },
        );
        let days = write_partitions(&output, &sources, details, unit_abilities, progress)?;
        println!(
            "Wrote {} replays into {} days after {:?}",
            sources.len(),
            days.len(),
            init_time.elapsed()
        );
        send_progress(
            progress,
            SnapshotProgress::Step {
                index: 5,
                total: SNAPSHOT_STEPS,
                name: format!("Writing {SNAPSHOT_METADATA_FNAME}"),
            },
        );
        let elapsed_ms = u64::try_from(init_time.elapsed().as_millis()).unwrap_or(u64::MAX);
        let metadata = root_metadata(&output, sources.len(), elapsed_ms)?;
        metadata.write_to_dir(&output)?;
        for file in &metadata.files {
            send_progress(progress, SnapshotProgress::FileWritten(file.clone()));
        }
        send_progress(progress, SnapshotProgress::Done { elapsed_ms });
        Ok(metadata)
    }
}
