//! The commands of the replays of a directory that the balance data does not name, by protocol
//! version, ability link and command index, with the units that gave them:
//!
//! S2PROTOCOL_TEST_REPLAYS=~/SC2Replays/aiarena_net S2PROTOCOL_TEST_MAX_FILES=200 \
//!   cargo test --release --test ability_names -- --ignored --nocapture

use s2protocol::basic_replay_data::SC2ReplayBasicData;
use s2protocol::game_events::ability::balance_data::json_handler::read_balance_data_from_included_assets;
use s2protocol::{SC2EventIterator, SC2EventType, UnitChangeHint};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

#[derive(Default)]
struct Unnamed {
    commands: usize,
    units: HashMap<String, usize>,
}

#[test]
#[ignore]
fn ability_names_of_replays() {
    let source = PathBuf::from(std::env::var("S2PROTOCOL_TEST_REPLAYS").unwrap());
    let max_files = std::env::var("S2PROTOCOL_TEST_MAX_FILES")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(100);
    let balance = read_balance_data_from_included_assets().unwrap();
    let files = s2protocol::get_matching_files(source, max_files, 8).unwrap();
    // By version: the commands, the named ones, and the unnamed ones by link and command index.
    let mut totals: BTreeMap<u32, (usize, usize)> = BTreeMap::new();
    let mut unnamed: BTreeMap<(u32, u16, i64), Unnamed> = BTreeMap::new();
    let mut games: BTreeMap<u32, usize> = BTreeMap::new();
    for (idx, file) in files.iter().enumerate() {
        let Ok(basic_data) = SC2ReplayBasicData::new(file, idx as u64) else {
            continue;
        };
        let version = basic_data.init_data.version;
        let Ok(iterator) = SC2EventIterator::new(&basic_data, &balance) else {
            continue;
        };
        *games.entry(version).or_default() += 1;
        for item in iterator {
            let SC2EventType::Game { .. } = item.event_type else {
                continue;
            };
            let UnitChangeHint::Abilities { units, event, .. } = item.change_hint else {
                continue;
            };
            let Some(ability) = event.m_abil else {
                continue;
            };
            let total = totals.entry(version).or_default();
            total.0 += 1;
            if !ability.ability.is_empty() {
                total.1 += 1;
                continue;
            }
            let entry = unnamed
                .entry((version, ability.m_abil_link, ability.m_abil_cmd_index))
                .or_default();
            entry.commands += 1;
            for unit in units {
                *entry.units.entry(unit.name).or_default() += 1;
            }
        }
    }
    for (version, (commands, named)) in &totals {
        println!(
            "{version}: {} games, {commands} commands, {named} named ({:.1}%)",
            games[version],
            100. * *named as f64 / (*commands).max(1) as f64
        );
        let mut missing: Vec<_> = unnamed
            .iter()
            .filter(|((unnamed_version, _, _), _)| unnamed_version == version)
            .collect();
        missing.sort_by_key(|(_, entry)| std::cmp::Reverse(entry.commands));
        for ((_, link, cmd_index), entry) in missing.iter().take(25) {
            let mut units: Vec<_> = entry.units.iter().collect();
            units.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
            let units: Vec<String> = units
                .iter()
                .take(3)
                .map(|(name, count)| format!("{name}:{count}"))
                .collect();
            println!(
                "  link {link:>4} index {cmd_index:>2}: {:>7} unnamed, units {}",
                entry.commands,
                units.join(", ")
            );
        }
    }
}
