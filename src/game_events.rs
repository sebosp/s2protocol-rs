//! Decodes the Game Events.
//! These are stored in an embebdded file in the MPQ file called 'replay.game.events'

use std::convert::TryFrom;
use std::str::Utf8Error;

/// A list of errors when handling TrackerEvents
#[derive(Debug, thiserror::Error)]
pub enum GameEventError {
    /// An error to be used in TryFrom, when converting from protocol-specific types into our
    /// consolidated-types
    #[error("Unsupported Event Type")]
    UnsupportedEventType,
    /// Conversion to UTF-8 failed, from the Vec<u8> _name fields in the proto fields
    #[error("Utf8 conversion error")]
    Utf8Error(#[from] Utf8Error),
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameEvent {
    pub delta: i64,
    pub user_id: i64,
    pub event: ReplayGameEvent,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReplayGameEvent {
    CameraSave(GameSCameraSaveEvent),
    /*Cmd(GameSCmdEvent),
    SelectionDelta(GameSSelectionDeltaEvent),
    ControlGroupUpdate(GameSControlGroupUpdateEvent),
    SelectionSyncCheck(GameSSelectionSyncCheckEvent),
    TriggerChatMessage(GameSTriggerChatMessageEvent),
    UnitClick(GameSUnitClickEvent),
    UnitHighlight(GameSUnitHighlightEvent),
    TriggerReplySelected(GameSTriggerReplySelectedEvent),
    CameraUpdate(GameSCameraUpdateEvent),
    TriggerMouseClicked(GameSTriggerMouseClickedEvent),
    TriggerMouseMoved(GameSTriggerMouseMovedEvent),
    TriggerHotkeyPressed(GameSTriggerHotkeyPressedEvent),
    TriggerTargetModeUpdate(GameSTriggerTargetModeUpdateEvent),
    TriggerKeyPressed(GameSTriggerKeyPressedEvent),
    TriggerMouseWheel(GameSTriggerMouseWheelEvent),
    TriggerButtonPressed(GameSTriggerButtonPressedEvent),
    GameUserLeave(GameSGameUserLeaveEvent),
    GameUserJoin(GameSGameUserJoinEvent),
    CommandManagerState(GameSCommandManagerStateEvent),
    CmdUpdateTargetPoint(GameSCmdUpdateTargetPointEvent),
    CmdUpdateTargetUnit(GameSCmdUpdateTargetUnitEvent),
    TriggerAnimLengthQueryByName(GameSTriggerAnimLengthQueryByNameEvent),
    TriggerAnimLengthQueryByProps(GameSTriggerAnimLengthQueryByPropsEvent),*/
}
#[derive(Debug, PartialEq, Clone)]
pub struct GameSCameraSaveEvent {
    pub m_which: i64,
    pub m_target: GameSPointMini,
}

pub type GameTFixedMiniBitsUnsigned = i64;

#[derive(Debug, PartialEq, Clone)]
pub struct GameSPointMini {
    pub x: GameTFixedMiniBitsUnsigned,
    pub y: GameTFixedMiniBitsUnsigned,
}

impl TryFrom<crate::versions::protocol87702::bit_packed::GameEEventId> for ReplayGameEvent {
    type Error = GameEventError;
    fn try_from(
        value: crate::versions::protocol87702::bit_packed::GameEEventId,
    ) -> Result<Self, Self::Error> {
        match value {
            crate::versions::protocol87702::bit_packed::GameEEventId::ECameraSave(e) => {
                Ok(e.to_versionless()?)
            }
            _ => Err(GameEventError::UnsupportedEventType),
        }
    }
}

impl crate::versions::protocol87702::bit_packed::GameSCameraSaveEvent {
    pub fn to_versionless(self) -> Result<ReplayGameEvent, GameEventError> {
        Ok(ReplayGameEvent::CameraSave(GameSCameraSaveEvent {
            m_which: self.m_which,
            m_target: GameSPointMini {
                x: self.m_target.x.value.value,
                y: self.m_target.y.value.value,
            },
        }))
    }
}

impl TryFrom<crate::versions::protocol88500::bit_packed::GameEEventId> for ReplayGameEvent {
    type Error = GameEventError;
    fn try_from(
        value: crate::versions::protocol88500::bit_packed::GameEEventId,
    ) -> Result<Self, Self::Error> {
        match value {
            crate::versions::protocol88500::bit_packed::GameEEventId::ECameraSave(e) => {
                Ok(e.to_versionless()?)
            }
            _ => Err(GameEventError::UnsupportedEventType),
        }
    }
}

impl crate::versions::protocol88500::bit_packed::GameSCameraSaveEvent {
    pub fn to_versionless(self) -> Result<ReplayGameEvent, GameEventError> {
        Ok(ReplayGameEvent::CameraSave(GameSCameraSaveEvent {
            m_which: self.m_which,
            m_target: GameSPointMini {
                x: self.m_target.x.value.value,
                y: self.m_target.y.value.value,
            },
        }))
    }
}

impl TryFrom<crate::versions::protocol89634::bit_packed::GameEEventId> for ReplayGameEvent {
    type Error = GameEventError;
    fn try_from(
        value: crate::versions::protocol89634::bit_packed::GameEEventId,
    ) -> Result<Self, Self::Error> {
        match value {
            crate::versions::protocol89634::bit_packed::GameEEventId::ECameraSave(e) => {
                Ok(e.to_versionless()?)
            }
            _ => Err(GameEventError::UnsupportedEventType),
        }
    }
}

impl crate::versions::protocol89634::bit_packed::GameSCameraSaveEvent {
    pub fn to_versionless(self) -> Result<ReplayGameEvent, GameEventError> {
        Ok(ReplayGameEvent::CameraSave(GameSCameraSaveEvent {
            m_which: self.m_which,
            m_target: GameSPointMini {
                x: self.m_target.x.value.value,
                y: self.m_target.y.value.value,
            },
        }))
    }
}

impl TryFrom<crate::versions::protocol89720::bit_packed::GameEEventId> for ReplayGameEvent {
    type Error = GameEventError;
    fn try_from(
        value: crate::versions::protocol89720::bit_packed::GameEEventId,
    ) -> Result<Self, Self::Error> {
        match value {
            crate::versions::protocol89720::bit_packed::GameEEventId::ECameraSave(e) => {
                Ok(e.to_versionless()?)
            }
            _ => Err(GameEventError::UnsupportedEventType),
        }
    }
}

impl crate::versions::protocol89720::bit_packed::GameSCameraSaveEvent {
    pub fn to_versionless(self) -> Result<ReplayGameEvent, GameEventError> {
        Ok(ReplayGameEvent::CameraSave(GameSCameraSaveEvent {
            m_which: self.m_which,
            m_target: GameSPointMini {
                x: self.m_target.x.value.value,
                y: self.m_target.y.value.value,
            },
        }))
    }
}
