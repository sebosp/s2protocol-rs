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

pub fn unit_tag(unit_tag_index: u32, unit_tag_recycle: u32) -> i64 {
    ((unit_tag_index as i64) << 18usize) + unit_tag_recycle as i64
}

pub fn unit_tag_index(unit_tag: i64) -> u32 {
    ((unit_tag >> 18) & 0x00003fff) as u32
}

pub fn unit_tag_recycle(unit_tag: i64) -> u32 {
    ((unit_tag) & 0x0003ffff) as u32
}
