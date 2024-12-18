//! Iterator for ReplayTrackerEEventId

use super::handle_tracker_event;
use crate::error::S2ProtocolError;
use crate::tracker_events::{self, TrackerEvent};
use crate::versions::protocol75689::byte_aligned::ReplayTrackerEEventId as Protocol75689ReplayTrackerEEventId;
use crate::versions::protocol87702::byte_aligned::ReplayTrackerEEventId as Protocol87702ReplayTrackerEEventId;
use crate::{SC2EventType, SC2ReplayFilters, SC2ReplayState, UnitChangeHint, TRACKER_SPEED_RATIO};
use nom::*;
use serde::{Deserialize, Serialize};
use std::iter::Iterator;
use std::path::PathBuf;

/// Keeps track of the progress of the iterator through the open MPQ file.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TrackertEventIteratorState {
    /// The MPQ replay.tracker.events sector bytes
    event_data: Vec<u8>,
    /// The accumulated tracker event delta
    event_loop: i64,
    /// The current byte index consumed by nom
    byte_index: usize,
}

impl From<Vec<u8>> for TrackertEventIteratorState {
    fn from(event_data: Vec<u8>) -> Self {
        Self {
            event_data,
            event_loop: 0,
            byte_index: 0,
        }
    }
}

impl TrackertEventIteratorState {
    /// Based on the protocol version, returns the TrackerEvent pair.
    pub fn read_versioned_tracker_event(
        &mut self,
        protocol_version: u32,
    ) -> Result<TrackerEvent, S2ProtocolError> {
        let (delta, event) = match protocol_version {
            0..=75689 => {
                let (tail, (delta, event)) = Protocol75689ReplayTrackerEEventId::parse_event_pair(
                    &self.event_data[self.byte_index..],
                )?;
                self.byte_index +=
                    tail.as_ptr() as usize - self.event_data[self.byte_index..].as_ptr() as usize;
                self.event_loop += delta as i64;
                (delta, event.try_into()?)
            }
            _ => {
                // The protocol may be compatible, if we do not know it, just attempt to parse
                let (tail, (delta, event)) = Protocol87702ReplayTrackerEEventId::parse_event_pair(
                    &self.event_data[self.byte_index..],
                )?;
                self.byte_index +=
                    tail.as_ptr() as usize - self.event_data[self.byte_index..].as_ptr() as usize;
                self.event_loop += delta as i64;
                (delta, event.try_into()?)
            }
        };
        Ok(TrackerEvent { delta, event })
    }

    /// Attempt to find the next possible supported event.
    /// If an event is not "de-versioned", then it is skipped, thus the internal loop
    #[tracing::instrument(level = "debug", skip(self, sc2_state, filters), fields(event_loop = self.event_loop))]
    pub fn transist_to_next_supported_event(
        &mut self,
        protocol_version: u32,
        sc2_state: &mut SC2ReplayState,
        filters: &Option<SC2ReplayFilters>,
    ) -> Option<(SC2EventType, UnitChangeHint)> {
        loop {
            let current_slice: &[u8] = &self.event_data[self.byte_index..];
            if current_slice.input_len() == 0 {
                return None;
            }

            match self.read_versioned_tracker_event(protocol_version) {
                Ok(val) => {
                    // After the event is collected, the loop is adjusted to be in the same units as the
                    // game loops.
                    let adjusted_loop = (self.event_loop as f32 / TRACKER_SPEED_RATIO) as i64;
                    let updated_hint = handle_tracker_event(sc2_state, adjusted_loop, &val.event);
                    tracing::info!(
                        "Trac [{:>08}]: Evt:{:?} Hint:{:?}",
                        adjusted_loop,
                        val.event,
                        updated_hint
                    );
                    let event = SC2EventType::Tracker {
                        tracker_loop: adjusted_loop,
                        event: val.event,
                    };
                    if let Some(filters) = filters {
                        if self.shoud_skip_event(&event, filters) {
                            continue;
                        }
                    }
                    return Some((event, updated_hint));
                }
                Err(S2ProtocolError::UnsupportedEventType) => {}
                Err(err) => {
                    // At this point we can't read the events, our state is either corrupted or the
                    // SC2ReplayState is corrupted.
                    tracing::error!("Error reading tracker event: {:?}", err);
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

/// Creates an iterator over the TrackerEvents
/// This Iterator is for consumers that are only interested in the TrackerEvents and not any
/// GameEvents
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TrackerEventIterator {
    /// The protocol version
    protocol_version: u32,
    /// The replay state machine transducer
    sc2_state: SC2ReplayState,
    /// The IteratorState
    iterator_state: TrackertEventIteratorState,
    /// The filters to the iterator
    filters: Option<SC2ReplayFilters>,
}

impl TrackerEventIterator {
    /// Creates a new TrackerEventIterator from a PathBuf
    #[tracing::instrument(level = "debug")]
    pub fn new(source: &PathBuf) -> Result<Self, S2ProtocolError> {
        tracing::debug!("Processing {:?}", source);
        let file_contents = crate::read_file(source)?;
        let source_filename = format!("{:?}", source);
        let (_input, mpq) = crate::parser::parse(&file_contents)?;
        let (_tail, proto_header) = crate::read_protocol_header(&mpq)?;
        let (_event_tail, tracker_events) =
            mpq.read_mpq_file_sector("replay.tracker.events", false, &file_contents)?;
        Self::from_versioned_mpq(
            source_filename,
            tracker_events,
            proto_header.m_version.m_base_build,
        )
    }

    /// Creates a new GameEventIterator from previously read MPQ content
    #[tracing::instrument(level = "debug")]
    pub fn from_versioned_mpq(
        source_filename: String,
        tracker_events: Vec<u8>,
        protocol_version: u32,
    ) -> Result<Self, S2ProtocolError> {
        let sc2_state = SC2ReplayState {
            filename: source_filename,
            ..Default::default()
        };
        Ok(Self {
            protocol_version,
            sc2_state,
            iterator_state: TrackertEventIteratorState {
                event_data: tracker_events,
                event_loop: 0,
                byte_index: 0,
            },
            ..Default::default()
        })
    }

    /// Consumes the Iterator collecting only the PlayerStats events into a vector of PlayerStatsFlatRow
    pub fn collect_into_player_stats_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::PlayerStatsFlatRow> {
        self.into_iter()
            .filter_map(|(sc2_event, _change_hint)| match sc2_event {
                SC2EventType::Tracker {
                    tracker_loop,
                    event,
                } => {
                    if let tracker_events::ReplayTrackerEvent::PlayerStats(event) = event {
                        Some(tracker_events::PlayerStatsFlatRow::new(
                            event,
                            tracker_loop,
                            details.clone(),
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }

    /// Sets the filters for the iterator
    pub fn with_filters(mut self, filters: SC2ReplayFilters) -> Self {
        self.filters = Some(filters);
        self
    }

    /// Consumes the Iterator collecting only the Upgrade events into a vector of UpgradeEventFlatRow
    pub fn collect_into_upgrades_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::UpgradeEventFlatRow> {
        self.into_iter()
            .filter_map(|(sc2_event, _change_hint)| match sc2_event {
                SC2EventType::Tracker {
                    tracker_loop,
                    event,
                } => {
                    if let tracker_events::ReplayTrackerEvent::Upgrade(event) = event {
                        Some(tracker_events::UpgradeEventFlatRow::new(
                            event,
                            tracker_loop,
                            details.clone(),
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }

    /// Consumes the Iterator collecting only the UnitBorn events into a vector of UnitBornEventFlatRow
    pub fn collect_into_unit_born_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::UnitBornEventFlatRow> {
        self.into_iter()
            .filter_map(|(sc2_event, change_hint)| match sc2_event {
                SC2EventType::Tracker {
                    tracker_loop,
                    event,
                } => match event {
                    tracker_events::ReplayTrackerEvent::UnitBorn(event) => {
                        Some(tracker_events::UnitBornEventFlatRow::from_unit_born(
                            event,
                            tracker_loop,
                            details,
                        ))
                    }
                    tracker_events::ReplayTrackerEvent::UnitDone(event) => {
                        Some(tracker_events::UnitBornEventFlatRow::from_unit_done(
                            event,
                            tracker_loop,
                            details,
                            change_hint,
                        ))
                    }
                    tracker_events::ReplayTrackerEvent::UnitTypeChange(event) => {
                        match change_hint {
                            UnitChangeHint::None => None,
                            change_hint => {
                                Some(tracker_events::UnitBornEventFlatRow::from_unit_type_change(
                                    event,
                                    tracker_loop,
                                    details,
                                    change_hint,
                                ))
                            }
                        }
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect()
    }

    /// Consumes the Iterator collecting only the UnitDied events into a vector of UnitBornEventFlatRow
    pub fn collect_into_unit_died_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::UnitDiedEventFlatRow> {
        self.into_iter()
            .filter_map(|(sc2_event, change_hint)| match sc2_event {
                SC2EventType::Tracker {
                    tracker_loop,
                    event,
                } => {
                    if let tracker_events::ReplayTrackerEvent::UnitDied(event) = event {
                        Some(tracker_events::UnitDiedEventFlatRow::new(
                            details,
                            event,
                            tracker_loop,
                            change_hint,
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }
}

impl Iterator for TrackerEventIterator {
    /// The item is a tuple of the SC2EventType with the accumulated (adjusted) game loop, and a
    /// hint of what has changed. An adjusted game loop is the `event_loop` adjusted to be in the same units as the game loops.
    type Item = (SC2EventType, UnitChangeHint);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator_state.transist_to_next_supported_event(
            self.protocol_version,
            &mut self.sc2_state,
            &self.filters,
        )
    }
}
