//! A version agnostic Unit Owner Change Event

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::filters::SC2ReplayFilters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UnitOwnerChangeEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub control_player_id: u8,
    pub upkeep_player_id: u8,
}
impl UnitOwnerChangeEvent {
    pub fn should_skip(&self, filters: &SC2ReplayFilters) -> bool {
        if let Some(player_id) = filters.player_id {
            if self.control_player_id != player_id {
                return true;
            }
        }
        false
    }
}
