//! The Bit Packed Decoder.
//! As a difference to the Versioned Decoder, the fields are not tagged.

use crate::peek_bits;
use crate::{dbg_peek_bits, dbg_peek_hex};
use nom::bits::complete::take;
use nom::number::complete::i64;
use nom::number::Endianness;
use nom::*;

/// Reads a packed int, In offset binary representation, (also called excess-K or biased).
/// a signed number is represented by the bit pattern corresponding to the unsigned number plus K, with K being the biasing value or offset.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn parse_packed_int(
    input: (&[u8], usize),
    offset: i64,
    num_bits: usize,
) -> IResult<(&[u8], usize), i64> {
    let (tail, bits): ((&[u8], usize), Vec<u8>) = take(num_bits)(input)?;
    let (_, int) = dbg_peek_hex(i64(Endianness::Big), "i64")(&bits)?;
    let res = offset + int;
    Ok((tail, res))
}

#[cfg(test)]
mod tests {
    use crate::versions::protocol87702::bit_packed::SVarUint32;

    #[test_log::test]
    fn it_reads_game_events() {
        // Game events have first the delta and then some event.
        let data: Vec<u8> = vec![0x00, 0xf0, 0x64, 0x2b, 0x4b, 0xa4, 0x0c, 0x00];
        let ((_tail, _tail_bits), res) = SVarUint32::parse((&data, 0usize)).unwrap();
        assert_eq!(res, SVarUint32::Uint6(0));
    }
}
