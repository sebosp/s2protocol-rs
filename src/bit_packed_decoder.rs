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
    let mut byte_slice = 0usize;
    loop {
        let count = if remaining_bits > 8 {
            8
        } else {
            remaining_bits
        };
        let (new_tail, bits) =
            dbg_peek_bits(take::<&[u8], u8, usize, _>(count), "take_n_bits_into_i64")(tail)?;
        res += (bits as i64) << (byte_slice * 8usize);
        tail = new_tail;
        if remaining_bits > 8 {
            remaining_bits -= 8;
        } else {
            remaining_bits = 0;
        }
        if remaining_bits == 0 {
            break;
        }
        byte_slice += 1;
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
        let (tail, _) = dbg_peek_bits(
            take::<&[u8], u8, usize, _>(8usize - input.1),
            "take_n_bits_into_i64",
        )(input)?;
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
    use crate::versions::protocol87702::bit_packed::SVarUint32;
    use crate::versions::protocol87702::bit_packed::Uint6;

    #[test_log::test]
    fn it_reads_game_events() {
        // Game events have first the delta and then some event.
        // 0b -> tag 00 -> res = 000000 -> -> (0xf0, 0)
        let data: Vec<u8> = vec![0x00, 0xf0, 0x64, 0x2b, 0x4b, 0xa4, 0x0c, 0x00];
        let ((_tail, _tail_bits), res) = SVarUint32::parse((&data, 0usize)).unwrap();
        assert_eq!(res, SVarUint32::MUint6(Uint6 { value: 0 }));
    }
}
