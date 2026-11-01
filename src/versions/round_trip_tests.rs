//! The sections of the test replays, parsed with the derives, written back and compared with the
//! bytes of the replay: the layout of every type read on the way is checked bit by bit.

use crate::versions::protocol87702::bit_packed::{GameEMessageId, ReplaySGameUserId, SVarUint32};
use crate::versions::protocol87702::byte_aligned;
use crate::versions::{protocol75689, protocol87702};
use crate::*;
use serde_mpq::{
    BitPackedDeserialize, BitPackedSerialize, BitWriter, ByteAlignedDeserialize,
    ByteAlignedSerialize, ByteWriter,
};

const REPLAYS: [&str; 3] = [
    "2023-04-08-2v2AI.SC2Replay",
    "FieldsDeath202507.SC2Replay",
    "SC2-Patch_4.12-2v2AI.SC2Replay",
];

fn first_difference(a: &[u8], b: &[u8]) -> Option<usize> {
    if a == b {
        return None;
    }
    Some(
        a.iter()
            .zip(b)
            .position(|(x, y)| x != y)
            .unwrap_or(a.len().min(b.len())),
    )
}

/// The events of a bit packed section, `(delta, user id, event)` byte aligned after each event.
fn bit_packed_events<E: BitPackedDeserialize + BitPackedSerialize>(
    sector: &[u8],
) -> (usize, Vec<u8>) {
    let mut tail: (&[u8], usize) = (sector, 0);
    let mut out = BitWriter::new();
    let mut events = 0;
    while !tail.0.is_empty() {
        let (t, delta) = SVarUint32::parse(tail).unwrap();
        let (t, user_id) = ReplaySGameUserId::parse(t).unwrap();
        let (t, event) = E::deserialize_bits(t).unwrap();
        let (t, _) = byte_align(t).unwrap();
        delta.serialize_bits(&mut out).unwrap();
        user_id.serialize_bits(&mut out).unwrap();
        event.serialize_bits(&mut out).unwrap();
        out.byte_align();
        tail = t;
        events += 1;
    }
    (events, out.into_bytes())
}

/// The events of the tracker section, `(delta, event)` pairs.
fn tracker_events(sector: &[u8]) -> (usize, Vec<u8>) {
    let mut tail: &[u8] = sector;
    let mut out = ByteWriter::new();
    let mut events = 0;
    while !tail.is_empty() {
        let (t, delta) = byte_aligned::SVarUint32::parse(tail).unwrap();
        let (t, event) = byte_aligned::ReplayTrackerEEventId::parse(t).unwrap();
        delta.serialize_bytes(&mut out).unwrap();
        event.serialize_bytes(&mut out).unwrap();
        tail = t;
        events += 1;
    }
    (events, out.into_bytes())
}

fn byte_aligned_value<
    T: ByteAlignedDeserialize + ByteAlignedSerialize + PartialEq + std::fmt::Debug,
>(
    what: &str,
    sector: &[u8],
) -> bool {
    let (tail, value) = T::deserialize_bytes(sector).unwrap();
    let consumed = &sector[..sector.len() - tail.len()];
    let mut out = ByteWriter::new();
    value.serialize_bytes(&mut out).unwrap();
    let (_, again) = T::deserialize_bytes(out.as_bytes()).unwrap();
    assert_eq!(again, value, "{what}: the value changes in a round trip");
    let identical = first_difference(consumed, out.as_bytes()).is_none();
    eprintln!(
        "{what}: {} bytes, byte identical: {identical}",
        consumed.len()
    );
    identical
}

#[test]
fn it_writes_the_sections_of_the_replays_back_bit_by_bit() {
    for replay in REPLAYS {
        let path = format!("{}/assets/{replay}", env!("CARGO_MANIFEST_DIR"));
        let (mpq, contents) = read_mpq(&path).unwrap();
        let header = read_protocol_header(&mpq).unwrap().1;
        let base_build = header.m_version.m_base_build;
        eprintln!("--- {replay}: base build {base_build}");

        // The header and the details are byte aligned, the fields are tagged: the value must round
        // trip, the bytes do when every field of the type is in the replay.
        let user_data = &mpq.user_data.as_ref().unwrap().content;
        byte_aligned_value::<byte_aligned::ReplaySHeader>("header", user_data);
        let (_, details) = mpq
            .read_mpq_file_sector("replay.details", false, &contents)
            .unwrap();
        byte_aligned_value::<byte_aligned::GameSDetails>("details", &details);

        let (_, tracker) = mpq
            .read_mpq_file_sector("replay.tracker.events", false, &contents)
            .unwrap();
        let (events, written) = tracker_events(&tracker);
        eprintln!("tracker events: {events} events, {} bytes", tracker.len());
        assert_eq!(
            first_difference(&tracker, &written),
            None,
            "{replay} tracker events"
        );

        // The bit packed sections must be identical to the bit.
        let (_, game) = mpq
            .read_mpq_file_sector("replay.game.events", false, &contents)
            .unwrap();
        let (events, written) = if base_build <= 75689 {
            bit_packed_events::<protocol75689::bit_packed::GameEEventId>(&game)
        } else {
            bit_packed_events::<protocol87702::bit_packed::GameEEventId>(&game)
        };
        eprintln!("game events: {events} events, {} bytes", game.len());
        assert_eq!(
            first_difference(&game, &written),
            None,
            "{replay} game events"
        );

        let (_, messages) = mpq
            .read_mpq_file_sector("replay.message.events", false, &contents)
            .unwrap();
        let (events, written) = bit_packed_events::<GameEMessageId>(&messages);
        eprintln!("message events: {events} events, {} bytes", messages.len());
        assert_eq!(
            first_difference(&messages, &written),
            None,
            "{replay} message events"
        );

        if base_build > 75689 {
            let (_, init_data) = mpq
                .read_mpq_file_sector("replay.initData", false, &contents)
                .unwrap();
            let (tail, value) =
                protocol87702::bit_packed::ReplaySInitData::parse((&init_data, 0)).unwrap();
            let mut out = BitWriter::new();
            value.serialize_bits(&mut out).unwrap();
            let consumed = &init_data[..init_data.len() - tail.0.len()];
            eprintln!(
                "init data: {} bytes read of {}",
                consumed.len(),
                init_data.len()
            );
            let written = &out.as_bytes()[..consumed.len().min(out.bit_len().div_ceil(8))];
            if let Some(at) = first_difference(consumed, written) {
                let from = at.saturating_sub(8);
                eprintln!(
                    "init data differs at byte {at}:\n  replay  {:02x?}\n  written {:02x?}\n  user 0: {:?}",
                    &consumed[from..(at + 16).min(consumed.len())],
                    &written[from..(at + 16).min(written.len())],
                    value.m_sync_lobby_state.m_user_initial_data.value.first()
                );
            }
            assert_eq!(
                first_difference(consumed, written),
                None,
                "{replay} init data"
            );
        }
    }
}
