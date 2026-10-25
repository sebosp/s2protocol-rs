//! t3Terrain.xml parsing
//! <terrain version="115">
//<heightMap tileSet="DominionLab" uvtiling="20.250000 21.000000 -2.250000 -0.750000 " dim="217 225 " offset="0.000000 0.000000 0.000000 " scale="1.000000e+00 1.000000e+00 1.000000e+00 ">
//▏   <cliffSetList num="3">
//▏   ▏   <cliffSet i="0" name="CybrossCliff0"/>
//▏   ▏   <cliffSet i="1" name="HybridLabOuterCliff0"/>
//▏   ▏   <cliffSet i="2" name="HybridLabOuterCliff1"/>
//▏   </cliffSetList>
//▏   <rampList num="32">
//▏   ▏   <ramp
//▏   ▏   ▏dir="6"
//▏   ▏   ▏   hi="2"
//▏   ▏   ▏   lo="1"
//▏   ▏   ▏   leftLo="u(-1.000000e+00, 0.000000e+00) r(0.000000e+00, 1.000000e+00) c=(1.420000e+02, 4.400000e+01) w=2.000000e+00 h=2.000000e+00"
//▏   ▏   ▏   leftHi="u(0.000000e+00, 0.000000e+00) r(0.000000e+00, 0.000000e+00) c=(0.000000e+00, 0.000000e+00) w=0.000000e+00 h=0.000000e+00"
//▏   ▏   ▏   rightLo="u(-1.000000e+00, 0.000000e+00) r(0.000000e+00, 1.000000e+00) c=(1.480000e+02, 5.000000e+01) w=2.000000e+00 h=2.000000e+00"
//▏   ▏   ▏   rightHi="u(0.000000e+00, 0.000000e+00) r(0.000000e+00, 0.000000e+00) c=(4.000000e+00, 4.000000e+00) w=0.000000e+00 h=0.000000e+00"
//▏   ▏   ▏   base="u(-7.071068e-01, 7.071068e-01) r(7.071068e-01, 7.071068e-01) c=(1.430000e+02, 4.900000e+01) w=4.242640e+00 h=0.000000e+00"
//▏   ▏   ▏   mid="u(-7.071068e-01, 7.071068e-01) r(7.071068e-01, 7.071068e-01) c=(1.450000e+02, 4.700000e+01) w=4.242640e+00 h=2.828427e+00"
//▏   ▏   ▏   cid="1"
//▏   ▏   ▏   leftLoVar="0"
//▏   ▏   ▏   leftHiVar="4294967295"
//▏   ▏   ▏   rightLoVar="0"
//▏   ▏   ▏   rightHiVar="4294967295"
//▏   ▏   ▏   />
//▏   ▏   <ramp dir=

use crate::error::S2ProtocolError;
use nom_mpq::MPQ;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename = "terrain")]
pub struct T3Terrain {
    /// The cache_handle_id where the t3Terrain file was found.
    #[serde(skip)]
    pub cache_handle_id: String,
    #[serde(rename = "heightMap")]
    pub height_map: HeightMap,
    #[serde(rename = "@version")]
    pub version: u32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HeightMap {
    #[serde(rename = "rampList")]
    pub ramp_list: Option<RampListTag>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RampListTag {
    #[serde(rename = "ramp", default)]
    pub inner: Vec<Ramp>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ramp {
    #[serde(rename = "@dir", default)]
    pub dir: u8,
    /// Looks like cell layer/height
    #[serde(rename = "@hi", default)]
    pub hi: u8,
    #[serde(rename = "@lo", default)]
    pub lo: u8,
    // "u(-1.000000e+00, 0.000000e+00) r(0.000000e+00, 1.000000e+00) c=(1.420000e+02, 4.400000e+01) w=2.000000e+00 h=2.000000e+00"
    // Looks SVG-ish, maybe u=up r=right c=center w=width h=height ?
    #[serde(rename = "@leftLo", default)]
    pub left_lo: String,
    #[serde(rename = "@leftHi", default)]
    pub left_hi: String,
    #[serde(rename = "@rightLo", default)]
    pub right_lo: String,
    #[serde(rename = "@rightHi", default)]
    pub right_hi: String,
    #[serde(rename = "@base", default)]
    pub base: String,
    #[serde(rename = "@mid", default)]
    pub mid: String,
    #[serde(rename = "@cid", default)]
    pub cid: usize,
    #[serde(rename = "@leftLoVar", default)]
    pub left_lo_var: u32,
    #[serde(rename = "@leftHiVar", default)]
    pub left_hi_var: u32,
    #[serde(rename = "@rightLoVar", default)]
    pub right_lo_var: u32,
    #[serde(rename = "@rightHiVar", default)]
    pub right_hi_var: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_ramp_attributes() {
        let xml = r#"<terrain version="115"><heightMap tileSet="Lab"><rampList num="1">
            <ramp dir="6" hi="2" lo="1" leftLo="u(-1.000000e+00, 0.000000e+00) r(0.000000e+00, 1.000000e+00) c=(5.200000e+01, 2.000000e+01) w=2.000000e+00 h=2.000000e+00" cid="1" leftLoVar="0"/>
        </rampList></heightMap></terrain>"#;
        let terrain = T3Terrain::parse("id".to_string(), xml.as_bytes()).unwrap();
        let ramp = &terrain.height_map.ramp_list.unwrap().inner[0];
        assert_eq!((ramp.dir, ramp.hi, ramp.lo, ramp.cid), (6, 2, 1, 1));
        assert!(ramp.left_lo.contains("c=(5.200000e+01, 2.000000e+01)"));
    }
}

impl T3Terrain {
    #[instrument(level = "debug", skip(file_contents))]
    pub fn parse(cache_handle_id: String, file_contents: &[u8]) -> Result<Self, S2ProtocolError> {
        // Seems like the end of this XML may end ni <cliffCellList> rather than </cliffCellList>
        let str_content = str::from_utf8(file_contents)?.replace(
            "        <cliffCellList>\r\n    </heightMap>\r\n</terrain>",
            "        </cliffCellList>\r\n    </heightMap>\r\n</terrain>",
        );
        let mut res = serde_xml_rs::from_str::<T3Terrain>(&str_content)?;
        res.cache_handle_id = cache_handle_id;
        Ok(res)
    }

    /// Extract the xml file from the MPQ archive and parse its content.
    #[instrument(level = "debug", skip(mpq, file_contents))]
    pub fn from_mpq(
        cache_handle_id: String,
        mpq: &MPQ,
        file_contents: &[u8],
    ) -> Result<Self, S2ProtocolError> {
        let (_, t3_terrain_sector) =
            mpq.read_mpq_file_sector(super::T3_TERRAIN_MAP_FILE_NAME, false, file_contents)?;
        let t3_terrain = Self::parse(cache_handle_id, &t3_terrain_sector)?;
        Ok(t3_terrain)
    }
}
