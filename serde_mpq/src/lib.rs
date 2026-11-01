//! The two encodings of the sections of a StarCraft II replay, and the derive macros that read
//! and write Rust types in them:
//!
//! - **bit packed**, the game events, message events and init data: no tags, the values follow
//!   each other bit by bit, the width of every integer is a constant of the protocol. See
//!   [`bit_packed`].
//! - **byte aligned** (Blizzard calls it versioned), the header, details and tracker events: every
//!   value is preceded by the tag of its kind, the fields of a struct by their number, the integers
//!   are variable length. See [`byte_aligned`].
//!
//! # The declaration order is the wire format
//!
//! In the bit packed encoding a struct is read field by field in the order they are declared,
//! like `deku`, `binrw`, `bincode` or `postcard` do: the struct *is* the description of the
//! format. Reordering, adding or removing a field changes what the derive reads and writes. There
//! is no schema hash or field position to maintain, on purpose: a value written by a previous
//! version of a type is migrated by reading it with the previous version and writing it with the
//! new one. The byte aligned encoding tags every field with `#[mpq(tag = n)]`, so there the
//! declaration order does not matter.
//!
//! # Example
//!
//! ```ignore
//! use serde_mpq::{BitPackedDeserialize, MPQBitPackedDeserialize, MPQBitPackedSerialize};
//!
//! #[derive(Debug, PartialEq, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
//! struct GameSPoint {
//!     #[mpq(bits = 20, offset = -2097152)]
//!     x: i64,
//!     #[mpq(bits = 20, offset = -2097152)]
//!     y: i64,
//! }
//! ```
//!
//! The derives also generate a `parse_<field>` method per field and a `parse` method, with
//! `tracing` spans at the `trace` level that show the bits being read, so that an unexpected
//! value in a replay can be followed field by field.

pub mod bit_array;
pub mod bit_packed;
pub mod byte_aligned;
pub mod error;
pub mod peek;

pub use bit_array::BitArray;
pub use bit_packed::*;
pub use byte_aligned::*;
pub use error::{Error, Result};
pub use peek::*;
#[cfg(feature = "derive")]
pub use serde_mpq_derive::{
    MPQBitPackedDeserialize, MPQBitPackedSerialize, MPQByteAlignedDeserialize,
    MPQByteAlignedSerialize,
};

/// Pre-allocating memory is a nice optimization but count fields can't always be trusted.
/// (Copied from nom::multi source.). This is used for arrays.
pub const MAX_INITIAL_CAPACITY_BYTES: usize = 65536;

/// The input of the bit packed parsers: the bytes left and the bits already read of the first.
pub type BitInput<'a> = (&'a [u8], usize);
/// The result of a bit packed parser: the input left and the value.
pub type BitResult<'a, T> = Result<(BitInput<'a>, T)>;
/// The result of a byte aligned parser: the input left and the value.
pub type ByteResult<'a, T> = Result<(&'a [u8], T)>;

/// A type read from the bit packed encoding.
pub trait BitPackedDeserialize: Sized {
    fn deserialize_bits(input: BitInput<'_>) -> BitResult<'_, Self>;
}

/// A type written in the bit packed encoding.
pub trait BitPackedSerialize {
    fn serialize_bits(&self, out: &mut BitWriter) -> Result<()>;
}

/// A type read from the byte aligned encoding.
pub trait ByteAlignedDeserialize: Sized {
    fn deserialize_bytes(input: &[u8]) -> ByteResult<'_, Self>;
}

/// A type written in the byte aligned encoding.
pub trait ByteAlignedSerialize {
    fn serialize_bytes(&self, out: &mut ByteWriter) -> Result<()>;
}

impl BitPackedDeserialize for bool {
    fn deserialize_bits(input: BitInput<'_>) -> BitResult<'_, Self> {
        parse_bool(input)
    }
}

impl BitPackedSerialize for bool {
    fn serialize_bits(&self, out: &mut BitWriter) -> Result<()> {
        out.write_bool(*self)
    }
}

impl BitPackedDeserialize for () {
    fn deserialize_bits(input: BitInput<'_>) -> BitResult<'_, Self> {
        take_null(input)
    }
}

impl BitPackedSerialize for () {
    fn serialize_bits(&self, _out: &mut BitWriter) -> Result<()> {
        Ok(())
    }
}

impl ByteAlignedDeserialize for bool {
    fn deserialize_bytes(input: &[u8]) -> ByteResult<'_, Self> {
        Ok(tagged_bool(input)?)
    }
}

impl ByteAlignedSerialize for bool {
    fn serialize_bytes(&self, out: &mut ByteWriter) -> Result<()> {
        out.write_tagged_bool(*self);
        Ok(())
    }
}

macro_rules! byte_aligned_int {
    ($($ty:ty),*) => {
        $(
            impl ByteAlignedDeserialize for $ty {
                fn deserialize_bytes(input: &[u8]) -> ByteResult<'_, Self> {
                    let (tail, value) = tagged_vlq_int(input)?;
                    Ok((tail, <$ty>::try_from(value)?))
                }
            }

            impl ByteAlignedSerialize for $ty {
                fn serialize_bytes(&self, out: &mut ByteWriter) -> Result<()> {
                    out.write_tagged_int(*self as i64);
                    Ok(())
                }
            }
        )*
    };
}

byte_aligned_int!(u8, u16, u32, u64, i8, i16, i32, i64);
