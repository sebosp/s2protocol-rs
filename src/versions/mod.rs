//! The known protocol versions

use nom_mpq::MPQ;

use crate::tracker_events::TrackerEvent;

pub mod protocol87702;
pub mod protocol89634;

/// Attempts to read the tracker events, panics under unknown protocol
pub fn read_tracker_events(mpq: &MPQ, file_contents: &[u8]) -> Vec<TrackerEvent> {
    let (_tail, proto_header) = crate::read_protocol_header(&mpq).unwrap();
    assert_eq!(proto_header.m_signature, b"StarCraft II replay\x1b11"[..]);
    match proto_header.m_version.m_base_build {
        87702 => protocol87702::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        89634 => protocol89634::ReplayTrackerEEventId::read_tracker_events(mpq, file_contents),
        _ => panic!(
            "Unsupported Protocol Version: {}",
            proto_header.m_version.m_base_build
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::protocol87702::byte_aligned::*;

    #[test_log::test]
    fn it_reads_tracker_events() {
        let data: Vec<u8> = vec![
            0x03, 0x00, 0x09, 0x00, 0x09, 0x12, 0x05, 0x08, 0x00, 0x09, 0x02, 0x02, 0x09, 0x02,
            0x04, 0x04, 0x01, 0x09, 0x00, 0x06, 0x04, 0x01, 0x09, 0x00, 0x03, 0x00, 0x09, 0x00,
            0x09, 0x12, 0x05, 0x08, 0x00, 0x09, 0x04, 0x02, 0x09, 0x02, 0x04, 0x04, 0x01, 0x09,
            0x02, 0x06, 0x04, 0x01, 0x09, 0x02, 0x03, 0x00, 0x09, 0x00, 0x09, 0x12, 0x05, 0x08,
            0x00, 0x09, 0x06, 0x02, 0x09, 0x04, 0x04, 0x04, 0x00, 0x06, 0x04, 0x01, 0x09, 0x04,
        ];
        let (tail, (_delta, first_tracker_event)) =
            ReplayTrackerEEventId::parse_event_pair(&data).unwrap();
        if let ReplayTrackerEEventId::EPlayerSetup(setup_event) = first_tracker_event {
            assert_eq!(setup_event.m_player_id, 1);
            assert_eq!(setup_event.m_slot_id, Some(0));
            assert_eq!(setup_event.m_type, 1);
            assert_eq!(setup_event.m_user_id, Some(0));
        } else {
            assert!(false, "Expected type EPlayerSetup from first event");
        }
        let (_event_tail, (_delta, second_tracker_event)) =
            ReplayTrackerEEventId::parse_event_pair(&tail).expect("Unable to parse TrackerEvents");
        if let ReplayTrackerEEventId::EPlayerSetup(setup_event) = second_tracker_event {
            assert_eq!(setup_event.m_player_id, 2);
            assert_eq!(setup_event.m_slot_id, Some(1));
            assert_eq!(setup_event.m_type, 1);
            assert_eq!(setup_event.m_user_id, Some(1));
        } else {
            assert!(false, "Expected type EPlayerSetup from first event");
        }
    }
}
