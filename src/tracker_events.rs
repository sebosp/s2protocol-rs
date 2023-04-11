//! Decodes the Tracker Events.
//! These are stored in an embebdded file in the MPQ file called 'replay.tracker.events'

use std::str::Utf8Error;

/// A list of errors when handling TrackerEvents
#[derive(Debug, thiserror::Error)]
pub enum TrackerEventError {
    /// An error to be used in TryFrom, when converting from protocol-specific types into our
    /// consolidated-types
    #[error("Unsupported Event Type")]
    UnsupportedEventType,
    /// Conversion to UTF-8 failed, from the Vec<u8> _name fields in the proto fields
    #[error("Utf8 conversion error")]
    Utf8Error(#[from] Utf8Error),
}

/// A protocol agnostic Unit Born
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitBornEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub unit_type_name: String,
    pub control_player_id: u8,
    pub upkeep_player_id: u8,
    pub x: u8,
    pub y: u8,
    pub creator_unit_tag_index: Option<u32>,
    pub creator_unit_tag_recycle: Option<u32>,
    pub creator_ability_name: Option<String>,
}

/// A protocol agnostic Unit Died
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitDiedEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub killer_player_id: Option<u8>,
    pub x: u8,
    pub y: u8,
    pub killer_unit_tag_index: Option<u32>,
    pub killer_unit_tag_recycle: Option<u32>,
}

/// A protocol agnostic Unit Init Event
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitInitEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub unit_type_name: String,
    pub control_player_id: u8,
    pub upkeep_player_id: u8,
    pub x: u8,
    pub y: u8,
}

/// A protocol agnostic Unit Done Event
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitDoneEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
}

/// A protocol agnostic Unit Done Event
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitPositionsEvent {
    pub first_unit_index: u32,
    pub items: Vec<i32>,
}

impl UnitPositionsEvent {
    pub fn to_unit_positions_vec(self) -> Vec<UnitPosition> {
        let mut unit_index = self.first_unit_index as i32;
        let mut res = vec![];
        for relative_unit_pos in self.items.chunks_exact(3) {
            unit_index += relative_unit_pos[0];
            let x = relative_unit_pos[1] * 4;
            let y = relative_unit_pos[2] * 4;
            res.push(UnitPosition {
                tag: unit_index as u32,
                x,
                y,
            });
            // unit identified by unitIndex at the current event['_gameloop'] time is at approximate position (x, y)
        }
        res
    }
}

/// A single unit position
#[derive(Debug, Default, Clone)]
pub struct UnitPosition {
    /// The unit "tag" is the "index"?
    pub tag: u32,
    /// The X position.
    pub x: i32,
    /// The Y position.
    pub y: i32,
}

/// A unified Replay Tracker that is agnostic of any version.
/// This should hopefully only add fields to variants to make things backwards compatible
/// Many of the variants are not supported yet, they will be added as they are considered
/// relevant  for `swarmy` repo.
#[derive(Debug, PartialEq, Clone)]
pub enum ReplayTrackerEvent {
    PlayerStats(PlayerStatsEvent),
    UnitBorn(UnitBornEvent),
    UnitDied(UnitDiedEvent),
    UnitTypeChange(UnitTypeChangeEvent),
    UnitInit(UnitInitEvent),
    UnitDone(UnitDoneEvent),
    UnitPosition(UnitPositionsEvent),
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitTypeChangeEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub unit_type_name: String,
}

/// A Tracker Event is an event in the gameloop for a specific user id
#[derive(Debug, PartialEq, Clone)]
pub struct TrackerEvent {
    pub delta: u32,
    pub event: ReplayTrackerEvent,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerStatsEvent {
    pub player_id: u8,
    pub stats: PlayerStats,
}

// TODO: Split
#[derive(Debug, PartialEq, Clone)]
pub struct PlayerStats {
    pub minerals_current: i32,
    pub vespene_current: i32,
    pub minerals_collection_rate: i32,
    pub vespene_collection_rate: i32,
    pub workers_active_count: i32,
    pub minerals_used_in_progress_army: i32,
    pub minerals_used_in_progress_economy: i32,
    pub minerals_used_in_progress_technology: i32,
    pub vespene_used_in_progress_army: i32,
    pub vespene_used_in_progress_economy: i32,
    pub vespene_used_in_progress_technology: i32,
    pub minerals_used_current_army: i32,
    pub minerals_used_current_economy: i32,
    pub minerals_used_current_technology: i32,
    pub vespene_used_current_army: i32,
    pub vespene_used_current_economy: i32,
    pub vespene_used_current_technology: i32,
    pub minerals_lost_army: i32,
    pub minerals_lost_economy: i32,
    pub minerals_lost_technology: i32,
    pub vespene_lost_army: i32,
    pub vespene_lost_economy: i32,
    pub vespene_lost_technology: i32,
    pub minerals_killed_army: i32,
    pub minerals_killed_economy: i32,
    pub minerals_killed_technology: i32,
    pub vespene_killed_army: i32,
    pub vespene_killed_economy: i32,
    pub vespene_killed_technology: i32,
    pub food_used: i32,
    pub food_made: i32,
    pub minerals_used_active_forces: i32,
    pub vespene_used_active_forces: i32,
    pub minerals_friendly_fire_army: i32,
    pub minerals_friendly_fire_economy: i32,
    pub minerals_friendly_fire_technology: i32,
    pub vespene_friendly_fire_army: i32,
    pub vespene_friendly_fire_economy: i32,
    pub vespene_friendly_fire_technology: i32,
}

impl PlayerStats {
    ///  Creates a vector of Entity Path to value to be consumed by plots
    pub fn as_prop_name_value_vec(&self) -> Vec<(String, i32)> {
        let mut res = vec![];
        res.push((String::from("minerals/current"), self.minerals_current));
        res.push((String::from("vespene/current"), self.vespene_current));
        res.push((
            String::from("minerals/collection_rate"),
            self.minerals_collection_rate,
        ));
        res.push((
            String::from("vespene/collection_rate"),
            self.vespene_collection_rate,
        ));
        res.push((
            String::from("workers_active_count"),
            self.workers_active_count,
        ));
        res.push((
            String::from("minerals/used_in_progress_army"),
            self.minerals_used_in_progress_army,
        ));
        res.push((
            String::from("minerals/used_in_progress_economy"),
            self.minerals_used_in_progress_economy,
        ));
        res.push((
            String::from("minerals/used_in_progress_technology"),
            self.minerals_used_in_progress_technology,
        ));
        res.push((
            String::from("vespene/used_in_progress_army"),
            self.vespene_used_in_progress_army,
        ));
        res.push((
            String::from("vespene/used_in_progress_economy"),
            self.vespene_used_in_progress_economy,
        ));
        res.push((
            String::from("vespene/used_in_progress_technology"),
            self.vespene_used_in_progress_technology,
        ));
        res.push((
            String::from("minerals/used_current_army"),
            self.minerals_used_current_army,
        ));
        res.push((
            String::from("minerals/used_current_economy"),
            self.minerals_used_current_economy,
        ));
        res.push((
            String::from("minerals/used_current_technology"),
            self.minerals_used_current_technology,
        ));
        res.push((
            String::from("vespene/used_current_army"),
            self.vespene_used_current_army,
        ));
        res.push((
            String::from("vespene/used_current_economy"),
            self.vespene_used_current_economy,
        ));
        res.push((
            String::from("vespene/used_current_technology"),
            self.vespene_used_current_technology,
        ));
        res.push((String::from("minerals/lost_army"), self.minerals_lost_army));
        res.push((
            String::from("minerals/lost_economy"),
            self.minerals_lost_economy,
        ));
        res.push((
            String::from("minerals/lost_technology"),
            self.minerals_lost_technology,
        ));
        res.push((String::from("vespene/lost_army"), self.vespene_lost_army));
        res.push((
            String::from("vespene/lost_economy"),
            self.vespene_lost_economy,
        ));
        res.push((
            String::from("vespene/lost_technology"),
            self.vespene_lost_technology,
        ));
        res.push((
            String::from("minerals/killed_army"),
            self.minerals_killed_army,
        ));
        res.push((
            String::from("minerals/killed_economy"),
            self.minerals_killed_economy,
        ));
        res.push((
            String::from("minerals/killed_technology"),
            self.minerals_killed_technology,
        ));
        res.push((
            String::from("vespene/killed_army"),
            self.vespene_killed_army,
        ));
        res.push((
            String::from("vespene/killed_economy"),
            self.vespene_killed_economy,
        ));
        res.push((
            String::from("vespene/killed_technology"),
            self.vespene_killed_technology,
        ));
        res.push((String::from("food/used"), self.food_used));
        res.push((String::from("food/made"), self.food_made));
        res.push((
            String::from("minerals/used_active_forces"),
            self.minerals_used_active_forces,
        ));
        res.push((
            String::from("vespene/used_active_forces"),
            self.vespene_used_active_forces,
        ));
        res.push((
            String::from("minerals/friendly_fire_army"),
            self.minerals_friendly_fire_army,
        ));
        res.push((
            String::from("minerals/friendly_fire_economy"),
            self.minerals_friendly_fire_economy,
        ));
        res.push((
            String::from("minerals/friendly_fire_technology"),
            self.minerals_friendly_fire_technology,
        ));
        res.push((
            String::from("vespene/friendly_fire_army"),
            self.vespene_friendly_fire_army,
        ));
        res.push((
            String::from("vespene/friendly_fire_economy"),
            self.vespene_friendly_fire_economy,
        ));
        res.push((
            String::from("vespene/friendly_fire_technology"),
            self.vespene_friendly_fire_technology,
        ));
        res
    }
}

pub fn unit_tag(unit_tag_index: u32, unit_tag_recycle: u32) -> i64 {
    ((unit_tag_index as i64) << 18usize) + unit_tag_recycle as i64
}

pub fn unit_tag_index(unit_tag: i64) -> u32 {
    ((unit_tag >> 18) & 0x00003fff) as u32
}

pub fn unit_tag_recycle(unit_tag: i64) -> u32 {
    ((unit_tag) & 0x0003ffff) as u32
}
