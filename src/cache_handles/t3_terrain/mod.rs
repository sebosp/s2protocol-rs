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

use crate::{error::S2ProtocolError, read_mpq};
use nom_mpq::MPQ;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename = "terrain")]
pub struct T3Terrain {
    #[serde(rename = "heightMap")]
    pub height_map: HeightMap,
    #[serde(rename = "@version")]
    pub version: u32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HeightMap {
    #[serde(rename = "rampList")]
    pub ramp_list: RampListTag,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RampListTag {
    #[serde(rename = "ramp", default)]
    pub inner: Vec<Ramp>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ramp {
    #[serde(default)]
    pub dir: u8,
    /// Looks like cell layer/height
    #[serde(default)]
    pub hi: u8,
    #[serde(default)]
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
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub mid: String,
    #[serde(default)]
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

#[instrument]
pub fn try_get_t3_height_map_from_mpq(
    cache_handle_fname: &str,
) -> Result<T3Terrain, S2ProtocolError> {
    let (mpq, cache_contents) = read_mpq(cache_handle_fname)?;
    // based on sc2-map-analyzer/analyser/read.cpp
    for (file, _file_size) in mpq.get_files(&cache_contents)? {
        if file == "t3Terrain.xml" {
            return Ok(serde_xml_rs::from_str::<T3Terrain>(&file)?);
        }
    }
    Ok(T3Terrain::default())
}
impl T3Terrain {
    #[instrument(level = "debug", skip(file_contents))]
    pub fn parse(file_contents: &[u8]) -> Result<Self, S2ProtocolError> {
        let str_content = str::from_utf8(file_contents)?;
        Ok(serde_xml_rs::from_str::<T3Terrain>(str_content)?)
    }

    /// Extract the xml file from the MPQ archive and parse its content.
    #[instrument(level = "debug", skip(mpq, file_contents))]
    pub fn from_mpq(mpq: &MPQ, file_contents: &[u8]) -> Result<Self, S2ProtocolError> {
        let (_, t3_terrain_sector) =
            mpq.read_mpq_file_sector("t3Terrain.xml", false, file_contents)?;
        let t3_terrain = Self::parse(&t3_terrain_sector)?;
        Ok(t3_terrain)
    }
}
