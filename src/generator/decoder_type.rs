//! Generates code that works for either ByteAligned (VersionedDecoder) or BitPacked encoded data.

use std::collections::HashMap;

use super::proto_morphist::{proto_nnet_name_to_rust_name, str_nnet_name_to_rust_name};
use super::ProtoTypeConversion;
use convert_case::{Case, Casing};
use serde_json::Value;

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
    /// Further combinators will use `tail` instead of `input`.
    fn open_byte_aligned_struct_main_parse_fn(proto_num: u64, name: &str) -> String {
        format!(
        "#[tracing::instrument(name=\"{proto_num}::byte_aligned::{name}::Parse\", level = \"trace\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {{\n\
             let (tail, _) = validate_struct_tag(input)?;\n\
             let (mut tail, struct_field_count) = parse_vlq_int(tail)?;\n\
         ",
    )
    }

    /// Closes the struct main parse function.
    fn close_byte_aligned_struct_main_parse_fn() -> String {
        format!("}}")
    }

    /// In BitPacked there is a *NO* tag that validates a struct and there are also no field
    /// counts.
    /// The BitPacked variant takes an input of a byte slice plus the positional index in the
    /// current byte.
    /// Furthur combinators will use `tail` instead of `input`. To make this re-usable with the
    /// ByteAligned version we just make `tail` point to `input`
    fn open_bit_packed_struct_main_parse_fn(
        proto_num: u64,
        name: &str,
        not_const_num_fields: usize,
    ) -> String {
        // If there are fields, then tail doesn't need to be mutable, if we set it to mutable we
        // get compiler warnings;
        let tail_str = if not_const_num_fields != 0 {
            "mut tail"
        } else {
            "tail"
        };
        format!(
        "#[tracing::instrument(name=\"{proto_num}::bit_packed::{name}::Parse\", level = \"trace\", skip(input), fields(peek = peek_bits(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {{\n\
             let {tail_str} = input;
         ",
    )
    }

    /// Closes the struct main parse function.
    fn close_bit_packed_struct_main_parse_fn() -> String {
        format!("}}")
    }

    /// Opens the main parse function for the current DecoderType
    pub fn open_struct_main_parse_fn(
        self,
        proto_num: u64,
        name: &str,
        not_const_num_fields: usize,
    ) -> String {
        match self {
            Self::ByteAligned => Self::open_byte_aligned_struct_main_parse_fn(proto_num, name),
            Self::BitPacked => {
                Self::open_bit_packed_struct_main_parse_fn(proto_num, name, not_const_num_fields)
            }
        }
    }

    /// Opens the main parse function for the current DecoderType
    pub fn close_struct_main_parse_fn(self) -> String {
        match self {
            Self::ByteAligned => Self::close_byte_aligned_struct_main_parse_fn(),
            Self::BitPacked => Self::close_bit_packed_struct_main_parse_fn(),
        }
    }

    /// Generates a ProtoTypeConversion for byte aligned units. This tries to re-use as much as
    /// possible compatible Rust types. NNet types can be imported later.
    #[tracing::instrument(level = "debug")]
    fn byte_aligned_from_nnet_name(nnet_name: &str) -> ProtoTypeConversion {
        match nnet_name {
            "NNet.uint8"
            | "NNet.Replay.EReplayType"
            | "NNet.uint6"
            | "NNet.Game.TPlayerId"
            | "NNet.Game.TControlId"
            | "NNet.Game.TTeamId"
            | "NNet.Replay.Tracker.TUIntMiniBits" => ProtoTypeConversion {
                rust_ty: "u8".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                ..Default::default()
            },
            "NNet.uint32"
            | "NNet.uint14"
            | "NNet.uint22"
            | "NNet.Game.TDifficulty"
            | "NNet.Game.THandicap" => ProtoTypeConversion {
                rust_ty: "u32".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                ..Default::default()
            },
            "NNet.int32" | "NNet.Game.TFixedBits" => ProtoTypeConversion {
                rust_ty: "i32".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                ..Default::default()
            },
            "NNet.SVersion" => ProtoTypeConversion {
                rust_ty: "SVersion".to_string(),
                parser: "SVersion::parse".to_string(),
                ..Default::default()
            },
            "NNet.Game.TColorId" | "NNet.int64" => ProtoTypeConversion {
                rust_ty: "i64".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                ..Default::default()
            },
            "NNet.uint64" => ProtoTypeConversion {
                rust_ty: "u64".to_string(),
                do_try_from: true,
                parser: "tagged_vlq_int".to_string(),
                ..Default::default()
            },
            "FourCCType" => ProtoTypeConversion {
                rust_ty: "u32".to_string(),
                parser: "tagged_fourcc".to_string(),
                ..Default::default()
            },
            "BlobType"
            | "NNet.Replay.CSignature"
            | "StringType"
            | "NNet.Replay.Tracker.CatalogName"
            | "NNet.Game.CCacheHandle"
            | "NNet.CFilePath"
            | "NNet.CUserName" => ProtoTypeConversion {
                rust_ty: "Vec<u8>".to_string(),
                parser: "tagged_blob".to_string(),
                ..Default::default()
            },
            "BoolType" => ProtoTypeConversion {
                rust_ty: "bool".to_string(),
                parser: "tagged_bool".to_string(),
                ..Default::default()
            },
            "OptionalType" => ProtoTypeConversion {
                // Leaves placeholders to be replaced later by the actual enclosed types
                rust_ty: "Option<{}>".to_string(),
                parser: "{}".to_string(),
                is_optional: true,
                ..Default::default()
            },
            "ArrayType"
            | "DynArrayType"
            | "NNet.CUserArchiveDataArray"
            | "NNet.CUserInitialDataArray" => ProtoTypeConversion {
                // Leaves placeholders to be replaced later by the actual enclosed types
                rust_ty: "Vec<{}>".to_string(),
                parser: "{}".to_string(),
                is_vec: true,
                ..Default::default()
            },
            "NNet.SMD5" => ProtoTypeConversion {
                rust_ty: "Smd5".to_string(),
                parser: "Smd5::parse".to_string(),
                ..Default::default()
            },
            "NNet.EObserve" => ProtoTypeConversion {
                rust_ty: "EObserve".to_string(),
                parser: "EObserve::parse".to_string(),
                ..Default::default()
            },
            "NNet.Game.EResultDetails" => ProtoTypeConversion {
                rust_ty: "GameEResultDetails".to_string(),
                parser: "GameEResultDetails::parse".to_string(),
                ..Default::default()
            },
            "NNet.Game.CCacheHandles" => ProtoTypeConversion {
                rust_ty: "Vec<Vec<u8>>".to_string(),
                parser: "tagged_blob".to_string(),
                is_vec: true,
                ..Default::default()
            },
            "NNet.Replay.Tracker.SPlayerStats" => ProtoTypeConversion {
                rust_ty: "ReplayTrackerSPlayerStats".to_string(),
                parser: "ReplayTrackerSPlayerStats::parse".to_string(),
                ..Default::default()
            },
            "NNet.Game.CPlayerDetailsArray" => ProtoTypeConversion {
                rust_ty: "Vec<GameSPlayerDetails>".to_string(),
                parser: "GameSPlayerDetails::parse".to_string(),
                is_vec: true,
                ..Default::default()
            },
            "NNet.Game.SThumbnail" => ProtoTypeConversion {
                rust_ty: "GameSThumbnail".to_string(),
                parser: "GameSThumbnail::parse".to_string(),
                ..Default::default()
            },
            "NNet.Game.CModPaths" => ProtoTypeConversion {
                rust_ty: "Vec<Vec<u8>>".to_string(),
                parser: "tagged_blob".to_string(),
                is_vec: true,
                ..Default::default()
            },
            "NNet.Game.EGameSpeed" => ProtoTypeConversion {
                rust_ty: "GameEGameSpeed".to_string(),
                parser: "GameEGameSpeed::parse".to_string(),
                ..Default::default()
            },
            "NNet.Game.SToonNameDetails" => ProtoTypeConversion {
                rust_ty: "GameSToonNameDetails".to_string(),
                parser: "GameSToonNameDetails::parse".to_string(),
                ..Default::default()
            },
            "NNet.Game.SColor" => ProtoTypeConversion {
                rust_ty: "GameSColor".to_string(),
                parser: "GameSColor::parse".to_string(),
                ..Default::default()
            },
            _ => panic!("Unsupported type: {}", nnet_name),
        }
    }

    /// A coversion table with compatible Rust types to Protocol Types for the BitPacked variant.
    #[tracing::instrument(level = "debug")]
    fn bit_packed_from_nnet_name(nnet_name: &str) -> ProtoTypeConversion {
        if nnet_name.starts_with("NNet") {
            let proto_unit_type_name = str_nnet_name_to_rust_name(nnet_name.to_string());
            return ProtoTypeConversion {
                rust_ty: proto_unit_type_name.clone(),
                parser: format!("{}::parse", proto_unit_type_name),
                ..Default::default()
            };
        }
        match nnet_name {
            "BoolType" => ProtoTypeConversion {
                rust_ty: "bool".to_string(),
                parser: "parse_bool".to_string(),
                ..Default::default()
            },
            "OptionalType" => ProtoTypeConversion {
                // Leaves placeholders to be replaced later by the actual enclosed types
                rust_ty: "Option<{}>".to_string(),
                parser: "{}".to_string(),
                is_optional: true,
                ..Default::default()
            },
            "ArrayType" | "DynArrayType" => ProtoTypeConversion {
                // Leaves placeholders to be replaced later by the actual enclosed types
                rust_ty: "Vec<{}>".to_string(),
                parser: "{}".to_string(),
                is_vec: true,
                ..Default::default()
            },
            "BlobType" | "BitArrayType" | "AsciiStringType" | "StringType" => ProtoTypeConversion {
                rust_ty: "Vec<u8>".to_string(),
                parser: "take_unaligned_byte".to_string(),
                is_vec: true,
                ..Default::default()
            },
            "IntType" => ProtoTypeConversion {
                rust_ty: "i64".to_string(),
                parser: "parse_packed_int({})".to_string(),
                is_sized_int: true,
                ..Default::default()
            },
            "FourCCType" => ProtoTypeConversion {
                // The blob type is byte aligned
                rust_ty: "Vec<u8>".to_string(),
                parser: "take_fourcc".to_string(),
                ..Default::default()
            },
            "NullType" => ProtoTypeConversion {
                // The blob type is byte aligned
                rust_ty: "()".to_string(),
                parser: "take_null".to_string(),
                ..Default::default()
            },
            _ => panic!("Unsupported type: {}", nnet_name),
        }
    }

    /// Creates a ProtoTypeConversion conversion table entry
    pub fn from_nnet_name(self, nnet_name: &str) -> ProtoTypeConversion {
        match self {
            Self::ByteAligned => Self::byte_aligned_from_nnet_name(nnet_name),
            Self::BitPacked => Self::bit_packed_from_nnet_name(nnet_name),
        }
    }

    #[tracing::instrument(level = "debug")]
    pub fn open_byte_aligned_choice_main_parse_fn(
        proto_num: u64,
        name: &str,
        _num_fields: usize,
    ) -> String {
        format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::ChoiceType::parse\", level = \"trace\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {{\n\
         let (tail, _) = validate_choice_tag(input)?;\n\
         let (tail, variant_tag) = parse_vlq_int(tail)?;\n\
         ",
    )
    }

    #[tracing::instrument(level = "debug")]
    pub fn close_byte_aligned_choice_main_parse_fn() -> String {
        format!("}}")
    }

    #[tracing::instrument(level = "debug")]
    pub fn open_bit_packed_choice_main_parse_fn(
        proto_num: u64,
        name: &str,
        num_fields: usize,
    ) -> String {
        let num_bits = (num_fields as f32).log2().ceil() as usize;
        format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::ChoiceType::parse\", level = \"trace\", skip(input), fields(peek = peek_bits(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {{\n\
            // ChoiceType:
            // Use the number of elements in the json .fields to calculate how many
            // bits to have unique tags.
            // let num_fields: usize = {num_fields};\n\
            let offset = 0i64;\n\
            let num_bits: usize = {num_bits};\n\
            let (tail, variant_tag) = parse_packed_int(input, offset, num_bits)?;\n\
         ",
    )
    }
    #[tracing::instrument(level = "debug")]
    pub fn close_bit_packed_choice_main_parse_fn() -> String {
        format!("}}")
    }

    /// Generates the main parse function for a byte aligned choice type
    #[tracing::instrument(level = "debug")]
    pub fn open_choice_main_parse_fn(
        &self,
        proto_num: u64,
        name: &str,
        num_fields: usize,
    ) -> String {
        let mut res = match self {
            Self::ByteAligned => {
                Self::open_byte_aligned_choice_main_parse_fn(proto_num, name, num_fields)
            }
            Self::BitPacked => {
                Self::open_bit_packed_choice_main_parse_fn(proto_num, name, num_fields)
            }
        };
        res.push_str("match variant_tag {");
        res
    }
    /// Generates the main parse function for a byte aligned choice type
    #[tracing::instrument(level = "debug")]
    pub fn close_choice_main_parse_fn(&self) -> String {
        match self {
            Self::ByteAligned => Self::close_byte_aligned_choice_main_parse_fn(),
            Self::BitPacked => Self::close_bit_packed_choice_main_parse_fn(),
        }
    }

    #[tracing::instrument(
        level = "debug",
        skip(proto_mod, proto_type_def, struct_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_proto_struct_code(
        &self,
        proto_mod: &Value,
        proto_type_def: &mut String,
        struct_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        match self {
            Self::ByteAligned => Self::gen_byte_aligned_proto_struct_code(
                proto_mod,
                proto_type_def,
                struct_parse_impl_def,
                type_impl_def,
            ),
            Self::BitPacked => Self::gen_bit_packed_proto_struct_code(
                proto_mod,
                proto_type_def,
                struct_parse_impl_def,
                type_impl_def,
            ),
        }
    }

    /// Generates a Rust Struct code with fields and parsing methods per field for Byte Aligned
    /// encoding
    #[tracing::instrument(
        level = "debug",
        skip(proto_mod, proto_type_def, struct_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_byte_aligned_proto_struct_code(
        proto_mod: &Value,
        proto_type_def: &mut String,
        mut struct_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        let decoder = DecoderType::ByteAligned;
        //output.write_all(format!("\n/*{:#}*/\n", proto_mod).as_bytes())?;
        let field_array = proto_mod["type_info"]["fields"].as_array().unwrap();
        let has_tags = if field_array.len() == 1 && field_array[0]["tag"] == Value::Null {
            false
        } else {
            true
        };
        let mut struct_parse_return = String::from("Ok((tail, Self {");
        // Structs are prepend with the number of fields that follow, pressumably to account for
        // Optionals
        let mut struct_parse_fields = if has_tags {
            format!(
                "for i in 0..(struct_field_count as usize) {{\n\
                 let (new_tail, field_tag) = parse_vlq_int(tail)?;\n\
                 tail = new_tail;\n\
                 match field_tag {{\n\
                 "
            )
        } else {
            // If there are no tags, then it's just one single field inside the struct.
            String::from("")
        };
        // The first ConstDecl is the main "tag", which is parsed by the main enum of the type.
        // After this ConstDecl, other ConstDecl's may appear, but these are hardcoded in the
        // protocol and do not appear in the data itself
        for field in field_array {
            let nnet_field_type = field["type"].as_str().unwrap();
            let proto_field_type_info = match field["type_info"]["fullname"].as_str() {
                Some(val) => val,
                None => {
                    // Fallback to the `type_info.type' field
                    field["type_info"]["type"].as_str().unwrap()
                }
            };
            if nnet_field_type == "ConstDecl" {
                tracing::info!("Skipping ConstDecl for {proto_field_type_info}");
                continue;
            }
            if nnet_field_type != "MemberStructField" {
                panic!("Unknown {proto_field_type_info} field type: {nnet_field_type}");
            }
            let field_name = field["name"].as_str().unwrap().to_case(Case::Snake);
            proto_type_def.push_str(&format!("    pub {field_name}: ",));
            let mut morph = decoder.from_nnet_name(proto_field_type_info);
            // If the type is Optional, we need to find the internal parser in case the field is
            // provided.
            if proto_field_type_info == "OptionalType" {
                // We should wrap this in Option<T>, but, it may also be a Option<Vec<T>>
                // A Vec<Option<T>> has *NOT* been observed, otherwise we would need some complex
                // structure that can be arbirtrary wrapping of types and maybe we should then just
                // create the serde for the whole protocol file and from there generate the code....
                // Maybe that's step 2.
                let enclosed_morph = if field["type_info"]["type_info"]["type"] == "ArrayType" {
                    // The enclosed type is wrapped in an additional .element_type field.
                    let element_type = field["type_info"]["type_info"]["element_type"]["fullname"]
                        .as_str()
                        .expect("Field should have .type_info.type_info.element_type.fullname");
                    tracing::info!("Optional ArrayType Element Type: {}", element_type);
                    let mut internal_morph = decoder.from_nnet_name(element_type);
                    internal_morph.rust_ty = format!("Vec<{}>", internal_morph.rust_ty);
                    // The intenal type, i.e. u8, needs to now be marked both as Vec<> and as Option<>
                    internal_morph.is_vec = true;
                    internal_morph.is_optional = true;
                    internal_morph
                } else {
                    // The enclosed type is wrapped in an additional .type_info field.
                    tracing::info!("OptionalType field: {:?}", field);
                    let enclosed_type = match field["type_info"]["type_info"]["fullname"]
                        .as_str()
                        {
                            Some(val) => val,
                            None => {
                                field["type_info"]["type_info"]["type"].as_str()
                        .expect("Field should contain .type_info.type_info.fullname or .type_info.type_info.type")
                            }
                        };
                    decoder.from_nnet_name(enclosed_type)
                };
                morph.rust_ty = morph.rust_ty.replace("{}", &enclosed_morph.rust_ty);
                morph.parser = morph.parser.replace("{}", &enclosed_morph.parser);
                morph.do_try_from = enclosed_morph.do_try_from;
                morph.is_vec = enclosed_morph.is_vec;
                morph.is_optional = true;
            } else if proto_field_type_info == "ArrayType" {
                // The enclosed type is wrapped in an additional .element_type field.
                let element_type = field["type_info"]["element_type"]["fullname"]
                    .as_str()
                    .expect("Field should have .type_info.element_type.fullname");
                tracing::info!("ArrayType Element Type: {}", element_type);
                let enclosed_morph = decoder.from_nnet_name(element_type);
                morph.rust_ty = morph.rust_ty.replace("{}", &enclosed_morph.rust_ty);
                morph.parser = morph.parser.replace("{}", &enclosed_morph.parser);
                morph.do_try_from = enclosed_morph.do_try_from;
                morph.is_vec = true;
            }
            let field_type = &morph.rust_ty;
            let field_value_parser = &morph.parser;
            proto_type_def.push_str(&format!("{field_type},\n"));
            if morph.is_optional {
                // This the field is optional, premark it as "Some(x)", because we will unwrap later
                // and if the field is None it means it was a required field that wasn't provided.
                struct_parse_impl_def.push_str(&format!(
                    "let mut {field_name}: Option<{field_type}> = Some(None);\n"
                ));
            } else {
                struct_parse_impl_def.push_str(&format!(
                    "let mut {field_name}: Option<{field_type}> = None;\n"
                ));
            }
            if has_tags {
                let proto_field_tag = field["tag"]["value"].as_str().unwrap();
                assert!(
                    field["tag"]["type"]
                        .as_str()
                        .expect("Field should have .tag.type")
                        == String::from("IntLiteral")
                );
                struct_parse_fields.push_str(&format!(
                    " {proto_field_tag} => {{\n\
                       tracing::debug!(\"Field [{{i}}]: tagged '{proto_field_tag}' for field {field_name}\");\n"));
            }
            if morph.is_optional {
                // The OptionalType is a Some() since we will later unwrap and fail if a field is not
                // provided so we "pre-populate" it to empty.
                struct_parse_fields.push_str(&format!(" if let Some(None) = {field_name} "));
            } else {
                struct_parse_fields.push_str(&format!(" if {field_name}.is_none()"));
            }
            if has_tags {
                let proto_field_tag = field["tag"]["value"].as_str().unwrap();
                struct_parse_fields.push_str(&format!(
                    " {{\n\
                            let (new_tail, parsed_{field_name}) = Self::parse_{field_name}(tail)?;\n\
                            tail = new_tail;\n\
                            {field_name} = Some(parsed_{field_name});\n\
                            continue;\n\
                        }} else {{\n\
                            tracing::error!(\"Field {field_name} with tag {proto_field_tag} was already provided\");\n\
                            return Err(S2ProtocolError::DuplicateTag(String::from(\"{field_name}\"), proto_field_tag));\n\
                        }}\n\
                    }},\n"
                ));
            } else {
                struct_parse_fields.push_str(&format!(
                    " {{\n\
                     let (new_tail, parsed_{field_name}) = Self::parse_{field_name}(tail)?;\n\
                     tail = new_tail;\n\
                     {field_name} = Some(parsed_{field_name});\n\
                     }}\n\
                     "
                ));
            }
            struct_parse_return.push_str(&format!(
                "{field_name}: ok_or_return_missing_field_err!({field_name}),\n"
            ));
            // Create a parsing function
            type_impl_def.push_str(&format!(
                    "#[tracing::instrument(level = \"trace\", skip(input), fields(peek = peek_hex(input)))]\n\
                     pub fn parse_{field_name}(input: &[u8]) -> S2ProtoResult<&[u8], {field_type}> {{\n\
                    "));
            if morph.is_optional {
                type_impl_def.push_str("let (tail, _) = validate_opt_tag(input)?;\n");
                // If the next byte is a filled with zeros, then the field is None
                type_impl_def
                    .push_str("let (tail, is_provided) = nom::number::complete::u8(tail)?;\n");
                type_impl_def.push_str(&format!(
                    "let (tail, {field_name}) = if is_provided != 0 {{\n"
                ));
                if morph.is_vec {
                    type_impl_def.push_str(&format!(
                    "let (tail, _) = validate_array_tag(tail)?;\n\
                     let (tail, array_length) = parse_vlq_int(tail)?;\n\
                     tracing::debug!(\"Reading array length: {{array_length}}\");\n\
                     let (tail, array) = nom::multi::count({field_value_parser}, array_length as usize)(tail)?;\n"
                ));
                    if morph.do_try_from {
                        type_impl_def.push_str(
                            "let array = array.iter().map(|val| <_>::try_from(*val)?).collect();\n",
                        );
                    }
                    type_impl_def.push_str("(tail, Some(array))");
                } else {
                    type_impl_def
                        .push_str(&format!("let (tail, res) = {field_value_parser}(tail)?;\n"));
                    if morph.do_try_from {
                        type_impl_def.push_str("    (tail, Some(<_>::try_from(res)?))\n");
                    } else {
                        type_impl_def.push_str("    (tail, Some(res))\n");
                    }
                }
                type_impl_def.push_str("} else {\n"); // - if not provided, just return None
                type_impl_def.push_str("    (tail, None)\n");
                type_impl_def.push_str("};\n");
                type_impl_def.push_str(&format!(
                    "    tracing::debug!(\"{field_name}: {{:?}}\", {field_name});\n\
                     Ok((tail, {field_name}))\n"
                ));
            } else if morph.is_vec {
                type_impl_def.push_str(&format!(
                "let (tail, _) = validate_array_tag(input)?;\n\
                 let (tail, array_length) = parse_vlq_int(tail)?;\n\
                 tracing::debug!(\"Reading array length: {{array_length}}\");\n\
                 let (tail, array) = nom::multi::count({field_value_parser}, array_length as usize)(tail)?;\n"
                ));
                if morph.do_try_from {
                    type_impl_def.push_str(
                        "let array = array.iter().map(|val| <_>::try_from(*val)?).collect();\n",
                    );
                }
                type_impl_def.push_str("Ok((tail, array))\n");
            } else if morph.is_sized_int {
                type_impl_def.push_str(&format!(
                    " let (tail, {field_name}) = {field_value_parser}?;\n
                      tracing::debug!(\"{field_name}: {{:?}}\", {field_name});\n
                      Ok((tail, {field_name}))\n"
                ));
            } else {
                type_impl_def.push_str(&format!(
                    " let (tail, {field_name}) = {field_value_parser}(input)?;\n"
                ));
                if morph.do_try_from {
                    type_impl_def.push_str(&format!(
                        "        Ok((tail, {field_type}::try_from({field_name})?))\n"
                    ));
                } else {
                    type_impl_def.push_str(&format!("        Ok((tail, {field_name}))\n"));
                }
            }
            type_impl_def.push_str(&format!("}}\n"));
        }
        struct_parse_return.push_str("}))\n"); // Close function definition
        if has_tags {
            struct_parse_fields.push_str(
                "
                _ => {\n\
                    tracing::error!(\"Unknown tag {field_tag}\");\n\
                    Err(S2ProtocolError::UnknownTag(variant_tag))\n\
                },\n\
            }\n\
          }",
            ); // close the match and the for-loop
        }
        struct_parse_impl_def.push_str(&struct_parse_fields);
        struct_parse_impl_def.push_str(&struct_parse_return);
        type_impl_def.push_str(&struct_parse_impl_def);
    }

    /// Generates a Rust Struct code with fields and parsing methods per field for Bit Packed
    /// encoding
    #[tracing::instrument(
        level = "debug",
        skip(proto_mod, proto_type_def, struct_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_bit_packed_proto_struct_code(
        proto_mod: &Value,
        proto_type_def: &mut String,
        mut struct_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        let decoder = DecoderType::BitPacked;
        let mut is_bit_array = false;
        //output.write_all(format!("\n/*{:#}*/\n", proto_mod).as_bytes())?;
        let field_array = proto_mod["type_info"]["fields"].as_array().unwrap();
        let mut struct_parse_return = String::from("Ok((tail, Self {");
        // Fields are ordered as they are defined in the JSON definition.
        // Optional fields start with a bit bool that shows if the field is set or not.
        let mut struct_parse_fields = String::from("");
        // The first ConstDecl is the main "tag", which is parsed by the main enum of the type.
        // After this ConstDecl, other ConstDecl's may appear, but these are hardcoded in the
        // protocol and do not appear in the data itself
        for field in field_array {
            let nnet_field_type = field["type"].as_str().unwrap();
            let proto_field_type_info = match field["type_info"]["fullname"].as_str() {
                Some(val) => val,
                None => {
                    // Fallback to the `type_info.type' field
                    field["type_info"]["type"].as_str().unwrap()
                }
            };
            if nnet_field_type == "ConstDecl" {
                tracing::info!("Skipping ConstDecl for {proto_field_type_info}");
                continue;
            }
            if nnet_field_type != "MemberStructField" {
                panic!("Unknown {proto_field_type_info} field type: {nnet_field_type}");
            }
            let field_name = field["name"].as_str().unwrap().to_case(Case::Snake);
            tracing::debug!("Processing field {field_name}");
            proto_type_def.push_str(&format!("    pub {field_name}: ",));
            let mut morph =
                if field["name"] == "m_eventData" && proto_field_type_info == "ChoiceType" {
                    // This is a special case of embedded ChoiceType, hopefully is just one.
                    ProtoTypeConversion {
                        rust_ty: "MEventData".to_string(),
                        parser: "MEventData::parse".to_string(),
                        ..Default::default()
                    }
                } else {
                    decoder.from_nnet_name(proto_field_type_info)
                };
            // If the type if Optional, we need to find the internal parser in case the field is
            // provided.
            if proto_field_type_info == "OptionalType" {
                let type_info_type = field["type_info"]["type_info"]["type"]
                    .as_str()
                    .expect("OptionalType should have type_info.type_info.type")
                    .to_string();
                // We should wrap this in Option<T>, but, it may also be a Option<Vec<T>>
                // A Vec<Option<T>> has *NOT* been observed, otherwise we would need some complex
                // structure that can be arbirtrary wrapping of types and maybe we should then just
                // create the serde for the whole protocol file and from there generate the code....
                // Maybe that's step 2.
                let enclosed_morph = if type_info_type == "ArrayType"
                    || type_info_type == "BitArrayType"
                {
                    // The enclosed type is wrapped in an additional .element_type field.
                    let element_type = if type_info_type == "ArrayType"
                        || type_info_type == "DynArrayType"
                    {
                        field["type_info"]["type_info"]["element_type"]["fullname"]
                            .as_str()
                            .expect("Field should have .type_info.type_info.element_type.fullname")
                            .to_string()
                    } else {
                        is_bit_array = true;
                        "u8".to_string()
                    };
                    tracing::info!("Optional {} Element Type: {}", type_info_type, element_type);
                    let mut internal_morph = decoder.from_nnet_name(&element_type);
                    internal_morph.rust_ty = format!("Vec<{}>", internal_morph.rust_ty);
                    // The intenal type, i.e. u8, needs to now be marked both as Vec<> and as Option<>
                    internal_morph.is_vec = true;
                    internal_morph.is_optional = true;
                    internal_morph
                } else {
                    // The enclosed type is wrapped in an additional .type_info field.
                    let enclosed_type = match field["type_info"]["type_info"]["fullname"]
                        .as_str() {
                        Some(val) => val.to_string(),
                        None => field["type_info"]["type_info"]["type"].as_str()
                        .expect("Field should contain .type_info.type_info.fullname or .type_info.type_info.fullname")
                        .to_string(),
                    };
                    decoder.from_nnet_name(&enclosed_type)
                };
                morph.rust_ty = morph.rust_ty.replace("{}", &enclosed_morph.rust_ty);
                morph.parser = morph.parser.replace("{}", &enclosed_morph.parser);
                morph.do_try_from = enclosed_morph.do_try_from;
                morph.is_vec = enclosed_morph.is_vec;
                morph.is_optional = true;
                if enclosed_morph.is_sized_int {
                    let offset = field["type_info"]["type_info"]["bounds"]["min"]["evalue"]
                        .as_str()
                        .expect("Field should have .type_info.bounds.min.evalue");
                    let num_bits = Self::bounds_max_value_to_bit_size(
                        &field["type_info"]["type_info"]["bounds"],
                    );
                    tracing::info!("IntType offset: {}, num_bits: {}", offset, num_bits);
                    morph.parser = morph
                        .parser
                        .replace("{}", &format!("input, {}, {}usize", offset, num_bits));
                }
            } else if proto_field_type_info == "ArrayType"
                || proto_field_type_info == "BitArrayType"
                || proto_field_type_info == "DynArrayType"
            {
                // The enclosed type is wrapped in an additional .element_type field.
                let element_type = if proto_field_type_info == "ArrayType"
                    || proto_field_type_info == "DynArrayType"
                {
                    field["type_info"]["element_type"]["fullname"]
                        .as_str()
                        .expect("Field should have .type_info.element_type.fullname")
                        .to_string()
                } else {
                    is_bit_array = true;
                    "u8".to_string()
                };
                tracing::info!("{} Element Type: {}", proto_field_type_info, element_type);
                let enclosed_morph = decoder.from_nnet_name(&element_type);
                morph.rust_ty = morph.rust_ty.replace("{}", &enclosed_morph.rust_ty);
                morph.parser = morph.parser.replace("{}", &enclosed_morph.parser);
                morph.do_try_from = enclosed_morph.do_try_from;
                morph.is_sized_int = enclosed_morph.is_sized_int;
                morph.is_vec = true;
            }
            if morph.is_sized_int {
                let offset = field["type_info"]["bounds"]["min"]["evalue"]
                    .as_str()
                    .expect("Field should have .type_info.bounds.min.evalue");
                let num_bits = Self::bounds_max_value_to_bit_size(&field["type_info"]["bounds"]);
                tracing::info!("IntType offset: {}, num_bits: {}", offset, num_bits);
                morph.parser = morph
                    .parser
                    .replace("{}", &format!("input, {}, {}usize", offset, num_bits));
            }
            let field_type = &morph.rust_ty;
            let field_value_parser = &morph.parser;
            proto_type_def.push_str(&format!("{field_type},\n"));
            if morph.is_optional {
                // This the field is optional, premark it as "Some(x)", because we will unwrap later
                // and if the field is None it means it was a required field that wasn't provided.
                struct_parse_impl_def.push_str(&format!(
                    "let mut {field_name}: Option<{field_type}> = Some(None);\n"
                ));
            } else {
                struct_parse_impl_def.push_str(&format!(
                    "let mut {field_name}: Option<{field_type}> = None;\n"
                ));
            }
            if morph.is_optional {
                // The OptionalType is a Some() since we will later unwrap and fail if a field is not
                // provided so we "pre-populate" it to empty.
                struct_parse_fields.push_str(&format!(" if let Some(None) = {field_name} "));
            } else {
                struct_parse_fields.push_str(&format!(" if {field_name}.is_none()"));
            }
            struct_parse_fields.push_str(&format!(
                " {{\n\
                 let (new_tail, parsed_{field_name}) = Self::parse_{field_name}(tail)?;\n\
                 tail = new_tail;\n\
                 {field_name} = Some(parsed_{field_name});\n\
                 }}\n\
                 "
            ));
            struct_parse_return.push_str(&format!(
                "{field_name}: ok_or_return_missing_field_err!({field_name}),\n"
            ));
            // Create a parsing function
            type_impl_def.push_str(&format!(
                    "#[tracing::instrument(level = \"trace\", skip(input), fields(peek = peek_bits(input)))]\n\
                     pub fn parse_{field_name}(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), {field_type}> {{\n\
                    "));
            if morph.is_optional {
                // If the next bit is a zero, then the field is None
                type_impl_def.push_str(
                    "let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;\n",
                );
                type_impl_def.push_str(&format!("let (tail, {field_name}) = if is_provided {{\n"));
                if morph.is_vec {
                    let array_length = field["type_info"]["type_info"]["bounds"]["max"]["evalue"]
                        .as_str()
                        .expect("Field should have type_info.type_info.bounds.max.evalue");
                    tracing::info!(
                        "ArrayType(is_bit_array: {}) array Length: {}",
                        is_bit_array,
                        array_length
                    );
                    let array_size_bits =
                        (array_length.parse::<f32>().unwrap().log2() + 1.).floor() as usize;
                    type_impl_def.push_str(&format!(
                        "let (tail, array_length) = take_n_bits_into_i64(input, {array_size_bits})?;\n\
                        let array_length = array_length as usize;\n\
                         tracing::debug!(\"Reading array length: {array_length}\");\n\
                         "
                    ));
                    if is_bit_array {
                        type_impl_def.push_str(&format!(
                            "let (tail, array) = take_bit_array(input, array_length)?;\n"
                        ));
                    } else {
                        type_impl_def.push_str(&format!(
                            "let (tail, array) = nom::multi::count({field_value_parser}, array_length)(tail)?;\n"
                        ));
                    }
                    if morph.do_try_from {
                        type_impl_def.push_str(
                            "let array = array.iter().map(|val| <_>::try_from(*val)?).collect();\n",
                        );
                    }
                    type_impl_def.push_str("(tail, Some(array))");
                } else {
                    if field_value_parser.contains("input,") {
                        type_impl_def
                            .push_str(&format!("let (tail, res) = {field_value_parser}?;\n"));
                    } else {
                        type_impl_def
                            .push_str(&format!("let (tail, res) = {field_value_parser}(tail)?;\n"));
                    }
                    if morph.do_try_from {
                        type_impl_def.push_str("    (tail, Some(<_>::try_from(res)?))\n");
                    } else {
                        type_impl_def.push_str("    (tail, Some(res))\n");
                    }
                }
                type_impl_def.push_str("} else {\n"); // - if not provided, just return None
                type_impl_def.push_str("    (tail, None)\n");
                type_impl_def.push_str("};\n");
                type_impl_def.push_str(&format!(
                    "    Ok((tail, {field_name}))\n\
                     }}\n"
                ));
            } else if morph.is_vec {
                let array_length = match field["bounds"]["max"]["evalue"].as_str() {
                        Some(val) => val.to_string(),
                        None => field["type_info"]["bounds"]["max"]["evalue"].as_str()
                            .expect("Field is vec but has no .bounds.max.evalue or .type_info.bounds.max.evalue")
                            .to_string(),
                };
                tracing::info!("ArrayType array Length: {}", array_length);
                let array_size_bits =
                    (array_length.parse::<f32>().unwrap().log2() + 1.).floor() as usize;
                type_impl_def.push_str(&format!(
                     "let (mut tail, array_length) = take_n_bits_into_i64(input, {array_size_bits})?;\n\
                     let array_length = array_length as usize;\n\
                     tracing::debug!(\"Reading array length: {{array_length}}\");\n\
                     "
                ));
                type_impl_def.push_str(&format!(
                    " let mut array = vec![];\n\
                     for _ in 0..array_length {{\n\
                     let (new_tail, data) = {field_value_parser}(tail)?;\n\
                     tail = new_tail;\n\
                     array.push(data);\n\
                     }}\n"
                ));
                if morph.do_try_from {
                    type_impl_def.push_str(
                        "let array = array.iter().map(|val| <_>::try_from(*val)?).collect();\n",
                    );
                }
                type_impl_def.push_str(
                    "Ok((tail, array))\n\
                     }",
                );
            } else {
                if field_value_parser.contains("input,") {
                    // The field parser may already contain input parameter, no need to add it to
                    // the end.
                    type_impl_def.push_str(&format!(
                        " let (tail, {field_name}) = {field_value_parser}?;\n"
                    ));
                } else {
                    type_impl_def.push_str(&format!(
                        " let (tail, {field_name}) = {field_value_parser}(input)?;\n"
                    ));
                }
                if morph.do_try_from {
                    type_impl_def.push_str(&format!(
                        "        Ok((tail, {field_type}::try_from({field_name})?))\n\
                         }}\n"
                    ));
                } else {
                    type_impl_def.push_str(&format!(
                        "        Ok((tail, {field_name}))\n\
                         }}\n"
                    ));
                }
            }
        }
        struct_parse_return.push_str("}))\n"); // Close function definition
        struct_parse_impl_def.push_str(&struct_parse_fields);
        struct_parse_impl_def.push_str(&struct_parse_return);
        type_impl_def.push_str(&struct_parse_impl_def);
    }

    /// Creates a Rust Enum out of a Choice type, the Choice type is an Enum where Variants contain
    /// Types
    #[tracing::instrument(
        level = "debug",
        skip(proto_mod, proto_type_def, enum_parse_impl_def, type_impl_def)
    )]
    pub fn gen_byte_aligned_proto_choice_code(
        proto_mod: &Value,
        proto_type_def: &mut String,
        mut enum_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        let decoder = DecoderType::ByteAligned;
        // output.write_all(format!("\n/*{:#}*/\n", proto_mod).as_bytes())?;
        let variant_array = proto_mod["type_info"]["fields"].as_array().unwrap();
        for variant in variant_array {
            if variant["type_info"]["type"] == "NullType" {
                continue;
            }
            let variant_name = proto_nnet_name_to_rust_name(&variant["name"]);
            proto_type_def.push_str(&format!("    {variant_name}",));
            let proto_field_type_info = match variant["type_info"]["fullname"].as_str() {
                Some(val) => val,
                None => {
                    // Fallback to the `type_info.type' variant
                    variant["type_info"]["type"].as_str().unwrap()
                }
            };
            let mut morph = decoder.from_nnet_name(proto_field_type_info);
            if proto_field_type_info == "OptionalType" {
                // The enclosed type is wrapped in an additional .type_info field.
                let enclosed_type = variant["type_info"]["type_info"]["type"].to_string();
                morph.parser = morph.parser.replace("{}", &enclosed_type);
                morph.rust_ty = morph.rust_ty.replace("{}", &enclosed_type);
            }
            let proto_field_tag = variant["tag"]["value"].as_str().unwrap();
            assert!(variant["tag"]["type"].as_str().unwrap() == String::from("IntLiteral"));
            let field_type = &morph.rust_ty;
            let field_value_parser = &morph.parser;
            proto_type_def.push_str(&format!("({field_type}),\n"));
            enum_parse_impl_def.push_str(&format!(
                " {proto_field_tag} => {{\n\
                 tracing::debug!(\"Variant tagged '{proto_field_tag}' for {variant_name}\");\n\
                 "
            ));
            if proto_field_type_info == "OptionalType" {
                enum_parse_impl_def.push_str("let (tail, _) = validate_opt_tag(tail)?;\n");
                // If the next bit is a filled with zeros, then the field is None
                enum_parse_impl_def
                    .push_str("let (tail, is_provided) = nom::number::complete::u8(tail)?;\n");
                enum_parse_impl_def.push_str("if is_provided != 0 {\n");
                enum_parse_impl_def
                    .push_str(&format!("let (tail, res) = {field_value_parser}(tail)?;\n"));
                enum_parse_impl_def.push_str(&format!(
                    "    Ok((tail, Self::{variant_name}(Some(res))))\n"
                ));
                enum_parse_impl_def.push_str("} else {\n");
                enum_parse_impl_def
                    .push_str(&format!("    Ok((tail, Self::{variant_name}(None)))\n"));
                enum_parse_impl_def.push_str("}\n");
            } else {
                enum_parse_impl_def
                    .push_str(&format!("let (tail, res) = {field_value_parser}(tail)?;\n"));
                if morph.do_try_from {
                    enum_parse_impl_def.push_str(&format!(
                        "    Ok((tail, Self::{variant_name}({field_type}::try_from(res)?)))\n"
                    ));
                } else {
                    enum_parse_impl_def
                        .push_str(&format!("    Ok((tail, Self::{variant_name}(res)))\n"));
                }
            }
            enum_parse_impl_def.push_str("    },\n"); // The match stanza end.
        }
        enum_parse_impl_def.push_str(
            "
            _ => {\n\
                tracing::error!(\"Unknown variant for tag {variant_tag}\");\n\
                Err(S2ProtocolError::UnknownTag(variant_tag))\n\
            },\n\
          }",
        ); // close the match
        type_impl_def.push_str(&enum_parse_impl_def);
    }

    /// Creates a Rust Enum out of a Choice type, the Choice type is an Enum where Variants contain
    /// Types
    #[tracing::instrument(
        level = "debug",
        skip(proto_mod, proto_type_def, enum_parse_impl_def, type_impl_def)
    )]
    pub fn gen_bit_packed_proto_choice_code(
        proto_mod: &Value,
        proto_type_def: &mut String,
        mut enum_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        let decoder = DecoderType::BitPacked;
        // tracing::info!("\n/*{:#}*/\n", proto_mod);
        let variant_array = proto_mod["type_info"]["fields"].as_array().unwrap();
        for variant in variant_array {
            tracing::debug!("INIT variant: {:?}", variant);
            let variant_name = proto_nnet_name_to_rust_name(&variant["name"]);
            tracing::debug!("DONE variant: {:?}", variant);
            proto_type_def.push_str(&format!("    {variant_name}",));
            let proto_field_type_info = match variant["type_info"]["fullname"].as_str() {
                Some(val) => val,
                None => {
                    // Fallback to the `type_info.type' variant
                    variant["type_info"]["type"].as_str().unwrap()
                }
            };
            let mut morph = decoder.from_nnet_name(proto_field_type_info);
            if proto_field_type_info == "OptionalType" {
                // The enclosed type is wrapped in an additional .type_info field.
                let mut enclosed_type = variant["type_info"]["type_info"]["type"].to_string();
                if enclosed_type == "\"UserType\"" {
                    // If it's a UserType we need to find the internal field's fullname
                    enclosed_type = proto_nnet_name_to_rust_name(
                        &variant["type_info"]["type_info"]["fullname"],
                    );
                }
                morph.parser = morph.parser.replace("{}", &enclosed_type);
                morph.rust_ty = morph.rust_ty.replace("{}", &enclosed_type);
            }
            let proto_field_tag = variant["tag"]["value"].as_str().unwrap();
            assert!(variant["tag"]["type"].as_str().unwrap() == String::from("IntLiteral"));
            let field_type = &morph.rust_ty;
            let field_value_parser = &morph.parser;
            proto_type_def.push_str(&format!("({field_type}),\n"));
            enum_parse_impl_def.push_str(&format!(
                " {proto_field_tag} => {{\n\
                 tracing::debug!(\"Variant tagged '{proto_field_tag}' for {variant_name}\");\n\
                 "
            ));
            if proto_field_type_info == "OptionalType" {
                // If the next bit is a filled with zeros, then the field is None
                enum_parse_impl_def.push_str(
                    "let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(tail)?;\n",
                );
                enum_parse_impl_def.push_str("if is_provided {\n");
                enum_parse_impl_def.push_str(&format!(
                    "let (tail, res) = {field_value_parser}::parse(tail)?;\n"
                ));
                enum_parse_impl_def.push_str(&format!(
                    "    Ok((tail, Self::{variant_name}(Some(res))))\n"
                ));
                enum_parse_impl_def.push_str("} else {\n");
                enum_parse_impl_def
                    .push_str(&format!("    Ok((tail, Self::{variant_name}(None)))\n"));
                enum_parse_impl_def.push_str("}\n");
            } else {
                enum_parse_impl_def
                    .push_str(&format!("let (tail, res) = {field_value_parser}(tail)?;\n"));
                if morph.do_try_from {
                    enum_parse_impl_def.push_str(&format!(
                        "    Ok((tail, Self::{variant_name}({field_type}::try_from(res)?)))\n"
                    ));
                } else {
                    enum_parse_impl_def
                        .push_str(&format!("    Ok((tail, Self::{variant_name}(res)))\n"));
                }
            }
            enum_parse_impl_def.push_str("    },\n"); // The match stanza end.
        }
        enum_parse_impl_def.push_str(
            "
            _ => {\n\
                tracing::error!(\"Unknown variant for tag {variant_tag}\");\n\
                Err(S2ProtocolError::UnknownTag(variant_tag))\n\
            },\n\
          }",
        ); // close the match
        type_impl_def.push_str(&enum_parse_impl_def);
    }

    #[tracing::instrument(
        level = "debug",
        skip(proto_mod, proto_type_def, enum_parse_impl_def, type_impl_def)
    )]
    pub fn gen_proto_choice_code(
        &self,
        proto_mod: &Value,
        proto_type_def: &mut String,
        enum_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        match self {
            Self::ByteAligned => Self::gen_byte_aligned_proto_choice_code(
                proto_mod,
                proto_type_def,
                enum_parse_impl_def,
                type_impl_def,
            ),
            Self::BitPacked => Self::gen_bit_packed_proto_choice_code(
                proto_mod,
                proto_type_def,
                enum_parse_impl_def,
                type_impl_def,
            ),
        }
    }

    /// Opens the byte aligned version of the ByteInt parser
    pub fn open_byte_aligned_int_main_parse_fn(
        proto_num: u64,
        _bounds: &Value,
        name: &str,
    ) -> String {
        format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::IntType::Parse\", level = \"trace\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {{\n\
         let (tail, _) = validate_int_tag(input)?;\n\
         let (tail, value) = parse_vlq_int(tail)?;\n\
         Ok((tail, Self {{ value }}))\n\
         ",
    )
    }

    pub fn close_byte_aligned_int_main_parse_fn() -> String {
        format!("}}")
    }

    /// Transforms a `bounds[max][evalue]` possibly "inclusive" (i.e. "less than _or equal_) to the
    /// number of bits needed to represent such max value.
    pub fn bounds_max_value_to_bit_size(bounds: &Value) -> usize {
        tracing::info!("Max bound: {:?}", bounds);
        let mut res = bounds["max"]["evalue"]
            .as_str()
            .expect("Missing have .max.evalue string")
            .parse::<f64>()
            .expect(".max.evalue must be parseable usize");
        if bounds["min"]["inclusive"]
            .as_bool()
            .expect("Missing .min.inclusive")
            == false
        {
            res -= 1.;
        }
        if bounds["max"]["inclusive"]
            .as_bool()
            .expect("Missing .max.inclusive")
            == false
        {
            res -= 1.;
        };
        (res.log2() + 1.).floor() as usize
    }

    /// Opens the bit packed version of the ByteInt parser
    pub fn open_bit_packed_int_main_parse_fn(proto_num: u64, bounds: &Value, name: &str) -> String {
        assert!(bounds["type"] == "MinMaxConstraint");
        let offset = bounds["min"]["evalue"]
            .as_str()
            .expect("bounds should have .min.evalue");
        let mut num_bits = 0usize;
        let bound_type = if let Some(rhs_value) = bounds["max"]["value"]["rhs"]["value"].as_str() {
            num_bits = rhs_value
                .parse()
                .expect(".max.value.rhs.value should be usize");
            assert!(
                bounds["max"]["value"]["type"] == "PowExpr",
                "RHS Bound must be PowExpr expr"
            );
            String::from("PowExpr")
        } else {
            if bounds["max"]["evalue"].as_str().is_some() {
                num_bits = Self::bounds_max_value_to_bit_size(&bounds);
            }
            bounds["type"].as_str().unwrap().to_string()
        };
        //    .expect("bounds should have .max.value.rhs.value")
        if offset.starts_with('-') {
            // If the offset is negative, we need to account for one more bit.
            num_bits += 1;
        }
        format!(
            "#[tracing::instrument(name=\"{proto_num}::{name}::IntType::Parse::{bound_type}\", level = \"trace\", skip(input), fields(peek = peek_bits(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {{\n\
         let offset: i64 = {offset};
         let num_bits: usize = {num_bits};
         let (tail, res) = parse_packed_int(input, offset, num_bits)?;\n\
         Ok((tail, Self {{ value: <_>::try_from(res)? }}))\n\
         ",
        )
    }

    pub fn close_bit_packed_int_main_parse_fn() -> String {
        format!("}}")
    }

    pub fn open_int_main_parse_fn(&self, proto_num: u64, bounds: &Value, name: &str) -> String {
        match self {
            Self::ByteAligned => Self::open_byte_aligned_int_main_parse_fn(proto_num, bounds, name),
            Self::BitPacked => Self::open_bit_packed_int_main_parse_fn(proto_num, bounds, name),
        }
    }

    pub fn close_int_main_parse_fn(&self) -> String {
        match self {
            Self::ByteAligned => Self::close_byte_aligned_int_main_parse_fn(),
            Self::BitPacked => Self::close_bit_packed_int_main_parse_fn(),
        }
    }

    /// Opens the byte aligned version of the ByteInt parser
    pub fn open_byte_aligned_user_type_main_parse_fn(
        proto_num: u64,
        type_info: &Value,
        name: &str,
    ) -> String {
        let type_info = type_info
            .as_str()
            .expect("type_info.fullname should be string");
        format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::UserType::Parse\", level = \"trace\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {{\n\
         let (tail, value) = {type_info}::parse(input)?;\n\
         Ok((tail, Self {{ value }}))\n\
         ",
    )
    }

    pub fn close_byte_aligned_user_type_main_parse_fn() -> String {
        format!("}}")
    }

    /// Opens the bit packed version of the ByteInt parser
    pub fn open_bit_packed_user_type_main_parse_fn(
        proto_num: u64,
        type_info: &Value,
        name: &str,
    ) -> String {
        let type_info = proto_nnet_name_to_rust_name(type_info);

        format!(
            "#[tracing::instrument(name=\"{proto_num}::{name}::UserType::Parse\", level = \"trace\", skip(input), fields(peek = peek_bits(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {{\n\
         let (tail, value) = {type_info}::parse(input)?;\n\
         Ok((tail, Self {{ value }}))\n\
         ",
        )
    }

    pub fn close_bit_packed_user_type_main_parse_fn() -> String {
        format!("}}")
    }

    pub fn open_user_type_main_parse_fn(
        &self,
        proto_num: u64,
        type_info: &Value,
        name: &str,
    ) -> String {
        match self {
            Self::ByteAligned => {
                Self::open_byte_aligned_user_type_main_parse_fn(proto_num, type_info, name)
            }
            Self::BitPacked => {
                Self::open_bit_packed_user_type_main_parse_fn(proto_num, type_info, name)
            }
        }
    }

    pub fn close_user_type_main_parse_fn(&self) -> String {
        match self {
            Self::ByteAligned => Self::close_byte_aligned_user_type_main_parse_fn(),
            Self::BitPacked => Self::close_bit_packed_user_type_main_parse_fn(),
        }
    }

    /// Opens the bit packed version of the ByteArray parser
    pub fn open_bit_packed_bit_array_main_parse_fn(
        proto_num: u64,
        bounds: &Value,
        name: &str,
    ) -> String {
        assert!(bounds["type"] == "MinMaxConstraint");
        assert!(bounds["min"]["evalue"] == "0");
        let num_bits = Self::bounds_max_value_to_bit_size(&bounds);
        format!(
            "#[tracing::instrument(name=\"{proto_num}::{name}::BitArrayType::Parse\", level = \"trace\", skip(input), fields(peek = peek_bits(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {{\n\
         let bitarray_length_bits: usize = {num_bits};
         let (tail, bitarray_length) = take_n_bits_into_i64(input, bitarray_length_bits)?;
         let (tail, value) = take_bit_array(tail, bitarray_length as usize)?;\n\
         Ok((tail, Self {{ value }}))\n\
         ",
        )
    }

    pub fn close_bit_packed_bit_array_main_parse_fn() -> String {
        format!("}}")
    }

    pub fn open_bit_array_main_parse_fn(
        &self,
        proto_num: u64,
        bounds: &Value,
        name: &str,
    ) -> String {
        match self {
            Self::ByteAligned => panic!("BitArray not supported for ByteAligned types"),
            Self::BitPacked => {
                Self::open_bit_packed_bit_array_main_parse_fn(proto_num, bounds, name)
            }
        }
    }

    pub fn close_bit_array_main_parse_fn(&self) -> String {
        match self {
            Self::ByteAligned => panic!("BitArray not supported for ByteAligned types"),
            Self::BitPacked => Self::close_bit_packed_int_main_parse_fn(),
        }
    }

    /// Opens the bit packed version of the ByteArray parser
    pub fn open_bit_packed_blob_main_parse_fn(
        proto_num: u64,
        bounds: &Value,
        name: &str,
    ) -> String {
        assert!(bounds["type"] == "ExactConstraint");
        assert!(bounds["min"]["evalue"] == bounds["max"]["evalue"]);
        let num_bits = Self::bounds_max_value_to_bit_size(&bounds);
        format!(
            "#[tracing::instrument(name=\"{proto_num}::{name}::BlobType::Parse\", level = \"trace\", skip(input), fields(peek = peek_bits(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {{\n\
         let (tail, _) = byte_align(input)?;
         let num_bits: usize = {num_bits};
         let (tail, value) = take_bit_array(tail, num_bits)?;\n\
         Ok((tail, Self {{ value }}))\n\
         ",
        )
    }

    pub fn close_bit_packed_blob_main_parse_fn() -> String {
        format!("}}")
    }

    pub fn open_string_main_parse_fn(&self, proto_num: u64, bounds: &Value, name: &str) -> String {
        match self {
            Self::ByteAligned => panic!("BitArray not supported for ByteAligned types"),
            Self::BitPacked => Self::open_bit_packed_string_main_parse_fn(proto_num, bounds, name),
        }
    }

    pub fn close_string_main_parse_fn(&self) -> String {
        match self {
            Self::ByteAligned => panic!("BitArray not supported for ByteAligned types"),
            Self::BitPacked => Self::close_bit_packed_string_main_parse_fn(),
        }
    }

    /// Opens the bit packed version of the ByteArray parser
    pub fn open_bit_packed_string_main_parse_fn(
        proto_num: u64,
        bounds: &Value,
        name: &str,
    ) -> String {
        assert!(bounds["type"] == "MinMaxConstraint");
        assert!(bounds["min"]["evalue"] == "0");
        let mut res = bounds["max"]["evalue"]
            .as_str()
            .expect("Missing have .max value string")
            .parse::<f32>()
            .expect(".max value must be parseable usize");
        if bounds["max"]["inclusive"].as_bool() == Some(true) {
            res += 1.;
        }
        // this works for some numbers spotted, no idea why the +2.
        // 781 string max.evalue needs 12 bits read for its size
        // 79 string max.evalue needs 9 bits read for its size
        // 19 string max.evalue needs 7 bits read for its size
        let str_size_num_bits = (res.log2() + 1.).floor() as usize + 2;
        format!(
            "#[tracing::instrument(name=\"{proto_num}::{name}::StringType::Parse\", level = \"trace\", skip(input), fields(peek = peek_bits(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {{\n\
         let str_size_num_bits: usize = {str_size_num_bits};\n\
         let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;\n\
         let (tail, _) = byte_align(tail)?;
         let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;\n\
         Ok((tail, Self {{ value }}))\n\
         ",
        )
    }

    pub fn close_bit_packed_string_main_parse_fn() -> String {
        format!("}}")
    }

    /// Opens a definition of an ArrayType that contains a value: Vec<{}>
    pub fn open_array_main_parse_fn(
        &self,
        proto_num: u64,
        bounds: &Value,
        name: &str,
        internal_type: &str,
    ) -> String {
        match self {
            Self::ByteAligned => {
                Self::open_byte_aligned_array_main_parse_fn(proto_num, name, internal_type)
            }
            Self::BitPacked => {
                Self::open_bit_packed_array_main_parse_fn(proto_num, bounds, name, internal_type)
            }
        }
    }

    pub fn close_array_main_parse_fn(&self) -> String {
        match self {
            Self::ByteAligned => Self::close_byte_aligned_array_main_parse_fn(),
            Self::BitPacked => Self::close_bit_packed_array_main_parse_fn(),
        }
    }

    /// Opens the bit packed version of the ByteArray parser
    pub fn open_bit_packed_array_main_parse_fn(
        proto_num: u64,
        bounds: &Value,
        name: &str,
        internal_type: &str,
    ) -> String {
        let mut array_max_value = bounds["max"]["evalue"]
            .as_str()
            .expect("Missing have .max value in ArrayType")
            .parse::<f32>()
            .expect(".max value must be parseable usize");
        if bounds["max"]["inclusive"].as_bool() == Some(true) {
            array_max_value += 1.;
        }
        let array_length_num_bits = ((array_max_value as f32).log2() + 1.).floor() as usize;
        format!(
            "#[tracing::instrument(name=\"{proto_num}::{name}::ArrayType::Parse\", level = \"trace\", skip(input), fields(peek = peek_bits(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {{\n\
         let array_length_num_bits: usize = {array_length_num_bits};\n\
         let (tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;\n\
         let (tail, value) = nom::multi::count({internal_type}::parse, array_length as usize)(tail)?;\n\
         Ok((tail, Self {{ value }}))\n\
         ",
        )
    }

    pub fn close_bit_packed_array_main_parse_fn() -> String {
        format!("}}")
    }

    /// Opens the bit packed version of the ByteArray parser
    pub fn open_byte_aligned_array_main_parse_fn(
        proto_num: u64,
        name: &str,
        internal_type: &str,
    ) -> String {
        format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::ArrayType::Parse\", level = \"trace\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {{\n\
                 let (tail, _) = validate_array_tag(input)?;\n\
                 let (tail, array_length) = parse_vlq_int(tail)?;\n\
                 tracing::debug!(\"Reading array length: {{array_length}}\");\n\
                 let (tail, value) = nom::multi::count({internal_type}::parse, array_length as usize)(tail)?;\n\
                Ok((tail, Self {{ value }}))\n\
         ",
        )
    }

    pub fn close_byte_aligned_array_main_parse_fn() -> String {
        format!("}}")
    }

    pub fn open_blob_main_parse_fn(&self, proto_num: u64, bounds: &Value, name: &str) -> String {
        match self {
            Self::ByteAligned => panic!("BlobType not supported for ByteAligned types"),
            Self::BitPacked => Self::open_bit_packed_blob_main_parse_fn(proto_num, bounds, name),
        }
    }

    pub fn close_blob_main_parse_fn(&self) -> String {
        match self {
            Self::ByteAligned => panic!("BlobType not supported for ByteAligned types"),
            Self::BitPacked => Self::close_bit_packed_blob_main_parse_fn(),
        }
    }

    pub fn open_byte_aligned_enum_main_parse_fn(proto_num: u64, name: &str) -> String {
        format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::Parse\", level = \"trace\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {{\n\
         let (tail, _) = validate_int_tag(input)?;\n\
         let (tail, variant_tag) = parse_vlq_int(tail)?;\n\
         match variant_tag {{\n\
         ",
    )
    }

    pub fn close_byte_aligned_enum_main_parse_fn() -> String {
        // Close both the match and the function
        format!(
            "
         }}\n\
         }}"
        )
    }

    pub fn open_bit_packed_enum_main_parse_fn(
        proto_num: u64,
        name: &str,
        num_fields: usize,
    ) -> String {
        let num_bits = (num_fields as f32).log2().ceil() as usize;
        format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::Parse\", level = \"trace\", skip(input), fields(peek = peek_bits(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {{\n\
         // Total fields: {num_fields}\n\
         let num_bits: usize = {num_bits};\n\
         let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;\n\
         match variant_tag {{\n\
         ",
    )
    }

    pub fn close_bit_packed_enum_main_parse_fn() -> String {
        // Close both the match and the function
        format!(
            "
         }}\n\
         }}"
        )
    }

    pub fn open_enum_main_parse_fn(&self, proto_num: u64, name: &str, num_fields: usize) -> String {
        match self {
            Self::ByteAligned => Self::open_byte_aligned_enum_main_parse_fn(proto_num, name),
            Self::BitPacked => {
                Self::open_bit_packed_enum_main_parse_fn(proto_num, name, num_fields)
            }
        }
    }

    pub fn close_enum_main_parse_fn(&self) -> String {
        match self {
            Self::ByteAligned => Self::close_byte_aligned_enum_main_parse_fn(),
            Self::BitPacked => Self::close_bit_packed_enum_main_parse_fn(),
        }
    }

    /// Creates a Rust Enum out of a EnumType type.
    /// The enum variants do not contain internal types
    /// The same strategy works for byte aligned and bit packed
    #[tracing::instrument(
        level = "debug",
        skip(proto_mod, proto_type_def, enum_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_agnostic_proto_enum_code(
        proto_mod: &Value,
        proto_type_def: &mut String,
        mut enum_parse_impl_def: String,
        type_impl_def: &mut String,
        enum_tags: &HashMap<String, String>,
    ) {
        //output.write_all(format!("\n/*{:#}*/\n", proto_mod).as_bytes())?;
        let variant_array = proto_mod["type_info"]["fields"].as_array().unwrap();
        for variant in variant_array {
            let variant_name = proto_nnet_name_to_rust_name(&variant["name"]);
            proto_type_def.push_str(&format!("    {variant_name}"));
            let variant_value_fullname = format!(
                "{}::{}",
                str_nnet_name_to_rust_name(proto_mod["fullname"].to_string()),
                variant_name,
            );
            if let Some(struct_name) = enum_tags.get(&variant_value_fullname) {
                proto_type_def.push_str(&format!("({struct_name})"));
            }
            proto_type_def.push_str(&format!(",\n",));
            let proto_variant_value = variant["value"]["value"].as_str().unwrap();
            assert!(variant["value"]["type"].as_str().unwrap() == String::from("IntLiteral"));
            enum_parse_impl_def.push_str(&format!(
                " {proto_variant_value} => {{\n\
                 tracing::debug!(\"Variant {variant_name} for value '{proto_variant_value}'\");\n"
            ));
            if let Some(struct_name) = enum_tags.get(&variant_value_fullname) {
                enum_parse_impl_def.push_str(&format!(
                    "
                let (tail, res) = {struct_name}::parse(tail)?;
                Ok((tail, Self::{variant_name}(res)))\n\
                 }},\n"
                ));
            } else {
                enum_parse_impl_def.push_str(&format!(
                    "Ok((tail, Self::{variant_name}))\n\
                     }},\n"
                ));
            }
        }
        enum_parse_impl_def.push_str(
            "
            _ => {\n\
                tracing::error!(\"Unknown variant value {variant_tag}\");\n\
                Err(S2ProtocolError::UnknownTag(variant_tag))\n\
            },\n",
        );
        type_impl_def.push_str(&enum_parse_impl_def);
    }

    /// Creates a Rust Enum out of a EnumType type.
    /// The enum variants do not contain internal types
    #[tracing::instrument(
        level = "debug",
        skip(self, proto_mod, proto_type_def, enum_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_proto_enum_code(
        &self,
        proto_mod: &Value,
        proto_type_def: &mut String,
        enum_parse_impl_def: String,
        type_impl_def: &mut String,
        enum_tags: &HashMap<String, String>,
    ) {
        Self::gen_agnostic_proto_enum_code(
            proto_mod,
            proto_type_def,
            enum_parse_impl_def,
            type_impl_def,
            enum_tags,
        )
    }

    #[tracing::instrument(
        level = "debug",
        skip(proto_type_def, int_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_byte_aligned_proto_int_code(
        proto_type_def: &mut String,
        int_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        // The int_parse_impl_def already contains the int parsing functionality.
        proto_type_def.push_str(&format!("    pub value: i64,"));
        type_impl_def.push_str(&int_parse_impl_def);
    }

    #[tracing::instrument(
        level = "debug",
        skip(proto_type_def, int_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_bit_packed_proto_int_code(
        proto_type_def: &mut String,
        int_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        proto_type_def.push_str(&format!("    pub value: i64,"));
        type_impl_def.push_str(&int_parse_impl_def);
    }

    /// Creates a Rust Int out of a IntType type.
    /// The struct contains an interval .value field.
    #[tracing::instrument(
        level = "debug",
        skip(self, proto_type_def, int_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_proto_int_code(
        &self,
        proto_type_def: &mut String,
        int_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        match self {
            Self::ByteAligned => Self::gen_byte_aligned_proto_int_code(
                proto_type_def,
                int_parse_impl_def,
                type_impl_def,
            ),
            Self::BitPacked => Self::gen_bit_packed_proto_int_code(
                proto_type_def,
                int_parse_impl_def,
                type_impl_def,
            ),
        }
    }

    #[tracing::instrument(
        level = "debug",
        skip(proto_type_def, bit_packed_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_bit_packed_proto_blob_code(
        proto_type_def: &mut String,
        bit_packed_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        proto_type_def.push_str(&format!("    pub value: Vec<u8>,"));
        type_impl_def.push_str(&bit_packed_parse_impl_def);
    }

    /// Creates a Rust `Vec<u8>` out of a BitArray type.
    /// The struct contains an interval .value field.
    #[tracing::instrument(
        level = "debug",
        skip(self, proto_type_def, int_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_proto_blob_code(
        &self,
        proto_type_def: &mut String,
        int_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        match self {
            Self::ByteAligned => panic!("BitArrayType is not supported for ByteAligned"),
            Self::BitPacked => Self::gen_bit_packed_proto_blob_code(
                proto_type_def,
                int_parse_impl_def,
                type_impl_def,
            ),
        }
    }

    #[tracing::instrument(
        level = "debug",
        skip(proto_type_def, bit_packed_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_bit_packed_proto_bit_array_code(
        proto_type_def: &mut String,
        bit_packed_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        proto_type_def.push_str(&format!("    pub value: Vec<u8>,"));
        type_impl_def.push_str(&bit_packed_parse_impl_def);
    }

    /// Creates a Rust `Vec<u8>` out of a BitArray type.
    /// The struct contains an interval .value field.
    #[tracing::instrument(
        level = "debug",
        skip(self, proto_type_def, int_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_proto_bit_array_code(
        &self,
        proto_type_def: &mut String,
        int_parse_impl_def: String,
        type_impl_def: &mut String,
    ) {
        match self {
            Self::ByteAligned => panic!("BitArrayType is not supported for ByteAligned"),
            Self::BitPacked => Self::gen_bit_packed_proto_bit_array_code(
                proto_type_def,
                int_parse_impl_def,
                type_impl_def,
            ),
        }
    }

    #[tracing::instrument(
        level = "debug",
        skip(proto_type_def, bit_packed_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_bit_packed_proto_user_type_code(
        proto_type_def: &mut String,
        bit_packed_parse_impl_def: String,
        type_impl_def: &mut String,
        type_info: &Value,
    ) {
        let type_info_name = proto_nnet_name_to_rust_name(type_info);
        proto_type_def.push_str(&format!("    pub value: {type_info_name},"));
        type_impl_def.push_str(&bit_packed_parse_impl_def);
    }

    /// Creates a rust wrapper type for a protocol defined type.
    /// The struct contains an interval .value field.
    #[tracing::instrument(
        level = "debug",
        skip(self, proto_type_def, int_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_proto_user_type_code(
        &self,
        proto_type_def: &mut String,
        int_parse_impl_def: String,
        type_impl_def: &mut String,
        type_info: &Value,
    ) {
        match self {
            Self::ByteAligned => panic!("UserType is not supported for ByteAligned"),
            Self::BitPacked => Self::gen_bit_packed_proto_user_type_code(
                proto_type_def,
                int_parse_impl_def,
                type_impl_def,
                type_info,
            ),
        }
    }

    #[tracing::instrument(
        level = "debug",
        skip(proto_type_def, byte_aligned_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_byte_aligned_proto_array_code(
        proto_type_def: &mut String,
        byte_aligned_parse_impl_def: String,
        type_impl_def: &mut String,
        internal_type: &str,
    ) {
        proto_type_def.push_str(&format!("    pub value: Vec<{}>,", internal_type));
        type_impl_def.push_str(&byte_aligned_parse_impl_def);
    }

    #[tracing::instrument(
        level = "debug",
        skip(proto_type_def, bit_packed_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_bit_packed_proto_array_code(
        proto_type_def: &mut String,
        bit_packed_parse_impl_def: String,
        type_impl_def: &mut String,
        internal_type: &str,
    ) {
        proto_type_def.push_str(&format!("    pub value: Vec<{}>,", internal_type));
        type_impl_def.push_str(&bit_packed_parse_impl_def);
    }

    /// Creates a Rust Vec<{}> out of an ArrayType.
    /// The struct contains an interval .value field.
    #[tracing::instrument(
        level = "debug",
        skip(self, proto_type_def, int_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_proto_array_code(
        &self,
        proto_type_def: &mut String,
        int_parse_impl_def: String,
        type_impl_def: &mut String,
        internal_type: &str,
    ) {
        match self {
            Self::ByteAligned => Self::gen_byte_aligned_proto_array_code(
                proto_type_def,
                int_parse_impl_def,
                type_impl_def,
                internal_type,
            ),
            Self::BitPacked => Self::gen_bit_packed_proto_array_code(
                proto_type_def,
                int_parse_impl_def,
                type_impl_def,
                internal_type,
            ),
        }
    }
}
