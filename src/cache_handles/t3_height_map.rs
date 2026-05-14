use super::map::MapError;
use super::map::coords::*;
use super::map_info::MapInfo;
use crate::dbg_peek_hex;
use crate::{S2ProtoResult, S2ProtocolError};
use nom::bytes::complete::*;
use nom::number::complete::*;
use nom_mpq::MPQ;
use nom_mpq::parser::peek_hex;
use tracing::instrument;

#[derive(Default, Debug, Clone)]
pub struct T3HeightMap {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

impl T3HeightMap {
    #[instrument(skip(mpq, file_contents))]
    pub fn from_mpq(
        mpq: &MPQ,
        file_contents: &[u8],
        map_info: &MapInfo,
    ) -> Result<Self, S2ProtocolError> {
        let (_, t3_height_sector) =
            mpq.read_mpq_file_sector("t3HeightMap", false, file_contents)?;
        let (_, t3_height_map) = Self::parse(&t3_height_sector, map_info)?;
        Ok(t3_height_map)
    }

    #[tracing::instrument(level = "info", skip(input), fields(input = peek_hex(input)))]
    pub fn parse<'a>(input: &'a [u8], map_info: &MapInfo) -> S2ProtoResult<&'a [u8], Self> {
        let (tail, _) = dbg_peek_hex(tag(&b"HMAP"[..]), "read file magic, HMAP bytes")(input)?;
        let (tail, _) = dbg_peek_hex(
            tag(&[0x65, 0x00, 0x00, 0x00][..]),
            "read file version, 4 bytes",
        )(tail)?;

        let (tail, width_bytes) =
            dbg_peek_hex(take(4usize), "read map terrain width, 4 bytes")(tail)?;
        let (_, width) = i32(nom::number::Endianness::Little)(width_bytes)?;

        let (tail, height_bytes) =
            dbg_peek_hex(take(4usize), "read map terrain height, 4 bytes")(tail)?;
        let (_, height) = i32(nom::number::Endianness::Little)(height_bytes)?;

        let terrain_dimensions = MapTerrainCoord::new(width, height);
        let map_info_dimensions = map_info.terrain_dim_map();
        if terrain_dimensions != map_info_dimensions {
            return Err(S2ProtocolError::Map(
                MapError::T3HeightDimDoNotMatchMapInfoDim(terrain_dimensions, map_info_dimensions),
            ));
        }

        let (tail, _unknown_bytes) = dbg_peek_hex(take(16usize), "read 16 unknown bytes")(tail)?;

        // Expect terrain map to have 6 bytes per terrain unit.
        let expected_terrain_data_size = (width * height * 6) as usize;

        if tail.len() < expected_terrain_data_size {
            return Err(S2ProtocolError::Map(MapError::T3HeightNotEnoughBytes(
                expected_terrain_data_size,
                tail.len(),
            )));
        } else if tail.len() > expected_terrain_data_size {
            tracing::warn!(
                "T3HeightMap ------ Expected {} bytes of terrain data, but got {} bytes. Ignoring extra bytes.",
                expected_terrain_data_size,
                tail.len()
            );
        }
        let mut data: Vec<u8> = Vec::with_capacity(expected_terrain_data_size);
        let mut tail = tail;
        for _ in 0..(width * height) {
            let (new_tail, terrain_unit_bytes) =
                dbg_peek_hex(take(6usize), "read 6 bytes of terrain unit data")(tail)?;
            // The 5th byte is the height.
            let height_byte = terrain_unit_bytes[4];
            if height_byte > 4 {
                return Err(S2ProtocolError::Map(MapError::T3HeightUnitOutOfBounds(
                    height_byte as i32,
                )));
            }
            data.push(height_byte);
            tail = new_tail;
        }

        Ok((
            tail,
            Self {
                width,
                height,
                data,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::super::map_info::map_info_tests::map_info_cache_content;
    use super::MapError;
    use super::MapInfo;
    use super::*;
    #[test]
    fn test_parse_map_info() {
        // xxd -ps -c 1 -l 200 < t3HeightMap|sed 's/^/0x/g;s/$/,/g'|xargs echo -n
        #[rustfmt::skip]
        let cache_contents: Vec<u8> = vec![
            0x48, 0x4d, 0x41, 0x50, // 4 bytes HMAP Magic
            0x65, 0x00, 0x00, 0x00, // 4 bytes version.
            0xa9, 0x00, 0x00, 0x00, // 4 bytes width = 169
            0xa9, 0x00, 0x00, 0x00, // 4 bytes height = 169
            0x00, 0x00, 0x00, 0x00, // 16 bytes unknown
            0x00, 0x00, 0x00, 0x00, // ...
            0x00, 0x00, 0x00, 0x00, // ...
            0x00, 0x00, 0x00, 0x00, // ...
            // Six sets of bytes, the 5th byte is the height.
            0xc5, 0xf1, 0x73, 0x9c, 0x02, 0x00,
            0xc5, 0xf1, 0x73, 0x9c, 0x02, 0x00,
            0xc5, 0xf1, 0x73, 0x9c, 0x02, 0x00,
            0xc5, 0xf1, 0x73, 0x9c, 0x02, 0x00,
            0xc5, 0xf1, 0x73, 0x9c, 0x02, 0x00,
            0xc5, 0xf1, 0x73, 0x9c, 0x02, 0x00,
            0xc5, 0xf1, 0x73, 0x9c, 0x02, 0x00,
            0xc5, 0xf1, 0x73, 0x9c, 0x02, 0x00,
            0xc5, 0xf1, 0x39, 0x0e, 0x00, 0x00,
            0xc5, 0xf1, 0x39, 0x0e, 0x00, 0x00,
            0xc5, 0xf1, 0x39, 0x0e, 0x00, 0x00,
            0xc5, 0xf1, 0x39, 0x0e, 0x00, 0x00,
            0xc5, 0xf1, 0x39, 0x0e, 0x00, 0x00,
            0xc5, 0xf1, 0x01, 0x80, 0x01, 0x00,
            0xc5, 0xf1, 0x01, 0x80, 0x01, 0x00,
            0xc5, 0xf1, 0x01, 0x80, 0x01, 0x00,
            0xc5, 0xf1, 0x01, 0x80, 0x01, 0x00,
            0xc5, 0xf1, 0x01, 0x80, 0x01, 0x00,
            0xc5, 0xf1, 0x39, 0x0e, 0x00, 0x00,
            0xc5, 0xf1, 0x39, 0x0e, 0x00, 0x00,
            0xc5, 0xf1, 0x39, 0x0e, 0x00, 0x00,
            0xc5, 0xf1, 0x39, 0x0e, 0x00, 0x00,
            0xc5, 0xf1, 0x39, 0x0e, 0x00, 0x00,
            0xc5, 0xf1, 0x39, 0x0e, 0x00, 0x00,
            0xc5, 0xf1, 0x39, 0x0e, 0x00, 0x00,
            0xc5, 0xf1, 0xe6, 0xb8, 0x03, 0x00,
            0xc5, 0xf1, 0xe6, 0xb8, 0x03, 0x00,
            0xc5, 0xf1, 0xe6, 0xb8, 0x03, 0x00,
            // This would need 117366 (169*169*6) bytes to be complete, so let's expect the err.
        ];
        let (_, map_info) = MapInfo::parse(&map_info_cache_content()).unwrap();
        let t3_height_map = T3HeightMap::parse(&cache_contents, &map_info);
        if let Err(S2ProtocolError::Map(MapError::T3HeightNotEnoughBytes(171366, 168))) =
            t3_height_map
        {
            // Expected error
        } else {
            panic!(
                "Expected T3HeightNotEnoughBytes error, but got {:?}",
                t3_height_map
            );
        }
        //assert_eq!(t3_height_map.width, 169);
        //assert_eq!(t3_height_map.height, 169);
        //assert_eq!(t3_height_map.data[0], 0x02);
    }
}
