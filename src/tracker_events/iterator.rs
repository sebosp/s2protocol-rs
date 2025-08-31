//! Iterator for ReplayTrackerEEventId

use crate::error::S2ProtocolError;

use crate::tracker_events::TrackerEvent;
use crate::versions::protocol75689::byte_aligned::ReplayTrackerEEventId as Protocol75689ReplayTrackerEEventId;
use crate::versions::protocol87702::byte_aligned::ReplayTrackerEEventId as Protocol87702ReplayTrackerEEventId;
use crate::{SC2EventType, TRACKER_SPEED_RATIO};
use nom::*;
use serde::{Deserialize, Serialize};

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
    #[tracing::instrument(level = "debug", skip(self), fields(event_loop = self.event_loop))]
    pub fn get_next_event(&mut self, protocol_version: u32) -> Option<SC2EventType> {
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
                    let event = SC2EventType::Tracker {
                        tracker_loop: adjusted_loop,
                        event: val.event.clone(),
                    };
                    return Some(event);
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
}
