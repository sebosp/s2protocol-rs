//! ABANDON ALL HOPE, YE WHO ENTER HERE
//!
//! Protocol Code Parsing Generator.
//! This code reads the Protocol definition from the JSON Definition.
//! It generates structures for parsing progressively the Tracker Events for now.
//! It is meant to generate once the structures and method impl and then a human
//! can properly:
//! - RustFmt
//! - Document it.
//! Thus, [`syn`] and [`quote`] crates were not involved.
//! This is really ugly and probably only works for a handful of versions.

pub mod decoder_type;
use convert_case::{Case, Casing};
pub use decoder_type::DecoderType;
use serde_json::Value;
use std::collections::HashMap;
use std::convert::From;
use std::fs::File;
use std::io::prelude::*;
use std::str;

pub fn gen_struct_type_def_skel(name: &str) -> String {
    format!("#[derive(Debug, Default, PartialEq, Clone)]\npub struct {name} {{\n",)
}

pub fn gen_enum_type_def_skel(name: &str) -> String {
    format!("#[derive(Debug, PartialEq, Clone)]\npub enum {name} {{\n",)
}

pub fn gen_int_type_def_skel(name: &str) -> String {
    format!("#[derive(Debug, Default, PartialEq, Clone)]\npub struct {name}{{ \n",)
}

pub fn gen_type_def_skel(name: &str, unit_ty: &str) -> String {
    // TODO: A StringType would become a pub <name>(String) with impl Parse that calls
    // tagged_blob().
    tracing::debug!("Generating gen_type_def_skel for {name}: {unit_ty}");
    match unit_ty {
        "StructType" => gen_struct_type_def_skel(name),
        "EnumType" => gen_enum_type_def_skel(name),
        "ChoiceType" => gen_enum_type_def_skel(name),
        "IntType" => gen_int_type_def_skel(name),
        _ => panic!("Unknown unit type: {unit_ty}"),
    }
}

/// Initializes a type impl block to contain the methods
pub fn open_gen_type_impl_def(name: &str) -> String {
    format!("impl {name} {{\n",)
}

/// Closes the type impl scope.
pub fn close_gen_type_impl_def() -> String {
    b"}\n"
}
pub fn gen_choice_main_parse_fn(proto_num: &str, name: &str) -> String {
    format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::Parse\", level = \"debug\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {{\n\
         let (tail, _) = validate_choice_tag(input)?;\n\
         let (tail, variant_tag) = parse_vlq_int(tail)?;\n\
         match variant_tag {{
         ",
    )
}

pub fn gen_int_main_parse_fn(proto_num: &str, name: &str) -> String {
    format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::Parse\", level = \"debug\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {{\n\
         let (tail, _) = validate_int_tag(input)?;\n\
         let (tail, value) = parse_vlq_int(tail)?;\n\
         // TODO: Unsure about this. \n\
         Ok((tail, Self {{ value }}))\n\
         ",
    )
}

pub fn gen_enum_main_parse_fn(proto_num: &str, name: &str) -> String {
    format!(
        "#[tracing::instrument(name=\"{proto_num}::{name}::Parse\", level = \"debug\", skip(input), fields(peek = peek_hex(input)))]\n\
         pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {{\n\
         let (tail, _) = validate_int_tag(input)?;\n\
         let (tail, variant_tag) = parse_vlq_int(tail)?;\n\
         match variant_tag {{
         ",
    )
}

/// A mapping between proto types defined in the json and the rust equivalent
#[derive(Debug)]
pub struct ProtoTypeConversion {
    /// The destination type in Rust types
    pub rust_ty: String,
    /// Whether a TryInto should be attempted
    pub do_try_from: bool,
    /// A value parser
    pub parser: String,
    /// Whether the type is optional
    pub is_optional: bool,
    /// Whether the type is a vec, NOTE: No `Vec<Option<T>>` have been observed. But there are
    /// `Option<Vec<T>>`.
    pub is_vec: bool,
}

impl ProtoTypeConversion {}

fn proto_nnet_name_to_rust_name(nnet_name: &Value) -> String {
    str_nnet_name_to_rust_name(nnet_name.as_str().unwrap().to_string())
}

fn str_nnet_name_to_rust_name(input: String) -> String {
    input
        .replace(".", "")
        .replace("NNet", "")
        .trim_matches('"')
        .to_case(Case::Pascal)
}

/// Attempts to generate code for a structure read from the proto json. It may branch into Structs
/// or Enums.
#[tracing::instrument(level = "debug", skip(output, proto_mod, enum_tags))]
pub fn gen_proto_code(
    source_path: &str,
    output: &mut File,
    proto_mod: &Value,
    enum_tags: &HashMap<String, String>,
    decoder_type: DecoderType,
) -> std::io::Result<()> {
    // Try to get the numeric part of the protocol.
    // The source filename is called like: ./s2protocol/json/protocol87702.json
    let mut proto_num: String = format!(
        "{:?}",
        std::path::Path::new(source_path).file_name().unwrap()
    );
    proto_num.retain(|c| c >= '0' && c <= '9');
    // The pub <Type> <Name> {}
    let proto_unit_type = proto_mod["type_info"]["type"]
        .as_str()
        .expect(".type_info.type field expected in mod");
    let proto_unit_type_name = proto_nnet_name_to_rust_name(&proto_mod["fullname"]);
    tracing::debug!("Analyzing proto_unit_type_name {proto_unit_type_name}: '{proto_unit_type}'",);
    let proto_type_def = gen_type_def_skel(&proto_unit_type_name, proto_unit_type);
    // The impl <Name> {}
    let type_impl_def = open_gen_type_impl_def(&proto_unit_type_name);
    // The method for parsing all the fields into the struct as a whole.
    if proto_unit_type == "StructType" {
        let struct_parse_impl_def =
            decoder_type.open_gen_struct_main_parse_fn(&proto_num, &proto_unit_type_name);
        gen_proto_struct_code(
            output,
            proto_mod,
            proto_type_def,
            struct_parse_impl_def,
            type_impl_def,
            enum_tags,
            decoder_type,
        )?;
    } else if proto_unit_type == "ChoiceType" {
        let enum_parse_impl_def = gen_choice_main_parse_fn(&proto_num, &proto_unit_type_name);
        gen_proto_choice_code(
            output,
            proto_mod,
            proto_type_def,
            enum_parse_impl_def,
            type_impl_def,
            decoder_type,
        )?;
    } else if proto_unit_type == "EnumType" {
        let enum_parse_impl_def = gen_enum_main_parse_fn(&proto_num, &proto_unit_type_name);
        gen_proto_enum_code(
            output,
            proto_mod,
            proto_type_def,
            enum_parse_impl_def,
            type_impl_def,
            enum_tags,
            decoder_type,
        )?;
    } else if proto_unit_type == "IntType" {
        let int_parse_impl_def = gen_int_main_parse_fn(&proto_num, &proto_unit_type_name);
        gen_proto_int_code(
            output,
            proto_type_def,
            int_parse_impl_def,
            type_impl_def,
            decoder_type,
        )?;
    } else {
        tracing::error!("Unhandled protocol unit type: {:?}", proto_unit_type);
    }
    Ok(())
}

/// Generates a Rust Struct code with fields and parsing methods per field
#[tracing::instrument(
    level = "debug",
    skip(
        output,
        proto_mod,
        proto_type_def,
        struct_parse_impl_def,
        type_impl_def,
        enum_tags,
    )
)]
pub fn gen_proto_struct_code(
    output: &mut File,
    proto_mod: &Value,
    mut proto_type_def: String,
    mut struct_parse_impl_def: String,
    mut type_impl_def: String,
    enum_tags: &HashMap<String, String>,
    decoder_type: DecoderType,
) -> std::io::Result<()> {
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
        let mut morph = ProtoTypeConversion::from(proto_field_type_info);
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
                let mut internal_morph = ProtoTypeConversion::from(element_type);
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
                ProtoTypeConversion::from(enclosed_type)
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
            let enclosed_morph = ProtoTypeConversion::from(element_type);
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
            type_impl_def.push_str("let (tail, is_provided) = nom::number::complete::u8(tail)?;\n");
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
    type_impl_def.push_str(&struct_parse_impl_def);

    proto_type_def.push_str("}\n"); // Close struct definition
    type_impl_def.push_str(close_gen_type_impl_def())?;
    //
    output.write_all(format!("\n{}", proto_type_def).as_bytes())?;
    output.write_all(format!("{}\n", type_impl_def).as_bytes())?;
    Ok(())
}

/// Creates a Rust Enum out of a Choice type, the Choice type is an Enum where Variants contain
/// Types
#[tracing::instrument(
    level = "debug",
    skip(output, proto_mod, proto_type_def, enum_parse_impl_def, type_impl_def)
)]
pub fn gen_proto_choice_code(
    output: &mut File,
    proto_mod: &Value,
    mut proto_type_def: String,
    mut enum_parse_impl_def: String,
    mut type_impl_def: String,
    decoder_type: DecoderType,
) -> std::io::Result<()> {
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
        let mut morph = ProtoTypeConversion::from(proto_field_type_info);
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
            enum_parse_impl_def.push_str(&format!("    Ok((tail, Self::{variant_name}(None)))\n"));
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

    enum_parse_impl_def.push_str("}\n"); // Close function definition
    type_impl_def.push_str(&enum_parse_impl_def);

    proto_type_def.push_str("}\n"); // Close struct definition
    type_impl_def.push_str("}\n"); // Close impl definition
                                   //
    output.write_all(format!("\n{}", proto_type_def).as_bytes())?;
    output.write_all(format!("{}\n", type_impl_def).as_bytes())?;
    Ok(())
}

/// Generates the protocol-agnostic version of the TrackerEvents
/// This may fail in new/older protocols, and the generated code should be
/// validated, luckily at compile time.
#[tracing::instrument(level = "debug", skip(output,))]
pub fn generate_convert_into_versionless(output: &mut File) -> std::io::Result<()> {
    output.write_all(r#"
// Translate to the protocol agnostic version of tracker events.

impl ReplayTrackerEEventId {
    /// Reads a delta, TrackerEvent pair
    #[tracing::instrument(name="TrackerEvent::parse_event_pair", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_event_pair(input: &[u8]) -> IResult<&[u8], (u32, ReplayTrackerEEventId)> {
        let (tail, delta) = SVarUint32::parse(input)?;
        let (tail, event) = ReplayTrackerEEventId::parse(tail)?;
        let delta = match delta {
            SVarUint32::Uint6(val) => val as u32,
            SVarUint32::Uint14(val) | SVarUint32::Uint22(val) | SVarUint32::Uint32(val) => val,
        };
        Ok((tail, (delta, event)))
    }

    /// Read the Tracker Events
    pub fn read_tracker_events(mpq: &MPQ, file_contents: &[u8]) -> Vec<TrackerEvent> {
        // TODO: Make it return an Iterator.
        let (_event_tail, tracker_events) = mpq
            .read_mpq_file_sector("replay.tracker.events", false, &file_contents)
            .unwrap();
        let mut res = vec![];
        let mut event_tail: &[u8] = &tracker_events;
        loop {
            let (new_event_tail, (delta, event)) =
                Self::parse_event_pair(&event_tail).expect("Unable to parse TrackerEvents");
            event_tail = new_event_tail;
            match event.try_into() {
                Ok(val) => res.push(TrackerEvent { delta, event: val }),
                Err(err) => {
                    tracing::debug!("Skipping event: {:?}", err);
                }
            };
            if event_tail.input_len() == 0 {
                break;
            }
        }
        res
    }
}

impl TryFrom<ReplayTrackerEEventId> for ReplayTrackerEvent {
    type Error = TrackerEventError;

    fn try_from(value: ReplayTrackerEEventId) -> Result<Self, Self::Error> {
        match value {
            ReplayTrackerEEventId::EPlayerStats(_)
            | ReplayTrackerEEventId::EUnitOwnerChange(_)
            | ReplayTrackerEEventId::EUpgrade(_)
            | ReplayTrackerEEventId::EPlayerSetup(_) => {
                Err(TrackerEventError::UnsupportedEventType)
            }
            ReplayTrackerEEventId::EUnitBorn(e) => Ok(e.to_versionless()?),
            ReplayTrackerEEventId::EUnitDied(e) => Ok(e.to_versionless()?),
            ReplayTrackerEEventId::EUnitTypeChange(e) => Ok(e.to_versionless()?),
            ReplayTrackerEEventId::EUnitInit(e) => Ok(e.to_versionless()?),
            ReplayTrackerEEventId::EUnitDone(e) => Ok(e.to_versionless()?),
            ReplayTrackerEEventId::EUnitPosition(e) => Ok(e.to_versionless()?),
        }
    }
}

impl ReplayTrackerSUnitBornEvent {
    pub fn to_versionless(self) -> Result<ReplayTrackerEvent, TrackerEventError> {
        let creator_ability_name = if let Some(val) = self.m_creator_ability_name {
            Some(str::from_utf8(&val)?.to_string())
        } else {
            None
        };
        Ok(ReplayTrackerEvent::UnitBorn(UnitBornEvent {
            unit_tag_index: self.m_unit_tag_index,
            unit_tag_recycle: self.m_unit_tag_recycle,
            unit_type_name: str::from_utf8(&self.m_unit_type_name)?.to_string(),
            control_player_id: self.m_control_player_id,
            upkeep_player_id: self.m_upkeep_player_id,
            x: self.m_x,
            y: self.m_y,
            creator_unit_tag_index: self.m_creator_unit_tag_index,
            creator_unit_tag_recycle: self.m_creator_unit_tag_recycle,
            creator_ability_name,
        }))
    }
}

impl ReplayTrackerSUnitDiedEvent {
    pub fn to_versionless(self) -> Result<ReplayTrackerEvent, TrackerEventError> {
        Ok(ReplayTrackerEvent::UnitDied(UnitDiedEvent {
            unit_tag_index: self.m_unit_tag_index,
            unit_tag_recycle: self.m_unit_tag_recycle,
            killer_player_id: self.m_killer_player_id,
            x: self.m_x,
            y: self.m_y,
            killer_unit_tag_index: self.m_killer_unit_tag_index,
            killer_unit_tag_recycle: self.m_killer_unit_tag_recycle,
        }))
    }
}

impl ReplayTrackerSUnitTypeChangeEvent {
    pub fn to_versionless(self) -> Result<ReplayTrackerEvent, TrackerEventError> {
        Ok(ReplayTrackerEvent::UnitTypeChange(UnitTypeChangeEvent {
            unit_tag_index: self.m_unit_tag_index,
            unit_tag_recycle: self.m_unit_tag_recycle,
            unit_type_name: str::from_utf8(&self.m_unit_type_name)?.to_string(),
        }))
    }
}

impl ReplayTrackerSUnitInitEvent {
    pub fn to_versionless(self) -> Result<ReplayTrackerEvent, TrackerEventError> {
        Ok(ReplayTrackerEvent::UnitInit(UnitInitEvent {
            unit_tag_index: self.m_unit_tag_index,
            unit_tag_recycle: self.m_unit_tag_recycle,
            unit_type_name: str::from_utf8(&self.m_unit_type_name)?.to_string(),
            control_player_id: self.m_control_player_id,
            upkeep_player_id: self.m_upkeep_player_id,
            x: self.m_x,
            y: self.m_y,
        }))
    }
}

impl ReplayTrackerSUnitDoneEvent {
    pub fn to_versionless(self) -> Result<ReplayTrackerEvent, TrackerEventError> {
        Ok(ReplayTrackerEvent::UnitDone(UnitDoneEvent {
            unit_tag_index: self.m_unit_tag_index,
            unit_tag_recycle: self.m_unit_tag_recycle,
        }))
    }
}

impl ReplayTrackerSUnitPositionsEvent {
    pub fn to_versionless(self) -> Result<ReplayTrackerEvent, TrackerEventError> {
        Ok(ReplayTrackerEvent::UnitPosition(UnitPositionsEvent {
            first_unit_index: self.m_first_unit_index,
            items: self.m_items,
        }))
    }
}
    "#.as_bytes())?;
    Ok(())
}

/// Creates a Rust Enum out of a EnumType type.
/// The enum variants do not contain internal types
#[tracing::instrument(
    level = "debug",
    skip(
        output,
        proto_mod,
        proto_type_def,
        enum_parse_impl_def,
        type_impl_def,
        enum_tags
    )
)]
pub fn gen_proto_enum_code(
    output: &mut File,
    proto_mod: &Value,
    mut proto_type_def: String,
    mut enum_parse_impl_def: String,
    mut type_impl_def: String,
    enum_tags: &HashMap<String, String>,
    decoder_type: DecoderType,
) -> std::io::Result<()> {
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
            },\n\
          }",
    ); // close the match

    enum_parse_impl_def.push_str("}\n"); // Close function definition
    type_impl_def.push_str(&enum_parse_impl_def);

    proto_type_def.push_str("}\n"); // Close struct definition
    type_impl_def.push_str("}\n"); // Close impl definition
                                   //
    output.write_all(format!("\n{}", proto_type_def).as_bytes())?;
    output.write_all(format!("{}\n", type_impl_def).as_bytes())?;
    Ok(())
}

/// Creates a Rust Int out of a IntType type.
/// The struct contains an interval .value field.
#[tracing::instrument(
    level = "debug",
    skip(output, proto_type_def, int_parse_impl_def, type_impl_def,)
)]
pub fn gen_proto_int_code(
    output: &mut File,
    mut proto_type_def: String,
    mut int_parse_impl_def: String,
    mut type_impl_def: String,
    decoder_type: DecoderType,
) -> std::io::Result<()> {
    // The int_parse_impl_def already contains the int parsing functionality.
    // This is untested.
    proto_type_def.push_str(&format!("    value: i64,"));
    int_parse_impl_def.push_str("}\n"); // Close function definition
    type_impl_def.push_str(&int_parse_impl_def);

    proto_type_def.push_str("}\n"); // Close struct definition
    type_impl_def.push_str("}\n"); // Close impl definition
                                   //
    output.write_all(format!("\n{}", proto_type_def).as_bytes())?;
    output.write_all(format!("{}\n", type_impl_def).as_bytes())?;
    Ok(())
}

/// Traverses the current depth looking for StructTypes that we will need later to identify field
/// tags.
#[tracing::instrument(level = "debug", skip(enum_tags, mod_decl))]
fn fill_module_decl_structs(enum_tags: &mut HashMap<String, String>, mod_decl: &Value) {
    if mod_decl["type_info"]["type"] != "StructType" {
        tracing::warn!(
            "Skipping mod_decl {} with type: {}",
            mod_decl["fullname"],
            mod_decl["type_info"]["type"]
        );
        // Skip non StructType
        return;
    }
    tracing::info!(
        "Processing mod_decl {} with type: {}",
        mod_decl["fullname"],
        mod_decl["type_info"]["type"]
    );
    let field_array = mod_decl["type_info"]["fields"].as_array().unwrap();
    for field in field_array {
        let nnet_field_type = field["type"].as_str().unwrap();
        if nnet_field_type != "ConstDecl" {
            tracing::info!("Skipping non ConstDecl field, typed: {nnet_field_type}");
            continue;
        }
        let type_full_name = field["type_info"]["fullname"].as_str().unwrap().to_string();
        if field["value"]["type"] != "IdentifierExpr" {
            tracing::error!(
                "Unknown value.type for ConstDecl: {}",
                field["value"]["type"]
            );
            panic!(
                "Unknown value.type for ConstDecl: {}",
                field["value"]["type"]
            );
        }
        let type_variant_value = field["value"]["fullname"].as_str().unwrap().to_string();
        let type_variant_value =
            str_nnet_name_to_rust_name(type_variant_value.replace(&type_full_name, ""));
        let type_variant = str_nnet_name_to_rust_name(type_full_name);
        tracing::info!(
            "Found ConstDecl for fullname: {} should be referenced by: {}::{}",
            mod_decl["fullname"],
            type_variant,
            type_variant_value,
        );
        let key = format!("{}::{}", type_variant, type_variant_value);
        let new_val = str_nnet_name_to_rust_name(mod_decl["fullname"].to_string());
        if let Some(existing_tags) = enum_tags.get(&key) {
            tracing::info!(
                "enum_tags already contains key '{}' with values: '{}', ignoring '{}'",
                key,
                existing_tags,
                new_val,
            );
        } else {
            tracing::info!("enum_tags Init key '{}' -> '{}'", key, new_val);
            enum_tags.insert(key.clone(), new_val);
        }
    }
}

/// Reads a protocol.json file and generates a Rust module for it.
#[tracing::instrument(level = "debug", skip(path, output_name))]
pub fn generate_code_for_protocol(path: &str, output_name: &str) -> std::io::Result<()> {
    let mut output = File::create(output_name)?;
    let mut f = File::open(path).unwrap();
    let mut buffer: Vec<u8> = vec![];
    // read the whole file
    f.read_to_end(&mut buffer).unwrap();

    let mut proto_json: Value = serde_json::from_slice(&buffer).unwrap();

    output.write_all(format!("//! Generated code from source: {}\n", path).as_bytes())?;
    output.write_all(
        b"use crate::*;\n\
        use nom_mpq::parser::peek_hex;\n\
        use std::convert::TryFrom;\n\
        use nom::*;\n\
        pub mod byte_aligned {\n\
        ",
    )?;
    // So far only one root module has been observed.
    let mut proto_modules = proto_json["modules"][0]["decls"].take();
    let proto_modules_arr = proto_modules
        .as_array_mut()
        .expect("'.modules[0].decls' not found in json");
    let mut enum_tags: HashMap<String, String> = HashMap::new();
    // Traverse the JSON structure, looking for ConstDecl's that behave like
    // tags that would signal what type of struct follows.
    // FIRST PASS.
    tracing::info!("Collecting enum tags");
    for proto_mod in proto_modules_arr.iter() {
        // There is different depths for different type declarations:
        // Each '.' represents a level deep in the hierarchy:
        // NNet.Game.EEventId is an enum we must parse 3 levels in.
        // NNet.Replay.Tracker.EEventId is an enum we must parse 4 levels in.
        if proto_mod["fullname"] == "NNet.Replay" || proto_mod["fullname"] == "NNet.Game" {
            let sub_mods_arr = proto_mod["decls"].as_array().expect(&format!(
                "{} should have '.decls' array",
                proto_mod["fullname"]
            ));
            for sub_mod in sub_mods_arr.iter() {
                if let Some(sub_mod_decls_array) = sub_mod["decls"].as_array() {
                    // traverse inside the current type's decls.
                    for sub_mod_decl in sub_mod_decls_array {
                        fill_module_decl_structs(&mut enum_tags, sub_mod_decl);
                    }
                } else {
                    fill_module_decl_structs(&mut enum_tags, &sub_mod);
                }
            }
        }
    }
    tracing::info!("Collected enum tags: {:?}", enum_tags);
    // SECOND PASS. Collect ByteAligned Records. These are for the VersionedDecoder
    for proto_mod in proto_modules_arr.iter_mut() {
        if proto_mod["fullname"] == "NNet.SVersion" {
            gen_proto_code(
                path,
                &mut output,
                proto_mod,
                &enum_tags,
                DecoderType::ByteAligned,
            )?;
        }
        if proto_mod["fullname"] == "NNet.SVarUint32" {
            gen_proto_code(
                path,
                &mut output,
                proto_mod,
                &enum_tags,
                DecoderType::ByteAligned,
            )?;
        }
        if proto_mod["fullname"] == "NNet.SMD5" {
            gen_proto_code(
                path,
                &mut output,
                proto_mod,
                &enum_tags,
                DecoderType::ByteAligned,
            )?;
        }
        if proto_mod["fullname"] == "NNet.Replay" {
            let mut replay_mods = proto_mod["decls"].take();
            let replay_mods_arr = replay_mods
                .as_array_mut()
                .expect("NNet.Replay should have '.decls' array");
            for replay_mod in replay_mods_arr.iter_mut() {
                tracing::info!("Processing: {}", replay_mod["fullname"]);
                if replay_mod["fullname"] == "NNet.Replay.SHeader" {
                    gen_proto_code(
                        path,
                        &mut output,
                        replay_mod,
                        &enum_tags,
                        DecoderType::ByteAligned,
                    )?;
                }
                if replay_mod["fullname"] == "NNet.Replay.Tracker" {
                    let mut tracker_mods = replay_mod["decls"].take();
                    let tracker_mods_arr = tracker_mods
                        .as_array_mut()
                        .expect("NNet.Replay.Tracker should have '.decls' array");
                    for tracker_mod in tracker_mods_arr {
                        tracing::info!("Processing: {}", tracker_mod["fullname"]);
                        let tracker_mod_fullname = tracker_mod["fullname"].as_str().unwrap();
                        if tracker_mod_fullname.starts_with("NNet.Replay.Tracker.S")
                            || tracker_mod_fullname == "NNet.Replay.Tracker.EEventId"
                        {
                            gen_proto_code(
                                path,
                                &mut output,
                                tracker_mod,
                                &enum_tags,
                                DecoderType::ByteAligned,
                            )?;
                        }
                    }
                }
            }
        }
    }
    // Close the mod byte_aligned
    output.write_all(
        b"}
        pub mod bit_packed {\n\
        ",
    )?;
    // THIRD PASS. Collect ByteAligned Records. These are for the BitPacked entries
    for proto_mod in proto_modules_arr.iter_mut() {
        if proto_mod["fullname"] == "NNet.SVersion" {
            gen_proto_code(
                path,
                &mut output,
                proto_mod,
                &enum_tags,
                DecoderType::BitPacked,
            )?;
        }
        if proto_mod["fullname"] == "NNet.SVarUint32" {
            gen_proto_code(
                path,
                &mut output,
                proto_mod,
                &enum_tags,
                DecoderType::BitPacked,
            )?;
        }
        if proto_mod["fullname"] == "NNet.SMD5" {
            gen_proto_code(
                path,
                &mut output,
                proto_mod,
                &enum_tags,
                DecoderType::BitPacked,
            )?;
        }
        if proto_mod["fullname"] == "NNet.Game" {
            let mut game_mods = proto_mod["decls"].take();
            let game_mods_arr = game_mods
                .as_array_mut()
                .expect("NNet.Replay should have '.decls' array");
            for game_mod in game_mods_arr.iter_mut() {
                tracing::info!("Processing: {}", game_mod["fullname"]);
                gen_proto_code(
                    path,
                    &mut output,
                    game_mod,
                    &enum_tags,
                    DecoderType::BitPacked,
                )?;
            }
        }
    }
    output.write_all(b"}")?; // Close the mod bit_packed
    generate_convert_into_versionless(&mut output)?;
    Ok(())
}
