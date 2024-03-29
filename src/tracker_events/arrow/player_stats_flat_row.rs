//! Player stats flat row

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::tracker_events::PlayerStatsEvent;
use serde::{Deserialize, Serialize};

/// An experiment creating a flat row of PlayerStats for Arrow usage
/// Fields that start with ext_ are foreign fields collected from the Details MPQ sector
/// and the file system (fs) itself.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct PlayerStatsFlatRow {
    pub player_id: u8,
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
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_replay_sha256: String,
}

impl PlayerStatsFlatRow {
    /// Create a new PlayerStatsFlatRow from a PlayerStats and the fields from the Details MPQ sector
    pub fn new(
        event: PlayerStatsEvent,
        ext_replay_loop: i64,
        details: crate::details::Details,
    ) -> Self {
        let ext_replay_seconds = crate::convert_tracker_loop_to_seconds(ext_replay_loop);
        let stats = event.stats;
        Self {
            player_id: event.player_id,
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
            ext_replay_loop,
            ext_replay_seconds,
            ext_fs_replay_sha256: details.ext_fs_replay_sha256,
        }
    }
}
