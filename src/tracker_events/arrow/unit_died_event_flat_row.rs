//! Unit Died Event Flat Row
//!
#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::tracker_events::UnitDiedEvent;
use serde::{Deserialize, Serialize};

// TODO: Get the unit name that was killed and the unit name that killed it

/// A protocol agnostic Unit Died
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UnitDiedEventFlatRow {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub killer_player_id: Option<u8>,
    pub x: u8,
    pub y: u8,
    pub killer_unit_tag_index: Option<u32>,
    pub killer_unit_tag_recycle: Option<u32>,
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_replay_file_name: String,
    pub ext_fs_replay_sha256: String,
    pub ext_replay_detail_killer_player_name: String,
    pub ext_replay_detail_datetime: chrono::NaiveDateTime,
}

impl UnitDiedEventFlatRow {
    /// Create a new UpgradeEventFlatRow from a UpgradeEvent and the fields from the Details MPQ sector
    pub fn new(
        event: UnitDiedEvent,
        ext_replay_loop: i64,
        details: crate::details::Details,
    ) -> Self {
        let ext_replay_seconds = crate::convert_tracker_loop_to_seconds(ext_replay_loop);
        let ext_replay_detail_killer_player_name = match event.killer_player_id {
            Some(id) => details.get_player_name(id - 1),
            None => String::new(),
        };
        Self {
            unit_tag_index: event.unit_tag_index,
            unit_tag_recycle: event.unit_tag_recycle,
            killer_player_id: event.killer_player_id,
            x: event.x,
            y: event.y,
            killer_unit_tag_index: event.killer_unit_tag_index,
            killer_unit_tag_recycle: event.killer_unit_tag_recycle,
            ext_replay_loop,
            ext_replay_seconds,
            ext_fs_replay_file_name: details.ext_fs_replay_file_name,
            ext_fs_replay_sha256: details.ext_fs_replay_sha256,
            ext_replay_detail_killer_player_name,
            ext_replay_detail_datetime: details.ext_datetime,
        }
    }
}
