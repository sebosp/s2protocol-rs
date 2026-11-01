# serde_mpq

The two encodings of the sections of a StarCraft II replay, and derive macros that read and write
Rust types in them. It is the decoding layer of [s2protocol](https://crates.io/crates/s2protocol),
where the 285 types of protocol 87702 are plain structs and enums with `#[mpq(...)]` attributes.

- **Bit packed**, the game events, message events and init data: no tags, the values follow each
  other bit by bit and the width of every integer is a constant of the protocol.
- **Byte aligned**, Blizzard's *versioned* decoder, the header, details and tracker events: every
  value is preceded by the tag of its kind, the fields of a struct by their number, the integers
  are variable length.

```rust
use serde_mpq::{BitPackedDeserialize, BitPackedSerialize, BitWriter};
use serde_mpq::{MPQBitPackedDeserialize, MPQBitPackedSerialize};

#[derive(Debug, PartialEq, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
pub struct GameSPointMini {
    #[mpq(bits = 16, offset = 0)]
    pub x: i64,
    #[mpq(bits = 16, offset = 0)]
    pub y: i64,
}

let bytes = [0x34, 0x12, 0x78, 0x56];
let (tail, point) = GameSPointMini::parse((&bytes, 0)).unwrap();
let mut out = BitWriter::new();
point.serialize_bits(&mut out).unwrap();
assert_eq!(out.as_bytes(), &bytes);
```

## The declaration order is the wire format

In the bit packed encoding a struct is read field by field in the order they are declared, as
`deku`, `binrw`, `bincode` or `postcard` do: the struct *is* the description of the format.
Reordering, adding or removing a field changes what the derive reads and writes. There is no
schema hash or field position to maintain, on purpose: a value written by a previous version of a
type is migrated by reading it with the previous version and writing it with the new one. The byte
aligned encoding tags every field with `#[mpq(tag = n)]`, so there the declaration order does not
matter and a field missing from the input is an error, `None` for an `Option`, or its `Default`
with `#[mpq(default)]`.

## Attributes

On a struct or enum:

| attribute | meaning |
|---|---|
| `transparent` | a struct with one `value` field, read without the struct framing: the integer, string, array and bit array newtypes of the protocol |
| `choice, bits = n` | an enum whose variant is an `n` bits integer (bit packed) or a choice tag and a VLQ integer (byte aligned), followed by the payload of the variant |
| `int_enum, bits = n, offset = k` | an enum whose variant is an integer: `n` bits from `k` (bit packed), an int tag and a VLQ integer (byte aligned); the variants may carry payloads |

On a field, or a variant with `tag = n`:

| attribute | encoding | type | meaning |
|---|---|---|---|
| `tag = n` | byte aligned | any | the number of the field in the struct |
| `bits = n, offset = k` | bit packed | `i64` | an integer of `n` bits, stored minus `k` |
| `len_bits = n` | bit packed | `Vec<T>` | an array, its length in `n` bits, then the elements (`u8` elements are unaligned bytes) |
| `string, len_bits = n` | bit packed | `Vec<u8>` | Blizzard's blob: the length in `n` bits, a byte alignment, the bytes |
| `blob, bytes = n` | bit packed | `Vec<u8>` | a byte alignment and `n` bytes |
| `bitarray, len_bits = n` | bit packed | `BitArray` | the length in `n` bits, then the bits as the integer Blizzard's `read_bits` returns |
| `fourcc` | both | `Vec<u8>` / `u32` | four characters |
| `blob` | byte aligned | `Vec<u8>` | a blob with its tag and length |
| `default` | byte aligned | any | the `Default` when the field is missing |

`Option<T>` is a presence bit (bit packed) or a tagged presence byte (byte aligned), `bool` a
bit or a tagged byte, `()` nothing, and any other type one that has the derives. The integers of
the byte aligned encoding (`u8` to `i64`) need no attribute, they are VLQ.

## Debugging

The `Deserialize` derives generate a `parse` method and a `parse_<field>` method per field, each
with a `tracing` span at the `trace` level that shows the bits or bytes being read, and a
`debug` line with the value read: `RUST_LOG=trace` follows a replay field by field. The crate uses
`tracing` and expects it in the dependencies of the crate that derives.

## Size

The crate and its derive are about 2,200 lines, `serde_json` is about 18,400. With the derives the
protocol 87702 module of s2protocol went from 17,034 generated lines to 2,634 lines of types.
