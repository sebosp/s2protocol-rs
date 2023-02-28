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
pub mod proto_morphist;
pub mod versionless;
pub use decoder_type::DecoderType;
pub use proto_morphist::ProtoMorphist;
use serde_json::Value;
use std::convert::From;
use std::fs::File;
use std::io::prelude::*;
use std::str;

/// Generates the main parse function for a byte aligned choice type
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
        let mut morph = decoder_type.from_nnet_name(proto_field_type_info);
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

    proto_type_def.push_str(&close_gen_type_def_skel());
    type_impl_def.push_str(&close_gen_type_impl_def()); // Close impl definition
                                                        //
    output.write_all(format!("\n{}", proto_type_def).as_bytes())?;
    output.write_all(format!("{}\n", type_impl_def).as_bytes())?;
    Ok(())
}
