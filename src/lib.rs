//! S2 Protocol use of the MPQ archive

pub mod bit_packed_decoder;
pub mod details;
pub mod game_events;
pub mod generator;
pub mod message_events;
pub mod protocol_version_decoder;
pub mod state;
pub mod tracker_events;
pub mod versioned_decoder;
pub mod versions;

use crate::game_events::ReplayGameEvent;
pub use crate::state::*;
use crate::tracker_events::ReplayTrackerEvent;
use crate::versions::read_game_events;
use crate::versions::read_tracker_events;
pub use bit_packed_decoder::*;
use colored::*;
use nom::number::complete::u8;
use nom::IResult;
use nom_mpq::parser::peek_hex;
use nom_mpq::{parser, MPQ};
pub use protocol_version_decoder::read_protocol_header;
use std::collections::HashMap;
use std::str;
pub use versioned_decoder::*;

#[derive(thiserror::Error, Debug)]
pub enum S2ProtocolError {
    #[error("MPQ Error")]
    MPQ(#[from] nom_mpq::MPQParserError),
}

/// Reads the MPQ file and returns both the MPQ read file and the reference to its contents.
pub fn read_mpq(path: &str) -> (MPQ, Vec<u8>) {
    tracing::info!("Processing MPQ file {}", path);
    let file_contents = parser::read_file(path);
    let (_, mpq) = parser::parse(&file_contents).unwrap();
    (mpq, file_contents)
}

/// Creates a colored binary representation of the input.
/// The pre-amble bits are set to blue (these are bits previously processed)
/// The current position, is set to green color.
/// The remaining bits are colored in yellow. These are un-processed bits)
pub fn peek_bits(input: (&[u8], usize)) -> String {
    if input.0.is_empty() {
        return String::from("[]");
    }
    let input_str = format!("{:08b}", input.0[0]);
    let mut res = String::from("[0b");
    for (idx, bit_str) in input_str.chars().enumerate() {
        if idx < input.1 {
            res.push_str(&format!("{}", bit_str).blue());
        } else if idx == input.1 {
            res.push_str(&format!(">{}<", bit_str).green());
        } else {
            res.push_str(&format!("{}", bit_str).yellow());
        }
    }
    res.push_str("]");
    res.push_str(&peek_hex(input.0));
    res
}

/// Returns the 8 bytes following where the error was found for context.
pub fn dbg_peek_bits<'a, F, O, E: std::fmt::Debug>(
    f: F,
    context: &'static str,
) -> impl Fn((&'a [u8], usize)) -> IResult<(&'a [u8], usize), O, E>
where
    F: Fn((&'a [u8], usize)) -> IResult<(&'a [u8], usize), O, E>,
{
    move |i: (&'a [u8], usize)| match f(i) {
        Err(e) => {
            tracing::error!("{}: Error({:?}) at: {}", context, e, peek_bits(i));
            Err(e)
        }
        a => a,
    }
}

/// Returns the 8 bytes following where the error was found for context.
pub fn dbg_peek_hex<'a, F, O, E: std::fmt::Debug>(
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
    let (mut tail, mut v_int_value) = dbg_peek_hex(u8, "v_int")(&input)?;
    let is_negative = v_int_value & 1 != 0;
    let mut result: i64 = ((v_int_value >> 1) & 0x3f) as i64;
    let mut bits: i64 = 6;
    while (v_int_value & 0x80) != 0 {
        let (new_tail, new_v_int_value) = dbg_peek_hex(u8, "v_int")(&tail)?;
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
