//! Defines the Prototype Morph utilities, handles file writing, calling decoder types, stores
//! intermediate state mostly.

use super::decoder_type::DecoderType;
use convert_case::{Case, Casing};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// Contains the protocol definition in Rust types and nom parser that is progressively being
/// processed.
#[derive(Debug)]
pub struct ProtoMorphist {
    /// THe path of the json protocol definition
    json_proto_path: PathBuf,
    /// The protocol number
    proto_num: u64,
    /// The output file to write the protocol translation to.
    output: File,
    /// The json file tha contains the protocol definition.
    proto_json: Value,
    /// The collected enum_tags, these is a translation table between the json definition and types
    /// generated for rust
    enum_tags: HashMap<String, String>,
}

impl ProtoMorphist {
    /// Reads a protocol.json at `source_path` and generates a Rust module for it.
    #[tracing::instrument(level = "debug", skip(source_path, output_name))]
    pub fn new(source_path: &str, output_name: &str) -> std::io::Result<Self> {
        let json_proto_path = PathBuf::from(source_path);
        // Try to get the numeric part of the protocol.
        // The source path is called like: ./s2protocol/json/protocol87702.json
        let mut proto_num: String = format!("{:?}", json_proto_path.file_name().unwrap());
        proto_num.retain(|c| c >= '0' && c <= '9');
        let proto_num = proto_num.parse().unwrap();
        let output = File::create(output_name)?;
        let mut json_definition_file = File::open(&json_proto_path).unwrap();
        let mut json_content_buffer: Vec<u8> = vec![];
        // read the whole file
        json_definition_file
            .read_to_end(&mut json_content_buffer)
            .unwrap();

        Ok(Self {
            json_proto_path,
            proto_num,
            output,
            proto_json: serde_json::from_slice(&json_content_buffer).unwrap(),
            enum_tags: HashMap::new(),
        })
    }

    /// Creates the byte_aligned module imports.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn initialize_byte_aligned_crate_imports(&mut self) -> std::io::Result<()> {
        self.output.write_all(
            format!(
                "//! Generated code from source: {}\n",
                self.json_proto_path.display()
            )
            .as_bytes(),
        )?;
        self.output.write_all(
            b"use crate::*;\n\
              use nom_mpq::parser::peek_hex;\n\
              use nom::*;\n",
        )
    }

    /// Creates the bit packed module imports.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn initialize_bit_packed_crate_imports(&mut self) -> std::io::Result<()> {
        self.output.write_all(
            format!(
                "//! Generated code from source: {}\n",
                self.json_proto_path.display()
            )
            .as_bytes(),
        )?;
        self.output.write_all(
            b"use crate::*;\n\
              use nom::*;\n",
        )
    }

    /// Opens the sub-module for byte aligned structs and parsing.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn open_byte_aligned_mod(&mut self) -> std::io::Result<()> {
        self.output.write_all(b"pub mod byte_aligned {\n")
    }

    /// Closes the sub-module for byte aligned structs and parsing.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn close_byte_aligned_mod(&mut self) -> std::io::Result<()> {
        self.output.write_all(b"}\n")
    }

    /// Opens the sub-module for bit packed structs and parsing.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn open_bit_packed_mod(&mut self) -> std::io::Result<()> {
        self.output.write_all(b"pub mod bit_packed {\n")
    }

    /// Closes the sub-module for bit packed structs and parsing.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn close_bit_packed_mod(&mut self) -> std::io::Result<()> {
        self.output.write_all(b"}\n")
    }

    /// Collects the ConstDecls tags that prepend/identify type data.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn collect_enum_tags(&mut self) -> std::io::Result<()> {
        // So far only one root module has been observed.
        let proto_modules_arr = self.proto_json["modules"][0]["decls"]
            .as_array()
            .expect("'.modules[0].decls' not found in json")
            .clone();
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
                            self.fill_module_decl_structs(sub_mod_decl);
                        }
                    } else {
                        self.fill_module_decl_structs(&sub_mod);
                    }
                }
            }
        }
        tracing::info!("Collected enum tags: {:?}", self.enum_tags);
        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub fn gen_byte_aligned_mod(&mut self) -> std::io::Result<()> {
        self.open_byte_aligned_mod()?;
        self.initialize_byte_aligned_crate_imports()?;
        let proto_modules_arr = self.proto_json["modules"][0]["decls"]
            .as_array()
            .expect("'.modules[0].decls' not found in json")
            .clone();
        // SECOND PASS. Collect ByteAligned Records. These are for the VersionedDecoder
        for proto_mod in proto_modules_arr.iter() {
            let full_name = proto_mod["fullname"]
                .as_str()
                .expect("module decl should have fullname on fields");
            if full_name == "NNet.SVersion" {
                self.gen_proto_code(proto_mod, DecoderType::ByteAligned)?;
            }
            if full_name == "NNet.SVarUint32" {
                self.gen_proto_code(proto_mod, DecoderType::ByteAligned)?;
            }
            if full_name == "NNet.SMD5" {
                self.gen_proto_code(proto_mod, DecoderType::ByteAligned)?;
            }
            if full_name == "NNet.EObserve" {
                self.gen_proto_code(proto_mod, DecoderType::ByteAligned)?;
            }
            if full_name == "NNet.Replay" || full_name == "NNet.Game" {
                let replay_mods_arr = proto_mod["decls"]
                    .as_array()
                    .expect("NNet.Replay should have '.decls' array");
                for replay_mod in replay_mods_arr {
                    tracing::info!("Processing: {}", replay_mod["fullname"]);
                    if replay_mod["fullname"] == "NNet.Replay.SHeader" {
                        self.gen_proto_code(replay_mod, DecoderType::ByteAligned)?;
                    }
                    if replay_mod["fullname"] == "NNet.Replay.Tracker" {
                        let tracker_mods_arr = replay_mod["decls"]
                            .as_array()
                            .expect("NNet.Replay.Tracker should have '.decls' array");
                        for tracker_mod in tracker_mods_arr {
                            tracing::info!("Processing: {}", tracker_mod["fullname"]);
                            let tracker_mod_fullname = tracker_mod["fullname"].as_str().unwrap();
                            if tracker_mod_fullname.starts_with("NNet.Replay.Tracker.S")
                                || tracker_mod_fullname == "NNet.Replay.Tracker.EEventId"
                            {
                                self.gen_proto_code(tracker_mod, DecoderType::ByteAligned)?;
                            }
                        }
                    }
                    if replay_mod["fullname"] == "NNet.Game.CPlayerDetailsArray"
                        || replay_mod["fullname"] == "NNet.Game.SDetails"
                        || replay_mod["fullname"] == "NNet.Game.SPlayerDetails"
                        || replay_mod["fullname"] == "NNet.Game.SThumbnail"
                        || replay_mod["fullname"] == "NNet.Game.EGameSpeed"
                        || replay_mod["fullname"] == "NNet.Game.EResultDetails"
                        || replay_mod["fullname"] == "NNet.Game.SToonNameDetails"
                        || replay_mod["fullname"] == "NNet.Game.SColor"
                    {
                        self.gen_proto_code(replay_mod, DecoderType::ByteAligned)?;
                    }
                }
            }
        }

        self.close_byte_aligned_mod()
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub fn gen_bit_packed_mod(&mut self) -> std::io::Result<()> {
        self.open_bit_packed_mod()?;
        self.initialize_bit_packed_crate_imports()?;
        let proto_modules_arr = self.proto_json["modules"][0]["decls"]
            .as_array()
            .expect("'.modules[0].decls' not found in json")
            .clone();
        // THIRD PASS. Collect BitPacked Types
        for proto_mod in proto_modules_arr.iter() {
            let full_name = proto_mod["fullname"]
                .as_str()
                .expect("module decl should have fullname on fields");
            if full_name.starts_with("NNet.uint") {
                // Fetch all unsigned integer definitions
                self.gen_proto_code(proto_mod, DecoderType::BitPacked)?;
            }
            if full_name.starts_with("NNet.int") {
                // Fetch all signed integer definitions
                self.gen_proto_code(proto_mod, DecoderType::BitPacked)?;
            }
            if full_name == "NNet.SVarUint32"
                || full_name == "NNet.TUserId"
                || full_name == "NNet.ELeaveReason"
                || full_name == "NNet.CToonHandle"
                || full_name == "NNet.CFilePath"
                || full_name == "NNet.EObserve"
                || full_name == "NNet.CUserName"
                || full_name == "NNet.CClanTag"
                || full_name == "NNet.CUserInitialDataArray"
                || full_name == "NNet.EReconnectStatus"
                || full_name == "NNet.TUserCount"
                || full_name == "NNet.CCommanderHandle"
                || full_name == "NNet.CMountHandle"
                || full_name == "NNet.CSkinHandle"
                || full_name == "NNet.CHeroHandle"
                || full_name == "NNet.TRacePreference"
                || full_name == "NNet.CArtifactHandle"
                || full_name == "NNet.TRaceCount"
                || full_name == "NNet.TRaceId"
                || full_name == "NNet.SUserInitialData"
                || full_name == "NNet.CAllowedObserveTypes"
                || full_name == "NNet.CAllowedRaces"
                || full_name == "NNet.CCacheHandle"
                || full_name == "NNet.TTeamPreference"
            {
                self.gen_proto_code(proto_mod, DecoderType::BitPacked)?;
            }
            if full_name == "NNet.Replay" {
                let replay_mods_arr = proto_mod["decls"]
                    .as_array()
                    .expect("NNet.Replay should have '.decls' array");
                for replay_mod in replay_mods_arr {
                    tracing::info!("Processing: {}", replay_mod["fullname"]);
                    if replay_mod["fullname"] == "NNet.Replay.SGameUserId" {
                        self.gen_proto_code(replay_mod, DecoderType::BitPacked)?;
                    }
                }
            }
            if full_name == "NNet.SVersion" {
                self.gen_proto_code(proto_mod, DecoderType::BitPacked)?;
            }
            if full_name == "NNet.SMD5" {
                self.gen_proto_code(proto_mod, DecoderType::BitPacked)?;
            }
            if full_name == "NNet.Game" {
                let game_mods_arr = proto_mod["decls"]
                    .as_array()
                    .expect("NNet.Game should have '.decls' array");
                for game_mod in game_mods_arr {
                    tracing::info!("Processing: {}", game_mod["fullname"]);
                    if game_mod["fullname"] == "NNet.Game.ESenders" {
                        continue;
                    }
                    self.gen_proto_code(game_mod, DecoderType::BitPacked)?;
                    if game_mod["fullname"] == "NNet.Game.STriggerDialogControlEvent" {
                        let ctrl_event_fields = game_mod["type_info"]["fields"].as_array().expect(
                            "NNet.Game.STriggerDialogControlEvent should have '.type_info.fields' array",
                        );
                        for ctrl_event_field in ctrl_event_fields {
                            if ctrl_event_field["name"] == "m_eventData" {
                                self.gen_proto_code(ctrl_event_field, DecoderType::BitPacked)?;
                            }
                        }
                    }
                }
            }
        }
        self.close_bit_packed_mod()
    }

    /// Traverses the current depth looking for StructTypes that we will need later to identify field
    /// tags.
    #[tracing::instrument(level = "debug", skip(self, mod_decl))]
    fn fill_module_decl_structs(&mut self, mod_decl: &Value) {
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
            if let Some(existing_tags) = self.enum_tags.get(&key) {
                tracing::info!(
                    "enum_tags already contains key '{}' with values: '{}', ignoring '{}'",
                    key,
                    existing_tags,
                    new_val,
                );
            } else {
                tracing::info!("enum_tags Init key '{}' -> '{}'", key, new_val);
                self.enum_tags.insert(key.clone(), new_val);
            }
        }
    }

    /// Reads a protocol.json at `source_path` and generates a Rust module for it.
    #[tracing::instrument(level = "debug", skip(source_path, output_name))]
    pub fn gen(source_path: &str, output_name: &str) -> std::io::Result<()> {
        let mut generator = Self::new(source_path, output_name)?;
        generator.collect_enum_tags()?;
        generator
            .output
            .write_all(b"pub mod events;\npub use events::*;\n")?;
        generator.gen_byte_aligned_mod()?;
        generator.gen_bit_packed_mod()
    }

    /// Attempts to generate code for a structure read from the proto json. It may branch into Structs
    /// or Enums.
    #[tracing::instrument(level = "debug", skip(self, proto_mod))]
    pub fn gen_proto_code(
        &mut self,
        proto_mod: &Value,
        decoder_type: DecoderType,
    ) -> std::io::Result<()> {
        // The pub <Type> <Name> {}
        let field_type = proto_mod["type"].as_str().unwrap();
        if field_type == "ConstDecl" {
            tracing::info!("Skipping ConstDecl field, typed: {field_type}");
            return Ok(());
        }
        let proto_unit_type = proto_mod["type_info"]["type"]
            .as_str()
            .expect(".type_info.type field expected in mod");
        let proto_unit_type_name = match proto_mod["fullname"].as_str() {
            Some(val) => str_nnet_name_to_rust_name(val.to_string()),
            None => {
                // Special case where the m_eventData is a bit too complex in its depth
                assert!(proto_mod["name"] == "m_eventData");
                proto_nnet_name_to_rust_name(&proto_mod["name"])
            }
        };
        tracing::debug!(
            "Analyzing proto_unit_type_name {proto_unit_type_name}: '{proto_unit_type}'",
        );
        let mut proto_type_def = open_type_def_skel(&proto_unit_type_name, proto_unit_type);
        // The impl <Name> {}
        let mut type_impl_def = open_type_impl_def(&proto_unit_type_name);
        // The method for parsing all the fields into the struct as a whole.
        if proto_unit_type == "StructType" {
            let not_const_num_fields = proto_mod["type_info"]["fields"]
                .as_array()
                .expect("type_info should have .fields")
                .iter()
                .filter(|x| x["type"] != "ConstDecl")
                .collect::<Vec<_>>()
                .len();
            let struct_parse_impl_def: String = decoder_type.open_struct_main_parse_fn(
                self.proto_num,
                &proto_unit_type_name,
                not_const_num_fields,
            );
            decoder_type.gen_proto_struct_code(
                proto_mod,
                &mut proto_type_def,
                struct_parse_impl_def,
                &mut type_impl_def,
            );
            type_impl_def.push_str(&decoder_type.close_struct_main_parse_fn());
        } else if proto_unit_type == "ChoiceType" {
            let num_fields = proto_mod["type_info"]["fields"]
                .as_array()
                .expect("type_info should have .fields")
                .len();
            let enum_parse_impl_def = decoder_type.open_choice_main_parse_fn(
                self.proto_num,
                &proto_unit_type_name,
                num_fields,
            );
            decoder_type.gen_proto_choice_code(
                proto_mod,
                &mut proto_type_def,
                enum_parse_impl_def,
                &mut type_impl_def,
            );
            type_impl_def.push_str(&decoder_type.close_choice_main_parse_fn());
        } else if proto_unit_type == "EnumType" {
            let num_fields = proto_mod["type_info"]["fields"]
                .as_array()
                .expect("type_info should have .fields")
                .len();
            let enum_parse_impl_def = decoder_type.open_enum_main_parse_fn(
                self.proto_num,
                &proto_unit_type_name,
                num_fields,
            );
            decoder_type.gen_proto_enum_code(
                proto_mod,
                &mut proto_type_def,
                enum_parse_impl_def,
                &mut type_impl_def,
                &self.enum_tags,
            );
            type_impl_def.push_str(&decoder_type.close_enum_main_parse_fn());
        } else if proto_unit_type == "IntType" || proto_unit_type == "InumType" {
            let int_parse_impl_def = decoder_type.open_int_main_parse_fn(
                self.proto_num,
                &proto_mod["type_info"]["bounds"],
                &proto_unit_type_name,
            );
            decoder_type.gen_proto_int_code(
                &mut proto_type_def,
                int_parse_impl_def,
                &mut type_impl_def,
            );
            type_impl_def.push_str(&decoder_type.close_int_main_parse_fn());
        } else if proto_unit_type == "BitArrayType" {
            let bit_packed_parse_impl_def = decoder_type.open_bit_array_main_parse_fn(
                self.proto_num,
                &proto_mod["type_info"]["bounds"],
                &proto_unit_type_name,
            );
            decoder_type.gen_proto_bit_array_code(
                &mut proto_type_def,
                bit_packed_parse_impl_def,
                &mut type_impl_def,
            );
            type_impl_def.push_str(&decoder_type.close_bit_array_main_parse_fn());
        } else if proto_unit_type == "UserType" {
            let user_type_parse_impl_def = decoder_type.open_user_type_main_parse_fn(
                self.proto_num,
                &proto_mod["type_info"]["fullname"],
                &proto_unit_type_name,
            );
            decoder_type.gen_proto_user_type_code(
                &mut proto_type_def,
                user_type_parse_impl_def,
                &mut type_impl_def,
                &proto_mod["type_info"]["fullname"],
            );
            type_impl_def.push_str(&decoder_type.close_user_type_main_parse_fn());
        } else if proto_unit_type == "BlobType" {
            let blob_type_parse_impl_def = decoder_type.open_blob_main_parse_fn(
                self.proto_num,
                &proto_mod["type_info"]["bounds"],
                &proto_unit_type_name,
            );
            decoder_type.gen_proto_blob_code(
                &mut proto_type_def,
                blob_type_parse_impl_def,
                &mut type_impl_def,
            );
            type_impl_def.push_str(&decoder_type.close_blob_main_parse_fn());
        } else if proto_unit_type == "StringType" {
            let string_type_parse_impl_def = decoder_type.open_string_main_parse_fn(
                self.proto_num,
                &proto_mod["type_info"]["bounds"],
                &proto_unit_type_name,
            );
            decoder_type.gen_proto_blob_code(
                &mut proto_type_def,
                string_type_parse_impl_def,
                &mut type_impl_def,
            );
            type_impl_def.push_str(&decoder_type.close_string_main_parse_fn());
        } else if proto_unit_type == "ArrayType" || proto_unit_type == "DynArrayType" {
            // Note: This type seems to be handled here only be for BitPacked and only for GameEvents
            let internal_ty =
                proto_nnet_name_to_rust_name(&proto_mod["type_info"]["element_type"]["fullname"]);
            let array_type_parse_impl_def = decoder_type.open_array_main_parse_fn(
                self.proto_num,
                &proto_mod["type_info"]["bounds"],
                &proto_unit_type_name,
                &internal_ty,
            );
            decoder_type.gen_proto_array_code(
                &mut proto_type_def,
                array_type_parse_impl_def,
                &mut type_impl_def,
                &internal_ty,
            );
            type_impl_def.push_str(&decoder_type.close_array_main_parse_fn());
        } else {
            tracing::error!("Unhandled protocol unit type: {:?}", proto_unit_type);
        }
        proto_type_def.push_str(&close_type_def_skel());
        type_impl_def.push_str(&close_type_impl_def());
        //
        self.output
            .write_all(format!("\n{}", proto_type_def).as_bytes())?;
        self.output
            .write_all(format!("{}\n", type_impl_def).as_bytes())?;
        Ok(())
    }
}

/// Generates the start of a struct definition.
pub fn open_struct_type_def_skel(name: &str) -> String {
    format!("#[derive(Debug, PartialEq, Clone)]\npub struct {name}",)
}

/// Generates the start of an enum/choice definition.
pub fn open_enum_type_def_skel(name: &str) -> String {
    format!("#[derive(Debug, PartialEq, Clone)]\npub enum {name}",)
}

/// Generates the start of a type alias, tho for now this is being used as a one-element struct.
pub fn open_single_value_type_def_skel(name: &str) -> String {
    format!("#[derive(Debug, PartialEq, Clone)]\npub struct {name} ",)
}

/// Initializes a type impl block to contain the methods
pub fn open_type_impl_def(name: &str) -> String {
    format!("impl {name} {{\n",)
}

/// Closes the type impl scope.
pub fn close_type_impl_def() -> String {
    format!("}}\n")
}

/// Converts from a &Value that must be a str into a Rust-friendly type
pub fn proto_nnet_name_to_rust_name(nnet_name: &Value) -> String {
    str_nnet_name_to_rust_name(nnet_name.as_str().unwrap().to_string())
}

/// Converts from a String Rust-friendly type, removing dots, "NNet" and making the case Pascal.
pub fn str_nnet_name_to_rust_name(input: String) -> String {
    input
        .replace(".", "")
        .replace("NNet", "")
        .trim_matches('"')
        .to_case(Case::Pascal)
}

/// Generates the start of an type definition skeleton.
#[tracing::instrument(level = "debug")]
pub fn open_type_def_skel(name: &str, unit_ty: &str) -> String {
    // tagged_blob().
    let mut res = match unit_ty {
        "StructType" => open_struct_type_def_skel(name),
        "EnumType" => open_enum_type_def_skel(name),
        "ChoiceType" => open_enum_type_def_skel(name),
        "IntType" => open_struct_type_def_skel(name),
        "BitArrayType" => open_struct_type_def_skel(name),
        "UserType" => open_struct_type_def_skel(name),
        "StringType" => open_struct_type_def_skel(name),
        "ArrayType" => open_struct_type_def_skel(name),
        "DynArrayType" => open_struct_type_def_skel(name),
        "InumType" => open_struct_type_def_skel(name),
        "BlobType" => open_struct_type_def_skel(name),
        _ => panic!("Unknown unit type: {unit_ty}"),
    };
    res.push_str(" {\n");
    res
}

/// Closes the struct definition skeleton.
pub fn close_type_def_skel() -> String {
    String::from("}\n")
}
