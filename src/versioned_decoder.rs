//! The versioned decoder.
//! The fields on versioned structures are prepend by tags.

use super::*;
use nom::bytes::complete::{tag, take};
use nom::number::complete::{u32, u8};
use nom_mpq::parser::peek_hex;

/// Arrays have this tag prepend.
pub const ARRAY_TAG: &[u8; 1] = b"\x00";
/// Arrays have this tag prepend.
pub const BIT_ARRAY_TAG: &[u8; 1] = b"\x01";
/// Blobs have this tag prepend.
pub const BLOB_TAG: &[u8; 1] = b"\x02";
/// Choices have this tag prepend.
pub const CHOICE_TAG: &[u8; 1] = b"\x03";
/// Optionals have this tag prepend.
pub const OPT_TAG: &[u8; 1] = b"\x04";
/// Structs have this tag prepend.
pub const STRUCT_TAG: &[u8; 1] = b"\x05";
/// Bools have this tag prepend.
pub const BOOL_TAG: &[u8; 1] = b"\x06";
/// FourCC (Fourc Character Code) and Real32 have this tag prepend.
pub const FOURCC_TAG: &[u8; 1] = b"\x07";
/// Ints have this tag prepend.
pub const INT_TAG: &[u8; 1] = b"\x09";

/// The arrays are prepend by the [`ARRAY_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_array_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(ARRAY_TAG), "array tag")(input)
}

/// The bitarrays are prepend by the [`BIT_ARRAY_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_bitarray_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(BIT_ARRAY_TAG), "bitarray tag")(input)
}

/// The blobs are prepend by the [`BLOB_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_blob_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(BLOB_TAG), "blob tag")(input)
}

/// The choice/enums are prepend by the [`CHOICE_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_choice_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(CHOICE_TAG), "choice tag")(input)
}

/// The optionals are prepend by the [`OPT_TAG`] if provided.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_opt_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(OPT_TAG), "opt tag")(input)
}

/// The structs are prepend by the [`STRUCT_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_struct_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(STRUCT_TAG), "struct tag")(input)
}

/// The bools are prepend by the [`BOOL_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_bool_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(BOOL_TAG), "bool tag")(input)
}

/// The FourCC and Real32 are prepend by the [`FOURCC_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_fourcc_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(FOURCC_TAG), "fourcc tag")(input)
}

/// The ints are prepend by the [`INT_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_int_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(INT_TAG), "int tag")(input)
}

/// Reads a bitarray that is prepend by its tag.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn tagged_bitarray(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (tail, _) = validate_bitarray_tag(input)?;
    let (tail, array_length) = parse_vlq_int(tail)?;
    let array_length = (array_length as usize + 7usize) / 8usize;
    tracing::trace!("Reading array length: {}", array_length);
    let (tail, array) = dbg_peek_hex(take(array_length), "bitarray")(tail)?;
    Ok((tail, array.to_vec()))
}

/// Reads a blob that is prepend by its tag.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn tagged_blob(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (tail, _) = validate_blob_tag(input)?;
    let (tail, blob_length) = parse_vlq_int(tail)?;
    tracing::trace!("Reading blob length: {}", blob_length);
    let (tail, blob) = dbg_peek_hex(take(blob_length as usize), "blob")(tail)?;
    Ok((tail, blob.to_vec()))
}

/// Reads a VLQ Int that is prepend by its tag.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn tagged_vlq_int(input: &[u8]) -> IResult<&[u8], i64> {
    let (tail, _) = validate_int_tag(input)?;
    parse_vlq_int(tail)
}

/// Reads a Boolean that is prepend by its tag
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn tagged_bool(input: &[u8]) -> IResult<&[u8], bool> {
    let (tail, _) = validate_bool_tag(input)?;
    let (tail, value) = dbg_peek_hex(u8, "bool")(tail)?;
    Ok((tail, value != 0))
}

/// Reads a 4 bytes, the FourCC (Four Character Code).
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn tagged_fourcc(input: &[u8]) -> IResult<&[u8], u32> {
    let (tail, _) = validate_fourcc_tag(input)?;
    // TODO: We don't care much about the Endianness at this point
    let (tail, value) = dbg_peek_hex(u32(nom::number::Endianness::Big), "fourcc")(tail)?;
    Ok((tail, value))
}
