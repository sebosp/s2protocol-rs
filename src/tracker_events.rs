//! Decodes the Tracker Events.
//! These are stored in a file in the MPQ file called 'replay.tracker.events'

use nom::*;
use nom_mpq::parser::peek_hex;
use nom_mpq::MPQ;

/// A list of errors when handling TrackerEvents
#[derive(Debug, thiserror::Error)]
pub enum TrackerEventError {
    /// An error to be used in TryFrom, when converting from protocol-specific types into our
    /// consolidated-types
    #[error("Unsupported Event Type")]
    UnsupportedEventType,
}

/// A protocol agnostic Unit Born
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitBornEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub unit_type_name: String,
    pub control_player_id: u8,
    pub upkeep_player_id: u8,
    pub x: u8,
    pub y: u8,
    pub creator_unit_tag_index: Option<u32>,
    pub creator_unit_tag_recycle: Option<u32>,
    pub creator_ability_name: Option<String>,
}

/// A protocol agnostic Unit Died
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitDiedEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub killer_player_id: Option<u8>,
    pub x: u8,
    pub y: u8,
    pub killer_unit_tag_index: Option<u32>,
    pub killer_unit_tag_recycle: Option<u32>,
}

/// A protocol agnostic Unit Init Event
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitInitEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub unit_type_name: String,
    pub control_player_id: u8,
    pub upkeep_player_id: u8,
    pub x: u8,
    pub y: u8,
}

/// A protocol agnostic Unit Done Event
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitDoneEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
}

/// A protocol agnostic Unit Done Event
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitPositionsEvent {
    pub first_unit_index: u32,
    pub items: Vec<i32>,
}

/// A unified Replay Tracker that is agnostic of any version.
/// This should hopefully only add fields to variants to make things backwards compatible
/// Many of the variants are not supported yet, they will be added as they are considered
/// relevant  for `swarmy` repo.
#[derive(Debug, PartialEq, Clone)]
pub enum ReplayTrackerEvent {
    UnitBorn(UnitBornEvent),
    UnitDied(UnitDiedEvent),
    UnitTypeChange(UnitTypeChangeEvent),
    UnitInit(UnitInitEvent),
    UnitDone(UnitDoneEvent),
    UnitPosition(UnitPositionsEvent),
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitTypeChangeEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub unit_type_name: String,
}

/// A Tracker Event is an event in the gameloop for a specific user id
#[derive(Debug, PartialEq, Clone)]
pub struct TrackerEvent {
    pub delta: SVarUint32,
    pub event: ReplayTrackerEEventId,
}

impl TrackerEvent {
    #[tracing::instrument(name="TrackerEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, delta) = SVarUint32::parse(input)?;
        let (tail, event) = ReplayTrackerEEventId::parse(tail)?;
        Ok((tail, Self { delta, event }))
    }
}

/// Read the Tracker Events
pub fn read_tracker_events(mpq: &MPQ, file_contents: &[u8]) -> Vec<TrackerEvent> {
    let (_event_tail, tracker_events) = mpq
        .read_mpq_file_sector("replay.tracker.events", false, &file_contents)
        .unwrap();
    let mut res = vec![];
    let mut event_tail: &[u8] = &tracker_events;
    let mut game_loop: u32 = 0;
    loop {
        let (new_event_tail, mut tracker_event) =
            TrackerEvent::parse(&event_tail).expect("Unable to parse TrackerEvents");
        game_loop += match tracker_event.delta {
            SVarUint32::Uint6(val) => val as u32,
            SVarUint32::Uint14(val) | SVarUint32::Uint22(val) | SVarUint32::Uint32(val) => val,
        };
        event_tail = new_event_tail;
        res.push(tracker_event);
        if event_tail.input_len() == 0 {
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn it_reads_tracker_events() {
        let data: Vec<u8> = vec![
            0x03, 0x00, 0x09, 0x00, 0x09, 0x12, 0x05, 0x08, 0x00, 0x09, 0x02, 0x02, 0x09, 0x02,
            0x04, 0x04, 0x01, 0x09, 0x00, 0x06, 0x04, 0x01, 0x09, 0x00, 0x03, 0x00, 0x09, 0x00,
            0x09, 0x12, 0x05, 0x08, 0x00, 0x09, 0x04, 0x02, 0x09, 0x02, 0x04, 0x04, 0x01, 0x09,
            0x02, 0x06, 0x04, 0x01, 0x09, 0x02, 0x03, 0x00, 0x09, 0x00, 0x09, 0x12, 0x05, 0x08,
            0x00, 0x09, 0x06, 0x02, 0x09, 0x04, 0x04, 0x04, 0x00, 0x06, 0x04, 0x01, 0x09, 0x04,
        ];
        let (tail, first_tracker_event) = TrackerEvent::parse(&data).unwrap();
        if let ReplayTrackerEEventId::EPlayerSetup(setup_event) = first_tracker_event.event {
            assert_eq!(setup_event.m_player_id, 1);
            assert_eq!(setup_event.m_slot_id, Some(0));
            assert_eq!(setup_event.m_type, 1);
            assert_eq!(setup_event.m_user_id, Some(0));
        } else {
            assert!(false, "Expected type EPlayerSetup from first event");
        }
        let (_event_tail, second_tracker_event) =
            TrackerEvent::parse(&tail).expect("Unable to parse TrackerEvents");
        if let ReplayTrackerEEventId::EPlayerSetup(setup_event) = second_tracker_event.event {
            assert_eq!(setup_event.m_player_id, 2);
            assert_eq!(setup_event.m_slot_id, Some(1));
            assert_eq!(setup_event.m_type, 1);
            assert_eq!(setup_event.m_user_id, Some(1));
        } else {
            assert!(false, "Expected type EPlayerSetup from first event");
        }
    }
}
