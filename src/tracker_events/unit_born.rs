//! A protocol agnostic Unit Born Definition

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::filters::SC2ReplayFilters;
use serde::{Deserialize, Serialize};

/// Unit Borns are unit that are instant, they appear at the start of the game as well when certain
/// units appear already full, there is no construction time.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
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

impl UnitBornEvent {
    /// Returns true if the event should be skipped
    pub fn should_skip(&self, filters: &SC2ReplayFilters) -> bool {
        if let Some(player_id) = filters.player_id {
            if self.control_player_id != player_id {
                return true;
            }
        }
        if let Some(unit_type_name) = &filters.unit_name {
            if self.unit_type_name != *unit_type_name {
                return true;
            }
        }
        false
    }
}
