//! Arrow Specific handling of data.

#[cfg(feature = "dep_arrow")]
use ::arrow::{array::Array, datatypes::DataType, datatypes::Schema, record_batch::RecordBatch};
#[cfg(feature = "dep_arrow")]
use arrow_convert::{field::ArrowField, serialize::TryIntoArrow};
use init_data::InitData;
#[cfg(feature = "dep_arrow")]
use rayon::prelude::*;

use crate::cli::get_matching_files;
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
    /// Writes the [`crate::details::Details`] flat row to an Arrow IPC file
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
    /// Writes the [`crate::game_events::Cmd::TargetPoint`] to an Arrow IPC file
    CmdTargetPoint,
    /// Writes the [`crate::game_events::Cmd::TargetUnit`] to an Arrow IPC file
    CmdTargetUnit,
    /// Writes all the implemented flat row types to Arrow IPC files inside the output directory
    All,
}

impl ArrowIpcTypes {
    /// Returns the schema for the chosen output type
    pub fn schema(&self) -> Schema {
        match self {
            Self::InitData => {
                if let DataType::Struct(fields) = init_data::InitData::data_type() {
                    Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::Details => {
                if let DataType::Struct(fields) = details::Details::data_type() {
                    Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::Stats => {
                if let DataType::Struct(fields) = tracker_events::PlayerStatsFlatRow::data_type() {
                    Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::Upgrades => {
                if let DataType::Struct(fields) = tracker_events::UpgradeEventFlatRow::data_type() {
                    Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::UnitBorn => {
                if let DataType::Struct(fields) = tracker_events::UnitBornEventFlatRow::data_type()
                {
                    Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::UnitDied => {
                if let DataType::Struct(fields) = tracker_events::UnitDiedEventFlatRow::data_type()
                {
                    Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::CmdTargetPoint => {
                if let DataType::Struct(fields) =
                    game_events::CmdTargetPointEventFlatRow::data_type()
                {
                    Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::CmdTargetUnit => {
                if let DataType::Struct(fields) =
                    game_events::CmdTargetUnitEventFlatRow::data_type()
                {
                    Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            _ => unimplemented!(),
        }
    }

    /// Handles the write command for the chosen output type
    #[tracing::instrument(level = "debug")]
    pub fn handle_write_command(
        &self,
        sources: Vec<InitData>,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::InitData => self.handle_init_data_ipc_cmd(sources, output),
            Self::Details => self.handle_details_ipc_cmd(sources, output),
            Self::Stats => self.handle_tracker_events(sources, output),
            Self::Upgrades => self.handle_tracker_events(sources, output),
            Self::UnitBorn => self.handle_tracker_events(sources, output),
            Self::UnitDied => self.handle_tracker_events(sources, output),
            Self::CmdTargetPoint => self.handle_game_events(sources, output),
            Self::CmdTargetUnit => self.handle_game_events(sources, output),
            Self::All => {
                if !output.is_dir() {
                    panic!("Output must be a directory for types 'all'");
                }
                // output must be a directory, for this directory we will create the following files:
                // init_data.ipc
                // details.ipc
                // stats.ipc
                // upgrades.ipc
                // unit_born.ipc
                Self::InitData
                    .handle_init_data_ipc_cmd(sources.clone(), output.join("init_data.ipc"))?;
                Self::Details
                    .handle_details_ipc_cmd(sources.clone(), output.join("details.ipc"))?;
                Self::Stats.handle_tracker_events(sources.clone(), output.join("stats.ipc"))?;
                Self::Upgrades
                    .handle_tracker_events(sources.clone(), output.join("upgrades.ipc"))?;
                Self::UnitBorn
                    .handle_tracker_events(sources.clone(), output.join("unit_born.ipc"))?;
                Self::UnitDied
                    .handle_tracker_events(sources.clone(), output.join("unit_died.ipc"))?;
                Self::CmdTargetPoint
                    .handle_game_events(sources.clone(), output.join("cmd_target_point.ipc"))?;
                Self::CmdTargetUnit
                    .handle_game_events(sources.clone(), output.join("cmd_target_unit.ipc"))?;
                Ok(())
            }
            _ => todo!(),
        }
    }

    #[tracing::instrument(level = "debug")]
    pub fn handle_init_data_ipc_cmd(
        &self,
        sources: Vec<InitData>,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Processing InitData IPC write request");

        // process the sources in parallel consuming into the batch variable
        let res: Box<dyn Array> = sources.try_into_arrow()?;

        tracing::info!("Loaded {} records", res.len());
        let chunk: RecordBatch = RecordBatch::try_from_iter([res].to_vec()).flatten()?;
        write_batches(output, self.schema(), &[chunk])?;
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
                let source = PathBuf::from(&source.file_name);
                let details = crate::details::Details::try_from(source.clone()).ok()?;
                let tracker_events = TrackerEventIterator::new(&source).ok()?;
                let (res, batch_len): (Box<dyn Array>, usize) = match self {
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
                let source = PathBuf::from(&source.file_name);
                let details = crate::details::Details::try_from(source.clone()).ok()?;
                let game_events = GameEventIterator::new(&source).ok()?;
                let (res, batch_len): (Box<dyn Array>, usize) = match self {
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
        let res: Box<dyn Array> = sources
            .par_iter()
            .filter_map(|source| {
                crate::details::Details::try_from(PathBuf::from(&source.file_name)).ok()
            })
            .collect::<Vec<details::Details>>()
            .try_into_arrow()?;
        tracing::info!("Loaded {} records", res.len());
        let chunk = RecordBatch::try_from_iter([res].to_vec()).flatten()?;
        write_batches(output, self.schema(), &[chunk])?;
        Ok(())
    }

    /// Handles the Arrow IPC command variants
    #[tracing::instrument(level = "debug")]
    pub fn handle_arrow_ipc_cmd(
        &self,
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
            .filter_map(|source| crate::init_data::InitData::try_from(source.clone()).ok())
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
        self.handle_write_command(sources, output)
    }
}
