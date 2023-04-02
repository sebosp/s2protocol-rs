//! The Bit Packed Decoder.
//! As a difference to the Versioned Decoder, the fields are not tagged.

use crate::dbg_peek_bits;
use crate::peek_bits;
use nom::bits::complete::take;
use nom::*;

/*
 * Seems like the only use we could have for BitArray is the NNet.Game.SelectionMaskType
/// Stores a bit array.
#[derive(Debug, Clone, Copy)]
pub struct BitArray {
    /// The data read
    data: Vec<u8>,
    /// The last byte may have only a few addressed bits.
    left_overs: usize,
}*/

/// Takes n bits from the input bytes.
/// The bits are taken counting from right to left.
/// It returns the original input as if had not been processed.
/// The caller must call the normal take process afterwards.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn rtake_n_bits(input: (&[u8], usize), count: usize) -> IResult<(&[u8], usize), u8> {
    let res = if input.1 + count > 8usize {
        // We need to process the current left-over bits (from the left)
        let (_, res0) = take::<&[u8], u8, usize, _>(8usize - input.1)((input.0, 0usize))?;
        // Then advance the current bits
        let (tail, _) = take::<&[u8], u8, usize, _>(8usize)((input.0, 0usize))?;
        // and join them with the subsequent bits on teh next byte
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
    Ok(((input.0, input.1 + count), res))
}

/// Takes n total bits from the current u8 slice at the current offset and transforms the data into
/// an u64, this works with Big Endian
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn take_n_bits_into_i64(
    input: (&[u8], usize),
    total_bits: usize,
) -> IResult<(&[u8], usize), i64> {
    assert!(total_bits < 64);
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
/// a Vec<u8>
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn take_bit_array(
    input: (&[u8], usize),
    total_bits: usize,
) -> IResult<(&[u8], usize), Vec<u8>> {
    let mut res = vec![];
    let mut remaining_bits = total_bits;
    let mut tail = input;
    loop {
        let count = if remaining_bits > 8 {
            8
        } else {
            remaining_bits
        };
        let (new_tail, bits) =
            dbg_peek_bits(take::<&[u8], u8, usize, _>(count), "take_bit_array")(tail)?;
        res.push(bits);
        tail = new_tail;
        if remaining_bits > 8 {
            remaining_bits -= 8;
        } else {
            remaining_bits = 0;
        }
        if remaining_bits == 0 {
            break;
        }
    }
    Ok((tail, res))
}

/// Takes the remainder of bits in the current input to align the input to a byte boundary.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn byte_align(input: (&[u8], usize)) -> IResult<(&[u8], usize), ()> {
    if input.1 != 0 {
        let (tail, _) =
            dbg_peek_bits(take::<&[u8], u8, usize, _>(8usize - input.1), "byte_align")(input)?;
        Ok((tail, ()))
    } else {
        Ok((input, ()))
    }
}

/// Takes 1 unaligned bytes
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn take_unaligned_byte(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    let (tail, res) = take_bit_array(input, 8usize)?;
    Ok((tail, res[0]))
}

/// Takes 4 unaligned bytes
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn take_fourcc(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<u8>> {
    take_bit_array(input, 4usize * 8usize)
}

/// Reads a packed int, In offset binary representation, (also called excess-K or biased).
/// a signed number is represented by the bit pattern corresponding to the unsigned number plus K,
/// with K being the biasing value or offset.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn parse_packed_int(
    input: (&[u8], usize),
    offset: i64,
    num_bits: usize,
) -> IResult<(&[u8], usize), i64> {
    let (tail, num) = take_n_bits_into_i64(input, num_bits)?;
    let res = offset + num;
    Ok((tail, res))
}

/// Reads a single bit and transforms into a bool.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn parse_bool(input: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    let (tail, val) =
        dbg_peek_bits(take::<&[u8], u8, usize, _>(1usize), "take_bit_for_bool")(input)?;
    Ok((tail, val != 0))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::versions::protocol87702::bit_packed::SVarUint32;
    use crate::versions::protocol87702::bit_packed::TUserId;
    use crate::versions::protocol87702::bit_packed::Uint32;
    use crate::versions::protocol87702::bit_packed::Uint6;

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
        let (tail, res) = TUserId::parse(tail).unwrap();
        assert_eq!(res, TUserId { value: 16 });
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
}
