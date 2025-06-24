//! Unit Died Event Flat Row
//!
#[cfg(feature = "arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::details::Details;
use crate::tracker_events::UnitDiedEvent;
use crate::UnitChangeHint;
use serde::{Deserialize, Serialize};

/// A protocol agnostic Unit Died
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UnitDiedEventFlatRow {
    pub unit_died_name: String,
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub killer_player_id: Option<u8>,
    pub x: u8,
    pub y: u8,
    pub unit_killer_name: String,
    pub killer_unit_tag_index: Option<u32>,
    pub killer_unit_tag_recycle: Option<u32>,
    // TODO: Tho these fields should be equivalent, they were observed in a couple of games.
    // We should corroborate these numbers and remove the loop in favor of the seconds.
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_replay_sha256: String,
}

impl UnitDiedEventFlatRow {
    /// Create a new UpgradeEventFlatRow from a UpgradeEvent and the fields from the Details MPQ sector
    pub fn new(
        details: &Details,
        event: UnitDiedEvent,
        ext_replay_loop: i64,
        change_hint: UnitChangeHint,
    ) -> Self {
        let (unit_killer_name, unit_died_name) = match change_hint {
            UnitChangeHint::Unregistered { killer, killed } => {
                let unit_killer_name = match killer {
                    Some(killer) => killer.name.clone(),
                    None => String::new(),
                };
                (unit_killer_name, killed.name)
            }
            _ => unreachable!(),
        };
        let ext_replay_seconds = crate::convert_tracker_loop_to_seconds(ext_replay_loop);
        Self {
            unit_died_name,
            unit_tag_index: event.unit_tag_index,
            unit_tag_recycle: event.unit_tag_recycle,
            killer_player_id: event.killer_player_id,
            x: event.x,
            y: event.y,
            unit_killer_name,
            killer_unit_tag_index: event.killer_unit_tag_index,
            killer_unit_tag_recycle: event.killer_unit_tag_recycle,
            ext_replay_loop,
            ext_replay_seconds,
            ext_fs_replay_sha256: details.ext_fs_replay_sha256.clone(),
        }
    }
}
