//! A version agnostic Player Stats Event, their transformation and some massaging for users of the
//! data.

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::filters::SC2ReplayFilters;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct PlayerStatsEvent {
    pub player_id: u8,
    pub stats: PlayerStats,
}

impl PlayerStatsEvent {
    pub fn should_skip(&self, filters: &SC2ReplayFilters) -> bool {
        if let Some(player_id) = filters.player_id {
            if self.player_id != player_id {
                return true;
            }
        }
        if !filters.include_stats {
            return true;
        }
        false
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct PlayerStats {
    pub minerals_current: i32,
    pub vespene_current: i32,
    pub minerals_collection_rate: i32,
    pub vespene_collection_rate: i32,
    pub workers_active_count: i32,
    pub minerals_used_in_progress_army: i32,
    pub minerals_used_in_progress_economy: i32,
    pub minerals_used_in_progress_technology: i32,
    pub vespene_used_in_progress_army: i32,
    pub vespene_used_in_progress_economy: i32,
    pub vespene_used_in_progress_technology: i32,
    pub minerals_used_current_army: i32,
    pub minerals_used_current_economy: i32,
    pub minerals_used_current_technology: i32,
    pub vespene_used_current_army: i32,
    pub vespene_used_current_economy: i32,
    pub vespene_used_current_technology: i32,
    pub minerals_lost_army: i32,
    pub minerals_lost_economy: i32,
    pub minerals_lost_technology: i32,
    pub vespene_lost_army: i32,
    pub vespene_lost_economy: i32,
    pub vespene_lost_technology: i32,
    pub minerals_killed_army: i32,
    pub minerals_killed_economy: i32,
    pub minerals_killed_technology: i32,
    pub vespene_killed_army: i32,
    pub vespene_killed_economy: i32,
    pub vespene_killed_technology: i32,
    pub food_used: i32,
    pub food_made: i32,
    pub minerals_used_active_forces: i32,
    pub vespene_used_active_forces: i32,
    pub minerals_friendly_fire_army: i32,
    pub minerals_friendly_fire_economy: i32,
    pub minerals_friendly_fire_technology: i32,
    pub vespene_friendly_fire_army: i32,
    pub vespene_friendly_fire_economy: i32,
    pub vespene_friendly_fire_technology: i32,
}

impl PlayerStats {
    ///  Creates a vector of Entity Path to value to be consumed by plots
    pub fn as_prop_name_value_vec(&self) -> Vec<(String, i32)> {
        vec![
            (String::from("minerals/current"), self.minerals_current),
            (String::from("vespene/current"), self.vespene_current),
            (
                String::from("minerals/collection_rate"),
                self.minerals_collection_rate,
            ),
            (
                String::from("vespene/collection_rate"),
                self.vespene_collection_rate,
            ),
            (
                String::from("workers_active_count"),
                self.workers_active_count,
            ),
            (
                String::from("minerals/used_in_progress_army"),
                self.minerals_used_in_progress_army,
            ),
            (
                String::from("minerals/used_in_progress_economy"),
                self.minerals_used_in_progress_economy,
            ),
            (
                String::from("minerals/used_in_progress_technology"),
                self.minerals_used_in_progress_technology,
            ),
            (
                String::from("vespene/used_in_progress_army"),
                self.vespene_used_in_progress_army,
            ),
            (
                String::from("vespene/used_in_progress_economy"),
                self.vespene_used_in_progress_economy,
            ),
            (
                String::from("vespene/used_in_progress_technology"),
                self.vespene_used_in_progress_technology,
            ),
            (
                String::from("minerals/used_current_army"),
                self.minerals_used_current_army,
            ),
            (
                String::from("minerals/used_current_economy"),
                self.minerals_used_current_economy,
            ),
            (
                String::from("minerals/used_current_technology"),
                self.minerals_used_current_technology,
            ),
            (
                String::from("vespene/used_current_army"),
                self.vespene_used_current_army,
            ),
            (
                String::from("vespene/used_current_economy"),
                self.vespene_used_current_economy,
            ),
            (
                String::from("vespene/used_current_technology"),
                self.vespene_used_current_technology,
            ),
            (String::from("minerals/lost_army"), self.minerals_lost_army),
            (
                String::from("minerals/lost_economy"),
                self.minerals_lost_economy,
            ),
            (
                String::from("minerals/lost_technology"),
                self.minerals_lost_technology,
            ),
            (String::from("vespene/lost_army"), self.vespene_lost_army),
            (
                String::from("vespene/lost_economy"),
                self.vespene_lost_economy,
            ),
            (
                String::from("vespene/lost_technology"),
                self.vespene_lost_technology,
            ),
            (
                String::from("minerals/killed_army"),
                self.minerals_killed_army,
            ),
            (
                String::from("minerals/killed_economy"),
                self.minerals_killed_economy,
            ),
            (
                String::from("minerals/killed_technology"),
                self.minerals_killed_technology,
            ),
            (
                String::from("vespene/killed_army"),
                self.vespene_killed_army,
            ),
            (
                String::from("vespene/killed_economy"),
                self.vespene_killed_economy,
            ),
            (
                String::from("vespene/killed_technology"),
                self.vespene_killed_technology,
            ),
            (String::from("food/used"), self.food_used),
            (String::from("food/made"), self.food_made),
            (
                String::from("minerals/used_active_forces"),
                self.minerals_used_active_forces,
            ),
            (
                String::from("vespene/used_active_forces"),
                self.vespene_used_active_forces,
            ),
            (
                String::from("minerals/friendly_fire_army"),
                self.minerals_friendly_fire_army,
            ),
            (
                String::from("minerals/friendly_fire_economy"),
                self.minerals_friendly_fire_economy,
            ),
            (
                String::from("minerals/friendly_fire_technology"),
                self.minerals_friendly_fire_technology,
            ),
            (
                String::from("vespene/friendly_fire_army"),
                self.vespene_friendly_fire_army,
            ),
            (
                String::from("vespene/friendly_fire_economy"),
                self.vespene_friendly_fire_economy,
            ),
            (
                String::from("vespene/friendly_fire_technology"),
                self.vespene_friendly_fire_technology,
            ),
        ]
    }
}
