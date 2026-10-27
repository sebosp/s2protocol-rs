//! A version agnostic Upgrade Event

use crate::filters::SC2ReplayFilters;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpgradeEvent {
    pub player_id: u8,
    pub player_name: Option<String>,
    pub upgrade_type_name: String,
    pub count: i32,
}

impl UpgradeEvent {
    pub fn should_skip(&self, filters: &SC2ReplayFilters) -> bool {
        if let Some(player_id) = filters.player_id
            && self.player_id != player_id
        {
            return true;
        }
        false
    }
}
