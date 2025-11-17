use super::*;
pub mod cmd_get;
pub use cmd_get::*;

pub mod cmd_scan;
pub use cmd_scan::*;

#[cfg(feature = "syntax")]
use syntect::easy::HighlightLines;
#[cfg(feature = "syntax")]
use syntect::highlighting::{Color, Style, ThemeSet};
#[cfg(feature = "syntax")]
use syntect::parsing::SyntaxSet;
#[cfg(feature = "syntax")]
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

use crate::game_events::VersionedBalanceUnit;
use crate::game_events::ability::balance_data::json_handler::read_balance_data_from_included_assets;
use crate::game_events::ability::balance_data::json_handler::read_balance_data_from_json_dir;

use crate::game_events::ability::traverse_versioned_balance_abilities;
use crate::generator::proto_morphist::ProtoMorphist;
use crate::tracker_events::{unit_tag_index, unit_tag_recycle};

#[cfg(feature = "dep_arrow")]
use clap::Args;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand, Debug, Clone)]
pub enum CommandUtils {
    /// Translate unit tag to index, recycle pair
    XlateTagToIndexRecycle { tag: i64 },
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Generates Rust code for a specific protocol.
    Generate,

    /// Generates static json files for a specific Balance Data export.
    ///
    BalanceDataToJson,

    /// Scan a direcctory for SC2Replay files and returns basic information
    Scan,

    /// Gets a specific event type from the SC2Replay MPQ Archive
    #[command(subcommand)]
    Get(ReadTypes),

    /// Writes Arrow IPC files for a specific event type from the SC2Replay MPQ Archive
    #[cfg(feature = "dep_arrow")]
    WriteArrowIpc(WriteArrowIpcProps),

    /// Utilities, transforming tag index, recycle, etc.
    #[command(subcommand)]
    Util(CommandUtils),
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

    /// Quiet allows debugging without emitting events in json/etc.
    #[arg(long, default_value = "false")]
    pub quiet: bool,
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
            } else if let Some(ext) = path.extension()
                && ext == "SC2Replay"
                && path.is_file()
            {
                if sources.len() >= max_files {
                    break;
                }
                sources.push(path);
            }
        }
        Ok(sources)
    } else {
        Ok(vec![source.clone()])
    }
}

/// Prints the json strings with syntect::easy
#[cfg(feature = "syntax")]
pub fn syntect_json_highlight<'a>(
    json_str: &'a str,
    syntect_syntax_set: &SyntaxSet,
    syntect_theme_set: &ThemeSet,
) -> Vec<(Style, &'a str)> {
    let mut res: Vec<(Style, &str)> = vec![];
    let syntax = syntect_syntax_set.find_syntax_by_extension("json").unwrap();
    let mut highlighter =
        HighlightLines::new(syntax, &syntect_theme_set.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(json_str) {
        // LinesWithEndings enables use of newlines mode
        let ranges: Vec<(Style, &str)> = highlighter
            .highlight_line(line, syntect_syntax_set)
            .unwrap();
        res.extend(ranges);
    }
    res
}

/// Prints the json strings with syntect::easy
#[cfg(feature = "syntax")]
pub fn syntect_json_print(
    json_str: String,
    syntect_syntax_set: &SyntaxSet,
    syntect_theme_set: &ThemeSet,
) {
    syntect_json_highlight(&json_str, syntect_syntax_set, syntect_theme_set)
        .iter()
        .for_each(|segment| {
            print!("{}", as_24_bit_terminal_escaped(&[*segment], false));
        });
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
    #[cfg(feature = "syntax")]
    let syntect_syntax_set = SyntaxSet::load_defaults_newlines();
    #[cfg(feature = "syntax")]
    let mut syntect_theme_set = ThemeSet::load_defaults();
    #[cfg(feature = "syntax")]
    {
        syntect_theme_set
            .themes
            .get_mut("base16-ocean.dark")
            .unwrap()
            .settings
            .background = Some(Color {
            r: 0x00,
            g: 0x00,
            b: 0x00,
            a: 0x00,
        });
    }

    let versioned_abilities: HashMap<(u32, String), VersionedBalanceUnit> =
        if cli.json_balance_data_dir.is_empty() {
            read_balance_data_from_included_assets()?
        } else {
            read_balance_data_from_json_dir(PathBuf::from(&cli.json_balance_data_dir))?
        };
    match &cli.command {
        Commands::Generate => {
            ProtoMorphist::r#gen(&cli.source, &cli.output.expect("Requires --output"))?;
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
            cmd_get::handle_get_cmd(
                &cli,
                read_type,
                #[cfg(feature = "syntax")]
                &syntect_syntax_set,
                #[cfg(feature = "syntax")]
                &syntect_theme_set,
            )?;
        }
        #[cfg(feature = "dep_arrow")]
        Commands::WriteArrowIpc(cmd) => {
            ArrowIpcTypes::handle_arrow_ipc_cmd(
                PathBuf::from(&cli.source),
                PathBuf::from(&cli.output.expect("Requires --output")),
                cmd,
                &versioned_abilities,
            )?;
        }
        Commands::Util(cmd) => match cmd {
            CommandUtils::XlateTagToIndexRecycle { tag } => {
                let index = unit_tag_index(*tag);
                let recycle = unit_tag_recycle(*tag);
                println!("Index: {}, Recycle: {}", index, recycle)
            }
        },
        Commands::Scan => {
            let stats = handle_scan_cli_cmd(&cli, &versioned_abilities)?;
            tracing::info!("Scan complete: {:?}", stats);
        }
    }
    if cli.timing {
        println!("Total time: {:?}", init_time.elapsed());
    }
    Ok(())
}
