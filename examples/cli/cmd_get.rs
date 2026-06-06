use super::*;
use s2protocol::game_events::VersionedBalanceUnit;
use s2protocol::game_events::ability::balance_data::json_handler::*;
use s2protocol::read_details;
use s2protocol::read_message_events;
use s2protocol::state::SC2EventIterator;
#[path = "../tui/mod.rs"]
mod tui;

use nom_mpq::parser;
use std::path::PathBuf;

#[derive(Subcommand, Debug, Clone)]
pub enum ReadTypes {
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

pub fn handle_get_cmd(
    cli: &Cli,
    read_type: &ReadTypes,
    syntect_syntax_set: SyntaxSet,
    syntect_theme_set: ThemeSet,
) -> Result<(), Box<dyn std::error::Error>> {
    let versioned_abilities: HashMap<(u32, String), VersionedBalanceUnit> =
        if cli.json_balance_data_dir.is_empty() {
            read_balance_data_from_included_assets()?
        } else {
            read_balance_data_from_json_dir(PathBuf::from(&cli.json_balance_data_dir))?
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
        let file_contents = match s2protocol::read_file(&source) {
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
            .to_str()
            .expect("Failed to convert file name to str")
            .to_string();
        // NOTE: A fake "ext_fs_id" is created because the current impl is thought
        // of mainly for writing arrow ipc files... Maybe this is not a good idea.
        let init_data = s2protocol::InitData::new(&source_path, 0u64, &mpq, &file_contents)?;
        match read_type {
            ReadTypes::TrackerEvents => {
                let res = SC2EventIterator::new(&init_data, versioned_abilities.clone())?;
                println!("[");
                for evt in res.into_iter().filter(|e| e.is_tracker_event()) {
                    syntect_json_print(
                        serde_json::to_string(&evt).unwrap(),
                        &syntect_syntax_set,
                        &syntect_theme_set,
                    );
                }
                println!("]");
            }

            ReadTypes::GameEvents => {
                let res = SC2EventIterator::new(&init_data, versioned_abilities.clone())?;
                println!("[");
                for evt in res.into_iter().filter(|e| e.is_game_event()) {
                    syntect_json_print(
                        serde_json::to_string(&evt).unwrap(),
                        &syntect_syntax_set,
                        &syntect_theme_set,
                    );
                }
                println!("]");
            }
            ReadTypes::MessageEvents => {
                let res = read_message_events(&source_path, &mpq, &file_contents)?;
                println!("[");
                for evt in res {
                    syntect_json_print(
                        serde_json::to_string_pretty(&evt).unwrap(),
                        &syntect_syntax_set,
                        &syntect_theme_set,
                    );
                }
                println!("]");
            }
            ReadTypes::Details => {
                let evt = read_details(&source_path, &mpq, &file_contents)?;
                syntect_json_print(
                    serde_json::to_string_pretty(&evt).unwrap(),
                    &syntect_syntax_set,
                    &syntect_theme_set,
                );
            }
            ReadTypes::InitData => {
                syntect_json_print(
                    serde_json::to_string_pretty(&init_data).unwrap(),
                    &syntect_syntax_set,
                    &syntect_theme_set,
                );
            }
            ReadTypes::TransistEvents => {
                tracing::info!("Transducing through both Game and Tracker Events");
                let res = SC2EventIterator::new(&init_data, versioned_abilities.clone())?;
                let filters = s2protocol::filters::SC2ReplayFilters::from(cli.clone());
                let res = res.with_filters(filters);
                if cli.tui {
                    let details = read_details(&source_path, &mpq, &file_contents)?;
                    return Ok(tui::ratatui_main(
                        res,
                        details,
                        init_data,
                        syntect_syntax_set,
                        syntect_theme_set,
                    )?);
                }
                if !cli.quiet {
                    println!("[");
                }
                for evt in res.into_iter() {
                    if !cli.quiet {
                        syntect_json_print(
                            serde_json::to_string(&evt).unwrap(),
                            &syntect_syntax_set,
                            &syntect_theme_set,
                        );

                        println!(",");
                    }
                }
                if !cli.quiet {
                    println!("]");
                }
            }
        }
    }
    Ok(())
}
