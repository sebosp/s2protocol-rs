//! Iterator for GameEEventId

use super::handle_game_event;
use crate::error::S2ProtocolError;
#[cfg(feature = "dep_arrow")]
use crate::game_events;

use crate::game_events::GameEvent;
use crate::versions::protocol75689::bit_packed::GameEEventId as Protocol75689GameEEventId;
use crate::versions::protocol87702::bit_packed::GameEEventId as Protocol87702GameEEventId;
use crate::SC2ReplayFilters;
use crate::{SC2EventType, SC2ReplayState, UnitChangeHint};
use nom::*;
use serde::{Deserialize, Serialize};
use std::iter::Iterator;
use std::path::PathBuf;

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
    #[tracing::instrument(level = "debug", skip(self, sc2_state, filters), fields(event_loop = self.event_loop))]
    pub fn transist_to_next_supported_event(
        &mut self,
        protocol_version: u32,
        sc2_state: &mut SC2ReplayState,
        filters: &mut Option<SC2ReplayFilters>,
    ) -> Option<(SC2EventType, UnitChangeHint)> {
        loop {
            let current_slice: &[u8] = &self.event_data[self.byte_index..];
            if current_slice.input_len() == 0 {
                return None;
            }

            // After the event is collected, the loop is adjusted to be in the same units as the
            // game loops.
            match self.read_versioned_game_event(protocol_version) {
                Ok(val) => {
                    let updated_hint =
                        handle_game_event(sc2_state, self.event_loop, val.user_id, &val.event);
                    tracing::info!(
                        "Game [{:>08}]: uid: {} Evt:{:?} Hint:{:?}",
                        self.event_loop,
                        val.user_id,
                        val.event,
                        updated_hint
                    );
                    let event = SC2EventType::Game {
                        game_loop: self.event_loop,
                        event: val.event,
                        user_id: val.user_id,
                    };
                    if let Some(filters) = filters {
                        if self.shoud_skip_event(&event, filters) {
                            continue;
                        }
                        filters.decrease_allowed_event_counter();
                        if filters.is_max_event_reached() {
                            return None;
                        }
                    }
                    return Some((event, updated_hint));
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

    /// Returns true if the event should be skipped based on the filters
    fn shoud_skip_event(&self, event: &SC2EventType, filters: &SC2ReplayFilters) -> bool {
        if let Some(min_loop) = filters.min_loop {
            if let SC2EventType::Tracker { tracker_loop, .. } = event {
                if *tracker_loop < min_loop {
                    return true;
                }
            }
        }
        if let Some(max_loop) = filters.max_loop {
            if let SC2EventType::Tracker { tracker_loop, .. } = event {
                if *tracker_loop > max_loop {
                    return true;
                }
            }
        }
        event.should_skip(filters)
    }
}

/// Creates an iterator over the GameEvents
/// This Iterator is for consumers that are only interested in the GameEvents and not any
/// TrackerEvents
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameEventIterator {
    /// The protocol version
    protocol_version: u32,
    /// The replay state machine transducer
    sc2_state: SC2ReplayState,
    /// The IteratorState
    iterator_state: GameEventIteratorState,
    /// The filters to the iterator
    filters: Option<SC2ReplayFilters>,
}

impl GameEventIterator {
    /// Creates a new GameEventIterator from a PathBuf
    #[tracing::instrument(level = "debug")]
    pub fn new(source: &PathBuf) -> Result<Self, S2ProtocolError> {
        tracing::info!("Processing {:?}", source);
        let source_filename = format!("{source:?}");
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

    /// Creates a new GameEventIterator from previously read MPQ content
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
                byte_index: 0,
                bit_index: 0,
            },
            ..Default::default()
        })
    }

    /// Sets the filters for the iterator
    pub fn with_filters(mut self, filters: SC2ReplayFilters) -> Self {
        self.filters = Some(filters);
        self
    }

    /// Consumes the Iterator collecting only the CmdTargetPoint events into a vector of CmdEventFlatRow
    #[cfg(feature = "dep_arrow")]
    pub fn collect_into_game_cmd_target_points_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<game_events::CmdTargetPointEventFlatRow> {
        self.into_iter()
            .filter_map(|(sc2_event, change_hint)| match sc2_event {
                SC2EventType::Game {
                    event,
                    game_loop,
                    user_id,
                } => {
                    if let game_events::ReplayGameEvent::Cmd(event) = event {
                        if let game_events::GameSCmdData::TargetPoint(_) = event.m_data {
                            Some(game_events::CmdTargetPointEventFlatRow::new(
                                details,
                                event,
                                game_loop,
                                user_id,
                                change_hint,
                            ))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }

    /// Consumes the Iterator collecting only the CmdTargetUnit events into a vector of CmdEventFlatRow
    #[cfg(feature = "dep_arrow")]
    pub fn collect_into_game_cmd_target_units_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<game_events::CmdTargetUnitEventFlatRow> {
        self.into_iter()
            .filter_map(|(sc2_event, change_hint)| match sc2_event {
                SC2EventType::Game {
                    event,
                    game_loop,
                    user_id,
                } => {
                    if let game_events::ReplayGameEvent::Cmd(event) = event {
                        if let game_events::GameSCmdData::TargetUnit(_) = event.m_data {
                            Some(game_events::CmdTargetUnitEventFlatRow::new(
                                details,
                                event,
                                game_loop,
                                user_id,
                                change_hint,
                            ))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }
}

impl Iterator for GameEventIterator {
    /// The item is a tuple of the SC2EventType with the accumulated game loop, and a
    /// hint of what has changed.
    type Item = (SC2EventType, UnitChangeHint);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator_state.transist_to_next_supported_event(
            self.protocol_version,
            &mut self.sc2_state,
            &mut self.filters,
        )
    }
}
