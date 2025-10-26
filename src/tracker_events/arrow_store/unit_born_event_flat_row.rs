//! Unit Born Event Flat Row

#[cfg(feature = "dep_arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::state::UnitChangeHint;
use crate::tracker_events::{UnitBornEvent, UnitDoneEvent, UnitTypeChangeEvent};
use serde::{Deserialize, Serialize};

/// An experiment creating a flat row of UnitBornEventFlatRow for Arrow usage
/// TODO: is_init flag is not used yet, this happens when a unit is transitioning and could be
/// cancelled.
/// We should also add a field/enum for when it's Born/Done/TypeChange
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UnitBornEventFlatRow {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub unit_type_name: String,
    pub control_player_id: Option<u8>,
    pub upkeep_player_id: Option<u8>,
    pub x: f32,
    pub y: f32,
    pub creator_unit_tag_index: Option<u32>,
    pub creator_unit_tag_recycle: Option<u32>,
    pub creator_unit_type_name: Option<String>,
    pub creator_ability_name: Option<String>,
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_id: u64,
    pub player_name: Option<String>,
}

impl UnitBornEventFlatRow {
    /// Create a new UpgradeEventFlatRow from a UnitBornEvent.
    /// UnitBornEvents do not have build time and are instance, such as the ones created at the
    /// start of the game, or Broodlings or Locusts.
    pub fn from_unit_born(
        event: UnitBornEvent,
        ext_replay_loop: i64,
        details: &crate::details::Details,
        change_hint: UnitChangeHint,
    ) -> Option<Self> {
        let (unit, creator) = match change_hint {
            UnitChangeHint::Registered { unit, creator } => (unit, creator),
            _ => return None,
        };
        let creator_unit_type_name = match creator {
            Some(val) => Some(val.name.clone()),
            None => None,
        };
        let ext_replay_seconds = crate::convert_tracker_loop_to_seconds(ext_replay_loop);
        Some(Self {
            unit_tag_index: event.unit_tag_index,
            unit_tag_recycle: event.unit_tag_recycle,
            unit_type_name: event.unit_type_name,
            control_player_id: Some(event.control_player_id),
            upkeep_player_id: Some(event.upkeep_player_id),
            x: event.x as f32,
            y: event.y as f32,
            creator_unit_tag_index: event.creator_unit_tag_index,
            creator_unit_tag_recycle: event.creator_unit_tag_recycle,
            creator_ability_name: event.creator_ability_name,
            creator_unit_type_name,
            ext_replay_loop,
            ext_replay_seconds,
            ext_fs_id: details.ext_fs_id,
            player_name: unit.player_name,
        })
    }

    /// Create a new UnitBornEventFlatRow from a UnitDoneEvent.
    /// These are units that were previously under construction and are now finished.
    /// A UnitChangeHint is needed so that we can collect the x and y coordinates of when
    /// the unit began the UnitInit event.
    pub fn from_unit_done(
        event: UnitDoneEvent,
        ext_replay_loop: i64,
        details: &crate::details::Details,
        change_hint: UnitChangeHint,
    ) -> Option<Self> {
        // NOTE: It seems this can be "None" but our code (state) may not be able to handle this.
        let (unit, creator) = match change_hint {
            UnitChangeHint::Registered { unit, creator } => (unit, creator),
            _ => return None,
        };
        let creator_unit_type_name = match creator {
            Some(val) => Some(val.name.clone()),
            None => None,
        };
        let ext_replay_seconds = crate::convert_tracker_loop_to_seconds(ext_replay_loop);
        Some(Self {
            unit_tag_index: event.unit_tag_index,
            unit_tag_recycle: event.unit_tag_recycle,
            unit_type_name: unit.name,
            control_player_id: unit.user_id,
            upkeep_player_id: unit.user_id,
            x: unit.pos.x(),
            y: unit.pos.y(),
            creator_unit_tag_index: None,
            creator_unit_tag_recycle: None,
            creator_ability_name: unit.creator_ability_name,
            creator_unit_type_name,
            ext_replay_loop,
            ext_replay_seconds,
            ext_fs_id: details.ext_fs_id,
            player_name: unit.player_name,
        })
    }

    /// Create a new UpgradeEventFlatRow from a UnitTypeChange.
    /// These are units that transition from one type to another, such as zergling to banelingcocoon
    /// to baneling.
    pub fn from_unit_type_change(
        event: UnitTypeChangeEvent,
        ext_replay_loop: i64,
        details: &crate::details::Details,
        change_hint: UnitChangeHint,
    ) -> Option<Self> {
        let (unit, creator) = match change_hint {
            UnitChangeHint::Registered { unit, creator } => (unit, creator),
            _ => return None,
        };
        let creator_unit_type_name = match creator {
            Some(val) => Some(val.name.clone()),
            None => None,
        };
        let ext_replay_seconds = crate::convert_tracker_loop_to_seconds(ext_replay_loop);
        Some(Self {
            unit_tag_index: event.unit_tag_index,
            unit_tag_recycle: event.unit_tag_recycle,
            unit_type_name: unit.name,
            control_player_id: unit.user_id,
            upkeep_player_id: unit.user_id,
            x: unit.pos.x(),
            y: unit.pos.y(),
            creator_unit_tag_index: None,
            creator_unit_tag_recycle: None,
            creator_unit_type_name,
            creator_ability_name: unit.creator_ability_name,
            ext_replay_loop,
            ext_replay_seconds,
            ext_fs_id: details.ext_fs_id,
            player_name: unit.player_name,
        })
    }
}
