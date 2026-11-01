//! A bit array of the bit packed encoding: its length in bits precedes it in the input and the
//! bits are an integer, read as Blizzard's `read_bits` does. The length is kept with the bits for
//! the value to be written back the same.

/// The bits of a bit array as the integer Blizzard's decoder reads, in big endian bytes: the
/// first byte holds the most significant bits, the length is not always a multiple of 8 so the
/// first byte may start with padding zeros. [`BitArray::bit`] indexes the bits from the least
/// significant one, as the selection masks of the game events are read: unit `i` is bit `i`.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BitArray {
    /// The number of bits.
    pub bits: usize,
    /// The integer, big endian, `bits.div_ceil(8)` bytes.
    pub value: Vec<u8>,
}

impl BitArray {
    pub fn new(bits: usize, value: Vec<u8>) -> Self {
        debug_assert_eq!(value.len(), bits.div_ceil(8), "a bit array of {bits} bits");
        Self { bits, value }
    }

    /// The integer, its lowest 64 bits when there are more.
    pub fn to_i64(&self) -> i64 {
        self.value
            .iter()
            .fold(0i64, |acc, byte| (acc << 8) | *byte as i64)
    }

    /// Whether the bit at `index`, from the least significant one, is set.
    pub fn bit(&self, index: usize) -> bool {
        if index >= self.bits {
            return false;
        }
        let pos = self.value.len() * 8 - 1 - index;
        (self.value[pos / 8] >> (7 - pos % 8)) & 1 == 1
    }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_bits_as_an_integer() {
        assert_eq!(BitArray::new(3, vec![0b101]).to_i64(), 5);
        assert_eq!(BitArray::new(12, vec![0x0f, 0xfa]).to_i64(), 0xffa);
        assert_eq!(BitArray::new(16, vec![0xfe, 0xff]).to_i64(), 65279);
        assert_eq!(BitArray::new(0, vec![]).to_i64(), 0);
    }

    #[test]
    fn it_indexes_the_bits_from_the_least_significant_one() {
        // 10 units, the units 0 and 9 are set.
        let bits = BitArray::new(10, vec![0b10, 0b0000_0001]);
        assert!(bits.bit(0));
        assert!(!bits.bit(1));
        assert!(!bits.bit(8));
        assert!(bits.bit(9));
        assert!(!bits.bit(10));
        let bits = BitArray::new(3, vec![0b010]);
        assert_eq!(
            (bits.bit(0), bits.bit(1), bits.bit(2)),
            (false, true, false)
        );
    }
}
