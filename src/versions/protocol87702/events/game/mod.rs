//! Converts events from protocol-version specific to protocol-agnostic versions.

use super::bit_packed::*;
use crate::game_events::GameEvent;
use crate::game_events::GameEventError;
use crate::game_events::ReplayGameEvent;
use crate::*;
use nom::*;
use nom_mpq::MPQ;

pub mod camera;
pub use camera::*;
pub mod cmd;
pub use cmd::*;
pub mod mouse;
pub use mouse::*;
pub mod unit;
pub use unit::*;
pub mod selection;
pub use selection::*;
pub mod control_group;
pub use control_group::*;
pub mod user;
pub use user::*;

impl GameEEventId {
    /// Reads a delta, GameEvent set
    #[tracing::instrument(name="GameEvents::parse_events", level = "debug", skip(input), fields(peek = peek_bits(input)))]
    pub fn parse_event_triplet(
        input: (&[u8], usize),
    ) -> S2ProtoResult<(&[u8], usize), (i64, i64, GameEEventId)> {
        let (tail, delta) = SVarUint32::parse(input)?;
        let (tail, user_id) = ReplaySGameUserId::parse(tail)?;
        let (tail, event) = GameEEventId::parse(tail)?;
        let delta = match delta {
            SVarUint32::MUint6(val) => val.value,
            SVarUint32::MUint14(val) => val.value,
            SVarUint32::MUint22(val) => val.value,
            SVarUint32::MUint32(val) => val.value,
        };
        // The next event is byte aligned
        let (tail, _) = byte_align(tail)?;
        Ok((tail, (delta, user_id.m_user_id, event)))
    }

    /// Read the Tracker Events
    pub fn read_events(mpq: &MPQ, file_contents: &[u8]) -> Result<Vec<GameEvent>, S2ProtocolError> {
        // TODO: Make it return an Iterator.
        let (_event_tail, game_events) =
            mpq.read_mpq_file_sector("replay.game.events", false, file_contents)?;
        let mut res = vec![];
        let mut count = 1usize;
        let mut event_tail: (&[u8], usize) = (&game_events, 0usize);
        loop {
            tracing::debug!("-----------------------------------------------");
            tracing::debug!("Event number: {}", count);
            let (new_event_tail, (delta, user_id, event)) = Self::parse_event_triplet(event_tail)?;
            count += 1;
            event_tail = new_event_tail;
            match event.try_into() {
                Ok(val) => res.push(GameEvent {
                    delta,
                    user_id,
                    event: val,
                }),
                Err(err) => {
                    tracing::debug!("Skipping event: {:?}", err);
                }
            };
            if event_tail.0.input_len() == 0 {
                break;
            }
        }
        Ok(res)
    }
}

impl From<GameSMapCoord3D> for crate::game_events::GameSMapCoord3D {
    fn from(source: GameSMapCoord3D) -> crate::game_events::GameSMapCoord3D {
        crate::game_events::GameSMapCoord3D {
            x: source.x.value,
            y: source.y.value,
            z: source.z.value.value as i32,
        }
    }
}

impl From<GameSuiCoord> for crate::game_events::GameSuiCoord {
    fn from(source: GameSuiCoord) -> crate::game_events::GameSuiCoord {
        crate::game_events::GameSuiCoord {
            x: source.x.value as u16,
            y: source.y.value as u16,
        }
    }
}

impl TryFrom<GameEEventId> for ReplayGameEvent {
    type Error = GameEventError;
    fn try_from(value: GameEEventId) -> Result<Self, Self::Error> {
        match value {
            GameEEventId::EDropUser(e) => Ok(e.into()),
            GameEEventId::ECameraSave(e) => Ok(e.into()),
            GameEEventId::ECmd(e) => Ok(e.into()),
            GameEEventId::ESelectionDelta(e) => Ok(e.try_into()?),
            GameEEventId::EControlGroupUpdate(e) => Ok(e.try_into()?),
            GameEEventId::ESelectionSyncCheck(e) => Ok(e.into()),
            GameEEventId::ETriggerChatMessage(e) => Ok(e.try_into()?),
            GameEEventId::EUnitClick(e) => Ok(e.into()),
            GameEEventId::EUnitHighlight(e) => Ok(e.into()),
            GameEEventId::ETriggerReplySelected(e) => Ok(e.into()),
            GameEEventId::ECameraUpdate(e) => Ok(e.into()),
            GameEEventId::ETriggerMouseClicked(e) => Ok(e.into()),
            GameEEventId::ETriggerMouseMoved(e) => Ok(e.into()),
            GameEEventId::ETriggerHotkeyPressed(e) => Ok(e.into()),
            GameEEventId::ETriggerTargetModeUpdate(e) => Ok(e.into()),
            GameEEventId::ETriggerKeyPressed(e) => Ok(e.into()),
            GameEEventId::ETriggerMouseWheel(e) => Ok(e.into()),
            GameEEventId::ETriggerButtonPressed(e) => Ok(e.into()),
            GameEEventId::ECommandManagerState(e) => Ok(e.into()),
            GameEEventId::ECmdUpdateTargetPoint(e) => Ok(e.into()),
            GameEEventId::ECmdUpdateTargetUnit(e) => Ok(e.into()),
            _ => Err(GameEventError::UnsupportedEventType),
        }
    }
}

impl From<GameSPointMini> for game_events::GameSPointMini {
    fn from(source: GameSPointMini) -> game_events::GameSPointMini {
        crate::game_events::GameSPointMini {
            x: source.x.value.value,
            y: source.y.value.value,
        }
    }
}

impl From<GameTFixedMiniBitsUnsigned> for game_events::GameTFixedMiniBitsUnsigned {
    fn from(source: GameTFixedMiniBitsUnsigned) -> game_events::GameTFixedMiniBitsUnsigned {
        source.value.value
    }
}

impl From<GameTFixedMiniBitsSigned> for game_events::GameTFixedMiniBitsSigned {
    fn from(source: GameTFixedMiniBitsSigned) -> game_events::GameTFixedMiniBitsSigned {
        source.value.value as i16
    }
}

impl From<GameTSelectionIndex> for game_events::GameTSelectionIndex {
    fn from(source: GameTSelectionIndex) -> game_events::GameTSelectionIndex {
        source.value as u16
    }
}

impl From<GameTUnitLink> for game_events::GameTUnitLink {
    fn from(source: GameTUnitLink) -> game_events::GameTUnitLink {
        source.value.value as u16
    }
}

impl From<GameTUnitTag> for game_events::GameTUnitTag {
    fn from(source: GameTUnitTag) -> game_events::GameTUnitTag {
        source.value.value as u32
    }
}

impl From<GameTControlGroupCount> for game_events::GameTControlGroupCount {
    fn from(source: GameTControlGroupCount) -> game_events::GameTControlGroupCount {
        source.value as u8
    }
}

impl TryFrom<GameSTriggerChatMessageEvent> for game_events::ReplayGameEvent {
    type Error = GameEventError;
    fn try_from(source: GameSTriggerChatMessageEvent) -> Result<Self, Self::Error> {
        Ok(ReplayGameEvent::TriggerChatMessage(
            game_events::GameSTriggerChatMessageEvent {
                m_chat_message: str::from_utf8(&source.m_chat_message.value)?.to_string(),
            },
        ))
    }
}

impl From<GameSTriggerReplySelectedEvent> for game_events::ReplayGameEvent {
    fn from(source: GameSTriggerReplySelectedEvent) -> game_events::ReplayGameEvent {
        game_events::ReplayGameEvent::TriggerReplySelected(
            game_events::GameSTriggerReplySelectedEvent {
                m_conversation_id: source.m_conversation_id.into(),
                m_reply_id: source.m_reply_id.into(),
            },
        )
    }
}

impl From<GameSTriggerHotkeyPressedEvent> for game_events::ReplayGameEvent {
    fn from(source: GameSTriggerHotkeyPressedEvent) -> game_events::ReplayGameEvent {
        game_events::ReplayGameEvent::TriggerHotkeyPressed(
            game_events::GameSTriggerHotkeyPressedEvent {
                m_hotkey: source.m_hotkey.into(),
                m_down: source.m_down,
            },
        )
    }
}

impl From<GameSTriggerTargetModeUpdateEvent> for game_events::ReplayGameEvent {
    fn from(source: GameSTriggerTargetModeUpdateEvent) -> game_events::ReplayGameEvent {
        game_events::ReplayGameEvent::TriggerTargetModeUpdate(
            game_events::GameSTriggerTargetModeUpdateEvent {
                m_abil_link: source.m_abil_link.value.into(),
                m_abil_cmd_index: source.m_abil_cmd_index.into(),
                m_state: source.m_state.into(),
            },
        )
    }
}

impl From<GameSTriggerKeyPressedEvent> for game_events::ReplayGameEvent {
    fn from(source: GameSTriggerKeyPressedEvent) -> game_events::ReplayGameEvent {
        game_events::ReplayGameEvent::TriggerKeyPressed(game_events::GameSTriggerKeyPressedEvent {
            m_key: source.m_key.into(),
            m_flags: source.m_flags.into(),
        })
    }
}

impl From<GameSTriggerButtonPressedEvent> for game_events::ReplayGameEvent {
    fn from(source: GameSTriggerButtonPressedEvent) -> game_events::ReplayGameEvent {
        game_events::ReplayGameEvent::TriggerButtonPressed(
            game_events::GameSTriggerButtonPressedEvent {
                m_button: source.m_button.value.into(),
            },
        )
    }
}
