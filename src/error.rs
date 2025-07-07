//! Error handling of S2Protocol

use std::num::TryFromIntError;

use nom::error::ErrorKind;
use nom::error::ParseError;

/// Holds the result of parsing progress and the possibly failures
pub type S2ProtoResult<I, O> = Result<(I, O), S2ProtocolError>;

#[derive(thiserror::Error, Debug)]
pub enum S2ProtocolError {
    /// Unable to parse the MPQ file, could be corrupted or not a replay file
    #[error("MPQ Error")]
    MPQ(#[from] nom_mpq::MPQParserError),
    /// The protocol version is not yet supported.
    #[error("Unsupported Protocol Version: {0}")]
    UnsupportedProtocolVersion(u32),
    /// Unable to parse the byte aligned data types
    #[error("Nom ByteAligned Error {0}")]
    ByteAligned(String),
    /// Unable to parse the bit packed data types
    #[error("Nom BitPacked Error {0}")]
    BitPacked(String),
    /// The data structure tag is not recognized
    #[error("Unexpected Tag: {0}")]
    UnknownTag(i64),
    /// The data structure tag was already parsed
    #[error("Duplicate field {0} with tag {1}")]
    DuplicateTag(String, i64),
    /// A required field was not found
    #[error("Missing field {0}")]
    MissingField(String),
    /// Unable to parse a value that should have been an integer
    #[error("TryFromIntError")]
    ValueError(#[from] TryFromIntError),
    /// An I/O Error
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
    /// The path provided was not a file
    #[error("Expected a file, but got a directory")]
    PathNotADir,
    /// An error to be used in TryFrom, when converting from protocol-specific types into our
    /// consolidated-types
    #[error("Unsupported Event Type")]
    UnsupportedEventType,
    /// Conversion to UTF-8 failed, from the `Vec<u8>` "name" fields in the proto fields
    #[error("Utf8 conversion error")]
    Utf8Error(#[from] std::str::Utf8Error),
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
            nom::Err::Error(e) => S2ProtocolError::ByteAligned(format!("{e:?}")),
            nom::Err::Failure(e) => S2ProtocolError::ByteAligned(format!("{e:?}")),
        }
    }
}

impl<I> ParseError<I> for S2ProtocolError
where
    I: Clone,
{
    fn from_error_kind(_input: I, kind: ErrorKind) -> Self {
        S2ProtocolError::ByteAligned(format!("{kind:?}"))
    }

    fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}
