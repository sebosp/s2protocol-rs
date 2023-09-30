//! Iterator for the ReplayTrackerEEventId

use crate::error::S2ProtocolError;
use crate::tracker_events::{self, TrackerEvent};
use crate::versions::protocol75689::byte_aligned::ReplayTrackerEEventId as Protocol75689ReplayTrackerEEventId;
use crate::versions::protocol87702::byte_aligned::ReplayTrackerEEventId as Protocol87702ReplayTrackerEEventId;
use crate::{SC2EventType, SC2ReplayState, UnitChangeHint, TRACKER_SPEED_RATIO};
use nom::*;
use std::iter::Iterator;
use std::path::PathBuf;

// TODO:
//  - Details does not need to be cloned now that we only use the sha256.

use super::handle_tracker_event;

/// Creates an iterator over the TrackerEvents
pub struct TrackerEventIterator {
    /// The protocol version
    pub protocol_version: u32,
    /// The replay state machine transducer
    pub sc2_state: SC2ReplayState,
    /// The MPQ replay.tracker.events sector bytes
    pub event_data: Vec<u8>,
    /// The accumulated tracker event delta
    pub event_loop: i64,
    /// The current byte index consumed by nom
    pub byte_index: usize,
}

impl TrackerEventIterator {
    /// Creates a new TrackerEventIterator from a PathBuf
    pub fn new(source: &PathBuf) -> Result<Self, S2ProtocolError> {
        let file_contents = crate::read_file(source)?;
        let (_input, mpq) = crate::parser::parse(&file_contents)?;
        let (_tail, proto_header) = crate::read_protocol_header(&mpq)?;
        let (_event_tail, tracker_events) =
            mpq.read_mpq_file_sector("replay.tracker.events", false, &file_contents)?;
        let sc2_state: SC2ReplayState = Default::default();
        Ok(Self {
            protocol_version: proto_header.m_version.m_base_build,
            sc2_state,
            event_data: tracker_events,
            event_loop: 0,
            byte_index: 0,
        })
    }

    /// Based on the protocol version, returns the TrackerEvent pair.
    pub fn read_versioned_tracker_event(&mut self) -> Result<TrackerEvent, S2ProtocolError> {
        let (new_event_tail, delta, event) = match self.protocol_version {
            75689 => {
                let (new_event_tail, (delta, event)) =
                    Protocol75689ReplayTrackerEEventId::parse_event_pair(
                        &self.event_data[self.byte_index..],
                    )?;
                let event = match event.try_into() {
                    Ok(event) => event,
                    Err(err) => return Err(err),
                };
                (new_event_tail, delta, event)
            }
            83830 | 84643 | 88500 | 86383 | 87702 | 89165 | 89634 | 89720 | 90136 | 90779
            | 90870 => {
                // The protocol is known to be compatible with these versions
                let (new_event_tail, (delta, event)) =
                    Protocol87702ReplayTrackerEEventId::parse_event_pair(
                        &self.event_data[self.byte_index..],
                    )?;
                (new_event_tail, delta, event.try_into()?)
            }
            _ => {
                tracing::warn!(
                    "Unknown protocol version: {}, will attempt to use 87702",
                    self.protocol_version
                );
                // The protocol is known to be compatible with these versions
                let (new_event_tail, (delta, event)) =
                    Protocol87702ReplayTrackerEEventId::parse_event_pair(
                        &self.event_data[self.byte_index..],
                    )?;
                (new_event_tail, delta, event.try_into()?)
            }
        };
        self.byte_index +=
            new_event_tail.as_ptr() as usize - self.event_data[self.byte_index..].as_ptr() as usize;
        self.event_loop += delta as i64;
        Ok(TrackerEvent { delta, event })
    }

    /// Consumes the Iterator collecting only the PlayerStats events into a vector of PlayerStatsFlatRow
    pub fn collect_into_player_stats_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::PlayerStatsFlatRow> {
        self.into_iter()
            .filter_map(|(sc2_event, updated_units)| match sc2_event {
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

    /// Consumes the Iterator collecting only the Upgrade events into a vector of UpgradeEventFlatRow
    pub fn collect_into_upgrades_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::UpgradeEventFlatRow> {
        self.into_iter()
            .filter_map(|(sc2_event, updated_units)| match sc2_event {
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
            .filter_map(|(sc2_event, updated_units)| match sc2_event {
                SC2EventType::Tracker {
                    tracker_loop,
                    event,
                } => {
                    if let tracker_events::ReplayTrackerEvent::UnitBorn(event) = event {
                        Some(tracker_events::UnitBornEventFlatRow::new(
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
                            &details,
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
    /// The item is a tuple of the SC2EventType with the accumulated (adjusted) game loop, and the updated
    /// units. An adjusted game loop is the `event_loop` adjusted to be in the same units as the game loops.
    /// For now this only works with the Tracker events. When we start adding GameType events, we will need
    /// to find a way to peak into the next events and keep track of the events in both Game and
    /// Tracker sectors.
    type Item = (SC2EventType, UnitChangeHint);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current_slice: &[u8] = &self.event_data[self.byte_index..];
            if current_slice.input_len() == 0 {
                return None;
            }

            // After the event is collected, the loop is adjusted to be in the same units as the
            // game loops.
            match self.read_versioned_tracker_event() {
                Ok(val) => {
                    let adjusted_loop = (self.event_loop as f32 / TRACKER_SPEED_RATIO) as i64;
                    let updated_hint =
                        handle_tracker_event(&mut self.sc2_state, adjusted_loop, &val.event);
                    return Some((
                        SC2EventType::Tracker {
                            tracker_loop: adjusted_loop,
                            event: val.event,
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
