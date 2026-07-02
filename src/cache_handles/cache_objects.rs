//! Parses the Objects file from the caches.

use crate::S2ProtocolError;
use nom_mpq::MPQ;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacedObjects {
    #[serde(rename = "@Version")]
    pub version: u32,
    #[serde(rename = "ObjectPoint", default)]
    pub points: Vec<ObjectPoint>,
    #[serde(rename = "ObjectDoodad", default)]
    pub doodas: Vec<ObjectDoodad>,
    #[serde(rename = "ObjectUnit", default)]
    pub units: Vec<ObjectUnit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectDoodad {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(default, rename = "@Variation")]
    pub variation: String,
    #[serde(rename = "@Position")]
    pub position: String,
    #[serde(default, rename = "@Rotation")]
    pub rotation: String,
    #[serde(rename = "@Scale")]
    pub scale: String,
    #[serde(rename = "@Type")]
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectPoint {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "@Position")]
    pub position: String,
    #[serde(rename = "@Scale")]
    pub scale: String,
    #[serde(rename = "@Type")]
    pub kind: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Color")]
    pub color: String,
    #[serde(default, rename = "@PathingRadiusSoft")]
    pub pathing_radius_soft: u32,
    #[serde(default, rename = "@PathingRadiusHard")]
    pub pathing_radius_hard: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectUnit {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(default, rename = "@Variation")]
    pub variation: String,
    #[serde(rename = "@Position")]
    pub position: String,
    #[serde(rename = "@Scale")]
    pub scale: String,
    #[serde(rename = "@UnitType")]
    pub unit_kind: String,
}

impl PlacedObjects {
    #[instrument(level = "debug", skip(file_contents))]
    pub fn parse(file_contents: &[u8]) -> Result<Self, S2ProtocolError> {
        let str_content = str::from_utf8(file_contents)?;
        Ok(serde_xml_rs::from_str::<PlacedObjects>(str_content)?)
    }

    /// Extract the xml file from the MPQ archive and prase its content.
    #[instrument(level = "debug", skip(mpq, file_contents))]
    pub fn from_mpq(mpq: &MPQ, file_contents: &[u8]) -> Result<Self, S2ProtocolError> {
        let (_, placed_objects_sector) =
            mpq.read_mpq_file_sector(super::OBJECTS_FILE_NAME, false, file_contents)?;
        let placed_objects = Self::parse(&placed_objects_sector)?;
        Ok(placed_objects)
    }
}
