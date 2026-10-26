//! A version agnostic Unit Type Change Event

use crate::filters::SC2ReplayFilters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitTypeChangeEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub unit_type_name: String,
}

impl UnitTypeChangeEvent {
    pub fn should_skip(&self, filters: &SC2ReplayFilters) -> bool {
        if let Some(unit_type_name) = &filters.unit_name
            && self.unit_type_name != *unit_type_name
        {
            return true;
        }
        false
    }
}
