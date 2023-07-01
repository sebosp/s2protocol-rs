use clap::{Parser, Subcommand};
use nom_mpq::parser;
use s2protocol::generator::proto_morphist::ProtoMorphist;
use s2protocol::versions::read_game_events;
use s2protocol::versions::read_tracker_events;

#[derive(Subcommand)]
enum Commands {
    /// Generates a Rust file for a specific protocol.
    Generate {
        /// Generate a specific filename
        #[arg(short, long)]
        output: String,
    },
    /// Gets the tracker events from the file
    GetTrackerEvents,
    /// Gets the game events from the file
    GetGameEvents,
}
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    source: String,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Generate { output } => {
            ProtoMorphist::gen(&cli.source, output).unwrap();
        }
        Commands::GetTrackerEvents => {
            tracing::info!("Getting tracker events");
            let file_contents = parser::read_file(&cli.source);
            let (_input, mpq) = parser::parse(&file_contents).unwrap();
            let res = read_tracker_events(&mpq, &file_contents);
            println!("[");
            for evt in res {
                println!("{},", serde_json::to_string(&evt).unwrap());
            }
            println!("]");
        }
        Commands::GetGameEvents => {
            tracing::info!("Getting game events");
            let file_contents = parser::read_file(&cli.source);
            let (_input, mpq) = parser::parse(&file_contents).unwrap();
            let res = read_game_events(&mpq, &file_contents);
            println!("[");
            for evt in res {
                println!("{},", serde_json::to_string(&evt).unwrap());
            }
            println!("]");
        }
    }
}
