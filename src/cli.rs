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

    /// Writes Arrow IPC files for a specific event type from the SC2Replay MPQ Archive
    #[cfg(feature = "arrow")]
    #[command(subcommand)]
    WriteArrowIpc(ArrowIpcTypes),
}
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets the source of the data, can be a file or directory.
    #[arg(short, long, value_name = "PATH")]
    source: String,

    /// Turn debugging information on
    #[arg(short, long, default_value = "info")]
    verbosity_level: String,

    #[command(subcommand)]
    command: Commands,

    /// The output file to write to
    #[arg(short, long)]
    output: Option<String>,

    /// Show basic performance metrics
    #[arg(short, long, default_value = "false")]
    timing: bool,
}

/// Matches a list of files in case the cli.source param is a directory
#[tracing::instrument(level = "debug")]
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
    let init_time = std::time::Instant::now();
    let cli = Cli::parse();
    // use the cli verbosity level to set the tracing level
    let level = match cli.verbosity_level.as_str() {
        "error" => tracing::Level::ERROR,
        "warn" => tracing::Level::WARN,
        "info" => tracing::Level::INFO,
        "debug" => tracing::Level::DEBUG,
        "trace" => tracing::Level::TRACE,
        _ => tracing::Level::INFO,
    };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_env_filter("info")
        .init();
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
                tracing::info!("Processing {:?}", source);
                let file_contents = crate::read_file(&source).unwrap();
                let (_input, mpq) = match parser::parse(&file_contents) {
                    Ok(res) => res,
                    Err(e) => {
                        tracing::error!("Error parsing file: {:?}", e);
                        continue;
                    }
                };
                let source_path: String = source
                    .file_name()
                    .expect("Failed to get file name")
                    .to_str()
                    .expect("Failed to convert file name to str")
                    .to_string();
                match read_type {
                    ReadTypes::TrackerEvents => {
                        tracing::info!("Getting tracker events");
                        let res = read_tracker_events(&source_path, &mpq, &file_contents)?;
                        println!("[");
                        for evt in res {
                            println!("{},", serde_json::to_string(&evt)?);
                        }
                        println!("]");
                    }

                    ReadTypes::GameEvents => {
                        tracing::info!("Getting game events");
                        let res = read_game_events(&source_path, &mpq, &file_contents)?;
                        println!("[");
                        for evt in res {
                            println!("{},", serde_json::to_string(&evt)?);
                        }
                        println!("]");
                    }
                    ReadTypes::MessageEvents => {
                        tracing::info!("Getting message events");
                        let res = read_message_events(&source_path, &mpq, &file_contents)?;
                        println!("[");
                        for evt in res {
                            println!("{},", serde_json::to_string(&evt)?);
                        }
                        println!("]");
                    }
                    ReadTypes::Details => {
                        tracing::info!("Getting details");
                        let res = read_details(&source_path, &mpq, &file_contents)?;
                        println!("{},", serde_json::to_string(&res)?);
                    }
                    ReadTypes::InitData => {
                        tracing::info!("Getting initData");
                        let res = read_init_data(&source_path, &mpq, &file_contents)?;
                        println!("{},", serde_json::to_string(&res)?);
                    }
                }
            }
        }
        Commands::WriteArrowIpc(cmd) => {
            cmd.handle_arrow_ipc_cmd(
                PathBuf::from(&cli.source),
                PathBuf::from(&cli.output.expect("Requires --output")),
            )?;
        }
    }
    if cli.timing {
        println!("Total time: {:?}", init_time.elapsed());
    }
    Ok(())
}
