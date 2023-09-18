//! Arrow Specific handling of data.
//!

use crate::state::*;
use crate::*;
#[cfg(feature = "arrow")]
use arrow2::{array::Array, chunk::Chunk, datatypes::DataType};
#[cfg(feature = "arrow")]
use arrow2_convert::{field::ArrowField, serialize::TryIntoArrow};
use clap::Subcommand;
use nom_mpq::parser;
use rayon::prelude::*;
use std::path::PathBuf;

use crate::cli::get_matching_files;
use arrow2_convert::serialize::FlattenChunk;

#[derive(Subcommand)]
pub enum ArrowIpcTypes {
    /// Writes the tracker events to an Arrow IPC file
    TrackerEvents,
    /// Writes the game events to an Arrow IPC file
    GameEvents,
    /// Writes the message events to an Arrow IPC file
    MessageEvents,
    /// Writes the details to an Arrow IPC file
    Details,
    /// Writes the initData to an Arrow IPC file
    InitData,
    /// Writes the Stats to an Arrow IPC file, Parallel mode, may not fit in RAM, if error, try SerialStats
    Stats,
    /// Writes the Stats to an Arrow IPC file, Serial mode, needs less RAM but more time.
    SerialStats,
    /// Writes the UpgradeEvents to an Arrow IPC file
    Upgrades,
}

impl ArrowIpcTypes {
    pub fn schema(&self) -> arrow2::datatypes::Schema {
        match self {
            Self::Stats | Self::SerialStats => {
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
            Self::Details => {
                if let DataType::Struct(fields) = details::Details::data_type() {
                    arrow2::datatypes::Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            Self::InitData => {
                if let DataType::Struct(fields) = init_data::InitData::data_type() {
                    arrow2::datatypes::Schema::from(fields.clone())
                } else {
                    panic!("Invalid schema, expected struct");
                }
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(feature = "arrow")]
fn write_batches(
    path: PathBuf,
    schema: arrow2::datatypes::Schema,
    chunks: &[Chunk<Box<dyn Array>>],
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(&path)?;

    let options = arrow2::io::ipc::write::WriteOptions { compression: None };
    let mut writer = arrow2::io::ipc::write::FileWriter::new(file, schema, None, options);

    writer.start()?;
    for chunk in chunks {
        writer.write(chunk, None)?
    }
    writer.finish()?;
    Ok(())
}

pub fn handle_details_ipc_cmd(
    sources: Vec<PathBuf>,
    output: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // process the sources in parallel consuming into the batch variable
    let res: Box<dyn Array> = sources
        .par_iter()
        .filter_map(|source| {
            let file_contents = parser::read_file(source.display().to_string().as_str());
            let (_input, mpq) = parser::parse(&file_contents).ok()?;
            match read_details(&mpq, &file_contents) {
                Ok(details) => {
                    Some(details.set_metadata(source.file_name()?.to_str()?, &file_contents))
                }
                Err(err) => {
                    tracing::error!("Error reading details: {:?}", err);
                    None
                }
            }
        })
        .collect::<Vec<details::Details>>()
        .try_into_arrow()?;
    println!("Loaded {} records", res.len());
    let chunk = Chunk::new([res].to_vec()).flatten()?;
    write_batches(output, ArrowIpcTypes::Details.schema(), &[chunk])?;
    Ok(())
}

pub fn handle_init_data_ipc_cmd(
    sources: Vec<PathBuf>,
    output: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // process the sources in parallel consuming into the batch variable
    let res: Box<dyn Array> = sources
        .par_iter()
        .filter_map(|source| {
            let file_contents = parser::read_file(source.display().to_string().as_str());
            let (_input, mpq) = parser::parse(&file_contents).ok()?;
            match read_init_data(&mpq, &file_contents) {
                Ok(init_data) => Some(init_data),
                Err(err) => {
                    tracing::error!("Error reading details: {:?}", err);
                    None
                }
            }
        })
        .collect::<Vec<init_data::InitData>>()
        .try_into_arrow()?;
    println!("Loaded {} records", res.len());
    let chunk = Chunk::new([res].to_vec()).flatten()?;
    write_batches(output, ArrowIpcTypes::InitData.schema(), &[chunk])?;
    Ok(())
}

/// Creates a new Arrow IPC file with the stats data
/// This seems to be small enough to not need to be chunked and is done in parallel
/// This requires 1.5GB of RAM for 3600 files, so maybe not good for real players.
pub fn handle_stats_ipc_cmd(
    sources: Vec<PathBuf>,
    output: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // process the sources in parallel consuming into the batch variable
    let res: Box<dyn Array> = sources
        .par_iter()
        .filter_map(|source| {
            let file_contents = parser::read_file(source.display().to_string().as_str());
            let (_input, mpq) = parser::parse(&file_contents).ok()?;
            let details = match read_details(&mpq, &file_contents) {
                Ok(details) => details,
                Err(err) => {
                    tracing::error!("Error reading details: {:?}", err);
                    return None;
                }
            };
            // At this point we have already verified the filename is valid utf8
            let file_name = source.file_name().unwrap().to_str().unwrap().to_string();
            let sha256 = sha256::digest(&file_contents);
            let datetime = transform_to_naivetime(details.time_utc, details.time_local_offset)
                .unwrap_or_default();
            let tracker_events = match read_tracker_events(&mpq, &file_contents) {
                Ok(tracker_events) => tracker_events,
                Err(err) => {
                    tracing::error!("Error reading tracker events: {:?}", err);
                    return None;
                }
            };
            let mut tracker_loop = 0i64;
            let mut batch = vec![];
            for game_step in tracker_events {
                tracker_loop += game_step.delta as i64;
                let game_loop = (tracker_loop as f32 / TRACKER_SPEED_RATIO) as i64;
                if let tracker_events::ReplayTrackerEvent::PlayerStats(event) = game_step.event {
                    let player_name = details.get_player_name(event.player_id - 1);
                    batch.push(tracker_events::PlayerStatsFlatRow::new(
                        event,
                        game_loop,
                        file_name.clone(),
                        sha256.clone(),
                        player_name,
                        datetime,
                    ));
                }
            }
            Some(batch)
        })
        .flatten()
        .collect::<Vec<tracker_events::PlayerStatsFlatRow>>()
        .try_into_arrow()?;
    println!("Loaded {} records", res.len());
    let chunk = Chunk::new([res].to_vec()).flatten()?;
    write_batches(output, ArrowIpcTypes::Stats.schema(), &[chunk])?;
    Ok(())
}

/// Creates a new Arrow IPC file with the stats data
/// This seems to be small enough to not need to be chunked and is done in parallel
pub fn handle_upgrades_ipc_cmd(
    sources: Vec<PathBuf>,
    output: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(output)?;

    let options = arrow2::io::ipc::write::WriteOptions { compression: None };
    let mut writer = arrow2::io::ipc::write::FileWriter::new(
        file,
        ArrowIpcTypes::Upgrades.schema(),
        None,
        options,
    );

    writer.start()?;
    let writer = std::sync::Mutex::new(writer);

    // process the sources in parallel consuming into the batch variable
    let total_records = sources
        .par_iter()
        .filter_map(|source| {
            let file_contents = parser::read_file(source.display().to_string().as_str());
            let (_input, mpq) = parser::parse(&file_contents).ok()?;
            let details = match read_details(&mpq, &file_contents) {
                Ok(details) => details,
                Err(err) => {
                    tracing::error!("Error reading details: {:?}", err);
                    return None;
                }
            };
            let sha256 = sha256::digest(&file_contents);
            // At this point we have already verified the filename is valid utf8
            let file_name = source.file_name().unwrap().to_str().unwrap().to_string();
            let epoch = transform_to_naivetime(details.time_utc, details.time_local_offset)
                .unwrap_or_default();
            let tracker_events = match read_tracker_events(&mpq, &file_contents) {
                Ok(tracker_events) => tracker_events,
                Err(err) => {
                    tracing::error!("Error reading tracker events: {:?}", err);
                    return None;
                }
            };
            let mut tracker_loop = 0i64;
            let mut batch = vec![];
            for game_step in tracker_events {
                tracker_loop += game_step.delta as i64;
                let game_loop = (tracker_loop as f32 / TRACKER_SPEED_RATIO) as i64;
                if let tracker_events::ReplayTrackerEvent::Upgrade(event) = game_step.event {
                    let player_name = details.get_player_name(event.player_id - 1);
                    batch.push(tracker_events::UpgradeEventFlatRow::new(
                        event,
                        game_loop,
                        file_name.clone(),
                        sha256.clone(),
                        player_name,
                        epoch,
                    ));
                }
            }
            let batch_len = batch.len();
            let res: Box<dyn Array> = batch.try_into_arrow().ok()?;
            let mut file_lock = match writer.lock() {
                Ok(lock) => lock,
                Err(err) => {
                    tracing::error!("Error locking file: {:?}", err);
                    return None;
                }
            };
            let chunk = match Chunk::new([res].to_vec()).flatten() {
                Ok(chunk) => chunk,
                Err(err) => {
                    tracing::error!("Error converting to arrow: {:?}", err);
                    return None;
                }
            };
            match file_lock.write(&chunk, None) {
                Ok(_) => Some(batch_len),
                Err(err) => {
                    // At this point maybe we should fail because the lock write
                    // should fail for any other attempt?
                    tracing::error!("Error writing chunk: {:?}", err);
                    None
                }
            }
        })
        .sum::<usize>();
    println!("Loaded {} records", total_records);
    let mut writer = match writer.lock() {
        Ok(writer) => writer,
        Err(err) => {
            tracing::error!("Error locking file: {:?}", err);
            return Err("Lock error".into());
        }
    };
    Ok(writer.finish()?)
}

pub fn handle_stats_ipc_cmd_serially(
    _sources: Vec<PathBuf>,
    output: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(output)?;

    let options = arrow2::io::ipc::write::WriteOptions { compression: None };
    let mut writer =
        arrow2::io::ipc::write::FileWriter::new(file, ArrowIpcTypes::Stats.schema(), None, options);

    writer.start()?;
    //let chunk = Chunk::new([res].to_vec()).flatten()?;
    //chunks: &[Chunk<Box<dyn Array>>],
    // writer.write(chunk, None)?
    writer.finish()?;
    Ok(())
}

pub fn handle_arrow_ipc_cmd(
    cmd: &ArrowIpcTypes,
    source: PathBuf,
    output: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Processing Arrow write request");
    let sources = get_matching_files(source)?;
    if sources.is_empty() {
        panic!("No files found");
    } else {
        println!("Found {} files", sources.len());
    }
    match cmd {
        ArrowIpcTypes::InitData => handle_init_data_ipc_cmd(sources, output),
        ArrowIpcTypes::Details => handle_details_ipc_cmd(sources, output),
        ArrowIpcTypes::TrackerEvents => {
            panic!("TrackerEvents not supported. Use Stats instead");
        }
        ArrowIpcTypes::GameEvents => {
            panic!("GameEvents not supported. Use Stats instead");
        }
        ArrowIpcTypes::MessageEvents => {
            panic!("MessageEvents not supported. Use Stats instead");
        }
        ArrowIpcTypes::Stats => handle_stats_ipc_cmd(sources, output),
        ArrowIpcTypes::SerialStats => handle_stats_ipc_cmd_serially(sources, output),
        ArrowIpcTypes::Upgrades => handle_upgrades_ipc_cmd(sources, output),
    }
}
