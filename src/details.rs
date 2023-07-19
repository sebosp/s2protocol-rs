//! Decodes the Details.

use serde::{Deserialize, Serialize};
use std::str::Utf8Error;

/// A list of errors when handling MessageEvents
#[derive(Debug, thiserror::Error)]
pub enum DetailsError {
    /// An error to be used in TryFrom, when converting from protocol-specific types into our
    /// consolidated-types
    #[error("Unsupported Event Type")]
    UnsupportedEventType,
    /// Conversion to UTF-8 failed, from the Vec<u8> _name fields in the proto fields
    #[error("Utf8 conversion error")]
    Utf8Error(#[from] Utf8Error),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Details {
    pub delta: i64,
    pub user_id: i64,
    pub event: ReplayMessageEvent,
}
