//! S2 Protocol use of the MPQ archive

pub mod bit_packed_decoder;
pub mod generator;
pub mod protocol_version_decoder;
pub mod tracker_events;
pub mod versioned_decoder;
pub mod versions;
pub use bit_packed_decoder::*;
use nom::error::{ContextError, Error, ErrorKind};
use nom::number::complete::u8;
use nom::*;
use nom_mpq::parser::peek_hex;
pub use protocol_version_decoder::read_protocol_header;
use std::str;
pub use versioned_decoder::*;

/// Returns the 8 bytes following where the error was found for context.
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

/// Reads a VLQ Int that is prepend by its tag
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
