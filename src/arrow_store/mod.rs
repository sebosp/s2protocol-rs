//! Arrow Specific handling of data.

#[cfg(feature = "dep_arrow")]
use arrow::{array::Array, array::ArrayRef, datatypes::Schema, record_batch::RecordBatch};
#[cfg(feature = "dep_arrow")]
use arrow_convert::serialize::TryIntoArrow;
#[cfg(feature = "dep_arrow")]
use rayon::prelude::*;

use crate::basic_replay_data::SC2ReplayBasicData;
use crate::cache_handles::populate_map_info_digest_from_caches;
use crate::get_matching_files;

use crate::details::{PlayerLobbyDetails, PlayerLobbyDetailsFlatRow};
use crate::game_events::{
    CmdTargetPointEventFlatRow, CmdTargetUnitEventFlatRow, MultiVersionedBalanceUnits,
};
use crate::tracker_events::{
    PlayerStatsFlatRow, UnitBornEventFlatRow, UnitDiedEventFlatRow, UpgradeEventFlatRow,
};
use crate::*;

use std::path::PathBuf;
pub mod ipc_writer;
use ipc_writer::*;

pub const DETAILS_ARROW_FNAME: &'static str = "details.arrow";
pub const STATS_ARROW_FNAME: &'static str = "stats.arrow";
pub const UPGRADES_ARROW_FNAME: &'static str = "upgrades.arrow";
pub const UNIT_BORN_ARROW_FNAME: &'static str = "unit_born.arrow";
pub const UNIT_DIED_ARROW_FNAME: &'static str = "unit_died.arrow";
pub const CMD_TARGET_POINT_ARROW_FNAME: &'static str = "cmd_target_point.arrow";
pub const CMD_TARGET_UNIT_ARROW_FNAME: &'static str = "cmd_target_unit.arrow";

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
        let init_time = std::time::Instant::now();
        if !output.is_dir() {
            panic!("Output must be a directory for types 'all'");
        }
        tracing::info!("Processing TrackerEvents IPC write all request");
        Self::Details
            .handle_details_ipc_cmd(sources, output.join(DETAILS_ARROW_FNAME), cache_path)
            .await?;
        println!(
            "handle_write_snapshot {}: Total time: {:?}",
            DETAILS_ARROW_FNAME,
            init_time.elapsed()
        );
        let stat_writer =
            open_arrow_mutex_writer(output.join(STATS_ARROW_FNAME), PlayerStatsFlatRow::schema())?;
        let upgrades_writer = open_arrow_mutex_writer(
            output.join(UPGRADES_ARROW_FNAME),
            PlayerStatsFlatRow::schema(),
        )?;
        let unit_born_writer = open_arrow_mutex_writer(
            output.join(UNIT_BORN_ARROW_FNAME),
            PlayerStatsFlatRow::schema(),
        )?;
        let unit_died_writer = open_arrow_mutex_writer(
            output.join(UNIT_DIED_ARROW_FNAME),
            PlayerStatsFlatRow::schema(),
        )?;
        let target_point_writer = open_arrow_mutex_writer(
            output.join(CMD_TARGET_POINT_ARROW_FNAME),
            PlayerStatsFlatRow::schema(),
        )?;
        let target_unit_writer = open_arrow_mutex_writer(
            output.join(CMD_TARGET_UNIT_ARROW_FNAME),
            PlayerStatsFlatRow::schema(),
        )?;

        let total_records = sources
            .par_iter()
            .filter_map(|source| {
                let event_iterator = SC2EventIterator::new(&source, versioned_abilities).ok()?;

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
                                    stat_rows.push(PlayerStatsFlatRow::new(
                                        event,
                                        tracker_loop,
                                        ext_fs_id,
                                    ));
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
                                ReplayTrackerEvent::UnitTypeChange(event) => {
                                    match event_item.change_hint {
                                        UnitChangeHint::None => {}
                                        change_hint => {
                                            if let Some(new_row) =
                                                UnitBornEventFlatRow::from_unit_type_change(
                                                    event,
                                                    tracker_loop,
                                                    ext_fs_id,
                                                    change_hint,
                                                )
                                            {
                                                unit_born_rows.push(new_row);
                                            }
                                        }
                                    }
                                }
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
                let stat_count =
                    write_to_arrow_mutex_writer(&stat_writer, stat_rows.try_into_arrow().unwrap())
                        .unwrap_or(0usize);
                let upgrade_count = write_to_arrow_mutex_writer(
                    &upgrades_writer,
                    upgrade_rows.try_into_arrow().unwrap(),
                )
                .unwrap_or(0usize);
                let unit_born_count = write_to_arrow_mutex_writer(
                    &unit_born_writer,
                    unit_born_rows.try_into_arrow().unwrap(),
                )
                .unwrap_or(0usize);
                let unit_died_rows = write_to_arrow_mutex_writer(
                    &unit_died_writer,
                    unit_died_rows.try_into_arrow().unwrap(),
                )
                .unwrap_or(0usize);
                let target_point_count = write_to_arrow_mutex_writer(
                    &target_point_writer,
                    target_point_rows.try_into_arrow().unwrap(),
                )
                .unwrap_or(0usize);
                let target_unit_count = write_to_arrow_mutex_writer(
                    &target_unit_writer,
                    target_unit_rows.try_into_arrow().unwrap(),
                )
                .unwrap_or(0usize);
                Some(
                    stat_count
                        + upgrade_count
                        + unit_born_count
                        + unit_died_rows
                        + target_point_count
                        + target_unit_count,
                )
            })
            .sum::<usize>();
        close_arrow_mutex_writer(stat_writer)?;
        close_arrow_mutex_writer(upgrades_writer)?;
        close_arrow_mutex_writer(unit_born_writer)?;
        close_arrow_mutex_writer(unit_died_writer)?;
        close_arrow_mutex_writer(target_point_writer)?;
        close_arrow_mutex_writer(target_unit_writer)?;

        println!(
            "write_all_tracker_events completed after {:?} Total records: {}",
            init_time.elapsed(),
            total_records
        );
        Ok(())
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
        tracing::info!("Processing Details IPC write request");

        // Identify from the cache handle bundles where the map info is located and get its sha256 digest for uniqueness.
        let cache_handle_to_map_info_digest =
            populate_map_info_digest_from_caches(&sources, cache_path.to_string()).await;

        let details_flaw_rows: Vec<PlayerLobbyDetailsFlatRow> = sources
            .par_iter()
            .map(|source| std::convert::Into::<Vec<PlayerLobbyDetails>>::into(source))
            .flatten()
            .map(|mut detail| {
                for cache_id in &detail.cache_handles {
                    if let Some(Some(map_info_digest)) =
                        cache_handle_to_map_info_digest.get(cache_id)
                    {
                        detail.map_info_sha256 = map_info_digest.to_owned();
                        break;
                    }
                }
                detail
            })
            .map(|d| d.into())
            .collect::<Vec<PlayerLobbyDetailsFlatRow>>();
        let res: ArrayRef = details_flaw_rows.try_into_arrow()?;
        let chunk: RecordBatch = res
            .as_any()
            .downcast_ref::<arrow::array::StructArray>()
            .unwrap()
            .into();

        write_batches(output, Self::Details.schema(), chunk)?;
        Ok(())
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
        println!(
            "Processing Arrow write request with scan_max_files: {}, traverse_max_depth: {}, process_max_files: {}, min_version: {:?}, max_version: {:?}",
            cmd.scan_max_files,
            cmd.traverse_max_depth,
            cmd.process_max_files,
            cmd.min_version,
            cmd.max_version
        );
        let sources = get_matching_files(source, cmd.scan_max_files, cmd.traverse_max_depth)?;
        println!("Located {} matching files by extension", sources.len());
        let sources: Vec<SC2ReplayBasicData> = if disable_parallel_scans {
            tracing::debug!("Working serially");
            sources
                .iter()
                .enumerate()
                .filter_map(|(idx, source)| {
                    SC2ReplayBasicData::new(source, u64::try_from(idx).unwrap()).ok()
                })
                .collect::<Vec<SC2ReplayBasicData>>()
        } else {
            tracing::debug!("Working in parallel");
            sources
                .par_iter()
                .enumerate()
                .filter_map(|(idx, source)| {
                    SC2ReplayBasicData::new(source, u64::try_from(idx).unwrap()).ok()
                })
                .collect::<Vec<SC2ReplayBasicData>>()
        };
        let sources: Vec<SC2ReplayBasicData> = sources
            .into_iter()
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
        if sources.is_empty() {
            panic!("No files found");
        } else {
            println!(
                "{} files have valid init data, processing...",
                sources.len()
            );
        }
        Self::write_all_tracker_events(
            &sources,
            output,
            unit_abilities,
            disable_parallel_scans,
            &cache_path,
        )
        .await
    }
}
