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

impl From<GameSCmdEvent> for ReplayGameEvent {
    fn from(source: GameSCmdEvent) -> ReplayGameEvent {
        let m_abil = match source.m_abil {
            Some(val) => Some(val.into()),
            None => None,
        };
        ReplayGameEvent::Cmd(crate::game_events::GameSCmdEvent {
            m_cmd_flags: source.m_cmd_flags,
            m_abil,
            m_data: source.m_data.into(),
            m_sequence: source.m_sequence,
            m_other_unit: source.m_other_unit.map(|u| u.value.value as u32),
            m_unit_group: source.m_unit_group.map(|g| g.value as u32),
        })
    }
}

impl From<GameSCmdAbil> for crate::game_events::GameSCmdAbil {
    fn from(source: GameSCmdAbil) -> crate::game_events::GameSCmdAbil {
        crate::game_events::GameSCmdAbil {
            m_abil_link: source.m_abil_link.value.value as i32,
            m_abil_cmd_index: source.m_abil_cmd_index,
            m_abil_cmd_data: source.m_abil_cmd_data.map(|d| d.value as u8),
        }
    }
}

impl From<GameSCmdData> for crate::game_events::GameSCmdData {
    fn from(source: GameSCmdData) -> crate::game_events::GameSCmdData {
        match source {
            GameSCmdData::None(()) => crate::game_events::GameSCmdData::None,
            GameSCmdData::TargetPoint(val) => {
                crate::game_events::GameSCmdData::TargetPoint(val.into())
            }
            GameSCmdData::TargetUnit(val) => {
                crate::game_events::GameSCmdData::TargetUnit(val.into())
            }
            GameSCmdData::Data(val) => crate::game_events::GameSCmdData::Data(val.into()),
        }
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

impl From<GameSCmdDataTargetUnit> for crate::game_events::GameSCmdDataTargetUnit {
    fn from(source: GameSCmdDataTargetUnit) -> crate::game_events::GameSCmdDataTargetUnit {
        crate::game_events::GameSCmdDataTargetUnit {
            m_target_unit_flags: source.m_target_unit_flags.value as u16,
            m_timer: source.m_timer.value as u8,
            m_tag: source.m_tag.value.into(),
            m_snapshot_unit_link: source.m_snapshot_unit_link.value.into(),
            m_snapshot_control_player_id: source.m_snapshot_control_player_id.map(|p| p.value),
            m_snapshot_upkeep_player_id: source.m_snapshot_upkeep_player_id.map(|p| p.value),
            m_snapshot_point: source.m_snapshot_point.into(),
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

impl From<Uint16> for u16 {
    fn from(source: Uint16) -> u16 {
        source.value as u16
    }
}

impl TryFrom<GameEEventId> for ReplayGameEvent {
    type Error = GameEventError;
    fn try_from(value: GameEEventId) -> Result<Self, Self::Error> {
        match value {
            GameEEventId::ECameraSave(e) => Ok(e.into()),
            GameEEventId::ECmd(e) => Ok(e.into()),
            GameEEventId::ECameraUpdate(e) => Ok(e.into()),
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
