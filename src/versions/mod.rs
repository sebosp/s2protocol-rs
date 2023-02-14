//! The known protocol versions

use nom_mpq::MPQ;

pub mod protocol87702;
pub mod protocol89634;

#[derive(Debug)]
pub enum VersionedReplayTracker {
    V87702(
        Vec<(
            protocol87702::ReplayTrackerEEventId,
            protocol87702::SVarUint32,
        )>,
    ),
    V89634(
        Vec<(
            protocol89634::ReplayTrackerEEventId,
            protocol89634::SVarUint32,
        )>,
    ),
}

impl VersionedReplayTracker {
    pub fn read_tracker_events(mpq: &MPQ, file_contents: &[u8]) -> Self {
        let (_tail, proto_header) = crate::read_protocol_header(&mpq).unwrap();
        assert_eq!(proto_header.m_signature, b"StarCraft II replay\x1b11"[..]);
        match proto_header.m_version.m_base_build {
            87702 => Self::V87702(vec![]),
            _ => unimplemented!(),
        }
    }
}
