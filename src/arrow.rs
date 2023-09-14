//! Arrow Specific handling of data.
//!

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
    /// Writes the Stats to an Arrow IPC file
    Stats,
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

pub fn handle_arrow_ipc_cmd(
    cmd: &ArrowIpcTypes,
    source: PathBuf,
    output: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(&output)?;

    let options = arrow2::io::ipc::write::WriteOptions { compression: None };
    tracing::info!("Processing write request");
    let sources = get_matching_files(source)?;
    if sources.is_empty() {
        panic!("No files found");
    } else {
        println!("Found {} files", sources.len());
    }
    match cmd {
        small_type @ (ArrowIpcTypes::InitData | ArrowIpcTypes::Details) => {
            // process the sources in parallel consuming into the batch variable
            let res: Box<dyn Array> = match small_type {
                ArrowIpcTypes::Details => sources
                    .par_iter()
                    .filter_map(|source| {
                        let file_contents =
                            parser::read_file(source.display().to_string().as_str());
                        let (_input, mpq) = parser::parse(&file_contents).ok()?;
                        match read_details(&mpq, &file_contents) {
                            Ok(details) => Some(
                                details.set_metadata(source.file_name()?.to_str()?, &file_contents),
                            ),
                            Err(_) => None,
                        }
                    })
                    .collect::<Vec<details::Details>>()
                    .try_into_arrow()?,
                ArrowIpcTypes::InitData => sources
                    .par_iter()
                    .filter_map(|source| {
                        let file_contents =
                            parser::read_file(source.display().to_string().as_str());
                        let (_input, mpq) = parser::parse(&file_contents).ok()?;
                        match read_init_data(&mpq, &file_contents) {
                            Ok(init_data) => Some(
                                init_data.set_metadata(source.file_name()?.to_str()?, &file_contents),
                            ),
                            Err(_) => None,
                        }
                    })
                    .collect::<Vec<init_data::InitData>>()
                    .try_into_arrow()?,
                _ => unreachable!(),
            };
            println!("Loaded {} files", res.len());
            if let DataType::Struct(fields) = res.data_type() {
                let chunk = Chunk::new([res.clone()].to_vec()).flatten()?;
                write_batches(
                    output,
                    arrow2::datatypes::Schema::from(fields.clone()),
                    &[chunk],
                );
            }
        }
        ArrowIpcTypes::Stats => {
            // process the sources serially avoiding filling the entires replays
            // memory
            let mut writer =
                if let DataType::Struct(fields) = tracker_events::PlayerStatsFlatRow::data_type() {
                    let schema = arrow2::datatypes::Schema::from(fields.clone());
                    arrow2::io::ipc::write::FileWriter::new(file, schema, None, options)
                } else {
                    panic!("Invalid schema");
                };
            writer.start()?;
            for source in sources.iter() {
                println!("Processing {}", source.display());
                let file_contents = parser::read_file(source.display().to_string().as_str());
                let (_input, mpq) = match parser::parse(&file_contents) {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::error!("Error parsing file: {:?}", e);
                        continue;
                    }
                };
                let mut batch = vec![];
                if let Ok(details) = read_details(&mpq, &file_contents) {
                    let sha256 = sha256::digest(&file_contents);
                    let epoch = transform_time(details.time_utc, details.time_local_offset);
                    let mut replay = match state::SC2ReplayState::from_mpq(
                        mpq,
                        file_contents,
                        Default::default(),
                        true,
                    ) {
                        Ok(replay) => replay,
                        Err(_) => continue,
                    };
                    while let Some((event, _updated_units)) = replay.transduce() {
                        if let state::SC2EventType::Tracker {
                            tracker_loop,
                            event,
                        } = event
                        {
                            if let tracker_events::ReplayTrackerEvent::PlayerStats(stat_event) =
                                event
                            {
                                batch.push(tracker_events::PlayerStatsFlatRow::new(
                                    stat_event.stats,
                                    stat_event.player_id,
                                    tracker_loop,
                                    // At this point we have already verified the filename is valid utf8
                                    source.file_name().unwrap().to_str().unwrap().to_string(),
                                    sha256.clone(),
                                    epoch,
                                ));
                            }
                        }
                    }
                }
                let res: Box<dyn Array> = batch.try_into_arrow()?;
                let chunks: &[Chunk<Box<dyn Array>>] = &[Chunk::new([res].to_vec()).flatten()?];
                for chunk in chunks {
                    writer.write(chunk, None)?
                }
            }
            writer.finish()?;
        }
        ArrowIpcTypes::TrackerEvents => {
            panic!("TrackerEvents not supported. Use Stats instead");
        }
        ArrowIpcTypes::GameEvents => {
            panic!("GameEvents not supported. Use Stats instead");
        }
        ArrowIpcTypes::MessageEvents => {
            panic!("MessageEvents not supported. Use Stats instead");
        }
    };
    Ok(())
}
