//! Upgrade Events in a flat row for Arrow usage

#[cfg(feature = "dep_arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::tracker_events::UpgradeEvent;
use serde::{Deserialize, Serialize};

/// An experiment creating a flat row of PlayerStats for Arrow usage
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UpgradeEventFlatRow {
    pub player_id: u8,
    pub name: String,
    pub count: i32,
    // TODO: consider deprecating the replay loop and just using seconds
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_replay_sha256: String,
}

impl UpgradeEventFlatRow {
    /// Create a new UpgradeEventFlatRow from a UpgradeEvent and the fields from the Details MPQ sector
    pub fn new(
        event: UpgradeEvent,
        ext_replay_loop: i64,
        details: crate::details::Details,
    ) -> Self {
        let ext_replay_seconds = crate::convert_tracker_loop_to_seconds(ext_replay_loop);
        Self {
            player_id: event.player_id,
            name: event.upgrade_type_name,
            count: event.count,
            ext_replay_loop,
            ext_replay_seconds,
            ext_fs_replay_sha256: details.ext_fs_replay_sha256,
        }
    }
}
