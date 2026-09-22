//! Parsing the MapInfo file embedded in the caches.

use super::coords::*;
use super::*;
use crate::*;
#[cfg(feature = "nom_color_debug")]
use console::style;
use nom::bytes::complete::*;
use nom::number::complete::*;
use nom_mpq::MPQ;
use nom_mpq::parser::peek_hex;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// There are 8x8 pixels per terrain unit.
pub const IMAGE_DIMENSIONS_PER_TERRAIN_UNIT: i32 = 8;

/// There are 6x6 pixels per terrain unit.
pub const IMAGE_DIMENSIONS_PER_CELL_UNIT: i32 = 6;

/// For column format width on the variable being printed
pub const DBG_CONTEXT_WIDTH: usize = 42;
/// For column format width. on the hex representation of the value.
pub const DBG_HEX_VALUE_WIDTH: usize = 36;

/// The MapInfo coordinates's purpose is to translate "cell"coordinates to "terrain" coordinates,
/// Showing the playable terrain.
/// The width and height defined in the MapInfo determine the width and height of the
/// coords::MapCellCoord.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MapInfo {
    /// The cache_handle_id where the MapInfo was located.
    /// The cache_handle_id is itself a sha256 sum of its bundlsed contents.
    pub cache_handle_id: String,
    /// The MapInfo file sector sha256sum (in contrast to a bundle of files on cache_handle_id)
    pub sector_sha256_sum: String,
    pub file_version: i32,
    pub cell_width: usize,
    pub cell_height: usize,
    /// Mostly seen empty?
    pub first_string: String,
    /// Also empty?
    pub second_string: String,
    // Maybe a mode, light Dark/Light?
    pub third_string: String,
    // Some name, "Zerus" in the test case, maybe map maker?
    pub fourth_string: String,
    pub cell_left: usize,
    pub cell_bottom: usize,
    pub cell_right: usize,
    pub cell_top: usize,
}

#[macro_export]
macro_rules! dbg_displayable_and_tail {
    ( $i:ident) => {
        let peek_value = $i.to_string();
        #[cfg(feature = "nom_color_debug")]
        tracing::info!(
            "->{0:<DBG_CONTEXT_WIDTH$.DBG_CONTEXT_WIDTH$}|{1:<16}",
            stringify!($i),
            &style(format!("{}", peek_value))
                .magenta()
                .force_styling(true),
        );
        #[cfg(not(feature = "nom_color_debug"))]
        tracing::info!("+{0}:{1}", stringify!($i), peek_value,);
    };
}

#[macro_export]
macro_rules! dbg_bytes_and_tail {
    ( $i:ident, $b:ident, $t:ident) => {
        let peek_bytes = peek_hex($i);
        let peek_tail = peek_hex($t);
        let element_offset = ($i.as_ptr().addr() - $b);
        #[cfg(feature = "nom_color_debug")]
        let mut mem_addr = style(format!("0x{:0>8?}", element_offset))
            .cyan()
            .dim()
            .force_styling(true)
            .to_string();
        #[cfg(not(feature = "nom_color_debug"))]
        let mut mem_addr = format!("0x{:<8?}", element_offset);
        if $i.len() > 8usize {
            // Sshow the length displayed is incomplete.
            mem_addr.push_str("[...");
        } else {
            mem_addr.push_str("[   ");
        }
        #[cfg(feature = "nom_color_debug")]
        if $i.len() > 8usize {
            mem_addr.push_str(
                &style(&format!("{:0>3}", $i.len()))
                    .yellow()
                    .bright()
                    .force_styling(true)
                    .to_string(),
            );
        } else {
            mem_addr.push_str(
                &style(&format!("{:0>3}", $i.len()))
                    .yellow()
                    .dim()
                    .force_styling(true)
                    .to_string(),
            );
        }
        #[cfg(not(feature = "nom_color_debug"))]
        mem_addr.push_str(&format!("{:0<3.3}", $i.len()));
        mem_addr.push_str("]");
        tracing::info!(
            "|{0:<DBG_CONTEXT_WIDTH$.DBG_CONTEXT_WIDTH$} | {1}{2}{6:<3$.3$} | next {4}{6:>5$.5$} ",
            stringify!($i),
            mem_addr,
            peek_bytes,
            DBG_HEX_VALUE_WIDTH
                - console::measure_text_width(&peek_bytes).min(DBG_HEX_VALUE_WIDTH - 2),
            peek_tail,
            DBG_HEX_VALUE_WIDTH
                - console::measure_text_width(&peek_tail).min(DBG_HEX_VALUE_WIDTH - 2),
            " ",
        );
    };
}

impl MapInfo {
    #[instrument(skip(mpq, file_contents))]
    pub fn from_mpq(
        cache_handle_id: String,
        mpq: &MPQ,
        file_contents: &[u8],
    ) -> Result<Self, S2ProtocolError> {
        let (_, map_info_sector) =
            mpq.read_mpq_file_sector(MAP_INFO_FILE_NAME, false, file_contents)?;
        let (_, map_info) = Self::parse(cache_handle_id, &map_info_sector)?;
        Ok(map_info)
    }

    #[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
    pub fn parse(cache_handle_id: String, input: &[u8]) -> S2ProtoResult<&[u8], Self> {
        let input_base_addr = input.as_ptr().addr();
        tracing::info!("--> {} Parsing {}", peek_hex(input), cache_handle_id);
        let (tail, file_magic) =
            dbg_peek_hex(tag(&b"IpaM"[..]), "read file magic, IpaM bytes")(input)?;

        dbg_bytes_and_tail!(file_magic, input_base_addr, tail);

        let (mut tail, file_version_bytes) =
            dbg_peek_hex(take(4usize), "read file_version, 4 bytes")(tail)?;
        let (_, file_version) = i32(nom::number::Endianness::Little)(file_version_bytes)?;

        dbg_displayable_and_tail!(file_version);

        if file_version > 24 {
            // If file_version is more than 24 it seems to need 8 more bytes to read
            let (extra_tail, extra_bytes) =
                dbg_peek_hex(take(8usize), "file_version >= 24 needs 8 more extra bytes")(tail)?;
            tail = extra_tail;
            dbg_bytes_and_tail!(extra_bytes, input_base_addr, tail);
        }
        let (tail, cell_width_bytes) =
            dbg_peek_hex(take(4usize), "read map cell_width, 4 bytes")(tail)?;
        let (_, cell_width) = i32(nom::number::Endianness::Little)(cell_width_bytes)?;
        dbg_bytes_and_tail!(cell_width_bytes, input_base_addr, tail);
        let cell_width: usize = cell_width.try_into()?;
        dbg_displayable_and_tail!(cell_width);

        let (tail, cell_height_bytes) =
            dbg_peek_hex(take(4usize), "read map cell_height, 4 bytes")(tail)?;
        dbg_bytes_and_tail!(cell_height_bytes, input_base_addr, tail);
        let (_, cell_height) = i32(nom::number::Endianness::Little)(cell_height_bytes)?;
        let cell_height: usize = cell_height.try_into()?;
        dbg_displayable_and_tail!(cell_height);

        if cell_width > 256 || cell_height > 256 {
            tracing::warn!(
                "MapInfo cell_width({}) or cell_height({}) is larger than expected 256",
                cell_width,
                cell_height
            );
            return Err(S2ProtocolError::Map(MapError::InvalidMapSize(
                cell_width.max(cell_height),
            )));
        }
        let (tail, unknown_bytes_1) =
            dbg_peek_hex(take(8usize), "read 8 unknown_bytes after cell_height")(tail)?;
        dbg_bytes_and_tail!(unknown_bytes_1, input_base_addr, tail);

        let (tail, first_string_bytes) =
            dbg_peek_hex(take_while(|x| x != 0u8), "walk past the first string")(tail)?;
        let first_string = String::from_utf8_lossy(first_string_bytes).to_string();
        dbg_bytes_and_tail!(first_string_bytes, input_base_addr, tail);

        let (tail, _null_terminator) = dbg_peek_hex(
            take(1usize),
            "advance past termination character first string",
        )(tail)?;

        let (tail, second_string_bytes) =
            dbg_peek_hex(take_while(|x| x != 0u8), "walk past the second string")(tail)?;
        let second_string = String::from_utf8_lossy(second_string_bytes).to_string();
        dbg_bytes_and_tail!(second_string_bytes, input_base_addr, tail);

        let (tail, _null_terminator) = dbg_peek_hex(
            take(1usize),
            "advance past termination character second string",
        )(tail)?;

        let (tail, unknown_bytes_2) =
            dbg_peek_hex(take(8usize), "read 8 unknown bytes after second string")(tail)?;
        dbg_bytes_and_tail!(unknown_bytes_2, input_base_addr, tail);

        let (tail, padding_zeros) = dbg_peek_hex(
            take_while(|x| x == 0u8),
            "padding zeros before third string",
        )(tail)?;
        dbg_bytes_and_tail!(padding_zeros, input_base_addr, tail);

        let (tail, third_string_bytes) =
            dbg_peek_hex(take_while(|x| x != 0u8), "collect third string")(tail)?;
        let third_string = String::from_utf8_lossy(third_string_bytes).to_string();
        dbg_bytes_and_tail!(third_string_bytes, input_base_addr, tail);

        let (tail, _null_terminator) = dbg_peek_hex(
            take(1usize),
            "advance past termination character third string",
        )(tail)?;

        let (tail, fourth_string_bytes) =
            dbg_peek_hex(take_while(|x| x != 0u8), "collect fourth string")(tail)?;
        let fourth_string = String::from_utf8_lossy(fourth_string_bytes).to_string();
        dbg_bytes_and_tail!(fourth_string_bytes, input_base_addr, tail);

        let (tail, _null_terminator) = dbg_peek_hex(
            take(1usize),
            "advance past termination character fourth string",
        )(tail)?;

        let (tail, cell_left_bytes) =
            dbg_peek_hex(take(4usize), "read map cell_left, 4 bytes")(tail)?;
        let (_, cell_left) = i32(nom::number::Endianness::Little)(cell_left_bytes)?;
        let cell_left: usize = cell_left.try_into()?;
        dbg_displayable_and_tail!(cell_left);

        let (tail, cell_bottom_bytes) =
            dbg_peek_hex(take(4usize), "read map cell_bottom, 2 bytes")(tail)?;
        let (_, cell_bottom) = i32(nom::number::Endianness::Little)(cell_bottom_bytes)?;
        let cell_bottom: usize = cell_bottom.try_into()?;
        dbg_displayable_and_tail!(cell_bottom);

        let (tail, cell_right_bytes) =
            dbg_peek_hex(take(4usize), "read map cell_right, 4 bytes")(tail)?;
        let (_, cell_right) = i32(nom::number::Endianness::Little)(cell_right_bytes)?;
        let cell_right: usize = cell_right.try_into()?;
        dbg_displayable_and_tail!(cell_right);

        let (tail, cell_top_bytes) =
            dbg_peek_hex(take(4usize), "read map cell_top, 4 bytes")(tail)?;
        let (_, cell_top) = i32(nom::number::Endianness::Little)(cell_top_bytes)?;
        let cell_top: usize = cell_top.try_into()?;
        dbg_displayable_and_tail!(cell_top);

        if cell_left >= cell_right {
            return Err(S2ProtocolError::Map(MapError::InvalidCoordinateBounds {
                cache_id: cache_handle_id,
                ref_value_1: "MapInfoCellLeft".to_string(),
                value_1: cell_left,
                ref_value_2: "MapInfoCellRight".to_string(),
                value_2: cell_right,
            }));
        }
        if cell_bottom >= cell_top {
            return Err(S2ProtocolError::Map(MapError::InvalidCoordinateBounds {
                cache_id: cache_handle_id,
                ref_value_1: "MapInfoCellBottom".to_string(),
                value_1: cell_bottom,
                ref_value_2: "MapInfoCellTop".to_string(),
                value_2: cell_top,
            }));
        }

        if cell_right > cell_width {
            return Err(S2ProtocolError::Map(MapError::InvalidCoordinateBounds {
                cache_id: cache_handle_id,
                ref_value_1: "MapInfoCellRight".to_string(),
                value_1: cell_right,
                ref_value_2: "MapInfoCellWidth".to_string(),
                value_2: cell_width,
            }));
        }

        if cell_top > cell_height {
            return Err(S2ProtocolError::Map(MapError::InvalidCoordinateBounds {
                cache_id: cache_handle_id,
                ref_value_1: "MapInfoCellTop".to_string(),
                value_1: cell_top,
                ref_value_2: "MapInfoCellHeight".to_string(),
                value_2: cell_height,
            }));
        }

        Ok((
            tail,
            Self {
                sector_sha256_sum: sha256::digest(input),
                cache_handle_id,
                file_version,
                cell_width,
                cell_height,
                first_string,
                second_string,
                third_string,
                fourth_string,
                cell_left,
                cell_bottom,
                cell_right,
                cell_top,
            },
        ))
    }

    /// Returns the cell dimensions of the map.
    pub fn cell_dim_map(&self) -> MapCellCoord {
        // Previously cxDimMap, cyDimMap
        MapCellCoord::new(self.cell_width, self.cell_height)
    }

    /// Returns the dimensions of the map in terrain units.
    pub fn terrain_dim_map(&self) -> MapTerrainCoord {
        // Previously txDimMap, tyDimMap
        MapTerrainCoord::new(self.cell_width + 1, self.cell_height + 1)
    }

    pub fn cell_dim_playable(&self) -> MapCellCoord {
        // Previously cxDimPlayable, cyDimPlayable
        MapCellCoord::new(
            self.cell_right - self.cell_left,
            self.cell_top - self.cell_bottom,
        )
    }

    /// Returns the playable dimensions of the map in terrain units.
    pub fn terrain_dim_playable(&self) -> MapTerrainCoord {
        // Previously txDimPlayable, tyDimPlayable
        let cell_dim_playable = self.cell_dim_playable();
        MapTerrainCoord::new(cell_dim_playable.x + 1, cell_dim_playable.y + 1)
    }

    pub fn cell_left_bottom(&self) -> MapCellCoord {
        // Previously cLeftBottom
        MapCellCoord::new(self.cell_left, self.cell_bottom)
    }

    pub fn cell_right_top(&self) -> MapCellCoord {
        // Previously cRightTop
        MapCellCoord::new(self.cell_right, self.cell_top)
    }

    pub fn terrain_left_bottom(&self) -> MapTerrainCoord {
        // Previously tLeftBottom
        MapTerrainCoord::new(self.cell_left, self.cell_bottom)
    }

    pub fn terrain_right_top(&self) -> MapTerrainCoord {
        // Previously tRightTop
        MapTerrainCoord::new(self.cell_width + 1, self.cell_height + 1)
    }
}

#[cfg(test)]
pub mod map_info_tests {
    use super::*;

    pub fn map_info_cache_content() -> Vec<u8> {
        // xxd -ps -c 1 < MapInfo|sed 's/^/0x/g;s/$/,/g'|xargs echo -n
        vec![
            0x49, 0x70, 0x61, 0x4d, // 4 bytes IpaM Magic
            0x27, 0x00, 0x00, 0x00, // 4 bytes file_version, in this case more than 24.
            0xc3, 0x38, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, // Take 8 extra bytes for file_version > 24
            0xa8, 0x00, 0x00, 0x00, // 4 bytes width = 168
            0xa8, 0x00, 0x00, 0x00, // 4 bytes height = 168
            0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // Ignore 8 unknown bytes
            0x00, 0x00, // Pre-amble of strings, skip zeros.
            0x04, // First string
            0x00, // Advance termination character
            // Find terminator of second string, nothing is taken, would skip non-zero bytes
            0x00, // Advance termination character
            0x00, 0x00, 0x00, 0x00, // Skip 4 bytes
            0x00, // Advance an extra byte.
            0x44, 0x61, 0x72, 0x6b, // third string "Dark"
            0x00, // Advance termination character
            0x5a, 0x65, 0x72, 0x75, 0x73, // fourth string "Zerus"
            0x00, // Advance termination character
            0x0e, 0x00, 0x00, 0x00, // 6 bytes for "left"
            0x0e, 0x00, 0x00, 0x00, // 6 bytes for "bottom"
            0x9a, 0x00, 0x00, 0x00, // 6 bytes for "right"
            0x9a, 0x00, 0x00, 0x00, // 6 bytes for "top"
            // The rest of the data is unknown.
            0x00, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20,
            0x03, 0x00, 0x00, 0x58, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00,
            0xbd, 0xc6, 0x0a, 0x68, 0xa7, 0xc6, 0x0a, 0x68, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x01, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x00,
            0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x0f, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
    }

    #[test]
    fn test_parse_map_info() {
        let cache_contents: Vec<u8> = map_info_cache_content();
        let (_, map_info) = MapInfo::parse(String::from("test"), &cache_contents).unwrap();
        assert_eq!(map_info.cell_width, 168);
        assert_eq!(map_info.cell_height, 168);
        assert_eq!(map_info.third_string, "Dark".to_string());
        assert_eq!(map_info.fourth_string, "Zerus".to_string());
    }
}
