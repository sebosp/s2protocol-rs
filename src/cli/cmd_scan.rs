use super::*;
use crate::details::Details;
use crate::game_events::VersionedBalanceUnit;
use crate::game_events::ability::balance_data::json_handler::read_balance_data_from_included_assets;

use nom_mpq::parser;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SC2ReplaysDirStats {
    /// Total number of SC2Replay found in the directory recursively
    pub total_files: usize,
    /// Total number of replays with valid InitData
    pub total_supported_replays: usize,
    /// The number of replays that are supported by ability balance data
    pub ability_supported_replays: usize,
    /// Top 10 players by number of replays
    pub top_10_players: Vec<(String, usize)>,
    /// The top 10 maps
    pub top_10_maps: Vec<(String, usize)>,
}

impl SC2ReplaysDirStats {
    pub fn from_directory(dir_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let unit_abilities: HashMap<(u32, String), VersionedBalanceUnit> =
            read_balance_data_from_included_assets()?;
        scan_path(dir_path, &unit_abilities)
    }
}

pub fn handle_scan_cli_cmd(
    cli: &Cli,
    unit_abilities: &HashMap<(u32, String), VersionedBalanceUnit>,
) -> Result<SC2ReplaysDirStats, Box<dyn std::error::Error>> {
    scan_path(&cli.source, unit_abilities)
}

pub fn scan_path(
    path: &str,
    unit_abilities: &HashMap<(u32, String), VersionedBalanceUnit>,
) -> Result<SC2ReplaysDirStats, Box<dyn std::error::Error>> {
    let dir_path = PathBuf::from(path);
    tracing::info!("Scanning SC2 replays in directory: {:?}", dir_path);

    let sources = get_matching_files(dir_path.clone(), 1_000_000usize, 10usize)?;
    let mut stats = SC2ReplaysDirStats {
        total_files: sources.len(),
        total_supported_replays: 0,
        ability_supported_replays: 0,
        top_10_players: Vec::new(),
        top_10_maps: Vec::new(),
    };

    let mut user_freq: HashMap<String, usize> = HashMap::new();
    let mut map_freq: HashMap<String, usize> = HashMap::new();
    let versions_with_abilities: Vec<u32> =
        unit_abilities.keys().map(|(version, _)| *version).collect();

    let init_data_files: Vec<InitData> = sources
        .par_iter()
        .enumerate()
        .filter_map(|(index, source)| try_read_init_data(source, index as u64).ok())
        .collect();

    stats.total_supported_replays = init_data_files.len();

    for init_data in init_data_files {
        if versions_with_abilities.contains(&init_data.version) {
            stats.ability_supported_replays += 1;
        }
        let details: Details = match Details::try_from(&init_data) {
            Ok(details) => details,
            Err(_) => continue,
        };
        for user in details.get_player_names() {
            *user_freq.entry(user).or_insert(0) += 1;
        }
        *map_freq.entry(details.title).or_insert(0) += 1;
    }

    let mut user_freq_vec: Vec<(String, usize)> = user_freq.into_iter().collect();
    user_freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    stats.top_10_players = user_freq_vec.into_iter().take(10).collect();
    let mut map_freq_vec: Vec<(String, usize)> = map_freq.into_iter().collect();
    map_freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    stats.top_10_maps = map_freq_vec.into_iter().take(10).collect();

    Ok(stats)
}

pub fn try_read_init_data(
    source: &PathBuf,
    file_number: u64,
) -> Result<InitData, Box<dyn std::error::Error>> {
    let file_contents = crate::read_file(source)?;
    let (_, mpq) = parser::parse(&file_contents)?;
    let source_path: String = source
        .to_str()
        .expect("Failed to convert file name to str")
        .to_string();
    // NOTE: A fake "ext_fs_id" is created because the current impl is thought
    // of mainly for writing arrow ipc files... Maybe this is not a good idea.
    let init_data = InitData::new(&source_path, file_number, &mpq, &file_contents)?;
    Ok(init_data)
}
