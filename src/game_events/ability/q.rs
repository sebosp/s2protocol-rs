
#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "arrow",
  derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum Q {
  Que1Cancel,
  Que1CancelSlot,
  Que5AddonCancel,
  Que5AddonCancelSlot,
  Que5Cancel,
  Que5CancelSlot,
  Que5CancelToSelection,
  Que5CancelToSelectionCancelSlot,
  Que5LongBlendCancel,
  Que5LongBlendCancelSlot,
  Que5PassiveCancel,
  Que5PassiveCancelSlot,
  Que5PassiveCancelToSelection,
  Que5PassiveCancelToSelectionCancelSlot,
  Que8Cancel,
  Que8CancelSlot,
  QueenBuild,
  QueenBuildCancel,
  QueenBuildCreepTumor,
  QueenFly,
  QueenLand,
  QueenMpEnsnare,
  QueenMpInfestCommandCenter,
  QueenMpSpawnBroodlings,
  QueenShockwave,
}
impl Q {
    pub fn from_id(id: u16) -> Option<Self> {
      match id {
      304 => Some(Self::Que1Cancel),
      305 => Some(Self::Que1CancelSlot),
      312 => Some(Self::Que5AddonCancel),
      313 => Some(Self::Que5AddonCancelSlot),
      306 => Some(Self::Que5Cancel),
      307 => Some(Self::Que5CancelSlot),
      308 => Some(Self::Que5CancelToSelection),
      309 => Some(Self::Que5CancelToSelectionCancelSlot),
      310 => Some(Self::Que5LongBlendCancel),
      311 => Some(Self::Que5LongBlendCancelSlot),
      1831 => Some(Self::Que5PassiveCancel),
      1832 => Some(Self::Que5PassiveCancelSlot),
      1833 => Some(Self::Que5PassiveCancelToSelection),
      1834 => Some(Self::Que5PassiveCancelToSelectionCancelSlot),
      2077 => Some(Self::Que8Cancel),
      2078 => Some(Self::Que8CancelSlot),
      1695 => Some(Self::QueenBuild),
      1696 => Some(Self::QueenBuild),
      1697 => Some(Self::QueenBuild),
      1698 => Some(Self::QueenBuild),
      1699 => Some(Self::QueenBuild),
      1700 => Some(Self::QueenBuild),
      1701 => Some(Self::QueenBuild),
      1702 => Some(Self::QueenBuild),
      1703 => Some(Self::QueenBuild),
      1704 => Some(Self::QueenBuild),
      1705 => Some(Self::QueenBuild),
      1706 => Some(Self::QueenBuild),
      1707 => Some(Self::QueenBuild),
      1708 => Some(Self::QueenBuild),
      1709 => Some(Self::QueenBuild),
      1710 => Some(Self::QueenBuild),
      1711 => Some(Self::QueenBuild),
      1712 => Some(Self::QueenBuild),
      1713 => Some(Self::QueenBuild),
      1714 => Some(Self::QueenBuild),
      1715 => Some(Self::QueenBuild),
      1716 => Some(Self::QueenBuild),
      1717 => Some(Self::QueenBuild),
      1718 => Some(Self::QueenBuild),
      1719 => Some(Self::QueenBuild),
      1720 => Some(Self::QueenBuild),
      1721 => Some(Self::QueenBuild),
      1722 => Some(Self::QueenBuild),
      1723 => Some(Self::QueenBuild),
      1724 => Some(Self::QueenBuildCancel),
      1694 => Some(Self::QueenBuildCreepTumor),
      1938 => Some(Self::QueenFly),
      1939 => Some(Self::QueenFly),
      1936 => Some(Self::QueenLand),
      1937 => Some(Self::QueenLand),
      2493 => Some(Self::QueenMpEnsnare),
      2494 => Some(Self::QueenMpEnsnare),
      2497 => Some(Self::QueenMpInfestCommandCenter),
      2498 => Some(Self::QueenMpInfestCommandCenter),
      2495 => Some(Self::QueenMpSpawnBroodlings),
      2496 => Some(Self::QueenMpSpawnBroodlings),
      2925 => Some(Self::QueenShockwave),
      2926 => Some(Self::QueenShockwave),
      _ => None,
    }
  }
}
