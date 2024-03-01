//! Converts events from protocol-version specific to protocol-agnostic versions.

pub mod common;
pub mod details;
pub mod game;
pub mod init_data;
pub mod message;
pub mod tracker;
pub use super::*;
pub use game::*;
