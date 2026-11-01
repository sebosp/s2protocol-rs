//! S2 Protocol use of the MPQ archive

#[cfg(feature = "dep_arrow")]
pub mod arrow_store;
pub mod bit_packed_decoder;
pub mod cache_handles;

pub mod basic_replay_data;
pub mod common;
pub mod details;
pub mod dir_stats;
pub mod error;
pub mod filters;
pub mod game_events;
pub mod init_data;
pub mod message_events;
pub mod protocol_version_decoder;
pub mod snapshot;
pub mod state;
#[cfg(feature = "textures")]
pub mod textures;
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
pub use common::*;
pub use error::*;
pub use filters::*;
pub use init_data::*;
pub use protocol_version_decoder::read_protocol_header;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::str;
pub use versioned_decoder::*;

pub use nom_mpq::MPQ;
/// Re-export to avoid having to also add this crate to other consumers.
pub use nom_mpq::parser;
pub use serde_mpq::peek::*;
pub use serde_mpq::{
    BitArray, BitInput, BitPackedDeserialize, BitPackedSerialize, BitResult, BitWriter,
    ByteAlignedDeserialize, ByteAlignedSerialize, ByteResult, ByteWriter,
    MAX_INITIAL_CAPACITY_BYTES, MPQBitPackedDeserialize, MPQBitPackedSerialize,
    MPQByteAlignedDeserialize, MPQByteAlignedSerialize,
};

/// Reads a file into memory.
pub fn read_file(path: &PathBuf) -> Result<Vec<u8>, S2ProtocolError> {
    let mut f = std::fs::File::open(path)?;
    let mut buffer: Vec<u8> = vec![];
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Reads the MPQ file and returns both the MPQ read file and the reference to its contents.
pub fn read_mpq(path: &str) -> Result<(MPQ, Vec<u8>), S2ProtocolError> {
    tracing::info!("Processing MPQ file {}", path);
    let path = PathBuf::from(path);
    let file_contents = crate::read_file(&path)?;
    let (_, mpq) = parser::parse(&file_contents)?;
    Ok((mpq, file_contents))
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

/// The game loops per second at the Faster speed, the speed of the ladder and the tournaments.
/// The in-game clock shows real seconds at Faster since Legacy of the Void.
pub const GAME_LOOPS_PER_SECOND: f64 = 22.4;

/// Converts the tracker loop to game seconds at the Faster speed.
/// The tracker loop is first scaled to the same unit as the game loops.
pub fn convert_tracker_loop_to_seconds(tracker_replay_loop: i64) -> u32 {
    convert_game_loop_to_seconds((crate::TRACKER_SPEED_RATIO * tracker_replay_loop as f32) as i64)
}

/// Converts the game loop to game seconds at the Faster speed, [`GAME_LOOPS_PER_SECOND`].
pub fn convert_game_loop_to_seconds(game_replay_loop: i64) -> u32 {
    (game_replay_loop.max(0) as f64 / GAME_LOOPS_PER_SECOND) as u32
}

/// Matches a list of files in case the cli.source param is a directory
#[tracing::instrument(level = "debug")]
pub fn get_matching_files(
    source: PathBuf,
    max_files: usize,
    max_depth: usize,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    if max_depth == 0 {
        tracing::info!("Reached max depth");
        return Ok(Vec::new());
    }
    if source.is_dir() {
        // if this is a directory, let's recurse to go through subdirectories.
        let mut sources = Vec::new();
        for entry in std::fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let mut sub_dir =
                    get_matching_files(path, max_files - sources.len(), max_depth - 1)?;
                if !sub_dir.is_empty() {
                    // store n sub_dir files without breaking the max_files limit
                    let remaining = max_files - sources.len();
                    if sub_dir.len() > remaining {
                        sub_dir.truncate(remaining);
                    }
                    sources.append(&mut sub_dir);
                }
            } else if let Some(ext) = path.extension()
                && ext == "SC2Replay"
                && path.is_file()
            {
                if sources.len() >= max_files {
                    break;
                }
                sources.push(path);
            }
        }
        Ok(sources)
    } else {
        Ok(vec![])
    }
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

    #[test]
    fn it_converts_the_loops_at_faster_speed() {
        assert_eq!(convert_game_loop_to_seconds(0), 0);
        assert_eq!(convert_game_loop_to_seconds(-16), 0);
        assert_eq!(convert_game_loop_to_seconds(224), 10);
        // A 10:13 game.
        assert_eq!(convert_game_loop_to_seconds(13735), 613);
        assert_eq!(convert_tracker_loop_to_seconds(22400), 1000);
    }
}
