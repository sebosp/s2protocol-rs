//! The byte aligned (versioned) encoding, the one of the header, details and tracker events:
//! every value is preceded by the tag of its kind, the fields of a struct by their number and
//! the integers are variable length (VLQ).

use crate::peek::{dbg_peek_hex, peek_hex};
use nom::IResult;
use nom::bytes::complete::{tag, take};
use nom::number::complete::{u8, u32};

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
    dbg_peek_hex(tag(&ARRAY_TAG[..]), "array tag")(input)
}

/// The bitarrays are prepend by the [`BIT_ARRAY_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_bitarray_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(&BIT_ARRAY_TAG[..]), "bitarray tag")(input)
}

/// The blobs are prepend by the [`BLOB_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_blob_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(&BLOB_TAG[..]), "blob tag")(input)
}

/// The choice/enums are prepend by the [`CHOICE_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_choice_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(&CHOICE_TAG[..]), "choice tag")(input)
}

/// The optionals are prepend by the [`OPT_TAG`] if provided.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_opt_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(&OPT_TAG[..]), "opt tag")(input)
}

/// The structs are prepend by the [`STRUCT_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_struct_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(&STRUCT_TAG[..]), "struct tag")(input)
}

/// The bools are prepend by the [`BOOL_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_bool_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(&BOOL_TAG[..]), "bool tag")(input)
}

/// The FourCC and Real32 are prepend by the [`FOURCC_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_fourcc_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(&FOURCC_TAG[..]), "fourcc tag")(input)
}

/// The ints are prepend by the [`INT_TAG`]
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn validate_int_tag(input: &[u8]) -> IResult<&[u8], &[u8]> {
    dbg_peek_hex(tag(&INT_TAG[..]), "int tag")(input)
}

/// Reads a VLQ Int: the lowest bit of the first byte is the sign, then 6 bits of the value, and 7
/// bits per following byte while the highest bit is set.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
#[allow(clippy::unnecessary_cast)]
pub fn parse_vlq_int(input: &[u8]) -> IResult<&[u8], i64> {
    let (mut tail, mut v_int_value) = dbg_peek_hex(u8, "v_int")(input)?;
    let is_negative = v_int_value & 1 != 0;
    let mut result: i64 = ((v_int_value >> 1) & 0x3f) as i64;
    let mut bits: i64 = 6;
    while (v_int_value & 0x80) != 0 {
        let (new_tail, new_v_int_value) = dbg_peek_hex(u8, "v_int")(tail)?;
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

/// Reads a bitarray that is prepend by its tag.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn tagged_bitarray(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (tail, _) = validate_bitarray_tag(input)?;
    let (tail, array_length) = parse_vlq_int(tail)?;
    let array_length = (array_length as usize).div_ceil(8usize);
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

/// Reads the presence byte of an optional, after its tag.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
pub fn tagged_opt_presence(input: &[u8]) -> IResult<&[u8], bool> {
    let (tail, _) = validate_opt_tag(input)?;
    let (tail, is_provided) = dbg_peek_hex(u8, "opt presence")(tail)?;
    Ok((tail, is_provided != 0))
}

/// Writes the byte aligned encoding, the inverse of the parsers above.
#[derive(Debug, Default, Clone)]
pub struct ByteWriter {
    bytes: Vec<u8>,
}

impl ByteWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn write_raw(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    /// Writes a VLQ Int, the inverse of [`parse_vlq_int`].
    pub fn write_vlq_int(&mut self, value: i64) {
        let mut magnitude = value.unsigned_abs();
        let mut byte = ((magnitude & 0x3f) as u8) << 1 | (value < 0) as u8;
        magnitude >>= 6;
        if magnitude > 0 {
            byte |= 0x80;
        }
        self.bytes.push(byte);
        while magnitude > 0 {
            let mut byte = (magnitude & 0x7f) as u8;
            magnitude >>= 7;
            if magnitude > 0 {
                byte |= 0x80;
            }
            self.bytes.push(byte);
        }
    }

    /// Writes an int with its tag, the inverse of [`tagged_vlq_int`].
    pub fn write_tagged_int(&mut self, value: i64) {
        self.bytes.extend_from_slice(INT_TAG);
        self.write_vlq_int(value);
    }

    /// Writes a blob with its tag and length, the inverse of [`tagged_blob`].
    pub fn write_tagged_blob(&mut self, value: &[u8]) {
        self.bytes.extend_from_slice(BLOB_TAG);
        self.write_vlq_int(value.len() as i64);
        self.bytes.extend_from_slice(value);
    }

    /// Writes a bool with its tag, the inverse of [`tagged_bool`].
    pub fn write_tagged_bool(&mut self, value: bool) {
        self.bytes.extend_from_slice(BOOL_TAG);
        self.bytes.push(value as u8);
    }

    /// Writes a FourCC with its tag, the inverse of [`tagged_fourcc`].
    pub fn write_tagged_fourcc(&mut self, value: u32) {
        self.bytes.extend_from_slice(FOURCC_TAG);
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    /// Writes the tag and presence byte of an optional, the value follows when it is present.
    pub fn write_opt_presence(&mut self, is_provided: bool) {
        self.bytes.extend_from_slice(OPT_TAG);
        self.bytes.push(is_provided as u8);
    }

    /// Writes the tag and length of an array, the elements follow.
    pub fn write_array_header(&mut self, len: usize) {
        self.bytes.extend_from_slice(ARRAY_TAG);
        self.write_vlq_int(len as i64);
    }

    /// Writes the tag and field count of a struct, the tagged fields follow.
    pub fn write_struct_header(&mut self, fields: usize) {
        self.bytes.extend_from_slice(STRUCT_TAG);
        self.write_vlq_int(fields as i64);
    }

    /// Writes the number of a field of a struct, its value follows.
    pub fn write_field_tag(&mut self, tag: i64) {
        self.write_vlq_int(tag);
    }

    /// Writes the tag and variant of a choice, the value of the variant follows.
    pub fn write_choice_header(&mut self, variant: i64) {
        self.bytes.extend_from_slice(CHOICE_TAG);
        self.write_vlq_int(variant);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_writes_the_vlq_ints_the_parser_reads() {
        for value in [
            0i64,
            1,
            -1,
            63,
            64,
            -64,
            127,
            128,
            8191,
            8192,
            1 << 20,
            -(1 << 20),
            i64::MAX / 2,
        ] {
            let mut out = ByteWriter::new();
            out.write_vlq_int(value);
            let (tail, read) = parse_vlq_int(out.as_bytes()).unwrap();
            assert_eq!(read, value, "{value}");
            assert!(tail.is_empty());
        }
    }

    #[test]
    fn it_writes_the_tagged_values_the_parsers_read() {
        let mut out = ByteWriter::new();
        out.write_tagged_int(-300);
        out.write_tagged_blob(b"swarmy");
        out.write_tagged_bool(true);
        out.write_tagged_fourcc(0x53_43_32_00);
        out.write_opt_presence(false);
        let bytes = out.into_bytes();
        let (tail, v) = tagged_vlq_int(&bytes).unwrap();
        assert_eq!(v, -300);
        let (tail, blob) = tagged_blob(tail).unwrap();
        assert_eq!(blob, b"swarmy");
        let (tail, b) = tagged_bool(tail).unwrap();
        assert!(b);
        let (tail, fourcc) = tagged_fourcc(tail).unwrap();
        assert_eq!(fourcc, 0x53_43_32_00);
        let (tail, present) = tagged_opt_presence(tail).unwrap();
        assert!(!present);
        assert!(tail.is_empty());
    }
}
