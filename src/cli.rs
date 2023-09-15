#[cfg(feature = "arrow")]
use super::*;

use crate::generator::proto_morphist::ProtoMorphist;
use crate::versions::read_details;
use crate::versions::read_game_events;
use crate::versions::read_init_data;
use crate::versions::read_message_events;
use crate::versions::read_tracker_events;
use clap::{Parser, Subcommand};
use nom_mpq::parser;
use std::path::PathBuf;

#[derive(Subcommand)]
enum ReadTypes {
    /// Reads the tracker events from an SC2Replay MPQ Archive
    TrackerEvents,
    /// Reads the game events from an SC2Replay MPQ Archive
    GameEvents,
    /// Reads the message events from an SC2Replay MPQ Archive
    MessageEvents,
    /// Reads the details from an SC2Replay MPQ Archive
    Details,
    /// Reads the initData from an SC2Replay MPQ Archive
    InitData,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates Rust code for a specific protocol.
    Generate,
    /// Gets a specific event type from the SC2Replay MPQ Archive
    #[command(subcommand)]
    Get(ReadTypes),
    #[cfg(feature = "arrow")]
    #[command(subcommand)]
    WriteArrowIpc(ArrowIpcTypes),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "PATH")]
    source: String,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    output: Option<String>,
}

/// Matches a list of files in case the cli.source param is a directory
pub fn get_matching_files(source: PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    if source.is_dir() {
        let mut sources = Vec::new();
        for entry in std::fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();
            // the filename must end in .SC2Replay
            if let Some(ext) = path.extension() {
                if ext == "SC2Replay" && path.is_file() {
                    sources.push(path);
                }
            }
        }
        Ok(sources)
    } else {
        Ok(vec![source.clone()])
    }
}

/// Handles the request from the CLI when used as a binary
pub fn process_cli_request() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Generate => {
            ProtoMorphist::gen(&cli.source, &cli.output.expect("Requires --output"))?;
        }
        Commands::Get(read_type) => {
            let sources: Vec<PathBuf> = if PathBuf::from(&cli.source).is_dir() {
                let mut sources = Vec::new();
                for entry in std::fs::read_dir(&cli.source)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        sources.push(path);
                    }
                }
                sources
            } else {
                vec![cli.source.clone().into()]
            };
            for source in sources {
                let file_contents = parser::read_file(source.display().to_string().as_str());
                let (_input, mpq) = match parser::parse(&file_contents) {
                    Ok(res) => res,
                    Err(e) => {
                        tracing::error!("Error parsing file: {:?}", e);
                        continue;
                    }
                };
                match read_type {
                    ReadTypes::TrackerEvents => {
                        tracing::info!("Getting tracker events");
                        let res = read_tracker_events(&mpq, &file_contents)?;
                        println!("[");
                        for evt in res {
                            println!("{},", serde_json::to_string(&evt)?);
                        }
                        println!("]");
                    }

                    ReadTypes::GameEvents => {
                        tracing::info!("Getting game events");
                        let res = read_game_events(&mpq, &file_contents)?;
                        println!("[");
                        for evt in res {
                            println!("{},", serde_json::to_string(&evt)?);
                        }
                        println!("]");
                    }
                    ReadTypes::MessageEvents => {
                        tracing::info!("Getting message events");
                        let res = read_message_events(&mpq, &file_contents)?;
                        println!("[");
                        for evt in res {
                            println!("{},", serde_json::to_string(&evt)?);
                        }
                        println!("]");
                    }
                    ReadTypes::Details => {
                        tracing::info!("Getting details");
                        let res = read_details(&mpq, &file_contents)?;
                        println!("{},", serde_json::to_string(&res)?);
                    }
                    ReadTypes::InitData => {
                        tracing::info!("Getting initData");
                        let res = read_init_data(&mpq, &file_contents)?;
                        println!("{},", serde_json::to_string(&res)?);
                    }
                }
            }
        }
        Commands::WriteArrowIpc(cmd) => {
            arrow::handle_arrow_ipc_cmd(
                cmd,
                PathBuf::from(&cli.source),
                PathBuf::from(&cli.output.expect("Requires --output")),
            )?;
        }
    }
    Ok(())
}
