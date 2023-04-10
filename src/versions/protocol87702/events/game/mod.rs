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

impl GameEEventId {
    /// Reads a delta, GameEvent set
    #[tracing::instrument(name="GameEvents::parse_events", level = "debug", skip(input), fields(peek = peek_bits(input)))]
    pub fn parse_event_triplet(
        input: (&[u8], usize),
    ) -> IResult<(&[u8], usize), (i64, i64, GameEEventId)> {
        let (tail, delta) = SVarUint32::parse(input)?;
        tracing::debug!("Delta: {:?}", delta);
        let (tail, user_id) = ReplaySGameUserId::parse(tail)?;
        tracing::debug!("UserId: {:?}", user_id);
        let (tail, event) = GameEEventId::parse(tail)?;
        tracing::debug!("Event: {:?}", event);
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
    pub fn read_events(mpq: &MPQ, file_contents: &[u8]) -> Vec<GameEvent> {
        // TODO: Make it return an Iterator.
        let (_event_tail, game_events) = mpq
            .read_mpq_file_sector("replay.game.events", false, &file_contents)
            .unwrap();
        let mut res = vec![];
        let mut count = 1usize;
        let mut event_tail: (&[u8], usize) = (&game_events, 0usize);
        loop {
            tracing::debug!("-----------------------------------------------");
            tracing::debug!("Event number: {}", count);
            let (new_event_tail, (delta, user_id, event)) =
                Self::parse_event_triplet(event_tail).expect("Unable to parse GameEvents");
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
        res
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

impl From<Uint32> for u32 {
    fn from(source: Uint32) -> u32 {
        source.value as u32
    }
}

impl From<Int8> for i8 {
    fn from(source: Int8) -> i8 {
        source.value as i8
    }
}

impl From<Uint8> for u8 {
    fn from(source: Uint8) -> u8 {
        source.value as u8
    }
}

impl From<Uint16> for u16 {
    fn from(source: Uint16) -> u16 {
        source.value as u16
    }
}

impl From<Int16> for i16 {
    fn from(source: Int16) -> i16 {
        source.value as i16
    }
}

impl TryFrom<GameEEventId> for ReplayGameEvent {
    type Error = GameEventError;
    fn try_from(value: GameEEventId) -> Result<Self, Self::Error> {
        match value {
            GameEEventId::ECameraSave(e) => Ok(e.into()),
            GameEEventId::ECmd(e) => Ok(e.into()),
            GameEEventId::EUnitClick(e) => Ok(e.into()),
            GameEEventId::EUnitHighlight(e) => Ok(e.into()),
            GameEEventId::ECameraUpdate(e) => Ok(e.into()),
            GameEEventId::ETriggerMouseClicked(e) => Ok(e.into()),
            GameEEventId::ETriggerMouseMoved(e) => Ok(e.into()),
            GameEEventId::ETriggerMouseWheel(e) => Ok(e.into()),
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
        source.value.value as i64
    }
}

impl From<GameTFixedMiniBitsSigned> for game_events::GameTFixedMiniBitsSigned {
    fn from(source: GameTFixedMiniBitsSigned) -> game_events::GameTFixedMiniBitsSigned {
        source.value.value as i16
    }
}
