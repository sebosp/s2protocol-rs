//! Unit Born Event Flat Row

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::tracker_events::UnitBornEvent;
use serde::{Deserialize, Serialize};

/// An experiment creating a flat row of UnitBornEventFlatRow for Arrow usage
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UnitBornEventFlatRow {
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
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_replay_file_name: String,
    pub ext_fs_replay_sha256: String,
    pub ext_replay_detail_player_name: String,
    pub ext_replay_detail_datetime: chrono::NaiveDateTime,
}

impl UnitBornEventFlatRow {
    /// Create a new UpgradeEventFlatRow from a UpgradeEvent and the fields from the Details MPQ sector
    pub fn new(
        event: UnitBornEvent,
        ext_replay_loop: i64,
        details: crate::details::Details,
    ) -> Self {
        let ext_replay_seconds = crate::convert_tracker_loop_to_seconds(ext_replay_loop);
        let ext_replay_detail_player_name = details.get_player_name(event.control_player_id - 1);
        Self {
            unit_tag_index: event.unit_tag_index,
            unit_tag_recycle: event.unit_tag_recycle,
            unit_type_name: event.unit_type_name,
            control_player_id: event.control_player_id,
            upkeep_player_id: event.upkeep_player_id,
            x: event.x,
            y: event.y,
            creator_unit_tag_index: event.creator_unit_tag_index,
            creator_unit_tag_recycle: event.creator_unit_tag_recycle,
            creator_ability_name: event.creator_ability_name,
            ext_replay_loop,
            ext_replay_seconds,
            ext_fs_replay_file_name: details.ext_fs_replay_file_name,
            ext_fs_replay_sha256: details.ext_fs_replay_sha256,
            ext_replay_detail_player_name,
            ext_replay_detail_datetime: details.ext_datetime,
        }
    }
}
