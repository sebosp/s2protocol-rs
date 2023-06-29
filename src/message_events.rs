//! Decodes the Message Events.
//! These are stored in an embedded file in the MPQ file called 'replay.message.events'
//! Somehow it should 4 bits instead of 3 bits for the GameEMessageId...
//! In our code it translates to 3 bits needed to represent 5 possible enum variants.

use std::str::Utf8Error;

/// A list of errors when handling MessageEvents
#[derive(Debug, thiserror::Error)]
pub enum MessageEventError {
    /// An error to be used in TryFrom, when converting from protocol-specific types into our
    /// consolidated-types
    #[error("Unsupported Event Type")]
    UnsupportedEventType,
    /// Conversion to UTF-8 failed, from the Vec<u8> _name fields in the proto fields
    #[error("Utf8 conversion error")]
    Utf8Error(#[from] Utf8Error),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MessageEvent {
    pub delta: i64,
    pub user_id: i64,
    pub event: ReplayMessageEvent,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReplayMessageEvent {
    /*EChat(GameSChatMessage),
EPing(GameSPingMessage),
ELoadingProgress(GameSLoadingProgressMessage),
EServerPing(GameSServerPingMessage),
EReconnectNotify(GameSReconnectNotifyMessage),*/}
