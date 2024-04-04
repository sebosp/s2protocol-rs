
#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "arrow",
  derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum Y {
  Yamato,
  YamatoYamatoGun,
  Yoink,
  YoinkFaceEmbrace,
}
impl Y {
    pub fn from_id(id: u16) -> Option<Self> {
      match id {
      402 => Some(Self::Yamato),
      401 => Some(Self::YamatoYamatoGun),
      2068 => Some(Self::Yoink),
      2067 => Some(Self::YoinkFaceEmbrace),
      _ => None,
    }
  }
}
