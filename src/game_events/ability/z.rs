
#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "arrow",
  derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum Z {
  ZeratulBlink,
  ZeratulStun,
  ZergBuild,
  ZergBuildBanelingNest,
  ZergBuildCancel,
  ZergBuildCreepTumor,
  ZergBuildEvolutionChamber,
  ZergBuildExtractor,
  ZergBuildHatchery,
  ZergBuildHydraliskDen,
  ZergBuildInfestationPit,
  ZergBuildMutateintoLurkerDen,
  ZergBuildNydusNetwork,
  ZergBuildRoachWarren,
  ZergBuildSpawningPool,
  ZergBuildSpineCrawler,
  ZergBuildSpire,
  ZergBuildSporeCrawler,
  ZergBuildUltraliskCavern,
}
impl Z {
    pub fn from_id(id: u16) -> Option<Self> {
      match id {
      3192 => Some(Self::ZeratulBlink),
      3193 => Some(Self::ZeratulBlink),
      3298 => Some(Self::ZeratulStun),
      3299 => Some(Self::ZeratulStun),
      1164 => Some(Self::ZergBuild),
      1168 => Some(Self::ZergBuild),
      1169 => Some(Self::ZergBuild),
      1170 => Some(Self::ZergBuild),
      1171 => Some(Self::ZergBuild),
      1172 => Some(Self::ZergBuild),
      1173 => Some(Self::ZergBuild),
      1174 => Some(Self::ZergBuild),
      1175 => Some(Self::ZergBuild),
      1176 => Some(Self::ZergBuild),
      1177 => Some(Self::ZergBuild),
      1178 => Some(Self::ZergBuild),
      1179 => Some(Self::ZergBuild),
      1180 => Some(Self::ZergBuild),
      1181 => Some(Self::ZergBuild),
      1162 => Some(Self::ZergBuildBanelingNest),
      1182 => Some(Self::ZergBuildCancel),
      1153 => Some(Self::ZergBuildCreepTumor),
      1156 => Some(Self::ZergBuildEvolutionChamber),
      1154 => Some(Self::ZergBuildExtractor),
      1152 => Some(Self::ZergBuildHatchery),
      1157 => Some(Self::ZergBuildHydraliskDen),
      1160 => Some(Self::ZergBuildInfestationPit),
      1163 => Some(Self::ZergBuildMutateintoLurkerDen),
      1161 => Some(Self::ZergBuildNydusNetwork),
      1165 => Some(Self::ZergBuildRoachWarren),
      1155 => Some(Self::ZergBuildSpawningPool),
      1166 => Some(Self::ZergBuildSpineCrawler),
      1158 => Some(Self::ZergBuildSpire),
      1167 => Some(Self::ZergBuildSporeCrawler),
      1159 => Some(Self::ZergBuildUltraliskCavern),
      _ => None,
    }
  }
}
