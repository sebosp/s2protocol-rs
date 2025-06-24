//! A version agnostic Player Setup Event

#[cfg(feature = "dep_arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::filters::SC2ReplayFilters;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct PlayerSetupEvent {
    pub player_id: u8,
    pub m_type: u32,
    pub user_id: Option<u32>,
    pub slot_id: Option<u32>,
}

impl PlayerSetupEvent {
    pub fn should_skip(&self, filters: &SC2ReplayFilters) -> bool {
        if let Some(player_id) = filters.player_id {
            if self.player_id != player_id {
                return true;
            }
        }
        false
    }
}
