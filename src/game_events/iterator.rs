//! Iterator for GameEEventId

use crate::error::S2ProtocolError;

use crate::game_events::{GameEvent, VersionedBalanceUnit};
use crate::versions::protocol75689::bit_packed::GameEEventId as Protocol75689GameEEventId;
use crate::versions::protocol87702::bit_packed::GameEEventId as Protocol87702GameEEventId;
use crate::{SC2EventType, SC2UserState};
use nom::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Keeps track of the progress of the iterator through the open MPQ file.
/// This can be re-used by other iterators.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameEventIteratorState {
    /// The MPQ replay.game.events sector bytes
    event_data: Vec<u8>,
    /// The accumulated game event delta
    event_loop: i64,
    /// The current byte index consumed by nom
    byte_index: usize,
    /// The current bit offset within the current byte consumed by nom
    bit_index: usize,
}

impl From<Vec<u8>> for GameEventIteratorState {
    fn from(event_data: Vec<u8>) -> Self {
        Self {
            event_data,
            event_loop: 0,
            byte_index: 0,
            bit_index: 0,
        }
    }
}

impl GameEventIteratorState {
    /// Based on the protocol version, returns the GameEvent pair.
    #[tracing::instrument(level = "debug", skip(self), fields(event_loop = self.event_loop))]
    pub fn read_versioned_game_event(
        &mut self,
        protocol_version: u32,
    ) -> Result<GameEvent, S2ProtocolError> {
        let (delta, user_id, event) = match protocol_version {
            0..=75689 => {
                let ((tail_byte, tail_bit), (delta, user_id, event)) =
                    Protocol75689GameEEventId::parse_event_triplet((
                        &self.event_data[self.byte_index..],
                        self.bit_index,
                    ))?;
                self.byte_index += tail_byte.as_ptr() as usize
                    - self.event_data[self.byte_index..].as_ptr() as usize;
                self.bit_index = tail_bit;
                self.event_loop += delta;
                (delta, user_id, event.try_into()?)
            }
            _ => {
                // The protocol may or may not be compatible, so we try to parse it anyway.
                let ((tail_byte, tail_bit), (delta, user_id, event)) =
                    Protocol87702GameEEventId::parse_event_triplet((
                        &self.event_data[self.byte_index..],
                        self.bit_index,
                    ))?;
                self.byte_index += tail_byte.as_ptr() as usize
                    - self.event_data[self.byte_index..].as_ptr() as usize;
                self.bit_index = tail_bit;
                self.event_loop += delta;
                (delta, user_id, event.try_into()?)
            }
        };
        Ok(GameEvent {
            delta,
            user_id,
            event,
        })
    }

    /// Attempt to find the next possible supported event.
    /// If an event is not "de-versioned", then it is skipped, thus the internal loop
    #[tracing::instrument(level = "debug", skip(self), fields(event_loop = self.event_loop))]
    pub fn get_next_event(
        &mut self,
        protocol_version: u32,
        user_state: &HashMap<i64, SC2UserState>,
        abilities: &HashMap<String, VersionedBalanceUnit>,
    ) -> Option<SC2EventType> {
        loop {
            let current_slice: &[u8] = &self.event_data[self.byte_index..];
            if current_slice.input_len() == 0 {
                return None;
            }

            // After the event is collected, the loop is adjusted to be in the same units as the
            // game loops.
            match self.read_versioned_game_event(protocol_version) {
                Ok(val) => {
                    // destructure the event into separate variables:
                    let GameEvent {
                        delta: _,
                        user_id,
                        event,
                    } = val;
                    let mut player_name: Option<String> = None;
                    if let Some(player_state) = user_state.get(&user_id) {
                        player_name = Some(player_state.player_details.name.clone());
                    }
                    let event = SC2EventType::Game {
                        game_loop: self.event_loop,
                        event: event.clone(),
                        user_id,
                        player_name,
                    };
                    return Some(event);
                }
                Err(S2ProtocolError::UnsupportedEventType) => {}
                Err(err) => {
                    // At this point we can't read the events, our state is either corrupted or the
                    // SC2ReplayState is corrupted.
                    tracing::error!("Error reading game event: {:?}", err);
                    return None;
                }
            }
        }
    }
}
