//! Error handling of S2Protocol

use std::num::TryFromIntError;

use nom::error::ErrorKind;
use nom::error::ParseError;

#[derive(thiserror::Error, Debug)]
pub enum S2ProtocolError {
    #[error("MPQ Error")]
    MPQ(#[from] nom_mpq::MPQParserError),
    #[error("Unsupported Protocol Version: {0}")]
    UnsupportedProtocolVersion(u32),
    #[error("Nom ByteAligned Error {0}")]
    ByteAligned(String),
    #[error("Nom BitPacked Error {0}")]
    BitPacked(String),
    #[error("Details parsing error")]
    Details(#[from] crate::details::DetailsError),
    #[error("TrackerEvent parsing error")]
    TrackerEvent(#[from] crate::tracker_events::TrackerEventError),
    #[error("GameEvent parsing error")]
    GameEvent(#[from] crate::game_events::GameEventError),
    #[error("MessageEvent parsing error")]
    MessageEvent(#[from] crate::message_events::MessageEventError),
    #[error("Unexpected Tag: {0}")]
    UnknownTagError(i64),
    #[error("Duplicate field {0} with tag {1}")]
    DuplicateTagError(String, i64),
    #[error("Missing field {0}")]
    MissingField(String),
    #[error("TryFromIntError")]
    ValueError(#[from] TryFromIntError),
}

impl From<nom::Err<nom::error::Error<&[u8]>>> for S2ProtocolError {
    fn from(err: nom::Err<nom::error::Error<&[u8]>>) -> Self {
        match err {
            nom::Err::Incomplete(_) => S2ProtocolError::ByteAligned(String::from("Incomplete")),
            nom::Err::Error(e) => S2ProtocolError::ByteAligned(format!("{:?}", e)),
            nom::Err::Failure(e) => S2ProtocolError::ByteAligned(format!("{:?}", e)),
        }
    }
}

impl From<nom::Err<nom::error::Error<(&[u8], usize)>>> for S2ProtocolError {
    fn from(err: nom::Err<nom::error::Error<(&[u8], usize)>>) -> Self {
        match err {
            nom::Err::Incomplete(_) => S2ProtocolError::BitPacked(String::from("Incomplete")),
            nom::Err::Error(e) => S2ProtocolError::BitPacked(format!("{:?}", e)),
            nom::Err::Failure(e) => S2ProtocolError::BitPacked(format!("{:?}", e)),
        }
    }
}
impl ParseError<&[u8]> for S2ProtocolError {
    fn from_error_kind(input: &[u8], kind: ErrorKind) -> Self {
        S2ProtocolError::ByteAligned(format!("{:?}: {}", kind, nom_mpq::parser::peek_hex(input)))
    }

    fn append(_input: &[u8], _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

impl ParseError<(&[u8], usize)> for S2ProtocolError {
    fn from_error_kind(input: (&[u8], usize), kind: ErrorKind) -> Self {
        S2ProtocolError::BitPacked(format!("{:?}: {}", kind, super::peek_bits(input)))
    }

    fn append(_input: (&[u8], usize), _kind: ErrorKind, other: Self) -> Self {
        other
    }
}
