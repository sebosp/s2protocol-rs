//! S2 Protocol use of the MPQ archive

#[cfg(feature = "dep_arrow")]
pub mod arrow_store;
pub mod bit_packed_decoder;
pub mod cli;
pub mod common;
pub mod details;
pub mod error;
pub mod filters;
pub mod game_events;
pub mod generator;
pub mod init_data;
pub mod message_events;
pub mod protocol_version_decoder;
pub mod state;
pub mod tracker_events;
pub mod versioned_decoder;
pub mod versions;

use crate::game_events::ReplayGameEvent;
pub use crate::state::*;
use crate::tracker_events::ReplayTrackerEvent;
pub use crate::versions::read_details;
pub use crate::versions::read_game_events;
pub use crate::versions::read_init_data;
pub use crate::versions::read_message_events;
pub use crate::versions::read_tracker_events;
#[cfg(feature = "dep_arrow")]
pub use arrow_store::*;
pub use bit_packed_decoder::*;
use chrono::DateTime;
use chrono::Utc;
pub use cli::*;
use colored::*;
pub use common::*;
pub use error::*;
pub use filters::*;
pub use init_data::*;
use nom::number::complete::u8;
use nom::IResult;
pub use protocol_version_decoder::read_protocol_header;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::str;
pub use versioned_decoder::*;

/// Re-export to avoid having to also add this crate to other consumers.
pub use nom_mpq::parser::{self, peek_hex};
pub use nom_mpq::MPQ;

/// Many fields are optional, this macro will return an Ok for the nom::IResult but the value will
/// be an Err(S2ProtocolError::MissingField) if the field is not present.
/// This allows for avoiding panic!() and instead can be ?
#[macro_export]
macro_rules! ok_or_return_missing_field_err {
    ($req_field:ident) => {
        match $req_field {
            Some(v) => v,
            None => {
                tracing::error!(
                    missing_field = stringify!($req_field),
                    "Required field not provided"
                );
                return Err(S2ProtocolError::MissingField(
                    stringify!($req_field).to_string(),
                ));
            }
        }
    };
}

/// Reads a file into memory.
pub fn read_file(path: &PathBuf) -> Result<Vec<u8>, S2ProtocolError> {
    let mut f = std::fs::File::open(path)?;
    let mut buffer: Vec<u8> = vec![];
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Pre-allocating memory is a nice optimization but count fields can't
/// always be trusted. (Copied from nom::multi source.).
/// This is used for arrays.
pub const MAX_INITIAL_CAPACITY_BYTES: usize = 65536;

/// Reads the MPQ file and returns both the MPQ read file and the reference to its contents.
pub fn read_mpq(path: &str) -> Result<(MPQ, Vec<u8>), S2ProtocolError> {
    tracing::info!("Processing MPQ file {}", path);
    let file_contents = parser::read_file(path);
    let (_, mpq) = parser::parse(&file_contents)?;
    Ok((mpq, file_contents))
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
        match idx.cmp(&input.1) {
            std::cmp::Ordering::Less => res.push_str(&format!("{bit_str}").blue()),
            std::cmp::Ordering::Equal => res.push_str(&format!(">{bit_str}<").green()),
            std::cmp::Ordering::Greater => res.push_str(&format!("{bit_str}").yellow()),
        };
    }
    res.push(']');
    res.push_str(&peek_hex(input.0));
    res
}

/// Returns the 8 bytes following where the error was found for context.
pub fn dbg_peek_bits<'a, F, O, E: std::fmt::Debug>(
    mut f: F,
    context: &'static str,
) -> impl FnMut((&'a [u8], usize)) -> IResult<(&'a [u8], usize), O, E>
where
    F: FnMut((&'a [u8], usize)) -> IResult<(&'a [u8], usize), O, E>,
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
    mut f: F,
    context: &'static str,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], O, E>
where
    F: FnMut(&'a [u8]) -> IResult<&'a [u8], O, E>,
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

/// Transforms the details MPQ sector time utc and local offset into possibly chrono::NaiveDateTime
pub fn transform_to_naivetime(
    time_utc: i64,
    time_local_offset: i64,
) -> Option<chrono::NaiveDateTime> {
    let micros = time_utc / 10 - 11_644_473_600_000_000 - time_local_offset / 10;
    let secs = micros.div_euclid(1_000_000);
    let nsecs = micros.rem_euclid(1_000_000) as u32 * 1000;
    // Some(chrono::NaiveDateTime::from_timestamp(secs, nsecs))
    // Use DateTime::from_timestamp and transform into NaiveDateTime:
    Some(DateTime::<Utc>::from_timestamp(secs, nsecs)?.naive_utc())
}

/// Converts the tracker loop to milliseconds,
/// First scale the tracker loop to the same unit as the game loops.
/// We also need to transform the adjusted game loop to seconds.
/// This was observed in a game with max game_loop = 13735 and a duration of 15:42 = 942 seconds.
/// 942000 / 13735 = 68.58391 loops in a second
/// This will only work for the Faster speed.
pub fn convert_tracker_loop_to_seconds(tracker_replay_loop: i64) -> u32 {
    convert_game_loop_to_seconds((crate::TRACKER_SPEED_RATIO * tracker_replay_loop as f32) as i64)
}

/// Converts the game loop to milliseconds,
pub fn convert_game_loop_to_seconds(game_replay_loop: i64) -> u32 {
    // TODO: For now let's use seconds, we'll move this to milliseconds as we need more precision.
    let ext_replay_milliseconds = game_replay_loop as f32 * 68.58391;
    (ext_replay_milliseconds / 1000.0) as u32
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
        let (_tail, v_int) = parse_vlq_int(tail).unwrap();
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
