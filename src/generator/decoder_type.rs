//! Generates code that works for either ByteAligned (VersionedDecoder) or BitPacked encoded data.

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
    /// Furthur combinators will use `tail` instead of `input`.
    fn open_byte_aligned_gen_struct_main_parse_fn(proto_num: u64, name: &str) -> String {
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
    fn open_bit_packed_gen_struct_main_parse_fn(proto_num: u64, name: &str) -> String {
        format!(
        "#[tracing::instrument(name=\"{proto_num}::bit_packed::{name}::Parse\", level = \"debug\", skip(input), fields(peek = peek_bits(input)))]\n\
         pub fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {{\n\
             let tail = input;
         ",
    )
    }

    /// Opens the main parse function for the current DecoderType
    pub fn open_gen_struct_main_parse_fn(self, proto_num: u64, name: &str) -> String {
        match self {
            Self::ByteAligned => Self::open_byte_aligned_gen_struct_main_parse_fn(proto_num, name),
            Self::BitPacked => Self::open_bit_packed_gen_struct_main_parse_fn(proto_num, name),
        }
    }

    /// Closes the struct main parse function.
    pub fn close_gen_struct_main_parse_fn() -> String {
        String::from("}")
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
                parser: "fixme".to_string(),
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

    #[tracing::instrument(
        level = "debug",
        skip(proto_mod, proto_type_def, struct_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_proto_struct_code(
        &self,
        proto_mod: &Value,
        &mut proto_type_def: String,
        &mut struct_parse_impl_def: String,
        &mut type_impl_def: String,
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
        &mut proto_type_def: String,
        &mut struct_parse_impl_def: String,
        &mut type_impl_def: String,
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
                    let mut internal_morph = self.from_nnet_name(element_type);
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

        struct_parse_impl_def.push_str("}\n"); // Close the main parse function definition
    }

    /// Generates a Rust Struct code with fields and parsing methods per field for Bit Packed
    /// encoding
    #[tracing::instrument(
        level = "debug",
        skip(proto_mod, proto_type_def, struct_parse_impl_def, type_impl_def,)
    )]
    pub fn gen_bit_packed_proto_struct_code(
        proto_mod: &Value,
        mut proto_type_def: String,
        mut struct_parse_impl_def: String,
        mut type_impl_def: String,
    ) {
        unimplemented!()
    }
}
