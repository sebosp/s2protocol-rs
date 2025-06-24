//! A protocol agnostic Unit Done Event

#[cfg(feature = "arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::filters::SC2ReplayFilters;
use serde::{Deserialize, Serialize};

/// Emitted when a unit that is previously in progress is completed.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UnitDoneEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
}

impl UnitDoneEvent {
    pub fn should_skip(&self, _filters: &SC2ReplayFilters) -> bool {
        // Currently no filters, maybe we should locate the user_id and evaluate it
        false
    }
}
