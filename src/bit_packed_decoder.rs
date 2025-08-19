//! The Bit Packed Decoder.
//! As a difference to the Versioned Decoder, the fields are not tagged.

use crate::dbg_peek_bits;
use crate::peek_bits;
use crate::S2ProtoResult;
use crate::S2ProtocolError;
use nom::bits::complete::take;

/// Takes n bits from the input bytes.
/// The bits are taken counting from right to left.
/// It returns the original input as if had not been processed.
/// The caller must call the normal take process afterwards.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
#[allow(clippy::let_and_return)]
pub fn rtake_n_bits(input: (&[u8], usize), count: usize) -> S2ProtoResult<(&[u8], usize), u8> {
    let res = if input.1 + count > 8usize {
        // We need to process the current left-over bits (from the left)
        let (_, res0) = take::<&[u8], u8, usize, _>(8usize - input.1)((input.0, 0usize))?;
        // Then advance the current bits
        let (tail, _) = take::<&[u8], u8, usize, _>(8usize)((input.0, 0usize))?;
        // and join them with the subsequent bits on the next byte
        let left_over_count = count - (8usize - input.1);
        let (_, res1) =
            take::<&[u8], u8, usize, _>(left_over_count)((tail.0, 8usize - left_over_count))?;
        // join them
        let res = (res0 << left_over_count) + res1;
        res
    } else {
        let (_, res) = take::<&[u8], u8, usize, _>(count)((input.0, 8usize - input.1 - count))?;
        res
    };
    tracing::debug!("rtake_n_bits: {}", res);
    Ok(((input.0, input.1 + count), res))
}

/// Takes n total bits from the current u8 slice at the current offset and transforms the data into
/// an u64, this works with Big Endian
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
#[allow(clippy::precedence)]
pub fn take_n_bits_into_i64(
    input: (&[u8], usize),
    total_bits: usize,
) -> S2ProtoResult<(&[u8], usize), i64> {
    if total_bits > 64 {
        return Err(S2ProtocolError::BitPackedMoreThan64Bits(total_bits));
    }
    let mut res = 0i64;
    let mut remaining_bits = total_bits;
    let mut tail = input;
    loop {
        let count = if remaining_bits > 8 {
            // Try to byte-align
            if tail.1 != 0 {
                8usize - tail.1
            } else {
                8usize
            }
        } else {
            remaining_bits
        };
        let (_, bits) = rtake_n_bits(tail, count)?;
        let (new_tail, _) =
            dbg_peek_bits(take::<&[u8], u8, usize, _>(count), "take_n_bits_into_i64")(tail)?;
        // clippy::precedence is disabled, when I tried to apply the proposed fix we can't parse.
        res |= (bits as i64) << remaining_bits - count;
        // copy << (total_bits - resultbits - copybits)
        tail = new_tail;
        remaining_bits -= count;
        if remaining_bits == 0 {
            break;
        }
    }
    Ok((tail, res))
}

/// Takes n total bits from the current u8 slice at the current offset and transforms the data into
/// a `Vec<u8>`
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn take_bit_array(
    input: (&[u8], usize),
    total_bits: usize,
) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
    let mut res = vec![];
    let mut remaining_bits = total_bits;
    let mut tail = input;
    loop {
        let count = if remaining_bits > 8 {
            8usize
        } else {
            remaining_bits
        };
        let (_, bits) = rtake_n_bits(tail, count)?;
        let (new_tail, _) =
            dbg_peek_bits(take::<&[u8], u8, usize, _>(count), "take_bit_array")(tail)?;
        res.push(bits);
        // copy << (total_bits - resultbits - copybits)
        tail = new_tail;
        remaining_bits -= count;
        if remaining_bits == 0 {
            break;
        }
    }
    tracing::trace!("take_bit_array: {:?}", res);
    Ok((tail, res))
}

/// Takes the remainder of bits in the current input to align the input to a byte boundary.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn byte_align(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), ()> {
    if input.1 != 0 {
        let (tail, _) =
            dbg_peek_bits(take::<&[u8], u8, usize, _>(8usize - input.1), "byte_align")(input)?;
        tracing::debug!("byte_align: took {} bits", 8usize - input.1);
        Ok((tail, ()))
    } else {
        tracing::debug!("byte_align: took 0 bits");
        Ok((input, ()))
    }
}

/// Takes 1 unaligned bytes
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn take_unaligned_byte(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), u8> {
    let (tail, res) = take_bit_array(input, 8usize)?;
    tracing::debug!("take_unaligned_byte: {:?}", res[0]);
    Ok((tail, res[0]))
}

/// Takes 4 unaligned bytes
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn take_fourcc(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
    let (tail, res) = take_bit_array(input, 4usize * 8usize)?;
    tracing::debug!("take_fourcc: {:?}", res);
    Ok((tail, res))
}

/// Just a function that would be called when a null is needed, this is for debugging purposes
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn take_null(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), ()> {
    tracing::debug!("take_null");
    Ok((input, ()))
}
/// Reads a packed int, In offset binary representation, (also called excess-K or biased).
/// a signed number is represented by the bit pattern corresponding to the unsigned number plus K,
/// with K being the biasing value or offset.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn parse_packed_int(
    input: (&[u8], usize),
    offset: i64,
    num_bits: usize,
) -> S2ProtoResult<(&[u8], usize), i64> {
    let (tail, num) = take_n_bits_into_i64(input, num_bits)?;
    let res = offset + num;
    tracing::debug!("parse_packed_int: {}", res);
    Ok((tail, res))
}

/// Reads a single bit and transforms into a bool.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn parse_bool(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
    let (_, val) = rtake_n_bits(input, 1usize)?;
    let (tail, _) = dbg_peek_bits(take::<&[u8], u8, usize, _>(1usize), "take_bit_for_bool")(input)?;
    tracing::debug!("parse_bool: {}", val != 0);
    Ok((tail, val != 0))
}
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
        assert_eq!(allowed_colors.value, 65279);
        let bitarray_length_bits: usize = 8;
        let (_tail, bitarray_length) = take_n_bits_into_i64(tail, bitarray_length_bits).unwrap();
        assert_eq!(bitarray_length, 3);
        let (tail, allowed_races) = CAllowedRaces::parse(tail).unwrap();
        assert_eq!(allowed_races.value, 7);
        let bitarray_length_bits: usize = 6;
        let (_tail, bitarray_length) = take_n_bits_into_i64(tail, bitarray_length_bits).unwrap();
        assert_eq!(bitarray_length, 32);
        let (tail, allowed_difficulty) = GameCAllowedDifficulty::parse(tail).unwrap();
        assert_eq!(allowed_difficulty.value, 4261871616);
        assert_eq!(tail.1, 4usize);
        assert_eq!(tail.0[0], 0xf0);
        let (_tail, bits) = take_n_bits_into_i64(tail, 8usize).unwrap();
        assert_eq!(bits, 0xff);
        let bitarray_length_bits: usize = 8;
        let (_tail, bitarray_length) = take_n_bits_into_i64(tail, bitarray_length_bits).unwrap();
        assert_eq!(bitarray_length, 255);
        let (tail, allowed_races) = GameCAllowedControls::parse(tail).unwrap();
        assert_eq!(allowed_races.value.len(), 32);
        let bitarray_length_bits: usize = 2;
        let (_tail, bitarray_length) = take_n_bits_into_i64(tail, bitarray_length_bits).unwrap();
        assert_eq!(bitarray_length, 3);
    }
}
