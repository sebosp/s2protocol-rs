
#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "arrow",
  derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum W {
  WarpGateTrain,
  WarpGateTrainDarkTemplar,
  WarpGateTrainHighTemplar,
  WarpGateTrainSentry,
  WarpGateTrainStalker,
  WarpGateTrainWarpInAdept,
  WarpGateTrainZealot,
  WarpPrismTransport,
  WarpPrismTransportBunkerUnloadAll,
  WarpPrismTransportMedivacLoad,
  WarpPrismTransportMedivacUnloadAll,
  WarpableCancel,
  WidowMineAttack,
  WidowMineBurrow,
  WidowMineBurrowCancel,
  WidowMineUnburrow,
  WormholeTransit,
  WraithCloak,
  WraithCloakCloakOff,
  WreckingCrewAssaultMode,
  WreckingCrewFighterMode,
}
impl W {
    pub fn from_id(id: u16) -> Option<Self> {
      match id {
      1415 => Some(Self::WarpGateTrain),
      1420 => Some(Self::WarpGateTrain),
      1421 => Some(Self::WarpGateTrain),
      1422 => Some(Self::WarpGateTrain),
      1423 => Some(Self::WarpGateTrain),
      1424 => Some(Self::WarpGateTrain),
      1425 => Some(Self::WarpGateTrain),
      1426 => Some(Self::WarpGateTrain),
      1427 => Some(Self::WarpGateTrain),
      1428 => Some(Self::WarpGateTrain),
      1429 => Some(Self::WarpGateTrain),
      1430 => Some(Self::WarpGateTrain),
      1431 => Some(Self::WarpGateTrain),
      1432 => Some(Self::WarpGateTrain),
      1417 => Some(Self::WarpGateTrainDarkTemplar),
      1416 => Some(Self::WarpGateTrainHighTemplar),
      1418 => Some(Self::WarpGateTrainSentry),
      1414 => Some(Self::WarpGateTrainStalker),
      1419 => Some(Self::WarpGateTrainWarpInAdept),
      1413 => Some(Self::WarpGateTrainZealot),
      914 => Some(Self::WarpPrismTransport),
      915 => Some(Self::WarpPrismTransport),
      912 => Some(Self::WarpPrismTransportBunkerUnloadAll),
      911 => Some(Self::WarpPrismTransportMedivacLoad),
      913 => Some(Self::WarpPrismTransportMedivacUnloadAll),
      1412 => Some(Self::WarpableCancel),
      2099 => Some(Self::WidowMineAttack),
      2100 => Some(Self::WidowMineAttack),
      2095 => Some(Self::WidowMineBurrow),
      2096 => Some(Self::WidowMineBurrowCancel),
      2097 => Some(Self::WidowMineUnburrow),
      2098 => Some(Self::WidowMineUnburrow),
      293 => Some(Self::WormholeTransit),
      294 => Some(Self::WormholeTransit),
      3300 => Some(Self::WraithCloak),
      3301 => Some(Self::WraithCloakCloakOff),
      3380 => Some(Self::WreckingCrewAssaultMode),
      3381 => Some(Self::WreckingCrewAssaultMode),
      3382 => Some(Self::WreckingCrewFighterMode),
      3383 => Some(Self::WreckingCrewFighterMode),
      _ => None,
    }
  }
}
