#[cfg(feature = "arrow")]
use super::*;

use crate::game_events::iterator::GameEventIterator;
use crate::generator::proto_morphist::ProtoMorphist;
use crate::read_details;
use crate::read_init_data;
use crate::read_message_events;
use crate::tracker_events::iterator::TrackerEventIterator;
use clap::{Args, Parser, Subcommand};
use nom_mpq::parser;
use std::iter::Iterator;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
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
    /// Transist through the state machine and print change hints
    TransistEvents,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generates Rust code for a specific protocol.
    Generate,

    /// Gets a specific event type from the SC2Replay MPQ Archive
    #[command(subcommand)]
    Get(ReadTypes),

    /// Writes Arrow IPC files for a specific event type from the SC2Replay MPQ Archive
    #[cfg(feature = "arrow")]
    WriteArrowIpc(WriteArrowIpcProps),
}

//  Create a subcommand that handles the max depth and max files to process

#[cfg(feature = "arrow")]
#[derive(Args, Debug)]
pub struct WriteArrowIpcProps {
    /// Reads these many  files recursing, these files may or may not be valid.
    #[arg(long, default_value = "1000000")]
    pub scan_max_files: usize,
    /// The maximum number of files to process
    #[arg(long, default_value = "1000000")]
    pub process_max_files: usize,
    /// The maximum directory depth to traverse
    #[arg(long, default_value = "8")]
    pub traverse_max_depth: usize,
    /// The minimum protocol version
    #[arg(long)]
    pub min_version: Option<u32>,
    /// The maximum protocol version
    #[arg(long)]
    pub max_version: Option<u32>,
    /// Writes the [`crate::init_data::InitData`] flat row to an Arrow IPC file
    #[command(subcommand)]
    kind: ArrowIpcTypes,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets the source of the data, can be a file or directory.
    #[arg(short, long, value_name = "PATH")]
    source: String,

    /// Turn debugging information on
    #[arg(short, long, default_value = "info")]
    verbosity_level: String,

    /// A command to run
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
pub fn get_matching_files(
    source: PathBuf,
    max_files: usize,
    max_depth: usize,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    if max_depth == 0 {
        tracing::info!("Reached max depth");
        return Ok(Vec::new());
    }
    if source.is_dir() {
        // if this is a directory, let's recurse to go through subdirectories.
        let mut sources = Vec::new();
        for entry in std::fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let mut sub_dir = get_matching_files(path, max_files, max_depth - 1)?;
                if !sub_dir.is_empty() {
                    // store n sub_dir files without breaking the max_files limit
                    let remaining = max_files - sources.len();
                    if sub_dir.len() > remaining {
                        sub_dir.truncate(remaining);
                    }
                    sources.append(&mut sub_dir);
                }
            } else if let Some(ext) = path.extension() {
                if ext == "SC2Replay" && path.is_file() {
                    if sources.len() >= max_files {
                        break;
                    }
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
                        let res = TrackerEventIterator::new(&source)?;
                        println!("[");
                        for evt in res.into_iter() {
                            println!("{},", serde_json::to_string(&evt)?);
                        }
                        println!("]");
                    }

                    ReadTypes::GameEvents => {
                        let res = GameEventIterator::new(&source)?;
                        println!("[");
                        for evt in res.into_iter() {
                            println!("{},", serde_json::to_string(&evt)?);
                        }
                        println!("]");
                    }
                    ReadTypes::MessageEvents => {
                        let res = read_message_events(&source_path, &mpq, &file_contents)?;
                        println!("[");
                        for evt in res {
                            println!("{},", serde_json::to_string(&evt)?);
                        }
                        println!("]");
                    }
                    ReadTypes::Details => {
                        let res = read_details(&source_path, &mpq, &file_contents)?;
                        println!("{},", serde_json::to_string(&res)?);
                    }
                    ReadTypes::InitData => {
                        let res = read_init_data(&source_path, &mpq, &file_contents)?;
                        println!("{},", serde_json::to_string(&res)?);
                    }
                    ReadTypes::TransistEvents => {
                        tracing::info!("Transducing through both Game and Tracker Events");
                        println!("[");
                        let res = crate::state::SC2EventIterator::new(&source)?;
                        for evt in res.into_iter() {
                            println!("{},", serde_json::to_string(&evt)?);
                        }
                        println!("]");
                    }
                }
            }
        }
        Commands::WriteArrowIpc(cmd) => {
            cmd.kind.handle_arrow_ipc_cmd(
                PathBuf::from(&cli.source),
                PathBuf::from(&cli.output.expect("Requires --output")),
                cmd,
            )?;
        }
    }
    if cli.timing {
        println!("Total time: {:?}", init_time.elapsed());
    }
    Ok(())
}
