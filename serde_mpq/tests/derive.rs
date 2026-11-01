//! The derives on types shaped like the ones of the protocol, in both directions.

use serde_mpq::*;

// ---------------------------------------------------------------------------- bit packed

/// An integer newtype, `Uint6` of the protocol.
#[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
#[mpq(transparent)]
pub struct Uint6 {
    #[mpq(bits = 6, offset = 0)]
    pub value: i64,
}

#[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
#[mpq(transparent)]
pub struct Uint32 {
    #[mpq(bits = 32, offset = 0)]
    pub value: i64,
}

/// A choice, `SVarUint32` of the protocol.
#[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
#[mpq(choice, bits = 2)]
pub enum SVarUint32 {
    #[mpq(tag = 0)]
    MUint6(Uint6),
    #[mpq(tag = 3)]
    MUint32(Uint32),
}

/// An int enum, `EObserve` of the protocol.
#[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
#[mpq(int_enum, bits = 2)]
pub enum EObserve {
    #[mpq(tag = 0)]
    ENone,
    #[mpq(tag = 1)]
    ESpectator,
    #[mpq(tag = 2)]
    EReferee,
}

/// A string newtype, `CUserName`.
#[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
#[mpq(transparent)]
pub struct CUserName {
    #[mpq(string, len_bits = 8)]
    pub value: Vec<u8>,
}

/// A blob newtype, `CCacheHandle`.
#[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
#[mpq(transparent)]
pub struct Blob4 {
    #[mpq(blob, bytes = 4)]
    pub value: Vec<u8>,
}

/// A bit array newtype, `CAllowedRaces`.
#[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
#[mpq(transparent)]
pub struct CAllowedRaces {
    #[mpq(bitarray, len_bits = 8)]
    pub value: BitArray,
}

/// An array newtype, `CUserInitialDataArray`.
#[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
#[mpq(transparent)]
pub struct Uint6Array {
    #[mpq(len_bits = 5)]
    pub value: Vec<Uint6>,
}

/// A struct with every kind of field, like `GameSCmdEvent`.
#[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
pub struct GameSCmdEvent {
    #[mpq(bits = 27, offset = 0)]
    pub m_cmd_flags: i64,
    pub m_abil: Option<Uint6>,
    pub m_data: SVarUint32,
    #[mpq(bits = 32, offset = 1)]
    pub m_sequence: i64,
    pub m_other_unit: Option<Uint32>,
    #[mpq(bits = 3, offset = -4)]
    pub m_signed: i64,
    pub m_flag: bool,
    #[mpq(len_bits = 4)]
    pub m_name: Vec<u8>,
    pub m_names: Uint6Array,
    pub m_observe: EObserve,
    pub m_races: CAllowedRaces,
    pub m_user: CUserName,
    pub m_blob: Blob4,
    pub m_none: Option<()>,
}

#[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
pub struct GameSStartGameEvent {}

#[test]
fn bit_packed_round_trip() {
    let event = GameSCmdEvent {
        m_cmd_flags: 0x4ab_cdef,
        m_abil: Some(Uint6 { value: 63 }),
        m_data: SVarUint32::MUint32(Uint32 {
            value: 1_656_011_340,
        }),
        m_sequence: 1 << 32,
        m_other_unit: None,
        m_signed: -1,
        m_flag: true,
        m_name: b"sc2".to_vec(),
        m_names: Uint6Array {
            value: vec![Uint6 { value: 1 }, Uint6 { value: 2 }, Uint6 { value: 3 }],
        },
        m_observe: EObserve::EReferee,
        m_races: CAllowedRaces {
            value: BitArray::new(3, vec![0b101]),
        },
        m_user: CUserName {
            value: b"Sazed".to_vec(),
        },
        m_blob: Blob4 {
            value: vec![1, 2, 3, 4],
        },
        m_none: Some(()),
    };
    let mut out = BitWriter::new();
    event.serialize_bits(&mut out).unwrap();
    let bytes = out.into_bytes();
    let (tail, read) = GameSCmdEvent::parse((&bytes, 0)).unwrap();
    assert_eq!(read, event);
    assert!(tail.0.len() <= 1, "{tail:?}");
    assert_eq!(read.m_races.value.to_i64(), 5);

    // The per field methods exist and read the same values.
    let (_, flags) = GameSCmdEvent::parse_m_cmd_flags((&bytes, 0)).unwrap();
    assert_eq!(flags, 0x4ab_cdef);
    let (_, empty) = GameSStartGameEvent::parse((&bytes, 3)).unwrap();
    assert_eq!(empty, GameSStartGameEvent {});
}

#[test]
fn bit_packed_matches_the_replay_bytes() {
    // The game event header of s2protocol's it_reads_game_events.
    let data: Vec<u8> = vec![0x00, 0xf0, 0x64, 0x2b, 0x4b, 0xa4, 0x0c, 0x00];
    let (tail, delta) = SVarUint32::parse((&data, 0)).unwrap();
    assert_eq!(delta, SVarUint32::MUint6(Uint6 { value: 0 }));
    assert_eq!(tail, (&data[1..], 0));
    let mut out = BitWriter::new();
    delta.serialize_bits(&mut out).unwrap();
    assert_eq!(out.as_bytes(), &data[..1]);
}

#[test]
fn bit_packed_errors() {
    let data = [0xffu8, 0xff];
    assert!(matches!(
        EObserve::parse((&data, 0)),
        Err(Error::UnknownTag(3))
    ));
    let mut out = BitWriter::new();
    assert!(matches!(
        Uint6 { value: 64 }.serialize_bits(&mut out),
        Err(Error::ValueOutOfRange { .. })
    ));
    assert!(matches!(
        Blob4 { value: vec![1] }.serialize_bits(&mut out),
        Err(Error::LengthMismatch { .. })
    ));
}

// -------------------------------------------------------------------------- byte aligned

/// `SVersion` of the header.
#[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
pub struct SVersion {
    #[mpq(tag = 0)]
    pub m_flags: u8,
    #[mpq(tag = 1)]
    pub m_major: u8,
    #[mpq(tag = 2)]
    pub m_minor: u8,
    #[mpq(tag = 3)]
    pub m_revision: u8,
    #[mpq(tag = 4)]
    pub m_build: u32,
    #[mpq(tag = 5)]
    pub m_base_build: u32,
}

#[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
#[mpq(int_enum)]
pub enum EObserveBytes {
    #[mpq(tag = 0)]
    ENone,
    #[mpq(tag = 1)]
    ESpectator,
}

#[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
#[mpq(choice)]
pub enum SVarUint32Bytes {
    #[mpq(tag = 0)]
    MUint6(u8),
    #[mpq(tag = 3)]
    MUint32(u32),
}

/// `GameSDetails`-like, with optionals, arrays, blobs and a default.
#[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
pub struct Details {
    #[mpq(tag = 0)]
    pub m_version: SVersion,
    #[mpq(tag = 1, blob)]
    pub m_title: Vec<u8>,
    #[mpq(tag = 2)]
    pub m_is_blizzard_map: bool,
    #[mpq(tag = 3)]
    pub m_time_utc: i64,
    #[mpq(tag = 4)]
    pub m_working_set_slot_id: Option<u8>,
    #[mpq(tag = 5, blob)]
    pub m_cache_handles: Option<Vec<Vec<u8>>>,
    #[mpq(tag = 6)]
    pub m_items: Vec<i32>,
    #[mpq(tag = 7, fourcc)]
    pub m_program_id: u32,
    #[mpq(tag = 8)]
    pub m_observe: EObserveBytes,
    #[mpq(tag = 9)]
    pub m_delta: SVarUint32Bytes,
    #[mpq(tag = 10, blob, default)]
    pub m_name: Vec<u8>,
    #[mpq(tag = 11)]
    pub m_data_deprecated: Option<Vec<u8>>,
}

#[test]
fn byte_aligned_round_trip() {
    let details = Details {
        m_version: SVersion {
            m_flags: 1,
            m_major: 5,
            m_minor: 0,
            m_revision: 14,
            m_build: 94137,
            m_base_build: 87702,
        },
        m_title: b"Fields of Death".to_vec(),
        m_is_blizzard_map: true,
        m_time_utc: 133_000_000_000_000_000,
        m_working_set_slot_id: Some(3),
        m_cache_handles: Some(vec![b"s2ma".to_vec(), vec![]]),
        m_items: vec![-5, 0, 1 << 20],
        m_program_id: 0x5332_0000,
        m_observe: EObserveBytes::ESpectator,
        m_delta: SVarUint32Bytes::MUint32(70_000),
        m_name: b"Sazed".to_vec(),
        m_data_deprecated: None,
    };
    let mut out = ByteWriter::new();
    details.serialize_bytes(&mut out).unwrap();
    let bytes = out.into_bytes();
    let (tail, read) = Details::parse(&bytes).unwrap();
    assert_eq!(read, details);
    assert!(tail.is_empty());
    let (_, version) = Details::parse_m_version(&bytes[3..]).unwrap();
    assert_eq!(version, details.m_version);
}

#[test]
fn byte_aligned_reads_the_fields_in_any_order_and_the_defaults() {
    // The struct header of SVersion with its fields in reverse, written by hand.
    let mut out = ByteWriter::new();
    out.write_struct_header(6);
    for (tag, value) in [(5, 87702), (4, 94137), (3, 14), (2, 0), (1, 5), (0, 1)] {
        out.write_field_tag(tag);
        out.write_tagged_int(value);
    }
    let (_, version) = SVersion::parse(out.as_bytes()).unwrap();
    assert_eq!(version.m_base_build, 87702);
    assert_eq!(version.m_flags, 1);

    // A missing field with a default, and a missing optional.
    let mut out = ByteWriter::new();
    out.write_struct_header(1);
    out.write_field_tag(0);
    out.write_tagged_int(7);
    #[derive(Debug, PartialEq, MPQByteAlignedDeserialize)]
    struct Partial {
        #[mpq(tag = 0)]
        pub a: u8,
        #[mpq(tag = 1, blob, default)]
        pub name: Vec<u8>,
        #[mpq(tag = 2)]
        pub maybe: Option<bool>,
    }
    let (_, partial) = Partial::parse(out.as_bytes()).unwrap();
    assert_eq!(
        partial,
        Partial {
            a: 7,
            name: vec![],
            maybe: None
        }
    );

    // A missing required field, a duplicate and an unknown tag.
    assert!(
        matches!(SVersion::parse(out.as_bytes()), Err(Error::MissingField(name)) if name == "m_major")
    );
    let mut out = ByteWriter::new();
    out.write_struct_header(2);
    out.write_field_tag(0);
    out.write_tagged_int(7);
    out.write_field_tag(0);
    out.write_tagged_int(7);
    assert!(
        matches!(Partial::parse(out.as_bytes()), Err(Error::DuplicateTag(name, 0)) if name == "a")
    );
    let mut out = ByteWriter::new();
    out.write_struct_header(1);
    out.write_field_tag(9);
    assert!(matches!(
        Partial::parse(out.as_bytes()),
        Err(Error::UnknownTag(9))
    ));
}

#[test]
fn byte_aligned_matches_the_header_bytes() {
    // The start of a replay header: struct tag, 2 fields, then field 0: a blob.
    let mut out = ByteWriter::new();
    out.write_struct_header(2);
    out.write_field_tag(0);
    out.write_tagged_blob(b"StarCraft II replay\x1b11");
    assert_eq!(&out.as_bytes()[..6], &[0x05, 0x04, 0x00, 0x02, 0x2c, b'S']);
}
