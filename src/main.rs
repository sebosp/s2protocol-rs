use std::path::PathBuf;

use clap::{Parser, Subcommand};
use nom_mpq::parser;
use s2protocol::generator::proto_morphist::ProtoMorphist;
use s2protocol::versions::read_details;
use s2protocol::versions::read_game_events;
use s2protocol::versions::read_init_data;
use s2protocol::versions::read_message_events;
use s2protocol::versions::read_tracker_events;

#[cfg(feature = "arrow")]
use arrow2::{array::Array, chunk::Chunk, datatypes::DataType};
#[cfg(feature = "arrow")]
use arrow2_convert::serialize::TryIntoArrow;
#[cfg(feature = "arrow")]
use polars::prelude::*;

#[derive(Subcommand)]
enum ArrowIpcTypes {
    /// Writes the tracker events to an Arrow IPC file
    TrackerEvents,
    /// Writes the game events to an Arrow IPC file
    GameEvents,
    /// Writes the message events to an Arrow IPC file
    MessageEvents,
    /// Writes the details to an Arrow IPC file
    Details,
}

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

fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Generate => {
            ProtoMorphist::gen(&cli.source, &cli.output.expect("Requires --output")).unwrap();
        }
        Commands::Get(read_type) => {
            let sources: Vec<PathBuf> = if PathBuf::from(&cli.source).is_dir() {
                let mut sources = Vec::new();
                for entry in std::fs::read_dir(&cli.source).unwrap() {
                    let entry = entry.unwrap();
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
                let (_input, mpq) = parser::parse(&file_contents).unwrap();
                match read_type {
                    ReadTypes::TrackerEvents => {
                        tracing::info!("Getting tracker events");
                        let res = read_tracker_events(&mpq, &file_contents).unwrap();
                        println!("[");
                        for evt in res {
                            println!("{},", serde_json::to_string(&evt).unwrap());
                        }
                        println!("]");
                    }

                    ReadTypes::GameEvents => {
                        tracing::info!("Getting game events");
                        let res = read_game_events(&mpq, &file_contents).unwrap();
                        println!("[");
                        for evt in res {
                            println!("{},", serde_json::to_string(&evt).unwrap());
                        }
                        println!("]");
                    }
                    ReadTypes::MessageEvents => {
                        tracing::info!("Getting message events");
                        let res = read_message_events(&mpq, &file_contents).unwrap();
                        println!("[");
                        for evt in res {
                            println!("{},", serde_json::to_string(&evt).unwrap());
                        }
                        println!("]");
                    }
                    ReadTypes::Details => {
                        tracing::info!("Getting details");
                        let res = read_details(&mpq, &file_contents).unwrap();
                        println!("{},", serde_json::to_string(&res).unwrap());
                    }
                    ReadTypes::InitData => {
                        tracing::info!("Getting initData");
                        let res = read_init_data(&mpq, &file_contents).unwrap();
                        println!("{},", serde_json::to_string(&res).unwrap());
                    }
                }
            }
        }
        Commands::WriteArrowIpc(_cmd) => {
            tracing::info!("Getting details");
            let sources: Vec<PathBuf> = if PathBuf::from(&cli.source).is_dir() {
                let mut sources = Vec::new();
                for entry in std::fs::read_dir(&cli.source).unwrap() {
                    let entry = entry.unwrap();
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
                let (_input, mpq) = parser::parse(&file_contents).unwrap();
                let res: Box<dyn Array> = read_details(&mpq, &file_contents)
                    .unwrap()
                    .player_list
                    .try_into_arrow()
                    .unwrap();
                // Make a sha of the file and add it to the Vec<s>
                #[cfg(feature = "arrow")]
                {
                    if let DataType::Struct(fields) = res.data_type() {
                        // let df = DataFrame::from_rows_and_schema(&[Chunk::new([res].to_vec())], &fields);
                        // println!("{}", df.head(Some(3)));
                        let fields = fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
                        println!(
                            "{},",
                            arrow2::io::print::write(&[Chunk::new([res].to_vec())], &fields)
                        );
                    }
                }
            }
        }
    }
}
