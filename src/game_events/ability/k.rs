
#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "arrow",
  derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum K {
  KarassPlasmaSurge,
  KarassPsiStorm,
  Kd8Charge,
  KerriganSearch,
}
impl K {
    pub fn from_id(id: u16) -> Option<Self> {
      match id {
      3135 => Some(Self::KarassPlasmaSurge),
      3136 => Some(Self::KarassPlasmaSurge),
      3137 => Some(Self::KarassPsiStorm),
      3138 => Some(Self::KarassPsiStorm),
      2588 => Some(Self::Kd8Charge),
      2589 => Some(Self::Kd8Charge),
      2917 => Some(Self::KerriganSearch),
      2918 => Some(Self::KerriganSearch),
      _ => None,
    }
  }
}
