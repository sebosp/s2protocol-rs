//! Generates code that works for either ByteAligned (VersionedDecoder) or BitPacked encoded data.

use super::ProtoTypeConversion;

/// The data inside the different MPQ embedded files may be encoded/decoded in different ways.
/// The ByteAligned version basically reads at least one byte.
#[derive(Debug, Clone, Copy)]
pub enum DecoderType {
    /// Data is stored in 8-bit boundaries, a byte can be read and it must contain only one data
    /// element.
    ByteAligned,
    /// Data is stored bit-packed. a byte can be read that contains multiple data elements
    BitPacked,
}

impl DecoderType {
    /// In ByteAligned (VersionedDecoder) there is a tag that validates a struct followed by the
    /// number of fields stored for this struct, pressumably to account Optional fields.
    /// The ByteAligned variant takes an input of a byte slice.
    /// Furthur combinators will use `tail` instead of `input`.
    fn open_byte_aligned_gen_struct_main_parse_fn(proto_num: &str, name: &str) -> String {
        format!(
        "#[tracing::instrument(name=\"{proto_num}::byte_aligned::{name}::Parse\", level = \"debug\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {{\n\
             let (tail, _) = validate_struct_tag(input)?;\n\
             let (mut tail, struct_field_count) = parse_vlq_int(tail)?;\n\
         ",
    )
    }
    /// In BitPacked there is a *NO* tag that validates a struct and there are also no field
    /// counts.
    /// The BitPacked variant takes an input of a byte slice plus the positional index in the
    /// current byte.
    /// Furthur combinators will use `tail` instead of `input`. To make this re-usable with the
    /// ByteAligned version we just make `tail` point to `input`
    fn open_bit_packed_gen_struct_main_parse_fn(proto_num: &str, name: &str) -> String {
        format!(
        "#[tracing::instrument(name=\"{proto_num}::bit_packed::{name}::Parse\", level = \"debug\", skip(input), fields(peek = peek_bits(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {{\n\
             let tail = input;
         ",
    )
    }

    /// Opens the main parse function for the current DecoderType
    pub fn open_gen_struct_main_parse_fn(self, proto_num: &str, name: &str) -> String {
        match self {
            Self::ByteAligned => Self::open_byte_aligned_gen_struct_main_parse_fn(proto_num, name),
            Self::BitPacked => Self::open_bit_packed_gen_struct_main_parse_fn(proto_num, name),
        }
    }

    /// Closes the struct main parse function.
    pub fn close_gen_struct_main_parse_fn(self, proto_num: &str, name: &str) -> String {
        String::from("}")
    }

    /// Generates a ProtoTypeConversion for byte aligned units. This tries to re-use as much as
    /// possible compatible Rust types. NNet types can be imported later.
    fn byte_aligned_from_nnet_name(nnet_name: &str) -> ProtoTypeConversion {
        match nnet_name {
            "NNet.uint8"
            | "NNet.Replay.EReplayType"
            | "NNet.uint6"
            | "NNet.Game.TPlayerId"
            | "NNet.Replay.Tracker.TUIntMiniBits" => ProtoTypeConversion {
                rust_ty: "u8".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.uint32" | "NNet.uint14" | "NNet.uint22" => ProtoTypeConversion {
                rust_ty: "u32".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.int32" | "NNet.Game.TFixedBits" => ProtoTypeConversion {
                rust_ty: "i32".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.SVersion" => ProtoTypeConversion {
                rust_ty: "SVersion".to_string(),
                do_try_from: false,
                parser: "SVersion::parse".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.Game.TColorId" => ProtoTypeConversion {
                rust_ty: "i64".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "BlobType"
            | "NNet.Replay.CSignature"
            | "StringType"
            | "NNet.Replay.Tracker.CatalogName" => ProtoTypeConversion {
                rust_ty: "Vec<u8>".to_string(),
                do_try_from: false,
                parser: "tagged_blob".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "BoolType" => ProtoTypeConversion {
                rust_ty: "bool".to_string(),
                do_try_from: false,
                parser: "tagged_bool".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "OptionalType" => ProtoTypeConversion {
                // Leaves placeholders to be replaced later by the actual enclosed types
                rust_ty: "Option<{}>".to_string(),
                do_try_from: false,
                parser: "{}".to_string(),
                is_vec: false,
                is_optional: true,
            },
            "ArrayType" => ProtoTypeConversion {
                // Leaves placeholders to be replaced later by the actual enclosed types
                rust_ty: "Vec<{}>".to_string(),
                do_try_from: false,
                parser: "{}".to_string(),
                is_vec: true,
                is_optional: false,
            },
            "NNet.SMD5" => ProtoTypeConversion {
                rust_ty: "Smd5".to_string(),
                do_try_from: false,
                parser: "Smd5::parse".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.Replay.Tracker.SPlayerStats" => ProtoTypeConversion {
                rust_ty: "ReplayTrackerSPlayerStats".to_string(),
                do_try_from: false,
                parser: "ReplayTrackerSPlayerStats::parse".to_string(),
                is_vec: false,
                is_optional: false,
            },
            _ => panic!("Unsupported type: {}", nnet_name),
        }
    }

    fn bit_aligned_from_nnet_name(nnet_name: &str) -> ProtoTypeConversion {
        match nnet_name {
            "NNet.uint8"
            | "NNet.Replay.EReplayType"
            | "NNet.uint6"
            | "NNet.Game.TPlayerId"
            | "NNet.Replay.Tracker.TUIntMiniBits" => ProtoTypeConversion {
                rust_ty: "u8".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.uint32" | "NNet.uint14" | "NNet.uint22" => ProtoTypeConversion {
                rust_ty: "u32".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.int32" | "NNet.Game.TFixedBits" => ProtoTypeConversion {
                rust_ty: "i32".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.SVersion" => ProtoTypeConversion {
                rust_ty: "SVersion".to_string(),
                do_try_from: false,
                parser: "SVersion::parse".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.Game.TColorId" => ProtoTypeConversion {
                rust_ty: "i64".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "BlobType"
            | "NNet.Replay.CSignature"
            | "StringType"
            | "NNet.Replay.Tracker.CatalogName" => ProtoTypeConversion {
                rust_ty: "Vec<u8>".to_string(),
                do_try_from: false,
                parser: "tagged_blob".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "BoolType" => ProtoTypeConversion {
                rust_ty: "bool".to_string(),
                do_try_from: false,
                parser: "tagged_bool".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "OptionalType" => ProtoTypeConversion {
                // Leaves placeholders to be replaced later by the actual enclosed types
                rust_ty: "Option<{}>".to_string(),
                do_try_from: false,
                parser: "{}".to_string(),
                is_vec: false,
                is_optional: true,
            },
            "ArrayType" => ProtoTypeConversion {
                // Leaves placeholders to be replaced later by the actual enclosed types
                rust_ty: "Vec<{}>".to_string(),
                do_try_from: false,
                parser: "{}".to_string(),
                is_vec: true,
                is_optional: false,
            },
            "NNet.SMD5" => ProtoTypeConversion {
                rust_ty: "Smd5".to_string(),
                do_try_from: false,
                parser: "Smd5::parse".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.Replay.Tracker.SPlayerStats" => ProtoTypeConversion {
                rust_ty: "ReplayTrackerSPlayerStats".to_string(),
                do_try_from: false,
                parser: "ReplayTrackerSPlayerStats::parse".to_string(),
                is_vec: false,
                is_optional: false,
            },
            _ => panic!("Unsupported type: {}", nnet_name),
        }
    }
    /// Opens the main parse function for the current DecoderType
    pub fn from_nnet_name(self, nnet_name: &str) -> ProtoTypeConversion {
        match self {
            Self::ByteAligned => Self::byte_aligned_from_nnet_name(nnet_name),
            Self::BitPacked => Self::bit_packed_from_nnet_name(nnet_name),
        }
    }
}
