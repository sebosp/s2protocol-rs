//! Experimenting with Arrow integration
//! The types here contain player_id, game_loop, sha256, file_name, and epoch fields
//! These are fiedls that come from the Details MPQ sector.
//! The rest of the fields are from the TrackerEvent.
//! The row can then be stored in .ipc file and loaded from polars.

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use super::*;
use serde::{Deserialize, Serialize};

/// An experiment creating a flat row of PlayerStats for Arrow usage
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct PlayerStatsFlatRow {
    pub player_id: u8,
    pub game_loop: i64,
    pub sha256: String,
    pub file_name: String,
    pub epoch: i64,
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

impl PlayerStatsFlatRow {
    pub fn new(
        stats: PlayerStats,
        player_id: u8,
        game_loop: i64,
        file_name: String,
        sha256: String,
        epoch: i64,
    ) -> Self {
        Self {
            player_id,
            game_loop,
            sha256,
            file_name,
            epoch,
            minerals_current: stats.minerals_current,
            vespene_current: stats.vespene_current,
            minerals_collection_rate: stats.minerals_collection_rate,
            vespene_collection_rate: stats.vespene_collection_rate,
            workers_active_count: stats.workers_active_count,
            minerals_used_in_progress_army: stats.minerals_used_in_progress_army,
            minerals_used_in_progress_economy: stats.minerals_used_in_progress_economy,
            minerals_used_in_progress_technology: stats.minerals_used_in_progress_technology,
            vespene_used_in_progress_army: stats.vespene_used_in_progress_army,
            vespene_used_in_progress_economy: stats.vespene_used_in_progress_economy,
            vespene_used_in_progress_technology: stats.vespene_used_in_progress_technology,
            minerals_used_current_army: stats.minerals_used_current_army,
            minerals_used_current_economy: stats.minerals_used_current_economy,
            minerals_used_current_technology: stats.minerals_used_current_technology,
            vespene_used_current_army: stats.vespene_used_current_army,
            vespene_used_current_economy: stats.vespene_used_current_economy,
            vespene_used_current_technology: stats.vespene_used_current_technology,
            minerals_lost_army: stats.minerals_lost_army,
            minerals_lost_economy: stats.minerals_lost_economy,
            minerals_lost_technology: stats.minerals_lost_technology,
            vespene_lost_army: stats.vespene_lost_army,
            vespene_lost_economy: stats.vespene_lost_economy,
            vespene_lost_technology: stats.vespene_lost_technology,
            minerals_killed_army: stats.minerals_killed_army,
            minerals_killed_economy: stats.minerals_killed_economy,
            minerals_killed_technology: stats.minerals_killed_technology,
            vespene_killed_army: stats.vespene_killed_army,
            vespene_killed_economy: stats.vespene_killed_economy,
            vespene_killed_technology: stats.vespene_killed_technology,
            food_used: stats.food_used,
            food_made: stats.food_made,
            minerals_used_active_forces: stats.minerals_used_active_forces,
            vespene_used_active_forces: stats.vespene_used_active_forces,
            minerals_friendly_fire_army: stats.minerals_friendly_fire_army,
            minerals_friendly_fire_economy: stats.minerals_friendly_fire_economy,
            minerals_friendly_fire_technology: stats.minerals_friendly_fire_technology,
            vespene_friendly_fire_army: stats.vespene_friendly_fire_army,
            vespene_friendly_fire_economy: stats.vespene_friendly_fire_economy,
            vespene_friendly_fire_technology: stats.vespene_friendly_fire_technology,
        }
    }
}
