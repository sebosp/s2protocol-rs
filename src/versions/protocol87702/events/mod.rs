//! Converts events from protocol-version specific to protocol-agnostic versions.

pub mod game;
pub mod tracker;
pub use super::*;
pub use game::*;
pub use tracker::*;
