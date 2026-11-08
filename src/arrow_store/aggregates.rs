//! The aggregations of a snapshot: what the views need from the 600 million rows of the event
//! files, written once when the snapshot changes.
//!
//! Every day partition has the facts of its games, a row per game, player and ability or unit:
//! [`GAME_ABILITIES_ARROW_FNAME`] and [`GAME_UNITS_ARROW_FNAME`], and [`GAME_PLAYERS_ARROW_FNAME`]
//! the ids of the players and the length of the games, [`GAME_MINUTES_ARROW_FNAME`] the economy at
//! each minute. Every day, month and year has
//! the same usage tables, per period, protocol version, race, player and ability or unit:
//! [`ABILITY_USAGE_ARROW_FNAME`], [`UNIT_USAGE_ARROW_FNAME`] and [`PLAYER_USAGE_ARROW_FNAME`]:
//!
//! ```text
//! ipcs/year=2026/ability_usage.arrow                  the games of the year
//! ipcs/year=2026/month=10/ability_usage.arrow         the games of the month
//! ipcs/year=2026/month=10/day=15/ability_usage.arrow  the games of the day
//! ipcs/year=2026/month=10/day=15/game_abilities.arrow the games, one by one
//! ```
//!
//! A day is computed from its files, so that the aggregations can be written without reading the
//! replays again; a month from its days and a year from its months, as all the columns add up.
//! An [`AGGREGATES_METADATA_FNAME`] written last in each directory tells what it was computed
//! from, [`find_gaps`] compares it with the current days.

use super::ipc_writer::write_batches;
use super::partitioned::key_dirs;
use super::rows::{ArrowRow, TryIntoArrow};
use crate::snapshot::{SnapshotMetadata, SnapshotProgress};
use arrow::array::{Array, AsArray, RecordBatch, StringArray, StructArray};
use arrow::datatypes::{Int32Type, Int64Type, UInt8Type, UInt32Type, UInt64Type};
use arrow::ipc::reader::FileReader;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// A row per game, player and ability, see [`GameAbilityRow`].
pub const GAME_ABILITIES_ARROW_FNAME: &str = "game_abilities.arrow";
/// A row per game, player and unit, see [`GameUnitRow`].
pub const GAME_UNITS_ARROW_FNAME: &str = "game_units.arrow";
/// A row per game and player, see [`GamePlayerRow`].
pub const GAME_PLAYERS_ARROW_FNAME: &str = "game_players.arrow";
/// A row per game, player and minute, see [`GameMinuteRow`].
pub const GAME_MINUTES_ARROW_FNAME: &str = "game_minutes.arrow";
/// A row per period, version, race, player and ability, see [`AbilityUsageRow`].
pub const ABILITY_USAGE_ARROW_FNAME: &str = "ability_usage.arrow";
/// A row per period, version, race, player and unit, see [`UnitUsageRow`].
pub const UNIT_USAGE_ARROW_FNAME: &str = "unit_usage.arrow";
/// A row per period, version, race and player, see [`PlayerUsageRow`].
pub const PLAYER_USAGE_ARROW_FNAME: &str = "player_usage.arrow";
/// What the aggregations of a directory were computed from, written last.
pub const AGGREGATES_METADATA_FNAME: &str = "aggregates.json";
/// Changes when the aggregations change without a new version of the snapshot files.
pub const AGGREGATES_VERSION: u32 = 1;

/// The glob of the table at a [`Grain`], relative to the snapshot root.
pub fn aggregate_glob(grain: Grain, file_name: &str) -> String {
    match grain {
        Grain::Day => format!("year=*/month=*/day=*/{file_name}"),
        Grain::Month => format!("year=*/month=*/{file_name}"),
        Grain::Year => format!("year=*/{file_name}"),
    }
}

/// The periods of the usage tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Grain {
    Day,
    Month,
    Year,
}

/// The uses of an ability by a player in a game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameAbilityRow {
    pub ext_fs_id: u64,
    pub player_name: String,
    pub ability: String,
    /// The commands given with the ability.
    pub uses: u32,
    /// The game second of the first command.
    pub first_seconds: u32,
}
crate::impl_arrow_row!(GameAbilityRow);

/// A unit type of a player in a game, the variants (burrowed, sieged, ...) are the same unit, see
/// [`base_unit_name`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameUnitRow {
    pub ext_fs_id: u64,
    pub player_name: String,
    pub unit: String,
    /// The units made, each unit once for each base type: burrowing, sieging, etc register the
    /// same unit again.
    pub born: u32,
    /// The units of this type of the player that died.
    pub died: u32,
    /// The enemy units that cost resources killed by this type of unit.
    pub kills: u32,
    /// The minerals and vespene of those units, with the costs of the newest balance data.
    pub value_killed: u64,
}
crate::impl_arrow_row!(GameUnitRow);

/// A player of a game: the tracker events have its id, the details its name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GamePlayerRow {
    pub ext_fs_id: u64,
    pub player_name: String,
    pub player_id: u8,
    /// The second of the last stats of the game, the length of the game.
    pub game_seconds: u32,
}
crate::impl_arrow_row!(GamePlayerRow);

/// The economy and army of a player at a minute of a game: the last stats of the
/// [`MINUTE_WINDOW_SECONDS`] before the minute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameMinuteRow {
    pub ext_fs_id: u64,
    pub player_id: u8,
    pub minute: u32,
    pub workers: i32,
    pub supply: i32,
    pub minerals_rate: i32,
    pub vespene_rate: i32,
    pub army_value: i32,
}
crate::impl_arrow_row!(GameMinuteRow);

/// The stats are written every 10 seconds.
pub const MINUTE_WINDOW_SECONDS: i64 = 15;

/// How an ability was used in a period.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbilityUsageRow {
    /// `2026`, `2026-10` or `2026-10-15`.
    pub period: String,
    pub ext_protocol_version: u32,
    /// As in the details, the race names are localized.
    pub player_race: String,
    /// `region-realm-id` of the toon handle, empty for the computers.
    pub account: String,
    /// The name without the clan tag.
    pub player: String,
    pub ability: String,
    /// The games of the player with the ability.
    pub games: u32,
    /// Those games won.
    pub wins: u32,
    pub uses: u64,
}
crate::impl_arrow_row!(AbilityUsageRow);

/// How a unit was used in a period.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnitUsageRow {
    pub period: String,
    pub ext_protocol_version: u32,
    pub player_race: String,
    pub account: String,
    pub player: String,
    pub unit: String,
    /// The games in which the player made the unit.
    pub games: u32,
    /// Those games won.
    pub wins: u32,
    pub born: u64,
    pub died: u64,
    pub kills: u64,
    pub value_killed: u64,
}
crate::impl_arrow_row!(UnitUsageRow);

/// The games of a player in a period, what the uses are divided by.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerUsageRow {
    pub period: String,
    pub ext_protocol_version: u32,
    pub player_race: String,
    pub account: String,
    pub player: String,
    pub games: u32,
    pub wins: u32,
    /// The 1v1 games against another race, a mirror always has a winner and a loser.
    pub versus_games: u32,
    /// Those games won.
    pub versus_wins: u32,
}
crate::impl_arrow_row!(PlayerUsageRow);

/// What a directory of aggregations was computed from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AggregatesMetadata {
    aggregates_version: u32,
    source: String,
}

/// The directories whose aggregations are missing or older than their days.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AggregateGaps {
    pub days: Vec<PathBuf>,
    pub months: Vec<PathBuf>,
    pub years: Vec<PathBuf>,
}

impl AggregateGaps {
    pub fn is_empty(&self) -> bool {
        self.days.is_empty() && self.months.is_empty() && self.years.is_empty()
    }
}

/// The unit name variants, such as burrowed or sieged, are the unit of the balance data.
pub fn base_unit_name(name: &str) -> &str {
    match name {
        "SiegeTankSieged" => "SiegeTank",
        "VikingAssault" => "VikingFighter",
        "LiberatorAG" => "Liberator",
        "ThorAP" => "Thor",
        "WarpPrismPhasing" => "WarpPrism",
        "ObserverSiegeMode" => "Observer",
        "OverseerSiegeMode" => "Overseer",
        _ => name.strip_suffix("Burrowed").unwrap_or(name),
    }
}

/// The minerals and vespene of the units that cost resources, with the newest balance data.
/// Zerglings are produced, and costed, in pairs.
fn unit_values() -> &'static HashMap<String, u64> {
    static VALUES: OnceLock<HashMap<String, u64>> = OnceLock::new();
    VALUES.get_or_init(|| {
        let balance =
            crate::game_events::read_balance_data_from_included_assets().unwrap_or_default();
        let Some((_, units)) = balance.iter().max_by_key(|(version, _)| **version) else {
            return HashMap::new();
        };
        units
            .iter()
            .filter_map(|(name, unit)| {
                let cost = unit.cost.as_ref()?;
                let (minerals, vespene) = (cost.minerals.unwrap_or(0), cost.vespene.unwrap_or(0));
                if minerals == 0 && vespene == 0 {
                    return None;
                }
                let per_cost = if name == "Zergling" { 2 } else { 1 };
                let value = (minerals / per_cost + vespene / per_cost) as u64;
                Some((name.clone(), value))
            })
            .collect()
    })
}

/// The names of a day as numbers, the rows of a day repeat a few of them millions of times.
#[derive(Default)]
struct Names {
    ids: HashMap<String, u32>,
    names: Vec<String>,
}

impl Names {
    fn id(&mut self, name: &str) -> u32 {
        if let Some(id) = self.ids.get(name) {
            return *id;
        }
        let id = self.names.len() as u32;
        self.ids.insert(name.to_string(), id);
        self.names.push(name.to_string());
        id
    }

    fn name(&self, id: u32) -> &str {
        &self.names[id as usize]
    }
}

type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// Calls `read` with the batches of the columns of a file; a file that does not exist has none.
fn read_columns(
    path: &Path,
    columns: &[&str],
    mut read: impl FnMut(&RecordBatch) -> Result<(), BoxError>,
) -> Result<(), BoxError> {
    if !path.is_file() {
        return Ok(());
    }
    let schema = FileReader::try_new(std::fs::File::open(path)?, None)?.schema();
    let projection = columns
        .iter()
        .map(|name| schema.index_of(name))
        .collect::<Result<Vec<_>, _>>()?;
    for batch in FileReader::try_new(std::fs::File::open(path)?, Some(projection))? {
        read(&batch?)?;
    }
    Ok(())
}

fn column<'a>(batch: &'a RecordBatch, name: &str) -> Result<&'a dyn Array, BoxError> {
    Ok(batch
        .column_by_name(name)
        .ok_or_else(|| format!("no column {name}"))?
        .as_ref())
}

fn strings<'a>(batch: &'a RecordBatch, name: &str) -> Result<&'a StringArray, BoxError> {
    column(batch, name)?
        .as_string_opt::<i32>()
        .ok_or_else(|| format!("{name} is not a string").into())
}

/// A player of a game.
#[derive(Debug, Clone)]
struct GamePlayer {
    version: u32,
    race: String,
    account: String,
    player: String,
    won: bool,
}

/// The players of the games of a day, not the observers, by game and name in the details.
fn read_players(day: &Path) -> Result<HashMap<(u64, String), GamePlayer>, BoxError> {
    let mut players = HashMap::new();
    let path = day.join(super::DETAILS_ARROW_FNAME);
    let columns = [
        "ext_fs_id",
        "player_name",
        "player_race",
        "player_result",
        "player_observe",
        "player_toon_region",
        "player_toon_realm",
        "player_toon_id",
        "ext_protocol_version",
    ];
    read_columns(&path, &columns, |batch| {
        let ids = column(batch, "ext_fs_id")?.as_primitive::<UInt64Type>();
        let names = strings(batch, "player_name")?;
        let races = strings(batch, "player_race")?;
        let results = strings(batch, "player_result")?;
        let observe = column(batch, "player_observe")?.as_primitive::<UInt8Type>();
        let regions = column(batch, "player_toon_region")?.as_primitive::<UInt8Type>();
        let realms = column(batch, "player_toon_realm")?.as_primitive::<UInt32Type>();
        let toons = column(batch, "player_toon_id")?.as_primitive::<UInt64Type>();
        let versions = column(batch, "ext_protocol_version")?.as_primitive::<UInt32Type>();
        for row in 0..batch.num_rows() {
            if observe.value(row) != 0 {
                continue;
            }
            let name = names.value(row);
            let account = match toons.value(row) {
                0 => String::new(),
                toon => format!("{}-{}-{toon}", regions.value(row), realms.value(row)),
            };
            players.insert(
                (ids.value(row), name.to_string()),
                GamePlayer {
                    version: versions.value(row),
                    race: races.value(row).to_string(),
                    account,
                    player: name.rsplit("<sp/>").next().unwrap_or(name).to_string(),
                    won: results.value(row) == "Win",
                },
            );
        }
        Ok(())
    })?;
    Ok(players)
}

/// The commands of each ability of each player of each game; a row of the cmd files is a command.
fn game_abilities(day: &Path) -> Result<Vec<GameAbilityRow>, BoxError> {
    let mut names = Names::default();
    let mut uses: HashMap<(u64, u32, u32), (u32, u32)> = HashMap::new();
    for file in [
        super::CMD_TARGET_POINT_ARROW_FNAME,
        super::CMD_TARGET_UNIT_ARROW_FNAME,
    ] {
        let columns = ["ext_fs_id", "player_name", "ability", "ext_replay_seconds"];
        read_columns(&day.join(file), &columns, |batch| {
            let ids = column(batch, "ext_fs_id")?.as_primitive::<UInt64Type>();
            let players = strings(batch, "player_name")?;
            let abilities = strings(batch, "ability")?;
            let seconds = column(batch, "ext_replay_seconds")?.as_primitive::<UInt32Type>();
            for row in 0..batch.num_rows() {
                let ability = abilities.value(row);
                if players.is_null(row) || ability.is_empty() {
                    continue;
                }
                let key = (
                    ids.value(row),
                    names.id(players.value(row)),
                    names.id(ability),
                );
                let entry = uses.entry(key).or_insert((0, u32::MAX));
                entry.0 += 1;
                entry.1 = entry.1.min(seconds.value(row));
            }
            Ok(())
        })?;
    }
    let mut rows: Vec<GameAbilityRow> = uses
        .into_iter()
        .map(
            |((ext_fs_id, player, ability), (uses, first_seconds))| GameAbilityRow {
                ext_fs_id,
                player_name: names.name(player).to_string(),
                ability: names.name(ability).to_string(),
                uses,
                first_seconds,
            },
        )
        .collect();
    rows.sort_by(|a, b| {
        (a.ext_fs_id, &a.player_name, &a.ability).cmp(&(b.ext_fs_id, &b.player_name, &b.ability))
    });
    Ok(rows)
}

/// The units made, lost and their kills, per game, player and unit type. The players of the
/// tracker events are their ids, the births tell the name of each id and the owner of each unit.
fn game_units(
    day: &Path,
    durations: &HashMap<u64, u32>,
) -> Result<(Vec<GameUnitRow>, Vec<GamePlayerRow>), BoxError> {
    #[derive(Default)]
    struct Counts {
        born: u32,
        died: u32,
        kills: u32,
        value_killed: u64,
    }
    let mut names = Names::default();
    let mut counts: HashMap<(u64, u32, u32), Counts> = HashMap::new();
    let mut player_ids: HashMap<(u64, u8), u32> = HashMap::new();
    let mut owners: HashMap<(u64, u32, u32), Option<u8>> = HashMap::new();
    let mut seen: HashSet<(u64, u32, u32, u32)> = HashSet::new();
    let columns = [
        "ext_fs_id",
        "player_name",
        "control_player_id",
        "unit_tag_index",
        "unit_tag_recycle",
        "unit_type_name",
    ];
    read_columns(&day.join(super::UNIT_BORN_ARROW_FNAME), &columns, |batch| {
        let ids = column(batch, "ext_fs_id")?.as_primitive::<UInt64Type>();
        let players = strings(batch, "player_name")?;
        let controls = column(batch, "control_player_id")?.as_primitive::<UInt8Type>();
        let tags = column(batch, "unit_tag_index")?.as_primitive::<UInt32Type>();
        let recycles = column(batch, "unit_tag_recycle")?.as_primitive::<UInt32Type>();
        let units = strings(batch, "unit_type_name")?;
        for row in 0..batch.num_rows() {
            let game = ids.value(row);
            let tag = (game, tags.value(row), recycles.value(row));
            let control = (!controls.is_null(row)).then(|| controls.value(row));
            owners.entry(tag).or_insert(control);
            if players.is_null(row) {
                continue;
            }
            let player = names.id(players.value(row));
            if let Some(control) = control {
                player_ids.entry((game, control)).or_insert(player);
            }
            let unit = names.id(base_unit_name(units.value(row)));
            if seen.insert((tag.0, tag.1, tag.2, unit)) {
                counts.entry((game, player, unit)).or_default().born += 1;
            }
        }
        Ok(())
    })?;
    let values = unit_values();
    let columns = [
        "ext_fs_id",
        "unit_tag_index",
        "unit_tag_recycle",
        "killer_player_id",
        "unit_killer_name",
        "unit_died_name",
    ];
    read_columns(&day.join(super::UNIT_DIED_ARROW_FNAME), &columns, |batch| {
        let ids = column(batch, "ext_fs_id")?.as_primitive::<UInt64Type>();
        let tags = column(batch, "unit_tag_index")?.as_primitive::<UInt32Type>();
        let recycles = column(batch, "unit_tag_recycle")?.as_primitive::<UInt32Type>();
        let killers = column(batch, "killer_player_id")?.as_primitive::<UInt8Type>();
        let killer_units = strings(batch, "unit_killer_name")?;
        let died_units = strings(batch, "unit_died_name")?;
        for row in 0..batch.num_rows() {
            let game = ids.value(row);
            let owner = owners
                .get(&(game, tags.value(row), recycles.value(row)))
                .copied()
                .flatten();
            let died = base_unit_name(died_units.value(row));
            if let Some(player) = owner.and_then(|owner| player_ids.get(&(game, owner))) {
                let unit = names.id(died);
                counts.entry((game, *player, unit)).or_default().died += 1;
            }
            if killers.is_null(row) || killer_units.value(row).is_empty() {
                continue;
            }
            let killer = killers.value(row);
            let (Some(player), Some(value), false) = (
                player_ids.get(&(game, killer)).copied(),
                values.get(died),
                owner == Some(killer),
            ) else {
                continue;
            };
            let unit = names.id(base_unit_name(killer_units.value(row)));
            let entry = counts.entry((game, player, unit)).or_default();
            entry.kills += 1;
            entry.value_killed += value;
        }
        Ok(())
    })?;
    let mut rows: Vec<GameUnitRow> = counts
        .into_iter()
        .map(|((ext_fs_id, player, unit), counts)| GameUnitRow {
            ext_fs_id,
            player_name: names.name(player).to_string(),
            unit: names.name(unit).to_string(),
            born: counts.born,
            died: counts.died,
            kills: counts.kills,
            value_killed: counts.value_killed,
        })
        .collect();
    rows.sort_by(|a, b| {
        (a.ext_fs_id, &a.player_name, &a.unit).cmp(&(b.ext_fs_id, &b.player_name, &b.unit))
    });
    let mut players: Vec<GamePlayerRow> = player_ids
        .into_iter()
        .map(|((ext_fs_id, player_id), player)| GamePlayerRow {
            ext_fs_id,
            player_name: names.name(player).to_string(),
            player_id,
            game_seconds: durations.get(&ext_fs_id).copied().unwrap_or_default(),
        })
        .collect();
    players.sort_by_key(|row| (row.ext_fs_id, row.player_id));
    Ok((rows, players))
}

/// The length of each game and the stats of its minutes, from the stats of the day.
fn game_stats(day: &Path) -> Result<(HashMap<u64, u32>, Vec<GameMinuteRow>), BoxError> {
    let mut last_loops: HashMap<u64, i64> = HashMap::new();
    let mut minutes: HashMap<(u64, u8, u32), (i64, GameMinuteRow)> = HashMap::new();
    let columns = [
        "ext_fs_id",
        "player_id",
        "ext_replay_loop",
        "workers_active_count",
        "food_used",
        "minerals_collection_rate",
        "vespene_collection_rate",
        "minerals_used_current_army",
        "vespene_used_current_army",
    ];
    read_columns(&day.join(super::STATS_ARROW_FNAME), &columns, |batch| {
        let ids = column(batch, "ext_fs_id")?.as_primitive::<UInt64Type>();
        let players = column(batch, "player_id")?.as_primitive::<UInt8Type>();
        let loops = column(batch, "ext_replay_loop")?.as_primitive::<Int64Type>();
        let int = |name: &str| -> Result<&arrow::array::Int32Array, BoxError> {
            Ok(column(batch, name)?.as_primitive::<Int32Type>())
        };
        let workers = int("workers_active_count")?;
        let supply = int("food_used")?;
        let minerals_rate = int("minerals_collection_rate")?;
        let vespene_rate = int("vespene_collection_rate")?;
        let army_minerals = int("minerals_used_current_army")?;
        let army_vespene = int("vespene_used_current_army")?;
        for row in 0..batch.num_rows() {
            let game_loop = loops.value(row);
            let last_loop = last_loops.entry(ids.value(row)).or_default();
            *last_loop = game_loop.max(*last_loop);
            // The minute at or after the stats, if they are in the window before it.
            let seconds = crate::convert_game_loop_to_seconds(game_loop) as i64;
            let minute = (seconds + 59) / 60;
            if minute == 0 || minute * 60 - seconds >= MINUTE_WINDOW_SECONDS {
                continue;
            }
            let key = (ids.value(row), players.value(row), minute as u32);
            let is_later = minutes
                .get(&key)
                .is_none_or(|(sample_loop, _)| game_loop > *sample_loop);
            if is_later {
                minutes.insert(
                    key,
                    (
                        game_loop,
                        GameMinuteRow {
                            ext_fs_id: key.0,
                            player_id: key.1,
                            minute: key.2,
                            workers: workers.value(row),
                            supply: supply.value(row),
                            minerals_rate: minerals_rate.value(row),
                            vespene_rate: vespene_rate.value(row),
                            army_value: army_minerals.value(row) + army_vespene.value(row),
                        },
                    ),
                );
            }
        }
        Ok(())
    })?;
    let durations = last_loops
        .into_iter()
        .map(|(id, game_loop)| (id, crate::convert_game_loop_to_seconds(game_loop)))
        .collect();
    let mut rows: Vec<GameMinuteRow> = minutes.into_values().map(|(_, row)| row).collect();
    rows.sort_by_key(|row| (row.ext_fs_id, row.player_id, row.minute));
    Ok((durations, rows))
}

/// Writes the rows, next to the final file until they are complete. A table without rows is
/// written too, a glob over the directories finds all of them.
fn write_rows<T: ArrowRow>(dir: &Path, name: &str, rows: &[T]) -> Result<(), BoxError> {
    let array = rows
        .try_into_arrow()
        .map_err(|err| format!("{name}: {err}"))?;
    let batch: RecordBatch = array
        .as_any()
        .downcast_ref::<StructArray>()
        .ok_or("the rows are not a struct array")?
        .into();
    let tmp = dir.join(format!("{name}.tmp"));
    write_batches(tmp.clone(), T::schema(), batch).map_err(|err| format!("{name}: {err}"))?;
    std::fs::rename(tmp, dir.join(name))?;
    Ok(())
}

fn read_rows<T: ArrowRow>(path: &Path) -> Result<Vec<T>, BoxError> {
    let mut rows = vec![];
    for batch in FileReader::try_new(std::fs::File::open(path)?, None)? {
        let mut batch_rows: Vec<T> = serde_arrow::from_record_batch(&batch?)?;
        rows.append(&mut batch_rows);
    }
    Ok(rows)
}

/// The key of the rows of a usage table, without the period.
type UsageKey = (u32, String, String, String, String);

/// A row of a usage table: the rows of the same key add up.
trait UsageRow: ArrowRow + Clone {
    fn key(&self) -> UsageKey;
    fn add(&mut self, other: &Self);
    fn set_period(&mut self, period: &str);
}

macro_rules! usage_key {
    ($row:expr, $item:expr) => {
        (
            $row.ext_protocol_version,
            $row.player_race.clone(),
            $row.account.clone(),
            $row.player.clone(),
            $item,
        )
    };
}

impl UsageRow for AbilityUsageRow {
    fn key(&self) -> UsageKey {
        usage_key!(self, self.ability.clone())
    }
    fn add(&mut self, other: &Self) {
        self.games += other.games;
        self.wins += other.wins;
        self.uses += other.uses;
    }
    fn set_period(&mut self, period: &str) {
        self.period = period.to_string();
    }
}

impl UsageRow for UnitUsageRow {
    fn key(&self) -> UsageKey {
        usage_key!(self, self.unit.clone())
    }
    fn add(&mut self, other: &Self) {
        self.games += other.games;
        self.wins += other.wins;
        self.born += other.born;
        self.died += other.died;
        self.kills += other.kills;
        self.value_killed += other.value_killed;
    }
    fn set_period(&mut self, period: &str) {
        self.period = period.to_string();
    }
}

impl UsageRow for PlayerUsageRow {
    fn key(&self) -> UsageKey {
        usage_key!(self, String::new())
    }
    fn add(&mut self, other: &Self) {
        self.games += other.games;
        self.wins += other.wins;
        self.versus_games += other.versus_games;
        self.versus_wins += other.versus_wins;
    }
    fn set_period(&mut self, period: &str) {
        self.period = period.to_string();
    }
}

/// Adds up the rows of the same key, in the order of the keys.
fn add_up<T: UsageRow>(rows: impl IntoIterator<Item = T>, period: &str) -> Vec<T> {
    let mut totals: BTreeMap<UsageKey, T> = BTreeMap::new();
    for mut row in rows {
        match totals.get_mut(&row.key()) {
            Some(total) => total.add(&row),
            None => {
                row.set_period(period);
                totals.insert(row.key(), row);
            }
        }
    }
    totals.into_values().collect()
}

/// The value of a `key=value` directory name.
fn key_value(dir: &Path) -> &str {
    dir.file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| name.split_once('='))
        .map(|(_, value)| value)
        .unwrap_or_default()
}

/// `2026-10-15`, `2026-10` or `2026`, from the directory of a day, month or year.
fn period_of(dir: &Path, depth: usize) -> String {
    let mut parts: Vec<&str> = dir.ancestors().take(depth).map(key_value).collect();
    parts.reverse();
    parts.join("-")
}

fn read_metadata(dir: &Path) -> Option<AggregatesMetadata> {
    let contents = std::fs::read_to_string(dir.join(AGGREGATES_METADATA_FNAME)).ok()?;
    serde_json::from_str(&contents).ok()
}

fn write_metadata(dir: &Path, source: String) -> Result<(), BoxError> {
    let metadata = AggregatesMetadata {
        aggregates_version: AGGREGATES_VERSION,
        source,
    };
    std::fs::write(
        dir.join(AGGREGATES_METADATA_FNAME),
        serde_json::to_string(&metadata)?,
    )?;
    Ok(())
}

/// Whether the aggregations of the directory were computed from `source`.
fn is_current(dir: &Path, source: &str) -> bool {
    read_metadata(dir).is_some_and(|metadata| {
        metadata.aggregates_version == AGGREGATES_VERSION && metadata.source == source
    })
}

/// What the aggregations of a day are computed from: its files, written at this time.
fn day_source(day: &Path) -> Option<String> {
    let metadata = SnapshotMetadata::read_from_dir(day).ok()?;
    (metadata.s2protocol_version == crate::snapshot::S2PROTOCOL_VERSION)
        .then(|| metadata.created_at.to_rfc3339())
}

/// The sub directories of a month or year, as a text that changes when any of them changes.
fn children_source<'a>(children: impl Iterator<Item = (&'a Path, String)>) -> String {
    children
        .map(|(dir, source)| format!("{}:{source}", key_value(dir)))
        .collect::<Vec<_>>()
        .join(";")
}

/// The days, months and years with their sources, oldest first.
struct SnapshotTree {
    days: Vec<(PathBuf, String)>,
    months: Vec<(PathBuf, String)>,
    years: Vec<(PathBuf, String)>,
}

fn snapshot_tree(root: &Path) -> Result<SnapshotTree, std::io::Error> {
    let mut tree = SnapshotTree {
        days: vec![],
        months: vec![],
        years: vec![],
    };
    for year in key_dirs(root, "year")? {
        let mut month_sources = vec![];
        for month in key_dirs(&year, "month")? {
            let mut day_sources = vec![];
            for day in key_dirs(&month, "day")? {
                // The days of another version are parsed again, they are told apart.
                if let Some(source) = day_source(&day) {
                    day_sources.push((day, source));
                }
            }
            let source = children_source(
                day_sources
                    .iter()
                    .map(|(day, source)| (day.as_path(), source.clone())),
            );
            tree.days.append(&mut day_sources);
            if !source.is_empty() {
                month_sources.push((month, source));
            }
        }
        let source = children_source(
            month_sources
                .iter()
                .map(|(month, source)| (month.as_path(), source.clone())),
        );
        tree.months.append(&mut month_sources);
        if !source.is_empty() {
            tree.years.push((year, source));
        }
    }
    Ok(tree)
}

/// The days, months and years whose aggregations are missing or do not match their days.
pub fn find_gaps(root: &Path) -> Result<AggregateGaps, std::io::Error> {
    let tree = snapshot_tree(root)?;
    let stale = |dirs: Vec<(PathBuf, String)>| -> Vec<PathBuf> {
        dirs.into_iter()
            .filter(|(dir, source)| !is_current(dir, source))
            .map(|(dir, _)| dir)
            .collect()
    };
    Ok(AggregateGaps {
        days: stale(tree.days),
        months: stale(tree.months),
        years: stale(tree.years),
    })
}

/// Computes the aggregations of a day from its files.
fn build_day(day: &Path, source: &str) -> Result<(), BoxError> {
    let _ = std::fs::remove_file(day.join(AGGREGATES_METADATA_FNAME));
    let period = period_of(day, 3);
    let players = read_players(day)?;
    let abilities = game_abilities(day)?;
    let (durations, minutes) = game_stats(day)?;
    let (units, game_players) = game_units(day, &durations)?;
    let player_of = |game: u64, name: &str| players.get(&(game, name.to_string()));
    let mut game_races: HashMap<u64, Vec<&str>> = HashMap::new();
    for ((game, _), player) in &players {
        game_races.entry(*game).or_default().push(&player.race);
    }
    let player_rows = players.iter().map(|((game, _), player)| {
        let races = &game_races[game];
        let versus = races.len() == 2 && races[0] != races[1];
        PlayerUsageRow {
            period: String::new(),
            ext_protocol_version: player.version,
            player_race: player.race.clone(),
            account: player.account.clone(),
            player: player.player.clone(),
            games: 1,
            wins: player.won as u32,
            versus_games: versus as u32,
            versus_wins: (versus && player.won) as u32,
        }
    });
    let ability_rows = abilities.iter().filter_map(|row| {
        let player = player_of(row.ext_fs_id, &row.player_name)?;
        Some(AbilityUsageRow {
            period: String::new(),
            ext_protocol_version: player.version,
            player_race: player.race.clone(),
            account: player.account.clone(),
            player: player.player.clone(),
            ability: row.ability.clone(),
            games: 1,
            wins: player.won as u32,
            uses: row.uses as u64,
        })
    });
    let unit_rows = units.iter().filter_map(|row| {
        let player = player_of(row.ext_fs_id, &row.player_name)?;
        let made = row.born > 0;
        Some(UnitUsageRow {
            period: String::new(),
            ext_protocol_version: player.version,
            player_race: player.race.clone(),
            account: player.account.clone(),
            player: player.player.clone(),
            unit: row.unit.clone(),
            games: made as u32,
            wins: (made && player.won) as u32,
            born: row.born as u64,
            died: row.died as u64,
            kills: row.kills as u64,
            value_killed: row.value_killed,
        })
    });
    write_rows(
        day,
        ABILITY_USAGE_ARROW_FNAME,
        &add_up(ability_rows, &period),
    )?;
    write_rows(day, UNIT_USAGE_ARROW_FNAME, &add_up(unit_rows, &period))?;
    write_rows(day, PLAYER_USAGE_ARROW_FNAME, &add_up(player_rows, &period))?;
    write_rows(day, GAME_ABILITIES_ARROW_FNAME, &abilities)?;
    write_rows(day, GAME_UNITS_ARROW_FNAME, &units)?;
    write_rows(day, GAME_PLAYERS_ARROW_FNAME, &game_players)?;
    write_rows(day, GAME_MINUTES_ARROW_FNAME, &minutes)?;
    write_metadata(day, source.to_string())
}

/// Adds up a usage table of the sub directories into the directory.
fn roll_up<T: UsageRow>(
    dir: &Path,
    period: &str,
    children: &[PathBuf],
    name: &str,
) -> Result<(), BoxError> {
    let mut rows = vec![];
    for child in children {
        rows.append(&mut read_rows::<T>(&child.join(name))?);
    }
    write_rows(dir, name, &add_up(rows, period))
}

/// Computes the aggregations of a month or year from the ones of its sub directories.
fn build_parent(
    dir: &Path,
    depth: usize,
    source: &str,
    children: &[PathBuf],
) -> Result<(), BoxError> {
    let _ = std::fs::remove_file(dir.join(AGGREGATES_METADATA_FNAME));
    let period = period_of(dir, depth);
    roll_up::<AbilityUsageRow>(dir, &period, children, ABILITY_USAGE_ARROW_FNAME)?;
    roll_up::<UnitUsageRow>(dir, &period, children, UNIT_USAGE_ARROW_FNAME)?;
    roll_up::<PlayerUsageRow>(dir, &period, children, PLAYER_USAGE_ARROW_FNAME)?;
    write_metadata(dir, source.to_string())
}

/// Removes the months and years that have no days left, their days were written by another
/// version and are being parsed again, or are gone.
fn remove_empty_periods(root: &Path) -> Result<(), std::io::Error> {
    for year in key_dirs(root, "year")? {
        for month in key_dirs(&year, "month")? {
            if key_dirs(&month, "day")?.is_empty() {
                std::fs::remove_dir_all(&month)?;
            }
        }
        if key_dirs(&year, "month")?.is_empty() {
            std::fs::remove_dir_all(&year)?;
        }
    }
    Ok(())
}

/// Writes the aggregations that are missing or do not match their days, returns the directories
/// written; `report` is told the days done. The events of the days are read, not the replays.
pub fn build_missing(
    root: &Path,
    report: impl Fn(SnapshotProgress) + Sync,
) -> Result<AggregateGaps, BoxError> {
    remove_empty_periods(root)?;
    let tree = snapshot_tree(root)?;
    let stale = |dirs: &[(PathBuf, String)]| -> Vec<(PathBuf, String)> {
        dirs.iter()
            .filter(|(dir, source)| !is_current(dir, source))
            .cloned()
            .collect()
    };
    let days = stale(&tree.days);
    let months = stale(&tree.months);
    let years = stale(&tree.years);
    let done = std::sync::atomic::AtomicUsize::new(0);
    let total = days.len();
    days.par_iter()
        .map(|(day, source)| {
            build_day(day, source).map_err(|err| format!("{}: {err}", day.display()))?;
            let done = done.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            report(SnapshotProgress::ReplaysProcessed { done, total });
            Ok::<(), String>(())
        })
        .collect::<Result<Vec<()>, String>>()?;
    let days_of = |month: &Path| -> Vec<PathBuf> {
        tree.days
            .iter()
            .filter(|(day, _)| day.parent() == Some(month))
            .map(|(day, _)| day.clone())
            .collect()
    };
    months
        .par_iter()
        .map(|(month, source)| {
            build_parent(month, 2, source, &days_of(month))
                .map_err(|err| format!("{}: {err}", month.display()))
        })
        .collect::<Result<Vec<()>, String>>()?;
    let months_of = |year: &Path| -> Vec<PathBuf> {
        tree.months
            .iter()
            .filter(|(month, _)| month.parent() == Some(year))
            .map(|(month, _)| month.clone())
            .collect()
    };
    years
        .par_iter()
        .map(|(year, source)| {
            build_parent(year, 1, source, &months_of(year))
                .map_err(|err| format!("{}: {err}", year.display()))
        })
        .collect::<Result<Vec<()>, String>>()?;
    Ok(AggregateGaps {
        days: days.into_iter().map(|(dir, _)| dir).collect(),
        months: months.into_iter().map(|(dir, _)| dir).collect(),
        years: years.into_iter().map(|(dir, _)| dir).collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrow_store::ipc_writer::open_arrow_mutex_writer;
    use crate::arrow_store::partitioned::partition_dir;

    fn details(id: u64, name: &str, race: &str, result: &str, toon: u64) -> serde_json::Value {
        serde_json::json!({
            "ext_fs_id": id, "player_name": name, "player_race": race, "player_result": result,
            "player_observe": 0, "player_toon_region": 2, "player_toon_realm": 1,
            "player_toon_id": toon, "ext_protocol_version": 94137,
        })
    }

    /// The columns the aggregations read, the other columns of the files are not needed: the
    /// files of the test only have these.
    fn write_json_rows(dir: &Path, name: &str, rows: Vec<serde_json::Value>) {
        use arrow::datatypes::{DataType, Field, Schema};
        let string = |name: &str, nullable: bool| Field::new(name, DataType::Utf8, nullable);
        let schema = match name {
            super::super::DETAILS_ARROW_FNAME => Schema::new(vec![
                Field::new("ext_fs_id", DataType::UInt64, false),
                string("player_name", false),
                string("player_race", false),
                string("player_result", false),
                Field::new("player_observe", DataType::UInt8, false),
                Field::new("player_toon_region", DataType::UInt8, false),
                Field::new("player_toon_realm", DataType::UInt32, false),
                Field::new("player_toon_id", DataType::UInt64, false),
                Field::new("ext_protocol_version", DataType::UInt32, false),
            ]),
            super::super::UNIT_BORN_ARROW_FNAME => Schema::new(vec![
                Field::new("ext_fs_id", DataType::UInt64, false),
                string("player_name", true),
                Field::new("control_player_id", DataType::UInt8, true),
                Field::new("unit_tag_index", DataType::UInt32, false),
                Field::new("unit_tag_recycle", DataType::UInt32, false),
                string("unit_type_name", false),
            ]),
            super::super::UNIT_DIED_ARROW_FNAME => Schema::new(vec![
                Field::new("ext_fs_id", DataType::UInt64, false),
                Field::new("unit_tag_index", DataType::UInt32, false),
                Field::new("unit_tag_recycle", DataType::UInt32, false),
                Field::new("killer_player_id", DataType::UInt8, true),
                string("unit_killer_name", false),
                string("unit_died_name", false),
            ]),
            _ => Schema::new(vec![
                Field::new("ext_fs_id", DataType::UInt64, false),
                string("player_name", true),
                string("ability", false),
                Field::new("ext_replay_seconds", DataType::UInt32, false),
            ]),
        };
        let fields: Vec<arrow::datatypes::FieldRef> = schema.fields().iter().cloned().collect();
        let arrays = serde_arrow::to_arrow(&fields, &rows).unwrap();
        let batch = RecordBatch::try_new(std::sync::Arc::new(schema.clone()), arrays).unwrap();
        let writer = open_arrow_mutex_writer(dir.join(name), schema).unwrap();
        let mut writer = writer.lock().unwrap();
        writer.write(&batch).unwrap();
        writer.finish().unwrap();
    }

    fn write_day(root: &Path, date: &str, game: u64, uses: usize) -> PathBuf {
        let day = root.join(partition_dir(date.parse().unwrap()));
        std::fs::create_dir_all(&day).unwrap();
        write_json_rows(
            &day,
            super::super::DETAILS_ARROW_FNAME,
            vec![
                details(game, "Clan<sp/>Ana", "Terran", "Win", 7),
                details(game, "Bob", "Zerg", "Loss", 0),
            ],
        );
        let command = |ability: &str, seconds: u32| {
            serde_json::json!({
                "ext_fs_id": game, "player_name": "Clan<sp/>Ana", "ability": ability,
                "ext_replay_seconds": seconds,
            })
        };
        let mut commands = vec![command("Yamato.Execute", 30)];
        commands.extend((0..uses).map(|use_index| command("Scan.Execute", 40 + use_index as u32)));
        // Commands without a player or an ability are not counted.
        commands.push(serde_json::json!({
            "ext_fs_id": game, "player_name": null, "ability": "Scan.Execute", "ext_replay_seconds": 1,
        }));
        commands.push(command("", 2));
        write_json_rows(&day, super::super::CMD_TARGET_POINT_ARROW_FNAME, commands);
        let born = |player: Option<&str>, control: Option<u8>, tag: u32, unit: &str| {
            serde_json::json!({
                "ext_fs_id": game, "player_name": player, "control_player_id": control,
                "unit_tag_index": tag, "unit_tag_recycle": 1, "unit_type_name": unit,
            })
        };
        write_json_rows(
            &day,
            super::super::UNIT_BORN_ARROW_FNAME,
            vec![
                born(Some("Clan<sp/>Ana"), Some(1), 1, "SiegeTank"),
                // The same unit sieged is not another unit.
                born(Some("Clan<sp/>Ana"), Some(1), 1, "SiegeTankSieged"),
                born(Some("Clan<sp/>Ana"), Some(1), 2, "Marine"),
                born(Some("Bob"), Some(2), 3, "Zergling"),
                born(None, None, 4, "MineralField"),
            ],
        );
        let died = |tag: u32, killer: Option<u8>, killer_unit: &str, unit: &str| {
            serde_json::json!({
                "ext_fs_id": game, "unit_tag_index": tag, "unit_tag_recycle": 1,
                "killer_player_id": killer, "unit_killer_name": killer_unit, "unit_died_name": unit,
            })
        };
        write_json_rows(
            &day,
            super::super::UNIT_DIED_ARROW_FNAME,
            vec![
                died(3, Some(1), "SiegeTankSieged", "Zergling"),
                // A unit killed by its owner is not a kill, it is still lost.
                died(2, Some(1), "SiegeTankSieged", "Marine"),
            ],
        );
        let metadata = SnapshotMetadata {
            s2protocol_version: crate::snapshot::S2PROTOCOL_VERSION.to_string(),
            created_at: chrono::Utc::now(),
            elapsed_ms: 0,
            replays: 1,
            files: vec![],
            partitions: 0,
            added: 1,
        };
        metadata.write_to_dir(&day).unwrap();
        day
    }

    fn temp_root(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("s2protocol-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    #[test]
    fn it_aggregates_the_days_into_months_and_years() {
        let root = temp_root("aggregates");
        let first = write_day(&root, "2026-10-15", 1, 2);
        write_day(&root, "2026-10-16", 2, 3);
        write_day(&root, "2026-11-01", 3, 1);
        let gaps = find_gaps(&root).unwrap();
        assert_eq!(
            (gaps.days.len(), gaps.months.len(), gaps.years.len()),
            (3, 2, 1)
        );
        build_missing(&root, |_| {}).unwrap();
        assert!(find_gaps(&root).unwrap().is_empty());

        let games: Vec<GameAbilityRow> =
            read_rows(&first.join(GAME_ABILITIES_ARROW_FNAME)).unwrap();
        assert_eq!(
            games
                .iter()
                .map(|row| (row.ability.as_str(), row.uses, row.first_seconds))
                .collect::<Vec<_>>(),
            vec![("Scan.Execute", 2, 40), ("Yamato.Execute", 1, 30)]
        );
        let units: Vec<GameUnitRow> = read_rows(&first.join(GAME_UNITS_ARROW_FNAME)).unwrap();
        let unit = |name: &str| units.iter().find(|row| row.unit == name).unwrap();
        assert_eq!(unit("SiegeTank").born, 1);
        assert_eq!(unit("SiegeTank").kills, 1);
        assert!(unit("SiegeTank").value_killed > 0);
        assert_eq!(unit("Marine").died, 1);
        assert_eq!(unit("Zergling").player_name, "Bob");
        assert_eq!(unit("Zergling").died, 1);

        let period_rows = |dir: PathBuf| -> Vec<AbilityUsageRow> {
            read_rows(&dir.join(ABILITY_USAGE_ARROW_FNAME)).unwrap()
        };
        let month = period_rows(root.join("year=2026/month=10"));
        let scan = month
            .iter()
            .find(|row| row.ability == "Scan.Execute")
            .unwrap();
        assert_eq!(scan.period, "2026-10");
        assert_eq!((scan.games, scan.wins, scan.uses), (2, 2, 5));
        assert_eq!(scan.player, "Ana");
        assert_eq!(scan.account, "2-1-7");
        let year = period_rows(root.join("year=2026"));
        let scan = year
            .iter()
            .find(|row| row.ability == "Scan.Execute")
            .unwrap();
        assert_eq!(scan.period, "2026");
        assert_eq!((scan.games, scan.uses), (3, 6));
        let players: Vec<PlayerUsageRow> =
            read_rows(&root.join("year=2026").join(PLAYER_USAGE_ARROW_FNAME)).unwrap();
        let bob = players.iter().find(|row| row.player == "Bob").unwrap();
        assert_eq!((bob.games, bob.wins, bob.account.as_str()), (3, 0, ""));
        assert_eq!((bob.versus_games, bob.versus_wins), (3, 0));
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn it_finds_the_periods_of_a_day_that_changed_or_is_new() {
        let root = temp_root("aggregate-gaps");
        write_day(&root, "2026-10-15", 1, 2);
        build_missing(&root, |_| {}).unwrap();
        assert!(find_gaps(&root).unwrap().is_empty());

        write_day(&root, "2026-10-16", 2, 3);
        let gaps = find_gaps(&root).unwrap();
        assert_eq!(gaps.days, vec![root.join("year=2026/month=10/day=16")]);
        assert_eq!(gaps.months, vec![root.join("year=2026/month=10")]);
        assert_eq!(gaps.years, vec![root.join("year=2026")]);
        let built = build_missing(&root, |_| {}).unwrap();
        assert_eq!(built.days.len(), 1);
        assert!(find_gaps(&root).unwrap().is_empty());

        // A month of days that are gone goes with them.
        std::fs::remove_dir_all(root.join("year=2026/month=10/day=15")).unwrap();
        std::fs::remove_dir_all(root.join("year=2026/month=10/day=16")).unwrap();
        build_missing(&root, |_| {}).unwrap();
        assert!(!root.join("year=2026").exists());
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_batch_of_the_writer_holds_many_replays() {
        use crate::arrow_store::ipc_writer::RECORD_BATCH_ROWS;
        let root = temp_root("batches");
        let rows: Vec<GameAbilityRow> = (0..RECORD_BATCH_ROWS / 4)
            .map(|row| GameAbilityRow {
                ext_fs_id: row as u64,
                player_name: "Ana".to_string(),
                ability: "Scan.Execute".to_string(),
                uses: 1,
                first_seconds: 1,
            })
            .collect();
        let array = rows.try_into_arrow().unwrap();
        let batch: RecordBatch = array.as_any().downcast_ref::<StructArray>().unwrap().into();
        let path = root.join("x.arrow");
        let writer = open_arrow_mutex_writer(path.clone(), GameAbilityRow::schema()).unwrap();
        {
            let mut writer = writer.lock().unwrap();
            for _ in 0..9 {
                writer.write(&batch).unwrap();
            }
            writer.finish().unwrap();
        }
        let batches: Vec<usize> = FileReader::try_new(std::fs::File::open(&path).unwrap(), None)
            .unwrap()
            .map(|batch| batch.unwrap().num_rows())
            .collect();
        assert_eq!(
            batches,
            vec![RECORD_BATCH_ROWS, RECORD_BATCH_ROWS, RECORD_BATCH_ROWS / 4]
        );
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_table_without_rows_is_a_file_without_batches() {
        let root = temp_root("no-rows");
        write_rows::<GameAbilityRow>(&root, "none.arrow", &[]).unwrap();
        let reader =
            FileReader::try_new(std::fs::File::open(root.join("none.arrow")).unwrap(), None)
                .unwrap();
        assert_eq!(reader.schema().fields(), GameAbilityRow::schema().fields());
        assert_eq!(reader.count(), 0);
        std::fs::remove_dir_all(&root).unwrap();
    }
}
