//! A protocol agnostic Unit Died

#[cfg(feature = "dep_arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::filters::SC2ReplayFilters;
use serde::{Deserialize, Serialize};

/// A Unit died event, it does not contain the unit owner player id.
/// Or the unit name.  For a process of enriching this data see UnitBornEventFlatRow.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UnitDiedEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub killer_player_id: Option<u8>,
    pub x: u8,
    pub y: u8,
    pub killer_unit_tag_index: Option<u32>,
    pub killer_unit_tag_recycle: Option<u32>,
}

impl UnitDiedEvent {
    pub fn should_skip(&self, filters: &SC2ReplayFilters) -> bool {
        // TODO: Should we add the unit_died_player_id/name?
        if let Some(player_id) = filters.player_id {
            if self.killer_player_id != Some(player_id) {
                return true;
            }
        }
        false
    }
}
