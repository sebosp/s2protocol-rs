//! Iterator for the ReplayTrackerEEventId

use crate::error::S2ProtocolError;
use crate::tracker_events::{self, TrackerEvent};
use crate::versions::protocol75689::byte_aligned::ReplayTrackerEEventId as Protocol75689ReplayTrackerEEventId;
use crate::versions::protocol87702::byte_aligned::ReplayTrackerEEventId as Protocol87702ReplayTrackerEEventId;
use crate::TRACKER_SPEED_RATIO;
use nom::*;
use std::iter::Iterator;
use std::path::PathBuf;

/// Creates an iterator over the TrackerEvents
pub struct TrackerEventIterator {
    /// The protocol version
    pub protocol_version: u32,
    /// The MPQ file sector content
    pub data: Vec<u8>,
    /// The delta between loops
    pub tracker_loop: i64,
    /// The current index consumed by nom
    pub index: usize,
}

impl TrackerEventIterator {
    /// Creates a new TrackerEventIterator from a PathBuf
    pub fn new(source: &PathBuf) -> Result<Self, S2ProtocolError> {
        let file_contents = crate::read_file(source)?;
        let (_input, mpq) = crate::parser::parse(&file_contents)?;
        let (_tail, proto_header) = crate::read_protocol_header(&mpq)?;
        let (_event_tail, tracker_events) =
            mpq.read_mpq_file_sector("replay.tracker.events", false, &file_contents)?;
        Ok(Self {
            protocol_version: proto_header.m_version.m_base_build,
            data: tracker_events,
            tracker_loop: 0,
            index: 0,
        })
    }

    /// Based on the protocol version, returns the TrackerEvent pair.
    pub fn read_versioned_tracker_event(&mut self) -> Result<TrackerEvent, S2ProtocolError> {
        let (new_event_tail, delta, event) = match self.protocol_version {
            75689 => {
                let (new_event_tail, (delta, event)) =
                    Protocol75689ReplayTrackerEEventId::parse_event_pair(&self.data[self.index..])?;
                let event = match event.try_into() {
                    Ok(event) => event,
                    Err(err) => return Err(err),
                };
                (new_event_tail, delta, event)
            }
            83830 | 84643 | 88500 | 86383 | 87702 | 89165 | 89634 | 89720 | 90136 | 90779
            | 90870 => {
                let (new_event_tail, (delta, event)) =
                    Protocol87702ReplayTrackerEEventId::parse_event_pair(&self.data[self.index..])?;
                (new_event_tail, delta, event.try_into()?)
            }
            _ => {
                return Err(S2ProtocolError::UnsupportedProtocolVersion(
                    self.protocol_version,
                ))
            }
        };
        self.index += new_event_tail.as_ptr() as usize - self.data[self.index..].as_ptr() as usize;
        self.tracker_loop += delta as i64;
        Ok(TrackerEvent { delta, event })
    }

    /// Consumes the Iterator collecting only the PlayerStats events into a vector of PlayerStatsFlatRow
    pub fn collect_into_player_stats_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::PlayerStatsFlatRow> {
        self.into_iter()
            .filter_map(|(game_step, game_loop)| {
                if let tracker_events::ReplayTrackerEvent::PlayerStats(event) = game_step.event {
                    Some(tracker_events::PlayerStatsFlatRow::new(
                        event,
                        game_loop,
                        details.clone(),
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Consumes the Iterator collecting only the Upgrade events into a vector of UpgradeEventFlatRow
    pub fn collect_into_upgrades_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::UpgradeEventFlatRow> {
        self.into_iter()
            .filter_map(|(game_step, game_loop)| {
                if let tracker_events::ReplayTrackerEvent::Upgrade(event) = game_step.event {
                    Some(tracker_events::UpgradeEventFlatRow::new(
                        event,
                        game_loop,
                        details.clone(),
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Consumes the Iterator collecting only the UnitBorn events into a vector of UnitBornEventFlatRow
    pub fn collect_into_unit_born_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::UnitBornEventFlatRow> {
        self.into_iter()
            .filter_map(|(game_step, game_loop)| {
                if let tracker_events::ReplayTrackerEvent::UnitBorn(event) = game_step.event {
                    Some(tracker_events::UnitBornEventFlatRow::new(
                        event,
                        game_loop,
                        details.clone(),
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Consumes the Iterator collecting only the UnitDied events into a vector of UnitBornEventFlatRow
    pub fn collect_into_unit_died_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::UnitDiedEventFlatRow> {
        self.into_iter()
            .filter_map(|(game_step, game_loop)| {
                if let tracker_events::ReplayTrackerEvent::UnitDied(event) = game_step.event {
                    Some(tracker_events::UnitDiedEventFlatRow::new(
                        event,
                        game_loop,
                        details.clone(),
                    ))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Iterator for TrackerEventIterator {
    /// The item is a tuple of the TrackerEvent and the adjusted game loop
    /// An adjusted game loop is the `tracker_loop` adjusted to be in the same units as the game loops.
    type Item = (TrackerEvent, i64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current_slice: &[u8] = &self.data[self.index..];
            if current_slice.input_len() == 0 {
                return None;
            }

            // After the event is collected, the loop is adjusted to be in the same units as the
            // game loops.
            match self.read_versioned_tracker_event() {
                Ok(val) => {
                    return Some((val, (self.tracker_loop as f32 / TRACKER_SPEED_RATIO) as i64))
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
