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
        "#[tracing::instrument(name=\"{proto_num}::byte_aligned::{name}::Parse\", level = \"debug\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {{\n\
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
    fn open_bit_packed_struct_main_parse_fn(proto_num: u64, name: &str) -> String {
        format!(
        "#[tracing::instrument(name=\"{proto_num}::bit_packed::{name}::Parse\", level = \"debug\", skip(input), fields(peek = peek_bits(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {{\n\
             let tail = input;
         ",
    )
    }

    /// Closes the struct main parse function.
    fn close_bit_packed_struct_main_parse_fn() -> String {
        format!("}}")
    }

    /// Opens the main parse function for the current DecoderType
    pub fn open_struct_main_parse_fn(self, proto_num: u64, name: &str) -> String {
        match self {
            Self::ByteAligned => Self::open_byte_aligned_struct_main_parse_fn(proto_num, name),
            Self::BitPacked => Self::open_bit_packed_struct_main_parse_fn(proto_num, name),
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

    /// A coversion table with compatible Rust types to Protocol Types for the ByteAligned variant.
    #[tracing::instrument(level = "debug")]
    fn bit_packed_from_nnet_name(nnet_name: &str) -> ProtoTypeConversion {
        // XXX: FIX THIS
        match nnet_name {
            "NNet.uint8"
            | "NNet.Replay.EReplayType"
            | "NNet.uint6"
            | "NNet.Game.TPlayerId"
            | "NNet.Replay.Tracker.TUIntMiniBits" => ProtoTypeConversion {
                rust_ty: "u8".to_string(),
                do_try_from: true,
                parser: "parse_packed_int".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.uint32" | "NNet.uint14" | "NNet.uint22" => ProtoTypeConversion {
                rust_ty: "u32".to_string(),
                do_try_from: true,
                parser: "fixme".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.int32" | "NNet.Game.TFixedBits" => ProtoTypeConversion {
                rust_ty: "i32".to_string(),
                do_try_from: true,
                parser: "fixme".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.SVersion" => ProtoTypeConversion {
                rust_ty: "SVersion".to_string(),
                do_try_from: false,
                parser: "fixme".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.Game.TColorId" => ProtoTypeConversion {
                rust_ty: "i64".to_string(),
                do_try_from: true,
                parser: "fixme".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "BlobType"
            | "NNet.Replay.CSignature"
            | "StringType"
            | "NNet.Replay.Tracker.CatalogName" => ProtoTypeConversion {
                rust_ty: "Vec<u8>".to_string(),
                do_try_from: false,
                parser: "fixme".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "BoolType" => ProtoTypeConversion {
                rust_ty: "bool".to_string(),
                do_try_from: false,
                parser: "fixme".to_string(),
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
                parser: "fixme".to_string(),
                is_vec: false,
                is_optional: false,
            },
            "NNet.Replay.Tracker.SPlayerStats" => ProtoTypeConversion {
                rust_ty: "ReplayTrackerSPlayerStats".to_string(),
                do_try_from: false,
                parser: "fixme".to_string(),
                is_vec: false,
                is_optional: false,
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
        "#[tracing::instrument(name=\"{proto_num}::{name}::ChoiceType::parse\", level = \"debug\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {{\n\
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
        format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::ChoiceType::parse\", level = \"debug\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {{\n\
            // ChoiceType:
            // Use the number of elements in the json .fields to calculate how many
            // bits to have unique tags.
            let num_fields: usize = {num_fields};\n\
            let offset = 0i64;\n\
            let num_bits = (num_fields as f32).sqrt().ceil() as usize;\n\
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
        // After this ConstDecl, other ConstDecl's may appear.
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
            // If the type if Optional, we need to find the internal parser in case the field is
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
                    let enclosed_type = field["type_info"]["type_info"]["fullname"]
                        .as_str()
                        .expect("Field should contain .type_info.type_info.fullname");
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
                            panic!(\"Unhandled duplicate field.\");
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
                "{field_name}: {field_name}.expect(\"Missing {field_name} from struct\"),\n"
            ));
            // Create a parsing function
            type_impl_def.push_str(&format!(
                    "#[tracing::instrument(level = \"debug\", skip(input), fields(peek = peek_hex(input)))]\n\
                     pub fn parse_{field_name}(input: &[u8]) -> IResult<&[u8], {field_type}> {{\n\
                    "));
            if morph.is_optional {
                type_impl_def.push_str("let (tail, _) = validate_opt_tag(input)?;\n");
                // If the next bit is a filled with zeros, then the field is None
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
                        "let array = array.iter().map(|val| <_>::try_from(*val).unwrap()).collect();\n",
                    );
                    }
                    type_impl_def.push_str("(tail, Some(array))");
                } else {
                    type_impl_def
                        .push_str(&format!("let (tail, res) = {field_value_parser}(tail)?;\n"));
                    if morph.do_try_from {
                        type_impl_def.push_str("    (tail, Some(<_>::try_from(res).unwrap()))\n");
                    } else {
                        type_impl_def.push_str("    (tail, Some(res))\n");
                    }
                }
                type_impl_def.push_str("} else {\n"); // - if not provided, just return None
                type_impl_def.push_str("    (tail, None)\n");
                type_impl_def.push_str("};\n");
                type_impl_def.push_str(&format!(
                    "    tracing::debug!(\"res: {{:?}}\", {field_name});\n\
                     Ok((tail, {field_name}))\n\
                     }}\n"
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
                    "let array = array.iter().map(|val| <_>::try_from(*val).unwrap()).collect();\n",
                );
                }
                type_impl_def.push_str(
                    "Ok((tail, array))\n\
                     }",
                );
            } else {
                type_impl_def.push_str(&format!(
                    " let (tail, {field_name}) = {field_value_parser}(input)?;\n"
                ));
                type_impl_def.push_str(&format!(
                    "    tracing::debug!(\"res: {{:?}}\", {field_name});\n"
                ));
                if morph.do_try_from {
                    type_impl_def.push_str(&format!(
                        "        Ok((tail, {field_type}::try_from({field_name}).unwrap()))\n\
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
        if has_tags {
            struct_parse_fields.push_str(
                "
                _ => {\n\
                    tracing::error!(\"Unknown tag {field_tag}\");\n\
                    panic!(\"Unknown tag {field_tag}\");\n\
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
        unimplemented!()
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
            let variant_name = proto_nnet_name_to_rust_name(&variant["type_info"]["name"]);
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
                enum_parse_impl_def.push_str(&format!(
                    "let (tail, res) = {field_value_parser}(tail)?;\n\
                     tracing::debug!(\"res: {{:?}}\", res);\n"
                ));
                enum_parse_impl_def.push_str(&format!(
                    "    Ok((tail, Self::{variant_name}(Some(res))))\n"
                ));
                enum_parse_impl_def.push_str("} else {\n");
                enum_parse_impl_def
                    .push_str(&format!("    Ok((tail, Self::{variant_name}(None)))\n"));
                enum_parse_impl_def.push_str("}\n");
            } else {
                enum_parse_impl_def.push_str(&format!(
                    "let (tail, res) = {field_value_parser}(tail)?;\n\
                     tracing::debug!(\"res: {{:?}}\", res);\n"
                ));
                if morph.do_try_from {
                    enum_parse_impl_def.push_str(&format!(
                    "    Ok((tail, Self::{variant_name}({field_type}::try_from(res).unwrap())))\n"
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
                panic!(\"Unknown variant tag {variant_tag}\");\n\
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
        // output.write_all(format!("\n/*{:#}*/\n", proto_mod).as_bytes())?;
        let variant_array = proto_mod["type_info"]["fields"].as_array().unwrap();
        for variant in variant_array {
            let variant_name = proto_nnet_name_to_rust_name(&variant["type_info"]["name"]);
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
                enum_parse_impl_def.push_str("let (tail, _) = fixme_validate_opt_tag(tail)?;\n");
                // If the next bit is a filled with zeros, then the field is None
                enum_parse_impl_def
                    .push_str("let (tail, is_provided) = parse_packed_int(tail, 0, 1)?;\n");
                enum_parse_impl_def.push_str("if is_provided != 0 {\n");
                enum_parse_impl_def.push_str(&format!(
                    "let (tail, res) = {field_value_parser}(tail)?;\n\
                     tracing::debug!(\"res: {{:?}}\", res);\n"
                ));
                enum_parse_impl_def.push_str(&format!(
                    "    Ok((tail, Self::{variant_name}(Some(res))))\n"
                ));
                enum_parse_impl_def.push_str("} else {\n");
                enum_parse_impl_def
                    .push_str(&format!("    Ok((tail, Self::{variant_name}(None)))\n"));
                enum_parse_impl_def.push_str("}\n");
            } else {
                enum_parse_impl_def.push_str(&format!(
                    "let (tail, res) = {field_value_parser}(tail)?;\n\
                     tracing::debug!(\"res: {{:?}}\", res);\n"
                ));
                if morph.do_try_from {
                    enum_parse_impl_def.push_str(&format!(
                    "    Ok((tail, Self::{variant_name}({field_type}::try_from(res).unwrap())))\n"
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
                panic!(\"Unknown variant tag {variant_tag}\");\n\
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
        "#[tracing::instrument(name=\"{proto_num}::{name}::IntType::Parse\", level = \"debug\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {{\n\
         let (tail, _) = validate_int_tag(input)?;\n\
         let (tail, value) = parse_vlq_int(tail)?;\n\
         // TODO: Unsure about this. \n\
         Ok((tail, Self {{ value }}))\n\
         ",
    )
    }

    pub fn close_byte_aligned_int_main_parse_fn() -> String {
        format!("}}")
    }

    /// Opens the bit packed version of the ByteInt parser
    pub fn open_bit_packed_int_main_parse_fn(proto_num: u64, bounds: &Value, name: &str) -> String {
        assert!(bounds["type"] == "MinMaxConstraint");
        let offset = bounds["min"]["evalue"]
            .as_str()
            .expect("bounds should have .min.evalue");
        let mut num_bits: usize = bounds["max"]["value"]["rhs"]
            .as_str()
            .expect("bounds should have .max.value.rhs")
            .parse()
            .expect(".max.value.rhs should be usize");
        if offset.starts_with('-') {
            // If the offset is negative, we need to account for one more bit.
            num_bits += 1;
        }
        format!(
            "#[tracing::instrument(name=\"{proto_num}::{name}::IntType::Parse\", level = \"debug\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {{\n\
         let offset: i64 = {offset};
         let num_bits: usize = {num_bits};
         let (tail, num) = parse_packed_int(input, offset, num_bits)?;\n\
         // TODO: Unsure about this. \n\
         Ok((tail, Self {{ value: <_>::try_from(res).unwrap() }}))\n\
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

    pub fn open_byte_aligned_enum_main_parse_fn(proto_num: u64, name: &str) -> String {
        format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::Parse\", level = \"debug\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {{\n\
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

    pub fn open_bit_packed_enum_main_parse_fn(proto_num: u64, name: &str) -> String {
        // XXX: untested
        format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::Parse\", level = \"debug\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {{\n\
         fixme()
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

    pub fn open_enum_main_parse_fn(&self, proto_num: u64, name: &str) -> String {
        match self {
            Self::ByteAligned => Self::open_byte_aligned_enum_main_parse_fn(proto_num, name),
            Self::BitPacked => Self::open_bit_packed_enum_main_parse_fn(proto_num, name),
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
    #[tracing::instrument(
        level = "debug",
        skip(proto_mod, proto_type_def, enum_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_byte_aligned_proto_enum_code(
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
                panic!(\"Unknown variant value {variant_tag}\");\n\
            },\n",
        );
        type_impl_def.push_str(&enum_parse_impl_def);
    }

    /// Creates a Rust Enum out of a EnumType type.
    /// The enum variants do not contain internal types
    #[tracing::instrument(
        level = "debug",
        skip(proto_mod, proto_type_def, enum_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_bit_packed_proto_enum_code(
        proto_mod: &Value,
        proto_type_def: &mut String,
        enum_parse_impl_def: String,
        type_impl_def: &mut String,
        enum_tags: &HashMap<String, String>,
    ) {
        unimplemented!()
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
        match self {
            Self::ByteAligned => Self::gen_byte_aligned_proto_enum_code(
                proto_mod,
                proto_type_def,
                enum_parse_impl_def,
                type_impl_def,
                enum_tags,
            ),
            Self::BitPacked => Self::gen_bit_packed_proto_enum_code(
                proto_mod,
                proto_type_def,
                enum_parse_impl_def,
                type_impl_def,
                enum_tags,
            ),
        }
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
        // XXX: This is untested.
        proto_type_def.push_str(&format!("    value: i64,"));
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
        proto_type_def.push_str(&format!("    value: i64,"));
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
}
