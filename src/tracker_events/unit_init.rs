//! A protocol agnostic Unit Init Event

#[cfg(feature = "dep_arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::filters::SC2ReplayFilters;
use serde::{Deserialize, Serialize};

/// Emitted when a unit takes time to be created and may be cancelled.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UnitInitEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub unit_type_name: String,
    pub control_player_id: u8,
    pub upkeep_player_id: u8,
    pub x: u8,
    pub y: u8,
}

impl UnitInitEvent {
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
