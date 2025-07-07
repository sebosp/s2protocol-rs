//! Experimenting with Arrow integration
//! The rows can then be stored in .ipc file and loaded from polars without joins or needing to
//! unnest complex structs

pub mod player_stats_flat_row;
pub mod unit_born_event_flat_row;
pub mod unit_died_event_flat_row;
pub mod upgrade_event_flat_row;

pub use player_stats_flat_row::PlayerStatsFlatRow;
pub use unit_born_event_flat_row::UnitBornEventFlatRow;
pub use unit_died_event_flat_row::UnitDiedEventFlatRow;
pub use upgrade_event_flat_row::UpgradeEventFlatRow;
