//! Arrow Specific handling of data.

#[cfg(feature = "arrow")]
use arrow2::{array::Array, chunk::Chunk, datatypes::DataType};
use arrow2_convert::serialize::FlattenChunk;
#[cfg(feature = "arrow")]
use arrow2_convert::{field::ArrowField, serialize::TryIntoArrow};
#[cfg(feature = "arrow")]
use rayon::prelude::*;

use crate::cli::get_matching_files;
use crate::tracker_events::{self, TrackerEventIterator};
use crate::*;
use clap::Subcommand;
use std::path::PathBuf;
pub mod ipc_writer;
use ipc_writer::*;

#[derive(Debug, Subcommand)]
pub enum ArrowIpcTypes {
    /// Writes the [`crate::init_data::InitData`] flat row to an Arrow IPC file
    InitData,
    /// Writes the [`crate::details::Details`] flat row to an Arrow IPC file
    Details,
    /// Writes the [`crate::tracker_events::PlayerStatsEvent`] to an Arrow IPC file
    Stats,
    /// Writes the [`crate::tracker_events::UpgradeEvents`] to an Arrow IPC file
    Upgrades,
    /// Writes the [`crate::tracker_events::UnitBornEvent`] to an Arrow IPC file
    UnitBorn,
    /// Writes the [`crate::tracker_events::UnitDiedEvent`] to an Arrow IPC file
    UnitDied,
    /// Writes the [`crate::message_events::MessageEvent`] to an Arrow IPC file
    MessageEvents,
    /// Writes all the implemented flat row types to Arrow IPC files inside the output directory
    All,
}

impl ArrowIpcTypes {
    /// Returns the schema for the chosen output type
    pub fn schema(&self) -> arrow2::datatypes::Schema {
        match self {
            Self::InitData => {
                if let DataType::Struct(fields) = init_data::InitData::data_type() {
                    arrow2::datatypes::Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::Details => {
                if let DataType::Struct(fields) = details::Details::data_type() {
                    arrow2::datatypes::Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::Stats => {
                if let DataType::Struct(fields) = tracker_events::PlayerStatsFlatRow::data_type() {
                    arrow2::datatypes::Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::Upgrades => {
                if let DataType::Struct(fields) = tracker_events::UpgradeEventFlatRow::data_type() {
                    arrow2::datatypes::Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::UnitBorn => {
                if let DataType::Struct(fields) = tracker_events::UnitBornEventFlatRow::data_type()
                {
                    arrow2::datatypes::Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::UnitDied => {
                if let DataType::Struct(fields) = tracker_events::UnitDiedEventFlatRow::data_type()
                {
                    arrow2::datatypes::Schema::from(fields.clone())
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
        sources: Vec<PathBuf>,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::InitData => self.handle_init_data_ipc_cmd(sources, output),
            Self::Details => self.handle_details_ipc_cmd(sources, output),
            Self::Stats => self.handle_tracker_events(sources, output),
            Self::Upgrades => self.handle_tracker_events(sources, output),
            Self::UnitBorn => self.handle_tracker_events(sources, output),
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
                Ok(())
            }
            _ => todo!(),
        }
    }

    #[tracing::instrument(level = "debug")]
    pub fn handle_init_data_ipc_cmd(
        &self,
        sources: Vec<PathBuf>,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Processing InitData IPC write request");

        // process the sources in parallel consuming into the batch variable
        let res: Box<dyn Array> = sources
            .par_iter()
            .filter_map(|source| crate::init_data::InitData::try_from(source.clone()).ok())
            .collect::<Vec<init_data::InitData>>()
            .try_into_arrow()?;
        tracing::info!("Loaded {} records", res.len());
        let chunk = Chunk::new([res].to_vec()).flatten()?;
        write_batches(output, self.schema(), &[chunk])?;
        Ok(())
    }

    /// Creates a new Arrow IPC file with the stats data
    /// This seems to be small enough to not need to be chunked and is done in parallel
    /// This requires 1.5GB of RAM for 3600 files, so maybe not good for real players.
    #[tracing::instrument(level = "debug")]
    pub fn handle_tracker_events(
        &self,
        sources: Vec<PathBuf>,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Processing TrackerEvents IPC write request: {:?}", self);
        let writer = open_arrow_mutex_writer(output, self.schema())?;

        // process files in parallel, the internal iterators will fight for the lock
        let total_records = sources
            .par_iter()
            .filter_map(|source| {
                let details = crate::details::Details::try_from(source.clone()).ok()?;
                let tracker_events = TrackerEventIterator::new(source).ok()?;
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

    /// Creates a new Arrow IPC file with the details data
    #[tracing::instrument(level = "debug")]
    pub fn handle_details_ipc_cmd(
        &self,
        sources: Vec<PathBuf>,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Processing Details IPC write request");
        // process the sources in parallel consuming into the batch variable
        let res: Box<dyn Array> = sources
            .par_iter()
            .filter_map(|source| crate::details::Details::try_from(source.clone()).ok())
            .collect::<Vec<details::Details>>()
            .try_into_arrow()?;
        tracing::info!("Loaded {} records", res.len());
        let chunk = Chunk::new([res].to_vec()).flatten()?;
        write_batches(output, self.schema(), &[chunk])?;
        Ok(())
    }

    /// Handles the Arrow IPC command variants
    #[tracing::instrument(level = "debug")]
    pub fn handle_arrow_ipc_cmd(
        &self,
        source: PathBuf,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Processing Arrow write request");
        let sources = get_matching_files(source)?;
        if sources.is_empty() {
            panic!("No files found");
        } else {
            tracing::info!("Found {} files", sources.len());
        }
        self.handle_write_command(sources, output)
    }
}
