#[cfg(feature = "dep_arrow")]
use super::*;

#[cfg(feature = "syntax")]
use bat::{Input, PrettyPrinter};

use crate::game_events::ability::json_handler::read_balance_data_from_json;
use crate::game_events::ability::traverse_versioned_balance_abilities;
use crate::game_events::VersionedBalanceUnit;
use crate::generator::proto_morphist::ProtoMorphist;
use crate::read_details;
use crate::read_init_data;
use crate::read_message_events;
use crate::state::SC2EventIterator;

#[cfg(feature = "dep_arrow")]
use clap::Args;

use clap::{Parser, Subcommand};
use nom_mpq::parser;
use std::path::PathBuf;

#[derive(Subcommand, Debug, Clone)]
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

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Generates Rust code for a specific protocol.
    Generate,

    /// Generates static json files for a specific Balance Data export.
    ///
    BalanceDataToJson,

    /// Gets a specific event type from the SC2Replay MPQ Archive
    #[command(subcommand)]
    Get(ReadTypes),

    /// Writes Arrow IPC files for a specific event type from the SC2Replay MPQ Archive
    #[cfg(feature = "dep_arrow")]
    WriteArrowIpc(WriteArrowIpcProps),
}

///  Create a subcommand that handles the max depth and max files to process
#[cfg(feature = "dep_arrow")]
#[derive(Args, Debug, Clone)]
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
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets the source of the data, can be a file or directory.
    /// Can be a path to a single SC2Replay file or a directory containing multiple SC2Replay files.
    /// If a directory is provided, it will recursively search for SC2Replay files.
    /// The path can contain a BalanceData with XMLs for the verisoned exported data.
    #[arg(short, long)]
    source: String,

    /// BalanceData directory path, these are directories exported from StarCraft II Editor.
    /// Main menu -> Export -> Balance Data.
    /// The data exported must then be saved into a versioned directory matching
    /// the protocol version used to export.
    #[arg(short, long, default_value = "")]
    xml_balance_data_dir: String,

    /// The xml balance data can be processed and exported into json files that are friendly to
    /// deser and store in git.
    #[arg(short, long, default_value = "")]
    json_balance_data_dir: String,

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

    /// colorize the output
    #[arg(short, long, default_value = "false")]
    color: bool,

    /// filters a specific player id.
    #[arg(long)]
    pub player_id: Option<u8>,

    /// Filters a min event loop, in game_event units
    #[arg(long)]
    pub min_loop: Option<i64>,

    /// Filters a max event loop
    #[arg(long)]
    pub max_loop: Option<i64>,

    /// Only show game of specific types
    #[arg(long)]
    pub event_type: Option<String>,

    /// Only show game of specific types
    #[arg(long)]
    pub unit_name: Option<String>,

    /// Allows setting up a max number of events of each type
    #[arg(long)]
    pub max_events: Option<usize>,

    /// Whether or not the PlayerStats event should be shown. To be replaced by a proper filter
    #[arg(long)]
    pub include_stats: bool,
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

/// Prints the json strings either with Bat PrettyPrint or just plain json
pub fn json_print(json_str: String, color: bool) {
    if color {
        #[cfg(feature = "syntax")]
        {
            PrettyPrinter::new()
                .language("json")
                .header(false)
                .grid(false)
                .line_numbers(false)
                .input(Input::from_bytes(json_str.as_bytes()))
                .print()
                .unwrap();
        }
        #[cfg(not(feature = "syntax"))]
        {
            println!("{json_str}");
        }
    } else {
        println!("{json_str}");
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
        _ => {
            tracing::warn!("Invalid verbosity level, defaulting to INFO");
            tracing::Level::INFO
        }
    };
    let color = cli.color;
    if color {
        tracing_subscriber::fmt()
            .with_max_level(level)
            .with_ansi(true)
            .with_env_filter(level.to_string())
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(level)
            .with_ansi(false)
            .with_env_filter(level.to_string())
            .init();
    }
    match &cli.command {
        Commands::Generate => {
            ProtoMorphist::gen(&cli.source, &cli.output.expect("Requires --output"))?;
        }
        Commands::BalanceDataToJson => {
            if cli.source.is_empty() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Source XML Balance Data directory must be provided",
                )));
            }
            if cli.json_balance_data_dir.is_empty() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Destination JSON Balance Data directory must be provided",
                )));
            }
            let versioned_abilities =
                traverse_versioned_balance_abilities(PathBuf::from(&cli.source))?;
            crate::game_events::ability::balance_data::json_handler::write_balance_data_to_json(
                &cli.json_balance_data_dir,
                versioned_abilities,
            )?;
        }
        Commands::Get(read_type) => {
            let versioned_abilities: HashMap<(u32, String), VersionedBalanceUnit> =
                if cli.json_balance_data_dir.is_empty() {
                    read_balance_data_from_json(PathBuf::from(&cli.source))?
                } else {
                    read_balance_data_from_json(PathBuf::from(&cli.json_balance_data_dir))?
                };
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
                let file_contents = match crate::read_file(&source) {
                    Ok(res) => res,
                    Err(e) => {
                        tracing::error!("Error reading file: {:?}", e);
                        continue;
                    }
                };
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
                        let res = SC2EventIterator::new(&source, versioned_abilities.clone())?;
                        println!("[");
                        for evt in res.into_iter().filter(|e| e.is_tracker_event()) {
                            json_print(serde_json::to_string(&evt).unwrap(), color);
                        }
                        println!("]");
                    }

                    ReadTypes::GameEvents => {
                        let res = SC2EventIterator::new(&source, versioned_abilities.clone())?;
                        println!("[");
                        for evt in res.into_iter().filter(|e| e.is_game_event()) {
                            json_print(serde_json::to_string(&evt).unwrap(), color);
                        }
                        println!("]");
                    }
                    ReadTypes::MessageEvents => {
                        let res = read_message_events(&source_path, &mpq, &file_contents)?;
                        println!("[");
                        for evt in res {
                            json_print(serde_json::to_string(&evt).unwrap(), color);
                        }
                        println!("]");
                    }
                    ReadTypes::Details => {
                        let evt = read_details(&source_path, &mpq, &file_contents)?;
                        json_print(serde_json::to_string(&evt).unwrap(), color);
                    }
                    ReadTypes::InitData => {
                        let evt = read_init_data(&source_path, &mpq, &file_contents)?;
                        json_print(serde_json::to_string(&evt).unwrap(), color);
                    }
                    ReadTypes::TransistEvents => {
                        tracing::info!("Transducing through both Game and Tracker Events");
                        let res = SC2EventIterator::new(&source, versioned_abilities.clone())?;
                        let filters = crate::filters::SC2ReplayFilters::from(cli.clone());
                        let res = res.with_filters(filters);
                        #[cfg(feature = "dep_ratatui")]
                        {
                            let details = read_details(&source_path, &mpq, &file_contents)?;
                            return Ok(crate::tui::ratatui_main(res, details)?);
                        }
                        #[cfg(not(feature = "dep_ratatui"))]
                        {
                            println!("[");
                            for evt in res.into_iter() {
                                json_print(serde_json::to_string(&evt).unwrap(), color);
                                println!(",");
                            }
                            println!("]");
                        }
                    }
                }
            }
        }
        #[cfg(feature = "dep_arrow")]
        Commands::WriteArrowIpc(cmd) => {
            let versioned_abilities: HashMap<(u32, String), VersionedBalanceUnit> =
                if cli.json_balance_data_dir.is_empty() {
                    read_balance_data_from_json(PathBuf::from(&cli.source))?
                } else {
                    read_balance_data_from_json(PathBuf::from(&cli.json_balance_data_dir))?
                };
            ArrowIpcTypes::handle_arrow_ipc_cmd(
                PathBuf::from(&cli.source),
                PathBuf::from(&cli.output.expect("Requires --output")),
                cmd,
                &versioned_abilities,
            )?;
        }
    }
    if cli.timing {
        println!("Total time: {:?}", init_time.elapsed());
    }
    Ok(())
}
