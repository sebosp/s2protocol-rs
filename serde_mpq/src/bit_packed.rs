//! The bit packed encoding, the one of the game events, message events and init data: there are
//! no tags, the values follow each other bit by bit. An integer is read in chunks: within a byte
//! the bits are consumed from the least significant one upwards, the first chunk read is the most
//! significant part of the value.

use crate::error::Error;
use crate::peek::{dbg_peek_bits, peek_bits};
use crate::{BitInput, BitResult, Result};
use nom::bits::complete::take;

/// Takes n bits from the input bytes.
/// The bits are taken counting from right to left.
/// It returns the original input as if had not been processed.
/// The caller must call the normal take process afterwards.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
#[allow(clippy::let_and_return)]
pub fn rtake_n_bits(input: BitInput<'_>, count: usize) -> BitResult<'_, u8> {
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
pub fn take_n_bits_into_i64(input: BitInput<'_>, total_bits: usize) -> BitResult<'_, i64> {
    if total_bits > 64 {
        return Err(Error::BitPackedMoreThan64Bits(total_bits));
    }
    let mut res = 0i64;
    let mut remaining_bits = total_bits;
    let mut tail = input;
    loop {
        let count = if remaining_bits > 8 {
            // Try to byte-align
            if tail.1 != 0 { 8usize - tail.1 } else { 8usize }
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
pub fn take_bit_array(input: BitInput<'_>, total_bits: usize) -> BitResult<'_, Vec<u8>> {
    let mut res = vec![];
    // An empty string or blob has no chunk (the old decoder returned `[0]`, a NUL character).
    if total_bits == 0 {
        return Ok((input, res));
    }
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

/// Takes `total_bits` bits as Blizzard's `read_bits` does, in chunks up to the next byte
/// boundary, the first chunk being the most significant part, and returns the integer in big
/// endian bytes: the value of a [`crate::BitArray`]. For up to 64 bits it is the bytes of
/// [`take_n_bits_into_i64`]; [`take_bit_array`] differs when the input is not byte aligned.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn take_bits_be(input: BitInput<'_>, total_bits: usize) -> BitResult<'_, Vec<u8>> {
    let mut out = vec![0u8; total_bits.div_ceil(8)];
    let pad = out.len() * 8 - total_bits;
    let mut written = 0usize;
    let mut remaining_bits = total_bits;
    let mut tail = input;
    while remaining_bits > 0 {
        let count = if remaining_bits > 8 {
            if tail.1 != 0 { 8usize - tail.1 } else { 8usize }
        } else {
            remaining_bits
        };
        let (_, chunk) = rtake_n_bits(tail, count)?;
        let (new_tail, _) =
            dbg_peek_bits(take::<&[u8], u8, usize, _>(count), "take_bits_be")(tail)?;
        for k in 0..count {
            if (chunk >> (count - 1 - k)) & 1 == 1 {
                let pos = pad + written + k;
                out[pos / 8] |= 1 << (7 - pos % 8);
            }
        }
        written += count;
        remaining_bits -= count;
        tail = new_tail;
    }
    tracing::trace!("take_bits_be: {:?}", out);
    Ok((tail, out))
}

/// Takes the remainder of bits in the current input to align the input to a byte boundary.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn byte_align(input: BitInput<'_>) -> BitResult<'_, ()> {
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
pub fn take_unaligned_byte(input: BitInput<'_>) -> BitResult<'_, u8> {
    let (tail, res) = take_bit_array(input, 8usize)?;
    tracing::debug!("take_unaligned_byte: {:?}", res[0]);
    Ok((tail, res[0]))
}

/// Takes 4 unaligned bytes
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn take_fourcc(input: BitInput<'_>) -> BitResult<'_, Vec<u8>> {
    let (tail, res) = take_bit_array(input, 4usize * 8usize)?;
    tracing::debug!("take_fourcc: {:?}", res);
    Ok((tail, res))
}

/// Just a function that would be called when a null is needed, this is for debugging purposes
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn take_null(input: BitInput<'_>) -> BitResult<'_, ()> {
    tracing::debug!("take_null");
    Ok((input, ()))
}

/// Reads a packed int, In offset binary representation, (also called excess-K or biased).
/// a signed number is represented by the bit pattern corresponding to the unsigned number plus K,
/// with K being the biasing value or offset.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn parse_packed_int(input: BitInput<'_>, offset: i64, num_bits: usize) -> BitResult<'_, i64> {
    let (tail, num) = take_n_bits_into_i64(input, num_bits)?;
    // The 64 bits fields have the offset of i64::MIN, the sum wraps to the value.
    let res = offset.wrapping_add(num);
    tracing::debug!("parse_packed_int: {}", res);
    Ok((tail, res))
}

/// Reads a single bit and transforms into a bool.
#[tracing::instrument(level = "debug", skip(input), fields(input = peek_bits(input)))]
pub fn parse_bool(input: BitInput<'_>) -> BitResult<'_, bool> {
    let (_, val) = rtake_n_bits(input, 1usize)?;
    let (tail, _) = dbg_peek_bits(take::<&[u8], u8, usize, _>(1usize), "take_bit_for_bool")(input)?;
    tracing::debug!("parse_bool: {}", val != 0);
    Ok((tail, val != 0))
}

/// Writes the bit packed encoding, the inverse of the parsers above: the bits of a byte are
/// filled from the least significant one upwards and a value is written most significant chunk
/// first, so that [`parse_packed_int`] reads back what [`BitWriter::write_packed_int`] wrote.
#[derive(Debug, Default, Clone)]
pub struct BitWriter {
    bytes: Vec<u8>,
    /// The bits used in the last byte, 0 when the next bit starts a byte.
    bit_pos: usize,
}

impl BitWriter {
    pub fn new() -> Self {
        Self::default()
    }

    /// The bytes written, the last one padded with zero bits.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The bits written so far.
    pub fn bit_len(&self) -> usize {
        if self.bit_pos == 0 {
            self.bytes.len() * 8
        } else {
            (self.bytes.len() - 1) * 8 + self.bit_pos
        }
    }

    /// Writes the lowest `count` bits of `value`, at most 64.
    pub fn write_bits(&mut self, value: u64, count: usize) -> Result<()> {
        if count > 64 {
            return Err(Error::BitPackedMoreThan64Bits(count));
        }
        let mut remaining = count;
        while remaining > 0 {
            if self.bit_pos == 0 {
                self.bytes.push(0);
            }
            let chunk = remaining.min(8 - self.bit_pos);
            let mask = if chunk == 64 {
                u64::MAX
            } else {
                (1u64 << chunk) - 1
            };
            let bits = (value >> (remaining - chunk)) & mask;
            *self.bytes.last_mut().expect("a byte was pushed") |= (bits as u8) << self.bit_pos;
            self.bit_pos = (self.bit_pos + chunk) % 8;
            remaining -= chunk;
        }
        Ok(())
    }

    /// Writes `value - offset` in `num_bits` bits, the inverse of [`parse_packed_int`].
    pub fn write_packed_int(&mut self, value: i64, offset: i64, num_bits: usize) -> Result<()> {
        let unsigned = value as i128 - offset as i128;
        if unsigned < 0 || unsigned >= 1i128 << num_bits.min(64) {
            return Err(Error::ValueOutOfRange {
                value,
                offset,
                bits: num_bits,
            });
        }
        self.write_bits(unsigned as u64, num_bits)
    }

    /// Writes a single bit.
    pub fn write_bool(&mut self, value: bool) -> Result<()> {
        self.write_bits(value as u64, 1)
    }

    /// Pads the current byte with zero bits, the inverse of [`byte_align`].
    pub fn byte_align(&mut self) {
        self.bit_pos = 0;
    }

    /// Writes the chunks of a bit array, `total_bits` in all, the inverse of [`take_bit_array`].
    pub fn write_bit_array(&mut self, chunks: &[u8], total_bits: usize) -> Result<()> {
        let mut remaining = total_bits;
        for chunk in chunks {
            let count = remaining.min(8);
            self.write_bits(*chunk as u64, count)?;
            remaining -= count;
        }
        if remaining != 0 {
            return Err(Error::LengthMismatch {
                field: "bit array",
                len: chunks.len() * 8,
                expected: total_bits,
            });
        }
        Ok(())
    }

    /// Writes the big endian integer of `total_bits` bits, the inverse of [`take_bits_be`].
    pub fn write_bits_be(&mut self, value: &[u8], total_bits: usize) -> Result<()> {
        if value.len() != total_bits.div_ceil(8) {
            return Err(Error::LengthMismatch {
                field: "bit array",
                len: value.len() * 8,
                expected: total_bits,
            });
        }
        let pad = value.len() * 8 - total_bits;
        let mut done = 0usize;
        while done < total_bits {
            let count = (total_bits - done).min(8 - self.bit_pos);
            let mut chunk = 0u64;
            for k in 0..count {
                let pos = pad + done + k;
                chunk = (chunk << 1) | ((value[pos / 8] >> (7 - pos % 8)) & 1) as u64;
            }
            self.write_bits(chunk, count)?;
            done += count;
        }
        Ok(())
    }

    /// Writes a byte that may not be byte aligned, the inverse of [`take_unaligned_byte`].
    pub fn write_unaligned_byte(&mut self, value: u8) -> Result<()> {
        self.write_bits(value as u64, 8)
    }

    /// Writes the 4 bytes of a FourCC, the inverse of [`take_fourcc`].
    pub fn write_fourcc(&mut self, value: &[u8]) -> Result<()> {
        if value.len() != 4 {
            return Err(Error::LengthMismatch {
                field: "fourcc",
                len: value.len(),
                expected: 4,
            });
        }
        self.write_bit_array(value, 32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_writes_what_the_parsers_read() {
        let mut out = BitWriter::new();
        out.write_packed_int(5, 0, 3).unwrap();
        out.write_bool(true).unwrap();
        out.write_packed_int(1_656_011_340, 0, 32).unwrap();
        out.write_packed_int(-3, -5, 4).unwrap();
        out.byte_align();
        out.write_bit_array(&[0xab, 0b101], 11).unwrap();
        out.write_unaligned_byte(0x5a).unwrap();
        let bytes = out.into_bytes();
        let tail: BitInput = (&bytes, 0);
        let (tail, v) = parse_packed_int(tail, 0, 3).unwrap();
        assert_eq!(v, 5);
        let (tail, b) = parse_bool(tail).unwrap();
        assert!(b);
        let (tail, v) = parse_packed_int(tail, 0, 32).unwrap();
        assert_eq!(v, 1_656_011_340);
        let (tail, v) = parse_packed_int(tail, -5, 4).unwrap();
        assert_eq!(v, -3);
        let (tail, _) = byte_align(tail).unwrap();
        let (tail, bits) = take_bit_array(tail, 11).unwrap();
        assert_eq!(bits, vec![0xab, 0b101]);
        let (tail, byte) = take_unaligned_byte(tail).unwrap();
        assert_eq!(byte, 0x5a);
        // 59 bits: the last byte has 3 bits read.
        assert_eq!((tail.0.len(), tail.1), (1, 3));
    }

    #[test]
    fn it_reads_and_writes_the_bit_arrays_as_blizzard_does() {
        // Unaligned: the first chunk is the 3 bits left in the byte, then whole bytes.
        let data = [0b1010_0000u8, 0b1100_1011, 0b0000_0110];
        let (tail, as_int) = take_n_bits_into_i64((&data, 5), 14).unwrap();
        let (tail_be, as_bytes) = take_bits_be((&data, 5), 14).unwrap();
        assert_eq!(tail, tail_be);
        assert_eq!(crate::BitArray::new(14, as_bytes.clone()).to_i64(), as_int);
        assert_eq!(as_int, 0b10_1110_0101_1110);
        let mut out = BitWriter::new();
        out.write_bits(0, 5).unwrap();
        out.write_bits_be(&as_bytes, 14).unwrap();
        assert_eq!(&out.as_bytes()[..2], &data[..2]);
        assert_eq!(out.as_bytes()[2] & 0b111, data[2] & 0b111);
        // More than 64 bits.
        let long: Vec<u8> = (0..40u32).map(|i| (i * 7) as u8).collect();
        let (_, bits) = take_bits_be((&long, 3), 255).unwrap();
        let mut out = BitWriter::new();
        out.write_bits(0, 3).unwrap();
        out.write_bits_be(&bits, 255).unwrap();
        let (_, again) = take_bits_be((out.as_bytes(), 3), 255).unwrap();
        assert_eq!(again, bits);
    }

    #[test]
    fn it_writes_the_game_event_header_bytes() {
        // The bytes of bit_packed_decoder::tests::it_reads_game_events of s2protocol.
        let mut out = BitWriter::new();
        out.write_packed_int(0, 0, 2).unwrap(); // SVarUint32::MUint6
        out.write_packed_int(0, 0, 6).unwrap();
        out.write_packed_int(16, 0, 5).unwrap(); // user id
        out.write_packed_int(116, 0, 7).unwrap(); // the event id
        out.write_packed_int(1_656_011_340, 0, 32).unwrap();
        assert_eq!(
            out.as_bytes(),
            &[0x00, 0xf0, 0x64, 0x2b, 0x4b, 0xa4, 0x0c][..]
        );
    }

    #[test]
    fn it_refuses_the_values_that_do_not_fit() {
        let mut out = BitWriter::new();
        assert!(out.write_packed_int(8, 0, 3).is_err());
        assert!(out.write_packed_int(-1, 0, 3).is_err());
        assert!(out.write_packed_int(7, 0, 3).is_ok());
        assert!(out.write_bit_array(&[1, 2], 9).is_ok());
        assert!(out.write_bit_array(&[1], 9).is_err());
    }
}
