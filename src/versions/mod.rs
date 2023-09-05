//! The known protocol versions

use crate::details::Details;
use crate::game_events::GameEvent;
use crate::message_events::MessageEvent;
use crate::tracker_events::TrackerEvent;
use crate::S2ProtocolError;
use nom_mpq::MPQ;

pub mod protocol75689;
pub mod protocol83830;
pub mod protocol84643;
pub mod protocol86383;
pub mod protocol87702;
pub mod protocol88500;
pub mod protocol89634;
pub mod protocol89720;
pub mod protocol90136;
pub mod protocol90779;
pub mod protocol90870;

/// Attempts to read the tracker events, panics under unknown protocol
pub fn read_tracker_events(
    mpq: &MPQ,
    file_contents: &[u8],
) -> Result<Vec<TrackerEvent>, S2ProtocolError> {
    let (_tail, proto_header) = crate::read_protocol_header(mpq)?;
    tracing::info!("Header: {:?}", proto_header);
    assert_eq!(proto_header.m_signature, b"StarCraft II replay\x1b11"[..]);
    match proto_header.m_version.m_base_build {
        75689 => protocol75689::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        83830 => protocol83830::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        84643 => protocol84643::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        88500 => protocol88500::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        86383 => protocol86383::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        87702 => protocol87702::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        89634 => protocol89634::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        89720 => protocol89720::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        90136 => protocol90136::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        90779 => protocol90779::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        90870 => protocol90870::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        _ => Err(S2ProtocolError::UnsupportedProtocolVersion(
            proto_header.m_version.m_base_build,
        )),
    }
}

/// Attempts to read the game events, panics under unknown protocol
pub fn read_game_events(
    mpq: &MPQ,
    file_contents: &[u8],
) -> Result<Vec<GameEvent>, S2ProtocolError> {
    let (_tail, proto_header) = crate::read_protocol_header(mpq)?;
    tracing::info!("Header: {:?}", proto_header);
    assert_eq!(proto_header.m_signature, b"StarCraft II replay\x1b11"[..]);
    match proto_header.m_version.m_base_build {
        75689 => protocol75689::bit_packed::GameEEventId::read_events(mpq, file_contents),
        83830 => protocol83830::bit_packed::GameEEventId::read_events(mpq, file_contents),
        84643 => protocol84643::bit_packed::GameEEventId::read_events(mpq, file_contents),
        88500 => protocol88500::bit_packed::GameEEventId::read_events(mpq, file_contents),
        86383 => protocol86383::bit_packed::GameEEventId::read_events(mpq, file_contents),
        87702 => protocol87702::bit_packed::GameEEventId::read_events(mpq, file_contents),
        89634 => protocol89634::bit_packed::GameEEventId::read_events(mpq, file_contents),
        89720 => protocol89720::bit_packed::GameEEventId::read_events(mpq, file_contents),
        90136 => protocol90136::bit_packed::GameEEventId::read_events(mpq, file_contents),
        90779 => protocol90779::bit_packed::GameEEventId::read_events(mpq, file_contents),
        90870 => protocol90870::bit_packed::GameEEventId::read_events(mpq, file_contents),
        _ => Err(S2ProtocolError::UnsupportedProtocolVersion(
            proto_header.m_version.m_base_build,
        )),
    }
}

/// Attempts to read the message events, panics under unknown protocol
pub fn read_message_events(
    mpq: &MPQ,
    file_contents: &[u8],
) -> Result<Vec<MessageEvent>, S2ProtocolError> {
    let (_tail, proto_header) = crate::read_protocol_header(mpq)?;
    tracing::info!("Header: {:?}", proto_header);
    assert_eq!(proto_header.m_signature, b"StarCraft II replay\x1b11"[..]);
    match proto_header.m_version.m_base_build {
        75689 => protocol75689::bit_packed::GameEMessageId::read_events(mpq, file_contents),
        83830 => protocol83830::bit_packed::GameEMessageId::read_events(mpq, file_contents),
        84643 => protocol84643::bit_packed::GameEMessageId::read_events(mpq, file_contents),
        88500 => protocol88500::bit_packed::GameEMessageId::read_events(mpq, file_contents),
        86383 => protocol86383::bit_packed::GameEMessageId::read_events(mpq, file_contents),
        87702 => protocol87702::bit_packed::GameEMessageId::read_events(mpq, file_contents),
        89634 => protocol89634::bit_packed::GameEMessageId::read_events(mpq, file_contents),
        89720 => protocol89720::bit_packed::GameEMessageId::read_events(mpq, file_contents),
        90136 => protocol90136::bit_packed::GameEMessageId::read_events(mpq, file_contents),
        90779 => protocol90779::bit_packed::GameEMessageId::read_events(mpq, file_contents),
        90870 => protocol90870::bit_packed::GameEMessageId::read_events(mpq, file_contents),
        _ => Err(S2ProtocolError::UnsupportedProtocolVersion(
            proto_header.m_version.m_base_build,
        )),
    }
}

/// Attempts to read the details, panics under unknown protocol
pub fn read_details(mpq: &MPQ, file_contents: &[u8]) -> Result<Details, S2ProtocolError> {
    let (_tail, proto_header) = crate::read_protocol_header(mpq)?;
    tracing::info!("Header: {:?}", proto_header);
    assert_eq!(proto_header.m_signature, b"StarCraft II replay\x1b11"[..]);
    match proto_header.m_version.m_base_build {
        75689 => protocol75689::byte_aligned::GameSDetails::read_details(mpq, file_contents),
        83830 => protocol83830::byte_aligned::GameSDetails::read_details(mpq, file_contents),
        84643 => protocol84643::byte_aligned::GameSDetails::read_details(mpq, file_contents),
        88500 => protocol88500::byte_aligned::GameSDetails::read_details(mpq, file_contents),
        86383 => protocol86383::byte_aligned::GameSDetails::read_details(mpq, file_contents),
        87702 => protocol87702::byte_aligned::GameSDetails::read_details(mpq, file_contents),
        89634 => protocol89634::byte_aligned::GameSDetails::read_details(mpq, file_contents),
        89720 => protocol89720::byte_aligned::GameSDetails::read_details(mpq, file_contents),
        90136 => protocol90136::byte_aligned::GameSDetails::read_details(mpq, file_contents),
        90779 => protocol90779::byte_aligned::GameSDetails::read_details(mpq, file_contents),
        90870 => protocol90870::byte_aligned::GameSDetails::read_details(mpq, file_contents),
        _ => Err(S2ProtocolError::UnsupportedProtocolVersion(
            proto_header.m_version.m_base_build,
        )),
    }
}
#[cfg(test)]
mod tests {
    use crate::versions::protocol89634::byte_aligned::ReplayTrackerEEventId;

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
            unreachable!("Expected type EPlayerSetup from first event");
        }
        let (_event_tail, (_delta, second_tracker_event)) =
            ReplayTrackerEEventId::parse_event_pair(tail).expect("Unable to parse TrackerEvents");
        if let ReplayTrackerEEventId::EPlayerSetup(setup_event) = second_tracker_event {
            assert_eq!(setup_event.m_player_id, 2);
            assert_eq!(setup_event.m_slot_id, Some(1));
            assert_eq!(setup_event.m_type, 1);
            assert_eq!(setup_event.m_user_id, Some(1));
        } else {
            unreachable!("Expected type EPlayerSetup from first event");
        }
    }
}
