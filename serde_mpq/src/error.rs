//! The errors of the two encodings.

use std::num::TryFromIntError;

/// The characters of the input shown in the errors.
pub const MAX_ERROR_CONTEXT_CHARS: usize = 128;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// A byte aligned value could not be read, the message starts with the bytes at the error.
    #[error("Nom ByteAligned {0}")]
    ByteAligned(String),
    /// A bit packed value could not be read, the message starts with the bits at the error.
    #[error("Nom BitPacked {0}")]
    BitPacked(String),
    /// The tag of a field or of a variant is not one of the type.
    #[error("Unexpected Tag: {0}")]
    UnknownTag(i64),
    /// A tagged field was found twice in a struct.
    #[error("Duplicate field {0} with tag {1}")]
    DuplicateTag(String, i64),
    /// A required field was not found in a tagged struct.
    #[error("Missing field {0}")]
    MissingField(String),
    /// An integer of the input does not fit the type of the field.
    #[error("TryFromInt")]
    ValueError(#[from] TryFromIntError),
    /// A bit packed integer of more than 64 bits.
    #[error("BitPackedTooLarge: {0}")]
    BitPackedMoreThan64Bits(usize),
    /// A value does not fit the bits of its field when it is written.
    #[error("The value {value} does not fit in {bits} bits (offset {offset})")]
    ValueOutOfRange {
        value: i64,
        offset: i64,
        bits: usize,
    },
    /// A blob or a bit array has another length than the one of its field when it is written.
    #[error("The length {len} of {field} is not {expected}")]
    LengthMismatch {
        field: &'static str,
        len: usize,
        expected: usize,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

/// `i64::try_from(i64)` cannot fail, the derive converts every integer the same way.
impl From<std::convert::Infallible> for Error {
    fn from(never: std::convert::Infallible) -> Self {
        match never {}
    }
}

/// Conversion of the errors of the nom parsers, the input at the error is kept.
impl<I> From<nom::Err<nom::error::Error<I>>> for Error
where
    I: Clone + std::fmt::Debug,
{
    fn from(err: nom::Err<nom::error::Error<I>>) -> Self {
        match err {
            nom::Err::Incomplete(_) => {
                unreachable!("The parsers are complete, not streaming")
            }
            nom::Err::Error(e) | nom::Err::Failure(e) => Error::ByteAligned(format!(
                "{1:.0$}: {2}",
                MAX_ERROR_CONTEXT_CHARS,
                format!("{:?}", e.input),
                e.code.description()
            )),
        }
    }
}
