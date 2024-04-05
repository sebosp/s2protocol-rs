//! Experimenting with Arrow integration
//! The rows can then be stored in .ipc file and loaded from polars without joins or needing to
//! unnest complex structs

pub mod cmd_event_flat_row;
pub use cmd_event_flat_row::CmdEventFlatRow;
