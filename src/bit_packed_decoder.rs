//! The Bit Packed Decoder, in serde_mpq since the derives of the protocol types. The tests
//! read the bytes of real replays with the protocol types.

pub use serde_mpq::bit_packed::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::versions::protocol87702::bit_packed::*;

    #[test_log::test]
    fn it_reads_game_events() {
        // Game events have first the delta and then some event.
        let data: Vec<u8> = vec![0x00, 0xf0, 0x64, 0x2b, 0x4b, 0xa4, 0x0c, 0x00];
        let tail: (&[u8], usize) = (&data, 0usize);
        let (tail, res) = SVarUint32::parse(tail).unwrap();
        assert_eq!(res, SVarUint32::MUint6(Uint6 { value: 0 }));
        // 2 bits for the ChoiceType of SVarUint32, these contain the binary 00 for MUint6
        // 6 bits then with value 0
        assert_eq!(tail.0[0], 0xf0);
        assert_eq!(tail.1, 0usize);
        let (tail, res) = ReplaySGameUserId::parse(tail).unwrap();
        assert_eq!(res, ReplaySGameUserId { m_user_id: 16 });
        assert_eq!(tail.1, 5usize);
        let (tail, variant_tag) = parse_packed_int(tail, 0, 7usize).unwrap();
        assert_eq!(tail.1, 4usize);
        assert_eq!(variant_tag, 116);
        let (_, uint32_first_4_bits) = rtake_n_bits(tail, 4usize).unwrap();
        assert_eq!(uint32_first_4_bits, 0x06);
        let (tail, m_sync_time) = Uint32::parse(tail).unwrap();
        assert_eq!(tail.1, 4usize);
        assert_eq!(m_sync_time.value, 1656011340);
    }
    #[test_log::test]
    fn it_reads_init_data_properties() {
        let data: Vec<u8> = vec![
            0x07, 0x75, 0x26, 0x7a, // 125118074
            0x50, 0xf8, 0xdf, 0x07, 0xbb, 0xf0, // colors and races
            0xe0, 0x70, 0x00, 0xf0, // difficulty
            // start of allowed controls
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // end of allowed controls
            0x7d, 0x00, 0x00, 0xc0, 0x01, 0x7f, 0x3c, 0x00, 0xc0, 0x03, 0x1f, 0x1c, 0x00, 0xc0,
            0x07, 0x1f, 0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        let tail: (&[u8], usize) = (&data, 0usize);
        let (tail, checksum) = Uint32::parse(tail).unwrap();
        assert_eq!(checksum.value, 125118074);
        let array_length_num_bits: usize = 5;
        let (tail, array_length) = parse_packed_int(tail, 0, array_length_num_bits).unwrap();
        assert_eq!(array_length, 16);
        let bitarray_length_bits: usize = 6;
        let (_tail, bitarray_length) = take_n_bits_into_i64(tail, bitarray_length_bits).unwrap();
        assert_eq!(bitarray_length, 16);
        let (tail, allowed_colors) = GameCAllowedColors::parse(tail).unwrap();
        assert_eq!(allowed_colors.value.to_i64(), 65279);
        let bitarray_length_bits: usize = 8;
        let (_tail, bitarray_length) = take_n_bits_into_i64(tail, bitarray_length_bits).unwrap();
        assert_eq!(bitarray_length, 3);
        let (tail, allowed_races) = CAllowedRaces::parse(tail).unwrap();
        assert_eq!(allowed_races.value.to_i64(), 7);
        let bitarray_length_bits: usize = 6;
        let (_tail, bitarray_length) = take_n_bits_into_i64(tail, bitarray_length_bits).unwrap();
        assert_eq!(bitarray_length, 32);
        let (tail, allowed_difficulty) = GameCAllowedDifficulty::parse(tail).unwrap();
        assert_eq!(allowed_difficulty.value.to_i64(), 4261871616);
        assert_eq!(tail.1, 4usize);
        assert_eq!(tail.0[0], 0xf0);
        let (_tail, bits) = take_n_bits_into_i64(tail, 8usize).unwrap();
        assert_eq!(bits, 0xff);
        let bitarray_length_bits: usize = 8;
        let (_tail, bitarray_length) = take_n_bits_into_i64(tail, bitarray_length_bits).unwrap();
        assert_eq!(bitarray_length, 255);
        let (tail, allowed_races) = GameCAllowedControls::parse(tail).unwrap();
        assert_eq!(allowed_races.value.value.len(), 32);
        let bitarray_length_bits: usize = 2;
        let (_tail, bitarray_length) = take_n_bits_into_i64(tail, bitarray_length_bits).unwrap();
        assert_eq!(bitarray_length, 3);
    }
}
