//! The known protocol versions

use crate::details::Details;
use crate::game_events::GameEvent;
use crate::message_events::MessageEvent;
use crate::tracker_events::TrackerEvent;
use crate::{InitData, S2ProtocolError};
use nom_mpq::MPQ;

pub mod protocol75689;
pub mod protocol87702;

/// Attempts to read the tracker events, panics under unknown protocol
#[tracing::instrument(level = "debug", skip(mpq, file_contents))]
pub fn read_tracker_events(
    file_name: &str,
    mpq: &MPQ,
    file_contents: &[u8],
) -> Result<Vec<TrackerEvent>, S2ProtocolError> {
    let (_tail, proto_header) = crate::read_protocol_header(mpq)?;
    tracing::debug!(
        "Proto version: {:?} reading TrackerEvents from {:?}",
        proto_header.m_version.m_base_build,
        file_name
    );
    assert_eq!(proto_header.m_signature, b"StarCraft II replay\x1b11"[..]);
    match proto_header.m_version.m_base_build {
        0..=75689 => protocol75689::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
            mpq,
            file_contents,
        ),
        83830 | 84643 | 88500 | 86383 | 87702 | 89634 | 89165 | 89720 | 90136 | 90779 | 90870 => {
            protocol87702::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
                mpq,
                file_contents,
            )
        }
        _ => {
            tracing::warn!(
                "Protocol version {:?} is not supported, falling back to 87702",
                proto_header.m_version.m_base_build
            );
            protocol87702::byte_aligned::ReplayTrackerEEventId::read_tracker_events(
                mpq,
                file_contents,
            )
        }
    }
}

/// Attempts to read the game events, panics under unknown protocol
#[tracing::instrument(level = "debug", skip(mpq, file_contents))]
pub fn read_game_events(
    file_name: &str,
    mpq: &MPQ,
    file_contents: &[u8],
) -> Result<Vec<GameEvent>, S2ProtocolError> {
    let (_tail, proto_header) = crate::read_protocol_header(mpq)?;
    tracing::debug!(
        "Proto version: {:?} reading GameEvents from {:?}",
        proto_header.m_version.m_base_build,
        file_name
    );
    assert_eq!(proto_header.m_signature, b"StarCraft II replay\x1b11"[..]);
    match proto_header.m_version.m_base_build {
        0..=75689 => protocol75689::bit_packed::GameEEventId::read_events(mpq, file_contents),
        83830 | 84643 | 88500 | 86383 | 87702 | 89634 | 89165 | 89720 | 90136 | 90779 | 90870 => {
            protocol87702::bit_packed::GameEEventId::read_events(mpq, file_contents)
        }
        _ => {
            tracing::warn!(
                "Protocol version {:?} is not supported, falling back to 87702",
                proto_header.m_version.m_base_build
            );
            protocol87702::bit_packed::GameEEventId::read_events(mpq, file_contents)
        }
    }
}

/// Attempts to read the message events, panics under unknown protocol
#[tracing::instrument(level = "debug", skip(mpq, file_contents))]
pub fn read_message_events(
    file_name: &str,
    mpq: &MPQ,
    file_contents: &[u8],
) -> Result<Vec<MessageEvent>, S2ProtocolError> {
    let (_tail, proto_header) = crate::read_protocol_header(mpq)?;
    tracing::debug!(
        "Proto version: {:?} reading MessageEvents from {:?}",
        proto_header.m_version.m_base_build,
        file_name
    );
    assert_eq!(proto_header.m_signature, b"StarCraft II replay\x1b11"[..]);
    match proto_header.m_version.m_base_build {
        0..=75689 => protocol75689::bit_packed::GameEMessageId::read_events(mpq, file_contents),
        83830 | 84643 | 88500 | 86383 | 87702 | 89634 | 89165 | 89720 | 90136 | 90779 | 90870 => {
            protocol87702::bit_packed::GameEMessageId::read_events(mpq, file_contents)
        }
        _ => {
            tracing::warn!(
                "Protocol version {:?} is not supported, falling back to 87702",
                proto_header.m_version.m_base_build
            );
            protocol87702::bit_packed::GameEMessageId::read_events(mpq, file_contents)
        }
    }
}

/// Attempts to read the details, panics under unknown protocol
#[tracing::instrument(level = "debug", skip(mpq, file_contents))]
pub fn read_details(
    file_name: &str,
    mpq: &MPQ,
    file_contents: &[u8],
) -> Result<Details, S2ProtocolError> {
    let (_tail, proto_header) = crate::read_protocol_header(mpq)?;
    tracing::debug!(
        "Proto version: {:?} reading Details from {:?}",
        proto_header.m_version.m_base_build,
        file_name
    );
    assert_eq!(proto_header.m_signature, b"StarCraft II replay\x1b11"[..]);
    match proto_header.m_version.m_base_build {
        0..=75689 => protocol75689::byte_aligned::GameSDetails::read_details(mpq, file_contents),
        83830 | 84643 | 88500 | 86383 | 87702 | 89634 | 89165 | 89720 | 90136 | 90779 | 90870 => {
            protocol87702::byte_aligned::GameSDetails::read_details(mpq, file_contents)
        }
        _ => {
            tracing::warn!(
                "Protocol version {:?} is not supported, falling back to 87702",
                proto_header.m_version.m_base_build
            );
            protocol87702::byte_aligned::GameSDetails::read_details(mpq, file_contents)
        }
    }
}

/// Attempts to read the initData, panics under unknown protocol
#[tracing::instrument(level = "error", skip(mpq, file_contents, file_name))]
pub fn read_init_data(
    file_name: &str,
    mpq: &MPQ,
    file_contents: &[u8],
) -> Result<InitData, S2ProtocolError> {
    let (_tail, proto_header) = crate::read_protocol_header(mpq)?;
    tracing::info!(
        "Proto version: {:?} reading InitData from {:?}",
        proto_header.m_version.m_base_build,
        file_name
    );
    assert_eq!(proto_header.m_signature, b"StarCraft II replay\x1b11"[..]);
    let res = match proto_header.m_version.m_base_build {
        0..=75689 => protocol75689::bit_packed::ReplaySInitData::read_init_data(mpq, file_contents),
        83830 | 84643 | 88500 | 86383 | 87702 | 89634 | 89165 | 89720 | 90136 | 90779 | 90870 => {
            protocol87702::bit_packed::ReplaySInitData::read_init_data(mpq, file_contents)
        }
        _ => {
            tracing::warn!(
                "Protocol version {:?} is not supported, falling back to 87702",
                proto_header.m_version.m_base_build
            );
            protocol87702::bit_packed::ReplaySInitData::read_init_data(mpq, file_contents)
        }
    };
    match res {
        Ok(mut res) => {
            res.set_version(proto_header.m_version.m_base_build);
            Ok(res)
        }
        Err(e) => {
            tracing::error!("Error reading InitData: {:?}", e);
            Err(e)
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::versions::protocol87702::byte_aligned::ReplayTrackerEEventId;

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
