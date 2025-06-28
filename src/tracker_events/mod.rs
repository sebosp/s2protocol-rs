//! Decodes the Tracker Events.
//! These are stored in an embebdded file in the MPQ file called 'replay.tracker.events'

#[cfg(feature = "dep_arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};
#[cfg(feature = "dep_arrow")]
pub mod arrow_store;
#[cfg(feature = "dep_arrow")]
pub use arrow_store::*;

use crate::filters::SC2ReplayFilters;
use serde::{Deserialize, Serialize};

pub mod iterator;
pub mod player_setup;
pub mod player_stats;
pub mod player_upgrade;
pub mod state;
pub mod unit_born;
pub mod unit_died;
pub mod unit_done;
pub mod unit_init;
pub mod unit_owner_change;
pub mod unit_positions;
pub mod unit_type_change;
pub use iterator::*;
pub use player_setup::*;
pub use player_stats::*;
pub use player_upgrade::*;
pub use state::*;
pub use unit_born::*;
pub use unit_died::*;
pub use unit_done::*;
pub use unit_init::*;
pub use unit_owner_change::*;
pub use unit_positions::*;
pub use unit_type_change::*;

/// A Tracker Event is an event in the gameloop for a specific user id
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct TrackerEvent {
    pub delta: u32,
    pub event: ReplayTrackerEvent,
}

/// A unified Replay Tracker that is agnostic of any version.
/// This should hopefully only add fields to variants to make things backwards compatible
/// Many of the variants are not supported yet, they will be added as they are considered
/// relevant  for `swarmy` repo.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "dep_arrow", arrow_field(type = "sparse"))]
pub enum ReplayTrackerEvent {
    PlayerStats(PlayerStatsEvent),
    UnitBorn(UnitBornEvent),
    UnitDied(UnitDiedEvent),
    UnitOwnerChange(UnitOwnerChangeEvent),
    UnitTypeChange(UnitTypeChangeEvent),
    Upgrade(UpgradeEvent),
    UnitInit(UnitInitEvent),
    UnitDone(UnitDoneEvent),
    UnitPosition(UnitPositionsEvent),
    PlayerSetup(PlayerSetupEvent),
}

impl ReplayTrackerEvent {
    pub fn should_skip(&self, filters: &SC2ReplayFilters) -> bool {
        match self {
            ReplayTrackerEvent::PlayerStats(evt) => evt.should_skip(filters),
            ReplayTrackerEvent::UnitBorn(evt) => evt.should_skip(filters),
            ReplayTrackerEvent::UnitDied(evt) => evt.should_skip(filters),
            ReplayTrackerEvent::UnitOwnerChange(evt) => evt.should_skip(filters),
            ReplayTrackerEvent::UnitTypeChange(evt) => evt.should_skip(filters),
            ReplayTrackerEvent::Upgrade(evt) => evt.should_skip(filters),
            ReplayTrackerEvent::UnitInit(evt) => evt.should_skip(filters),
            ReplayTrackerEvent::UnitDone(evt) => evt.should_skip(filters),
            ReplayTrackerEvent::UnitPosition(evt) => evt.should_skip(filters),
            ReplayTrackerEvent::PlayerSetup(evt) => evt.should_skip(filters),
        }
    }
}

/// Translates from a Tracker Event tag_index, tag_recycle pair into the unit tag used in the Game
/// Events
pub fn unit_tag(unit_tag_index: u32, unit_tag_recycle: u32) -> i64 {
    ((unit_tag_index as i64) << 18usize) + unit_tag_recycle as i64
}

/// Extracts the Unit Tag Recycle index from a Game Event Unit Tag
/// Useful to correlate between Game and Tracker Event units
pub fn unit_tag_index(unit_tag: i64) -> u32 {
    ((unit_tag >> 18) & 0x00003fff) as u32
}

/// Extracts the Unit Tag Recycle index from a Game Event Unit Tag
/// Useful to correlate between Game and Tracker Event units
pub fn unit_tag_recycle(unit_tag: i64) -> u32 {
    ((unit_tag) & 0x0003ffff) as u32
}
