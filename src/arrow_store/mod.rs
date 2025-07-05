//! Arrow Specific handling of data.

#[cfg(feature = "dep_arrow")]
use arrow::{
    array::Array, array::ArrayRef, datatypes::DataType, datatypes::Schema,
    record_batch::RecordBatch,
};
#[cfg(feature = "dep_arrow")]
use arrow_convert::{field::ArrowField, serialize::TryIntoArrow};
use init_data::InitData;
#[cfg(feature = "dep_arrow")]
use rayon::prelude::*;

use crate::cli::get_matching_files;
use crate::details::Details;
use crate::details::PlayerDetailsFlatRow;
use crate::game_events::GameEventIterator;
use crate::tracker_events::{self, TrackerEventIterator};
use crate::*;
use clap::Subcommand;
use std::path::PathBuf;
pub mod ipc_writer;
use ipc_writer::*;
use std::collections::HashSet;

/// The supported Arrow IPC types
#[derive(Debug, Subcommand, Clone)]
pub enum ArrowIpcTypes {
    /// Writes the [`crate::init_data::InitData`] flat row to an Arrow IPC file
    InitData,
    /// Writes the [`crate::details::PlayerDetailsFlatRow`] flat row to an Arrow IPC file
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
            Self::InitData => {
                if let DataType::Struct(fields) = init_data::UserInitDataFlatRow::data_type() {
                    Schema::new(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::Details => {
                if let DataType::Struct(fields) = details::PlayerDetailsFlatRow::data_type() {
                    Schema::new(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::Stats => {
                if let DataType::Struct(fields) = tracker_events::PlayerStatsFlatRow::data_type() {
                    Schema::new(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::Upgrades => {
                if let DataType::Struct(fields) = tracker_events::UpgradeEventFlatRow::data_type() {
                    Schema::new(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::UnitBorn => {
                if let DataType::Struct(fields) = tracker_events::UnitBornEventFlatRow::data_type()
                {
                    Schema::new(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::UnitDied => {
                if let DataType::Struct(fields) = tracker_events::UnitDiedEventFlatRow::data_type()
                {
                    Schema::new(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::CmdTargetPoint => {
                if let DataType::Struct(fields) =
                    game_events::CmdTargetPointEventFlatRow::data_type()
                {
                    Schema::new(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::CmdTargetUnit => {
                if let DataType::Struct(fields) =
                    game_events::CmdTargetUnitEventFlatRow::data_type()
                {
                    Schema::new(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
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
    /// very basic timepstamp write consistency.
    #[tracing::instrument(level = "debug")]
    pub fn handle_write_snapshot(
        sources: Vec<InitData>,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !output.is_dir() {
            panic!("Output must be a directory for types 'all'");
        }
        // output must be a directory, for this directory we will create the following files:
        // init_data.ipc
        // details.ipc
        // stats.ipc
        // upgrades.ipc
        // unit_born.ipc
        Self::InitData.handle_init_data_ipc_cmd(sources.clone(), output.join("init_data.ipc"))?;
        Self::Details.handle_details_ipc_cmd(sources.clone(), output.join("details.ipc"))?;
        Self::Stats.handle_tracker_events(sources.clone(), output.join("stats.ipc"))?;
        Self::Upgrades.handle_tracker_events(sources.clone(), output.join("upgrades.ipc"))?;
        Self::UnitBorn.handle_tracker_events(sources.clone(), output.join("unit_born.ipc"))?;
        Self::UnitDied.handle_tracker_events(sources.clone(), output.join("unit_died.ipc"))?;
        Self::CmdTargetPoint
            .handle_game_events(sources.clone(), output.join("cmd_target_point.ipc"))?;
        Self::CmdTargetUnit
            .handle_game_events(sources.clone(), output.join("cmd_target_unit.ipc"))?;
        Ok(())
    }

    #[tracing::instrument(level = "debug")]
    pub fn handle_init_data_ipc_cmd(
        &self,
        sources: Vec<InitData>,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Processing InitData IPC write request");

        let init_data_flaw_rows: Vec<init_data::UserInitDataFlatRow> = sources
            .iter()
            .flat_map(|source| {
                let res: Vec<init_data::UserInitDataFlatRow> = source.into();
                res
            })
            .collect();
        let res: ArrayRef = init_data_flaw_rows.try_into_arrow()?;
        let chunk: RecordBatch = res
            .as_any()
            .downcast_ref::<arrow::array::StructArray>()
            .unwrap()
            .into();

        write_batches(output, Self::InitData.schema(), chunk)?;
        Ok(())
    }

    /// Creates a new Arrow IPC file with the tracker events data
    /// This seems to be small enough to not need to be chunked and is done in parallel
    /// This requires 1.5GB of RAM for 3600 files, so maybe not good for real players.
    #[tracing::instrument(level = "debug")]
    pub fn handle_tracker_events(
        &self,
        sources: Vec<InitData>,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Processing TrackerEvents IPC write request: {:?}", self);
        let writer = open_arrow_mutex_writer(output, self.schema())?;

        // process files in parallel, the internal iterators will fight for the lock
        let total_records = sources
            .par_iter()
            .filter_map(|source| {
                let details: Details = Details::try_from(source).ok()?;
                let source_path = PathBuf::from(&source.file_name);
                let tracker_events = TrackerEventIterator::new(&source_path).ok()?;
                let (res, batch_len): (ArrayRef, usize) = match self {
                    Self::Stats => {
                        let batch = tracker_events.collect_into_player_stats_flat_rows(&details);
                        (batch.try_into_arrow().ok()?, batch.len())
                    }
                    Self::Upgrades => {
                        let batch = tracker_events.collect_into_upgrades_flat_rows(&details);
                        (batch.try_into_arrow().ok()?, batch.len())
                    }
                    Self::UnitBorn => {
                        let batch = tracker_events.collect_into_unit_born_flat_rows(&details);
                        (batch.try_into_arrow().ok()?, batch.len())
                    }
                    Self::UnitDied => {
                        let batch = tracker_events.collect_into_unit_died_flat_rows(&details);
                        (batch.try_into_arrow().ok()?, batch.len())
                    }
                    _ => unimplemented!(),
                };
                write_to_arrow_mutex_writer(&writer, res, batch_len)
            })
            .sum::<usize>();
        tracing::info!("Loaded {} records", total_records);
        close_arrow_mutex_writer(writer)
    }

    /// Creates a new Arrow IPC file with the game events data
    /// This requires 1.5GB of RAM for 3600 files, so maybe not good for real players.
    #[tracing::instrument(level = "debug")]
    pub fn handle_game_events(
        &self,
        sources: Vec<InitData>,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Processing GameEvents IPC write request: {:?}", self);
        let writer = open_arrow_mutex_writer(output, self.schema())?;

        // process files in parallel, the internal iterators will fight for the lock
        let total_records = sources
            .par_iter()
            .filter_map(|source| {
                let details = Details::try_from(source).ok()?;
                let source_path = PathBuf::from(&source.file_name);
                let game_events = GameEventIterator::new(&source_path).ok()?;
                let (res, batch_len): (ArrayRef, usize) = match self {
                    Self::CmdTargetPoint => {
                        let batch =
                            game_events.collect_into_game_cmd_target_points_flat_rows(&details);
                        (batch.try_into_arrow().ok()?, batch.len())
                    }
                    Self::CmdTargetUnit => {
                        let batch =
                            game_events.collect_into_game_cmd_target_units_flat_rows(&details);
                        (batch.try_into_arrow().ok()?, batch.len())
                    }
                    e => unimplemented!("{:?}", e),
                };
                write_to_arrow_mutex_writer(&writer, res, batch_len)
            })
            .sum::<usize>();
        tracing::info!("Loaded {} records", total_records);
        close_arrow_mutex_writer(writer)
    }

    /// Creates a new Arrow IPC file with the details data
    #[tracing::instrument(level = "debug")]
    pub fn handle_details_ipc_cmd(
        &self,
        sources: Vec<InitData>,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Processing Details IPC write request");
        // process the sources in parallel consuming into the batch variable

        let details_flaw_rows: Vec<PlayerDetailsFlatRow> = sources
            .iter()
            .flat_map(|source| {
                let res: Vec<PlayerDetailsFlatRow> = match Details::try_from(source) {
                    Ok(details) => details.into(),
                    Err(err) => {
                        tracing::error!("Error reading details: {:?}", err);
                        return vec![];
                    }
                };
                res
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

    /// Handles the Arrow IPC command variants
    #[tracing::instrument(level = "debug")]
    pub fn handle_arrow_ipc_cmd(
        source: PathBuf,
        output: PathBuf,
        cmd: &WriteArrowIpcProps,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!(
            "Processing Arrow write request with scan_max_files: {}, traverse_max_depth: {}, process_max_files: {}, min_version: {:?}, max_version: {:?}",
            cmd.scan_max_files,
            cmd.process_max_files,
            cmd.traverse_max_depth,
            cmd.min_version,
            cmd.max_version
        );
        let sources = get_matching_files(source, cmd.scan_max_files, cmd.traverse_max_depth)?;
        println!("Located {} matching files by extension", sources.len());
        let mut sources: Vec<InitData> = sources
            .par_iter()
            .enumerate()
            .filter_map(|(idx, source)| {
                InitData::try_from((source.clone(), u64::try_from(idx).unwrap())).ok()
            })
            .collect::<Vec<InitData>>()
            .into_iter()
            .filter(|source| {
                if let Some(min_version) = cmd.min_version {
                    if source.version < min_version {
                        return false;
                    }
                }
                if let Some(max_version) = cmd.max_version {
                    if source.version > max_version {
                        return false;
                    }
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
        // Identify the shortest unique sha256 hash fragment.
        let mut smallest_fragment = 1;
        while smallest_fragment < 64 {
            let mut hash_set = HashSet::new();
            for source in sources.iter() {
                let hash = source.sha256.clone();
                hash_set.insert(hash[..smallest_fragment].to_string());
            }
            if hash_set.len() == sources.len() {
                break;
            }
            smallest_fragment += 1;
        }
        for source in sources.iter_mut() {
            source.trim_sha(smallest_fragment)
        }
        tracing::info!(
            "Found {} readable files, sha256 fragment size: {}",
            sources.len(),
            smallest_fragment
        );
        Self::handle_write_snapshot(sources, output)
    }
}
