//! Error handling of S2Protocol

use std::num::TryFromIntError;

use nom::error::ErrorKind;
use nom::error::ParseError;

/// Holds the result of parsing progress and the possibly failures
pub type S2ProtoResult<I, O> = Result<(I, O), S2ProtocolError>;

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
    UnknownTag(i64),
    #[error("Duplicate field {0} with tag {1}")]
    DuplicateTag(String, i64),
    #[error("Missing field {0}")]
    MissingField(String),
    #[error("TryFromIntError")]
    ValueError(#[from] TryFromIntError),
}

/// Conversion of errors from byte aligned parser
impl<I> From<nom::Err<nom::error::Error<I>>> for S2ProtocolError
where
    I: Clone + std::fmt::Debug,
{
    fn from(err: nom::Err<nom::error::Error<I>>) -> Self {
        match err {
            nom::Err::Incomplete(_) => {
                unreachable!("This library is compatible with only complete parsers, not streaming")
            }
            nom::Err::Error(e) => S2ProtocolError::ByteAligned(format!("{:?}", e)),
            nom::Err::Failure(e) => S2ProtocolError::ByteAligned(format!("{:?}", e)),
        }
    }
}

impl<I> ParseError<I> for S2ProtocolError
where
    I: Clone,
{
    fn from_error_kind(_input: I, kind: ErrorKind) -> Self {
        S2ProtocolError::ByteAligned(format!("{:?}", kind))
    }

    fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}
