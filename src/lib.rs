//! S2 Protocol use of the MPQ archive

pub mod generator;
pub mod tracker_events;
pub mod version_decoder;
pub mod versions;
use nom::bytes::complete::{tag, take};
use nom::error::{ContextError, Error, ErrorKind};
use nom::number::complete::u8;
use nom::*;
use nom_mpq::parser::peek_hex;
use std::str;
pub use version_decoder::read_protocol_header;

pub fn dbg_peek<'a, F, O, E: std::fmt::Debug>(
    f: F,
    context: &'static str,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], O, E>
where
    F: Fn(&'a [u8]) -> IResult<&'a [u8], O, E>,
{
    move |i: &'a [u8]| match f(i) {
        Err(e) => {
            tracing::error!("{}: Error({:?}) at: {}", context, e, peek_hex(i));
            Err(e)
        }
        a => a,
    }
}

pub const ARRAY_TAG: &[u8; 1] = b"\x00";
pub const BLOB_TAG: &[u8; 1] = b"\x02";
pub const CHOICE_TAG: &[u8; 1] = b"\x03";
pub const OPT_TAG: &[u8; 1] = b"\x04";
pub const STRUCT_TAG: &[u8; 1] = b"\x05";
pub const BOOL_TAG: &[u8; 1] = b"\x06";
pub const INT_TAG: &[u8; 1] = b"\x09";

/// The protocol contains Variable-Length Quantities (VLQs) that are used to encode the size of
/// arrays etc.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn proto_vlq_tag<'a>(
    input: &'a [u8],
    tag: i64,
    tag_name: &'static str,
) -> IResult<&'a [u8], i64> {
    let (tail, vlq_val) = parse_vlq_int(input)?;
    if vlq_val != tag {
        return Err(Err::Error(Error::add_context(
            tail,
            tag_name,
            Error::new(tail, ErrorKind::Tag),
        )));
    }
    Ok((tail, vlq_val))
}

/// The arrays are prepend by the [`ARRAY_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_array_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek(tag(ARRAY_TAG), "array tag")(input)
}

/// The blobs are prepend by the [`BLOB_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_blob_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek(tag(BLOB_TAG), "blob tag")(input)
}

/// The choice/enums are prepend by the [`CHOICE_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_choice_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek(tag(CHOICE_TAG), "choice tag")(input)
}

/// The optionals are prepend by the [`OPT_TAG`] if provided.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_opt_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek(tag(OPT_TAG), "opt tag")(input)
}

/// The structs are prepend by the [`STRUCT_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_struct_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek(tag(STRUCT_TAG), "struct tag")(input)
}

/// The bools are prepend by the [`BOOL_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_bool_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek(tag(BOOL_TAG), "bool tag")(input)
}

/// The ints are prepend by the [`INT_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_int_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek(tag(INT_TAG), "int tag")(input)
}

/// Reads a blob that is prepend by its tag.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn tagged_blob(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (tail, _) = validate_blob_tag(input)?;
    let (tail, blob_length) = parse_vlq_int(tail)?;
    tracing::trace!("Reading blob length: {}", blob_length);
    let (tail, blob) = dbg_peek(take(blob_length as usize), "blob")(tail)?;
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
    let (tail, value) = dbg_peek(u8, "bool")(&tail)?;
    Ok((tail, value != 0))
}

#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn parse_vlq_int(input: &[u8]) -> IResult<&[u8], i64> {
    let (mut tail, mut v_int_value) = dbg_peek(u8, "v_int")(&input)?;
    let is_negative = v_int_value & 1 != 0;
    let mut result: i64 = ((v_int_value >> 1) & 0x3f) as i64;
    let mut bits: i64 = 6;
    while (v_int_value & 0x80) != 0 {
        let (new_tail, new_v_int_value) = dbg_peek(u8, "v_int")(&tail)?;
        tail = new_tail;
        result |= ((new_v_int_value as i64 & 0x7fi64) << bits) as i64;
        v_int_value = new_v_int_value;
        bits += 7;
    }
    if is_negative {
        result = -result;
    }
    Ok((tail, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn it_reads_v_int() {
        let data: Vec<u8> = vec![
            0x12, 0x2c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let (tail, v_int) = parse_vlq_int(&data).unwrap();
        assert_eq!(v_int, 9);
        let (_tail, v_int) = parse_vlq_int(&tail).unwrap();
        assert_eq!(v_int, 22);
        let input: Vec<u8> = vec![
            0xac, 0xda, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let (_tail, v_int) = parse_vlq_int(&input).unwrap();
        assert_eq!(v_int, 87702);
        // Check with the tag included:
        let input: Vec<u8> = vec![
            0x09, 0xac, 0xda, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let (_tail, v_int) = tagged_vlq_int(&input).unwrap();
        assert_eq!(v_int, 87702);
    }
}
