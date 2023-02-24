//! The Bit Packed Decoder.
//! As a difference to the Versioned Decoder, the fields are not tagged.

use super::*;
use nom::bits::complete::take as take_bits;
use nom::bytes::complete::take as take_bytes;
use nom::number::complete::u8;
use nom::*;

pub fn peek_bits(input: (&[u8], usize)) -> String {
    if input.0.len() > 0 {
        format!("{:#010b}[{}]", input.0[0], input.1)
    } else {
        String::from("NoData")
    }
}

/// Reads a number of bits
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn read_bits(input: (&[u8], usize), num_bits: usize) -> IResult<(&[u8], usize), u8> {
    take_bits(num_bits)(input)
}

/// Reads a packed int
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn parse_packed_int(
    input: (&[u8], usize),
    low_bound: u8,
    num_bits: usize,
) -> IResult<(&[u8], usize), u8> {
    let ((tail, tail_bit_offset), bits) = read_bits(input, num_bits)?;
    let res = low_bound + bits;
    Ok(((tail, tail_bit_offset), res))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::versions::protocol87702::SVarUint32;

    #[test_log::test]
    fn it_reads_game_events() {
        // Game events have first the delta and then some event.
        let data: Vec<u8> = vec![0x00, 0xf0, 0x64, 0x2b, 0x4b, 0xa4, 0x0c, 0x00];
        let (tail, res) = SVarUint32::parse(&data).unwrap();
        assert_eq!(res, SVarUint32::Uint6(08));
    }
}
