//! Parsing the DocumentHeader.
//!
//! There are a few fields that are related to MapInfo/Player01 name being Player\ 1
//! these are ignored.
//!

use crate::{S2ProtoResult, S2ProtocolError, dbg_peek_hex};
use nom::bytes::complete::*;
use nom::number::complete::*;
use nom_mpq::MPQ;
use nom_mpq::parser::peek_hex;
use tracing::instrument;

#[derive(Default, Debug, Clone)]
pub struct DocumentHeader {
    pub maybe_dimension_x1: i32,
    pub maybe_dimension_y1: i32,
    pub some_epoch_1: i32,
    pub some_epoch_2: i32,
    pub mod_info: String,
    // Aka the map title.
    pub name: String,
    /// A long description of the map.
    pub description_long: String,
    /// A short description of the map, in the few files I've checked it's empty.
    pub description_short: String,
}

impl DocumentHeader {
    /// Sets a field in the struct being read by the DocumentHeader file.
    pub fn set_field_by_name(&mut self, name: &str, value: String) {
        // Player00: Neutral
        // Player01: Player 1
        // Player02: Player 2
        // Player03: Hostile
        match name {
            "DocInfo/Name" => self.name = value,
            "DocInfo/DescLong" => self.description_long = value,
            "DocInfo/DescShort" => self.description_short = value,
            "MapInfo/Player00/Name"
            | "MapInfo/Player01/Name"
            | "MapInfo/Player02/Name"
            | "MapInfo/Player03/Name"
            // These values are seen on Heart Of The Swarm it seems, may be related to GameHeart.
            // Thus far the information I am looking for appears in the Legacy of the Void and the
            // structs are easy to parse, I hope I don't have to try to understand the Swarm one,
            // seems more complex:
            | "DocInfo/Screenshot01"
            | "DocInfo/Screenshot02"
            | "DocInfo/Screenshot03"
            | "DocInfo/HowToPlayBasic00"
            | "DocInfo/HowToPlayBasic01"
            | "DocInfo/HowToPlayBasic02"
            | "DocInfo/HowToPlayAdvanced00"
            | "DocInfo/HowToPlayAdvanced01"
            | "DocInfo/HowToPlayAdvanced02" => tracing::debug!("Ignoring {name}"),
            _ => {
                tracing::warn!("Unknown field name {name}")
            }
        }
    }

    #[instrument(skip(mpq, file_contents))]
    pub fn from_mpq(mpq: &MPQ, file_contents: &[u8]) -> Result<Self, S2ProtocolError> {
        let (_, document_header_sector) =
            mpq.read_mpq_file_sector("DocumentHeader", false, file_contents)?;
        let (_, document_header) = Self::parse(&document_header_sector)?;
        Ok(document_header)
    }

    #[tracing::instrument(level = "info", skip(input), fields(input = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
        let mut res = Self::default();
        let (tail, _) = dbg_peek_hex(tag(&b"H2CS"[..]), "read file magic, H2CS bytes")(input)?;

        let (tail, maybe_file_version_bytes) =
            dbg_peek_hex(take(4usize), "read maybe_file_version, 4 bytes")(tail)?;
        let (_, _maybe_file_version) =
            i32(nom::number::Endianness::Little)(maybe_file_version_bytes)?;

        let (tail, maybe_map_compatibility_bytes) =
            dbg_peek_hex(take(4usize), "read maybe_map_compatibility, 4 bytes")(tail)?;
        let _maybe_compatibility =
            String::from_utf8_lossy(maybe_map_compatibility_bytes).to_string();

        let (tail, _unknown_bytes) =
            dbg_peek_hex(take(4usize), "read _unknown_bytes after S2.., 4 bytes")(tail)?;

        let (tail, maybe_i32_bytes) =
            dbg_peek_hex(take(4usize), "read maybe_i32_bytes, 4 bytes")(tail)?;
        let (_, maybe_dimension_x1) = i32(nom::number::Endianness::Little)(maybe_i32_bytes)?;
        res.maybe_dimension_x1 = maybe_dimension_x1;

        let (tail, maybe_i32_bytes) =
            dbg_peek_hex(take(4usize), "read maybe_i32_bytes, 4 bytes")(tail)?;
        let (_, maybe_dimension_y1) = i32(nom::number::Endianness::Little)(maybe_i32_bytes)?;
        res.maybe_dimension_y1 = maybe_dimension_y1;

        let (tail, _padding_bytes) = dbg_peek_hex(
            take(4usize),
            "read _padding_bytes after maybe dimensions1, 4 bytes",
        )(tail)?;

        let (tail, maybe_i32_bytes) =
            dbg_peek_hex(take(4usize), "read maybe_i32_bytes, 4 bytes")(tail)?;
        let (_, some_epoch_1) = i32(nom::number::Endianness::Little)(maybe_i32_bytes)?;
        res.some_epoch_1 = some_epoch_1;

        let (tail, maybe_i32_bytes) =
            dbg_peek_hex(take(4usize), "read maybe_i32_bytes, 4 bytes")(tail)?;
        let (_, some_epoch_2) = i32(nom::number::Endianness::Little)(maybe_i32_bytes)?;
        res.some_epoch_2 = some_epoch_2;

        // 0200 0000 0000 0000 0100 0000 follows, no idea what these are...
        let (tail, _padding_bytes) = dbg_peek_hex(
            take(12usize),
            "read _padding_bytes after maybe dimensions2, 12 bytes: ",
        )(tail)?;

        let (tail, string_bytes) = dbg_peek_hex(take_while(|x| x != 0u8), "read mod string")(tail)?;
        res.mod_info = String::from_utf8_lossy(string_bytes).to_string();
        if res.mod_info != "bnet:Void (Mod)/0.0/999,file:Mods/Void.SC2Mod" {
            return Err(S2ProtocolError::Map(crate::cache_handles::MapError::Other(
                "Only parsed Void typed.".to_string(),
            )));
        }

        tracing::info!("Got mod_info: {}", res.mod_info);

        let (tail, _past_the_zero_delim) = dbg_peek_hex(
            take(1usize),
            "read _past_the_zero_delim on mod_info, 1 bytes",
        )(tail)?;

        let (tail, docinfo_field_count_bytes) =
            dbg_peek_hex(take(2usize), "read doc_info field count, 2 bytes")(tail)?;
        let (_, docinfo_field_count) =
            u16(nom::number::Endianness::Little)(docinfo_field_count_bytes)?;
        tracing::info!(
            "after read filed count filed count bytes: {:?} tail is {}",
            docinfo_field_count_bytes,
            peek_hex(tail)
        );

        let (mut new_tail, _) =
            dbg_peek_hex(tag(&[0, 0][..]), "doc_info element count delimiter.")(tail)?;

        tracing::info!("Expect to read {} fields", docinfo_field_count);
        for _ in 0..docinfo_field_count {
            tracing::info!("after read string size, tail is {}", peek_hex(new_tail));
            let (tail, next_str_size_bytes) =
                dbg_peek_hex(take(2usize), "read string size, 2 bytes")(new_tail)?;
            let (_, next_str_size) = u16(nom::number::Endianness::Little)(next_str_size_bytes)?;

            let (tail, docinfo_field_name_bytes) =
                dbg_peek_hex(take(next_str_size), "read docinfo_field_name, n bytes")(tail)?;
            let docinfo_field_name = String::from_utf8_lossy(docinfo_field_name_bytes).to_string();
            tracing::info!(
                "after field name: {} {}",
                docinfo_field_name,
                peek_hex(tail)
            );

            let (tail, _) =
                dbg_peek_hex(tag(&b"SUne"[..]), "doc_info file type, SUne bytes")(tail)?;

            let (tail, next_str_size_bytes) =
                dbg_peek_hex(take(2usize), "read string size, 2 bytes")(tail)?;
            let (_, next_str_size) = u16(nom::number::Endianness::Little)(next_str_size_bytes)?;

            let (tail, docinfo_field_value_bytes) =
                dbg_peek_hex(take(next_str_size), "read docinfo_field_value, n bytes")(tail)?;
            let docinfo_field_value =
                String::from_utf8_lossy(docinfo_field_value_bytes).to_string();
            res.set_field_by_name(&docinfo_field_name, docinfo_field_value);
            new_tail = tail;
        }

        Ok((tail, res))
    }
}

#[cfg(test)]
pub mod document_header_tests {
    use super::*;

    pub fn document_header_cache_content() -> Vec<u8> {
        // xxd -i -t < DocumentHeader
        vec![
            0x48, 0x32, 0x43, 0x53, // 4 bytes H2CS header.
            0x08, 0x00, 0x00, 0x00, // Maybe a file version?
            0x32, 0x53, 0x00, 0x00, // S2, maybe compatibility or rolated to Starcraft 2?
            0x01, 0x05, 0x00, 0x0e, 0x95, 0x6c, 0x01, 0x00, // Maybe dimension X1
            0x95, 0x6c, 0x01, 0x00, // Maybe dimension Y1
            0x00, 0x00, 0x00, 0x00, // Padding bytes
            0xe7, 0x9c, 0xc5, 0x58, // Maybe dimension X2
            0xe7, 0x9c, 0xc5, 0x58, // Maybe dimension Y2
            0x02, 0x00, 0x00, 0x00, // No idea.
            0x00, 0x00, 0x00, 0x00, // Padding
            0x01, 0x00, 0x00, 0x00, // No idea.
            0x62, 0x6e, 0x65, 0x74, 0x3a, 0x56, 0x6f, 0x69, 0x64, 0x20, 0x28, 0x4d, 0x6f, 0x64,
            0x29, 0x2f, 0x30, 0x2e, 0x30, 0x2f, 0x39, 0x39, 0x39, 0x2c, 0x66, 0x69, 0x6c, 0x65,
            0x3a, 0x4d, 0x6f, 0x64, 0x73, 0x2f, 0x56, 0x6f, 0x69, 0x64, 0x2e, 0x53, 0x43, 0x32,
            0x4d, 0x6f, 0x64, 0x00, // Until 0x0, a string with battle net SC2Mod information.
            0x07, 0x00, // Maybe doc info field count.
            0x00, 0x00, // Maybe padding.
            0x0c, 0x00, // next string size, 12 characters...
            0x44, 0x6f, 0x63, 0x49, 0x6e, 0x66, 0x6f, 0x2f, 0x4e, 0x61, 0x6d,
            0x65, // 12 characters up to here: DocInfo/Name
            0x53, 0x55, 0x6e, 0x65, // "SUne" Some field type identifier:
            // - S for String
            0x0a, 0x00, // next string size, 10 characters.
            0x54, 0x6f, 0x6b, 0x61, 0x6d, 0x61, 0x6b, 0x20, 0x4c, 0x45,
            // 10 charaters up to here: Tokamak LE
            0x15, 0x00, 0x4d, 0x61, 0x70, 0x49, 0x6e, 0x66, 0x6f, 0x2f, 0x50, 0x6c, 0x61, 0x79,
            0x65, 0x72, 0x30, 0x33, 0x2f, 0x4e, 0x61, 0x6d, 0x65, 0x53, 0x55, 0x6e, 0x65, 0x07,
            0x00, 0x48, 0x6f, 0x73, 0x74, 0x69, 0x6c, 0x65, 0x10, 0x00, 0x44, 0x6f, 0x63, 0x49,
            0x6e, 0x66, 0x6f, 0x2f, 0x44, 0x65, 0x73, 0x63, 0x4c, 0x6f, 0x6e, 0x67, 0x53, 0x55,
            0x6e, 0x65, 0xd1, 0x00, 0x57, 0x69, 0x64, 0x65, 0x20, 0x6d, 0x61, 0x63, 0x72, 0x6f,
            0x20, 0x6d, 0x61, 0x70, 0x20, 0x77, 0x69, 0x74, 0x63, 0x68, 0x20, 0x31, 0x36, 0x20,
            0x62, 0x61, 0x73, 0x65, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x72, 0x65, 0x20, 0x61,
            0x72, 0x65, 0x20, 0x74, 0x77, 0x6f, 0x20, 0x68, 0x79, 0x62, 0x72, 0x69, 0x64, 0x20,
            0x62, 0x61, 0x73, 0x65, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6d, 0x69, 0x6e,
            0x65, 0x72, 0x61, 0x6c, 0x73, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x69, 0x6e, 0x67,
            0x20, 0x61, 0x20, 0x70, 0x61, 0x74, 0x68, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70,
            0x61, 0x74, 0x68, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x6f, 0x70, 0x65,
            0x6e, 0x20, 0x62, 0x79, 0x20, 0x6d, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x72,
            0x20, 0x62, 0x79, 0x20, 0x64, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79, 0x69, 0x6e, 0x67,
            0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6f, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x74,
            0x6f, 0x77, 0x65, 0x72, 0x73, 0x20, 0x62, 0x65, 0x68, 0x69, 0x6e, 0x64, 0x20, 0x74,
            0x68, 0x65, 0x20, 0x62, 0x61, 0x73, 0x65, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20,
            0x63, 0x72, 0x75, 0x73, 0x68, 0x65, 0x73, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x62, 0x6c,
            0x75, 0x65, 0x20, 0x6d, 0x69, 0x6e, 0x65, 0x72, 0x61, 0x6c, 0x20, 0x6e, 0x6f, 0x64,
            0x65, 0x73, 0x2e, 0x15, 0x00, 0x4d, 0x61, 0x70, 0x49, 0x6e, 0x66, 0x6f, 0x2f, 0x50,
            0x6c, 0x61, 0x79, 0x65, 0x72, 0x30, 0x30, 0x2f, 0x4e, 0x61, 0x6d, 0x65, 0x53, 0x55,
            0x6e, 0x65, 0x07, 0x00, 0x4e, 0x65, 0x75, 0x74, 0x72, 0x61, 0x6c, 0x15, 0x00, 0x4d,
            0x61, 0x70, 0x49, 0x6e, 0x66, 0x6f, 0x2f, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x30,
            0x32, 0x2f, 0x4e, 0x61, 0x6d, 0x65, 0x53, 0x55, 0x6e, 0x65, 0x08, 0x00, 0x50, 0x6c,
            0x61, 0x79, 0x65, 0x72, 0x20, 0x32, 0x11, 0x00, 0x44, 0x6f, 0x63, 0x49, 0x6e, 0x66,
            0x6f, 0x2f, 0x44, 0x65, 0x73, 0x63, 0x53, 0x68, 0x6f, 0x72, 0x74, 0x53, 0x55, 0x6e,
            0x65, 0x01, 0x00, 0x32, 0x15, 0x00, 0x4d, 0x61, 0x70, 0x49, 0x6e, 0x66, 0x6f, 0x2f,
            0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x30, 0x31, 0x2f, 0x4e, 0x61, 0x6d, 0x65, 0x53,
            0x55, 0x6e, 0x65, 0x08, 0x00, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x20, 0x31, 0x00,
        ]
    }

    #[test_log::test]
    fn test_parse_document_header() {
        let cache_contents: Vec<u8> = document_header_cache_content();
        let (_, document_header) = DocumentHeader::parse(&cache_contents).unwrap();
        assert_eq!(document_header.name, "Tokamak LE");
        assert_eq!(
            document_header.mod_info,
            "bnet:Void (Mod)/0.0/999,file:Mods/Void.SC2Mod"
        );
    }
}
