//! Unit Born Event Flat Row

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::state::UnitChangeHint;
use crate::tracker_events::{UnitBornEvent, UnitDoneEvent, UnitTypeChangeEvent};
use serde::{Deserialize, Serialize};

/// An experiment creating a flat row of UnitBornEventFlatRow for Arrow usage
/// TODO: is_init flag is not used yet, this happens when a unit is transitioning and could be
/// cancelled.
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
    pub x: f32,
    pub y: f32,
    pub creator_unit_tag_index: Option<u32>,
    pub creator_unit_tag_recycle: Option<u32>,
    pub creator_ability_name: Option<String>,
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_replay_sha256: String,
    pub ext_replay_detail_player_name: String,
}

impl UnitBornEventFlatRow {
    /// Create a new UpgradeEventFlatRow from a UnitBornEvent.
    /// UnitBornEvents do not have build time and are instance, such as the ones created at the
    /// start of the game, or Broodlings or Locusts.
    pub fn from_unit_born(
        event: UnitBornEvent,
        ext_replay_loop: i64,
        details: &crate::details::Details,
    ) -> Self {
        let ext_replay_seconds = crate::convert_tracker_loop_to_seconds(ext_replay_loop);
        let ext_replay_detail_player_name = details.get_player_name(event.control_player_id);
        Self {
            unit_tag_index: event.unit_tag_index,
            unit_tag_recycle: event.unit_tag_recycle,
            unit_type_name: event.unit_type_name,
            control_player_id: event.control_player_id,
            upkeep_player_id: event.upkeep_player_id,
            x: event.x as f32,
            y: event.y as f32,
            creator_unit_tag_index: event.creator_unit_tag_index,
            creator_unit_tag_recycle: event.creator_unit_tag_recycle,
            creator_ability_name: event.creator_ability_name,
            ext_replay_loop,
            ext_replay_seconds,
            ext_fs_replay_sha256: details.ext_fs_replay_sha256.clone(),
            ext_replay_detail_player_name,
        }
    }

    /// Create a new UpgradeEventFlatRow from a UnitDoneEvent.
    /// These are units that were previously under construction and are now finished.
    /// A UnitChangeHint is needed so that we can collect the x and y coordinates of when
    /// the unit began the UnitInit event.
    pub fn from_unit_done(
        event: UnitDoneEvent,
        ext_replay_loop: i64,
        details: &crate::details::Details,
        change_hint: UnitChangeHint,
    ) -> Self {
        let unit = match change_hint {
            UnitChangeHint::Registered(unit) => unit,
            _ => unreachable!(),
        };
        let ext_replay_seconds = crate::convert_tracker_loop_to_seconds(ext_replay_loop);
        let ext_replay_detail_player_name = details.get_player_name(unit.user_id.unwrap_or(99));
        Self {
            unit_tag_index: event.unit_tag_index,
            unit_tag_recycle: event.unit_tag_recycle,
            unit_type_name: unit.name,
            control_player_id: unit.user_id.unwrap_or(99),
            upkeep_player_id: unit.user_id.unwrap_or(99),
            x: unit.pos.x(),
            y: unit.pos.y(),
            creator_unit_tag_index: None,
            creator_unit_tag_recycle: None,
            creator_ability_name: unit.creator_ability_name,
            ext_replay_loop,
            ext_replay_seconds,
            ext_fs_replay_sha256: details.ext_fs_replay_sha256.clone(),
            ext_replay_detail_player_name,
        }
    }

    /// Create a new UpgradeEventFlatRow from a UnitTypeChange.
    /// These are units that transition from one type to another, such as zergling to banelingcocoon
    /// to baneling.
    pub fn from_unit_type_change(
        event: UnitTypeChangeEvent,
        ext_replay_loop: i64,
        details: &crate::details::Details,
        change_hint: UnitChangeHint,
    ) -> Self {
        let unit = match change_hint {
            UnitChangeHint::Registered(unit) => unit,
            _ => unreachable!(),
        };
        let ext_replay_seconds = crate::convert_tracker_loop_to_seconds(ext_replay_loop);
        let ext_replay_detail_player_name = details.get_player_name(unit.user_id.unwrap_or(99));
        Self {
            unit_tag_index: event.unit_tag_index,
            unit_tag_recycle: event.unit_tag_recycle,
            unit_type_name: unit.name,
            control_player_id: unit.user_id.unwrap_or(99),
            upkeep_player_id: unit.user_id.unwrap_or(99),
            x: unit.pos.x(),
            y: unit.pos.y(),
            creator_unit_tag_index: None,
            creator_unit_tag_recycle: None,
            creator_ability_name: unit.creator_ability_name,
            ext_replay_loop,
            ext_replay_seconds,
            ext_fs_replay_sha256: details.ext_fs_replay_sha256.clone(),
            ext_replay_detail_player_name,
        }
    }
}
