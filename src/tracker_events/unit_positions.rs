//! A protocol agnostic Unit Done Event

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::filters::SC2ReplayFilters;
use serde::{Deserialize, Serialize};

/// A single unit position
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UnitPosition {
    /// The unit "tag" is the "index"?
    pub tag: u32,
    /// The X position.
    pub x: i32,
    /// The Y position.
    pub y: i32,
}

/// Emitted regularly to report unit positions.
/// Not all units are reported.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UnitPositionsEvent {
    pub first_unit_index: u32,
    pub items: Vec<i32>,
}

impl UnitPositionsEvent {
    /// Transforms the internal unit positions into a vector of UnitPosition
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

    pub fn should_skip(&self, _filters: &SC2ReplayFilters) -> bool {
        // Currently no filters, maybe we should locate the user_id and evaluate it
        // Same for the unit_type_name
        false
    }
}
