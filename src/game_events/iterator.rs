//! Iterator for GameEEventId

use super::handle_game_event;
use crate::error::S2ProtocolError;
use crate::game_events::GameEvent;
use crate::versions::protocol75689::bit_packed::GameEEventId as Protocol75689GameEEventId;
use crate::versions::protocol87702::bit_packed::GameEEventId as Protocol87702GameEEventId;
use crate::{SC2EventType, SC2ReplayState, UnitChangeHint};
use nom::*;
use std::iter::Iterator;
use std::path::PathBuf;

/// Keeps track of the progress of the iterator through the open MPQ file.
/// This can be re-used by other iterators.
#[derive(Debug)]
pub struct GameEventIteratorState {
    /// The MPQ replay.game.events sector bytes
    event_data: Vec<u8>,
    /// The accumulated game event delta
    event_loop: i64,
    /// The current (byte, bit) index consumed by nom
    bit_index: (usize, usize),
}

impl From<Vec<u8>> for GameEventIteratorState {
    fn from(event_data: Vec<u8>) -> Self {
        Self {
            event_data,
            event_loop: 0,
            bit_index: (0, 0),
        }
    }
}

impl GameEventIteratorState {
    /// Based on the protocol version, returns the GameEvent pair.
    pub fn read_versioned_game_event(
        &mut self,
        protocol_version: u32,
    ) -> Result<GameEvent, S2ProtocolError> {
        let (new_event_tail, delta, user_id, event) = match protocol_version {
            75689 => {
                let (new_event_tail, (delta, user_id, event)) =
                    Protocol75689GameEEventId::parse_event_triplet((
                        &self.event_data[self.bit_index.0..],
                        self.bit_index.1,
                    ))?;
                (new_event_tail, delta, user_id, event.try_into()?)
            }
            83830 | 84643 | 88500 | 86383 | 87702 | 89165 | 89634 | 89720 | 90136 | 90779
            | 90870 => {
                // The protocol is known to be compatible with these versions
                let (new_event_tail, (delta, user_id, event)) =
                    Protocol87702GameEEventId::parse_event_triplet((
                        &self.event_data[self.bit_index.0..],
                        self.bit_index.1,
                    ))?;
                (new_event_tail, delta, user_id, event.try_into()?)
            }
            _ => {
                tracing::warn!(
                    "Unknown protocol version: {}, will attempt to use 87702",
                    protocol_version
                );
                // The protocol is known to be compatible with these versions
                let (new_event_tail, (delta, user_id, event)) =
                    Protocol87702GameEEventId::parse_event_triplet((
                        &self.event_data[self.bit_index.0..],
                        self.bit_index.1,
                    ))?;
                (new_event_tail, delta, user_id, event.try_into()?)
            }
        };
        self.bit_index.0 += new_event_tail.0.as_ptr() as usize
            - self.event_data[self.bit_index.0..].as_ptr() as usize;
        self.bit_index.1 = new_event_tail.1;
        self.event_loop += delta;
        Ok(GameEvent {
            delta,
            user_id,
            event,
        })
    }

    /// Attempt to find the next possible supported event.
    /// If an event is not "de-versioned", then it is skipped, thus the internal loop
    pub fn transist_to_next_supported_event(
        &mut self,
        protocol_version: u32,
        sc2_state: &mut SC2ReplayState,
    ) -> Option<(SC2EventType, UnitChangeHint)> {
        loop {
            let current_slice: &[u8] = &self.event_data[self.bit_index.0..];
            if current_slice.input_len() == 0 {
                return None;
            }

            // After the event is collected, the loop is adjusted to be in the same units as the
            // game loops.
            match self.read_versioned_game_event(protocol_version) {
                Ok(val) => {
                    let updated_hint =
                        handle_game_event(sc2_state, self.event_loop, val.user_id, &val.event);
                    return Some((
                        SC2EventType::Game {
                            game_loop: self.event_loop,
                            event: val.event,
                            user_id: val.user_id,
                        },
                        updated_hint,
                    ));
                }
                Err(S2ProtocolError::UnsupportedEventType) => {}
                Err(err) => {
                    tracing::error!("Error reading tracker event: {:?}", err);
                    return None;
                }
            }
        }
    }
}

/// Creates an iterator over the GameEvents
/// This Iterator is for consumers that are only interested in the GameEvents and not any
/// TrackerEvents
#[derive(Debug)]
pub struct GameEventIterator {
    /// The protocol version
    protocol_version: u32,
    /// The replay state machine transducer
    sc2_state: SC2ReplayState,
    /// The IteratorState
    iterator_state: GameEventIteratorState,
}

impl GameEventIterator {
    /// Creates a new GameEventIterator from a PathBuf
    #[tracing::instrument(level = "debug")]
    pub fn new(source: &PathBuf) -> Result<Self, S2ProtocolError> {
        tracing::debug!("Processing {:?}", source);
        let source_filename = format!("{:?}", source);
        let file_contents = crate::read_file(source)?;
        let (_input, mpq) = crate::parser::parse(&file_contents)?;
        let (_tail, proto_header) = crate::read_protocol_header(&mpq)?;
        let (_event_tail, game_events) =
            mpq.read_mpq_file_sector("replay.game.events", false, &file_contents)?;
        Self::from_versioned_mpq(
            source_filename,
            game_events,
            proto_header.m_version.m_base_build,
        )
    }

    #[tracing::instrument(level = "debug")]
    pub fn from_versioned_mpq(
        source_filename: String,
        game_events: Vec<u8>,
        protocol_version: u32,
    ) -> Result<Self, S2ProtocolError> {
        let sc2_state = SC2ReplayState {
            filename: source_filename,
            ..Default::default()
        };
        Ok(Self {
            protocol_version,
            sc2_state,
            iterator_state: GameEventIteratorState {
                event_data: game_events,
                event_loop: 0,
                bit_index: (0, 0),
            },
        })
    }
}

impl Iterator for GameEventIterator {
    /// The item is a tuple of the SC2EventType with the accumulated game loop, and a
    /// hint of what has changed.
    type Item = (SC2EventType, UnitChangeHint);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator_state
            .transist_to_next_supported_event(self.protocol_version, &mut self.sc2_state)
    }
}
