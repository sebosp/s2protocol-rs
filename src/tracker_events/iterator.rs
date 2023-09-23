//! Iterator for the ReplayTrackerEEventId

use crate::error::S2ProtocolError;
use crate::tracker_events::TrackerEvent;
use crate::versions::protocol75689::byte_aligned::ReplayTrackerEEventId as Protocol75689ReplayTrackerEEventId;
use crate::versions::protocol87702::byte_aligned::ReplayTrackerEEventId as Protocol87702ReplayTrackerEEventId;
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
            83830 | 84643 | 88500 | 86383 | 87702 | 89634 | 89720 | 90136 | 90779 | 90870 => {
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
}

impl Iterator for TrackerEventIterator {
    type Item = (TrackerEvent, i64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current_slice: &[u8] = &self.data[self.index..];
            if current_slice.input_len() == 0 {
                return None;
            }

            match self.read_versioned_tracker_event() {
                Ok(val) => return Some((val, self.tracker_loop)),
                Err(S2ProtocolError::UnsupportedEventType) => {}
                Err(err) => {
                    tracing::error!("Error reading tracker event: {:?}", err);
                    return None;
                }
            }
        }
    }
}
