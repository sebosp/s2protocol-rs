
#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "arrow",
  derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum S {
  Sacrifice,
  SalvageBaneling,
  SalvageBanelingRefund,
  SalvageBunker,
  SalvageBunkerRefund,
  SalvageDrone,
  SalvageDroneRefund,
  SalvageHydralisk,
  SalvageHydraliskRefund,
  SalvageInfestor,
  SalvageInfestorRefund,
  SalvageQueen,
  SalvageQueenRefund,
  SalvageRoach,
  SalvageRoachRefund,
  SalvageShared,
  SalvageSwarmHost,
  SalvageSwarmHostRefund,
  SalvageUltralisk,
  SalvageUltraliskRefund,
  SalvageZergling,
  SalvageZerglingRefund,
  SapStructure,
  ScannerSweep,
  Scryer,
  ScvHarvest,
  ScvHarvestGather,
  ScvHarvestReturnCargo,
  SecurityGateDiagonalBlur,
  SecurityGateDiagonalBlurGateClose,
  SecurityGateDiagonalBlurLowered,
  SecurityGateDiagonalBlurLoweredGateOpen,
  SecurityGateDiagonalUlbr,
  SecurityGateDiagonalUlbrGateClose,
  SecurityGateDiagonalUlbrLowered,
  SecurityGateDiagonalUlbrLoweredGateOpen,
  SecurityGateStraightHorizontal,
  SecurityGateStraightHorizontalBf,
  SecurityGateStraightHorizontalBfGateClose,
  SecurityGateStraightHorizontalBfLowered,
  SecurityGateStraightHorizontalBfLoweredGateOpen,
  SecurityGateStraightHorizontalGateClose,
  SecurityGateStraightHorizontalLowered,
  SecurityGateStraightHorizontalLoweredGateOpen,
  SecurityGateStraightVertical,
  SecurityGateStraightVerticalGateClose,
  SecurityGateStraightVerticalLf,
  SecurityGateStraightVerticalLfGateClose,
  SecurityGateStraightVerticalLfLowered,
  SecurityGateStraightVerticalLfLoweredGateOpen,
  SecurityGateStraightVerticalLowered,
  SecurityGateStraightVerticalLoweredGateOpen,
  SeekerDummyChannel,
  SeekerMissile,
  SeekerMissileHunterSeekerMissile,
  SelendisHangar,
  SelendisHangarInterceptor,
  SelfRepair,
  SelfRepairCancel,
  SentryGunBurrow,
  SentryGunBurrowBurrowTurret,
  SentryGunUnburrow,
  SentryGunUnburrowUnburrowTurret,
  ShakurasLightBridgeNe10,
  ShakurasLightBridgeNe10BridgeRetract,
  ShakurasLightBridgeNe10Out,
  ShakurasLightBridgeNe10OutBridgeExtend,
  ShakurasLightBridgeNe12,
  ShakurasLightBridgeNe12BridgeRetract,
  ShakurasLightBridgeNe12Out,
  ShakurasLightBridgeNe12OutBridgeExtend,
  ShakurasLightBridgeNe8,
  ShakurasLightBridgeNe8BridgeRetract,
  ShakurasLightBridgeNe8Out,
  ShakurasLightBridgeNe8OutBridgeExtend,
  ShakurasLightBridgeNw10,
  ShakurasLightBridgeNw10BridgeRetract,
  ShakurasLightBridgeNw10Out,
  ShakurasLightBridgeNw10OutBridgeExtend,
  ShakurasLightBridgeNw12,
  ShakurasLightBridgeNw12BridgeRetract,
  ShakurasLightBridgeNw12Out,
  ShakurasLightBridgeNw12OutBridgeExtend,
  ShakurasLightBridgeNw8,
  ShakurasLightBridgeNw8BridgeRetract,
  ShakurasLightBridgeNw8Out,
  ShakurasLightBridgeNw8OutBridgeExtend,
  Shatter,
  ShieldBatteryRechargeChanneled,
  SiegeBreakerSiege,
  SiegeBreakerSiegeSiegeMode,
  SiegeBreakerUnsiege,
  SiegeMode,
  SingleRecall,
  Siphon,
  SiphonCancel,
  SlaynElementalGrab,
  Smart,
  Snipe,
  SnipeDoT,
  SnowRefineryTerranExtendingBridgeNeShort10,
  SnowRefineryTerranExtendingBridgeNeShort10BridgeRetract,
  SnowRefineryTerranExtendingBridgeNeShort10Out,
  SnowRefineryTerranExtendingBridgeNeShort10OutBridgeExtend,
  SnowRefineryTerranExtendingBridgeNeShort8,
  SnowRefineryTerranExtendingBridgeNeShort8BridgeRetract,
  SnowRefineryTerranExtendingBridgeNeShort8Out,
  SnowRefineryTerranExtendingBridgeNeShort8OutBridgeExtend,
  SnowRefineryTerranExtendingBridgeNwShort10,
  SnowRefineryTerranExtendingBridgeNwShort10BridgeRetract,
  SnowRefineryTerranExtendingBridgeNwShort10Out,
  SnowRefineryTerranExtendingBridgeNwShort10OutBridgeExtend,
  SnowRefineryTerranExtendingBridgeNwShort8,
  SnowRefineryTerranExtendingBridgeNwShort8BridgeRetract,
  SnowRefineryTerranExtendingBridgeNwShort8Out,
  SnowRefineryTerranExtendingBridgeNwShort8OutBridgeExtend,
  SoulChannel,
  SoulChannelCancel,
  SpawnChangeling,
  SpawnChangelingTarget,
  SpawnInfestedTerran,
  SpawnInfestedTerranLocustMp,
  SpawnLarva,
  SpawnLarvaMorphMorphalisk,
  SpawnLocustsTargeted,
  SpawnLocustsTargetedSwarmHost,
  SpawningPoolResearch,
  SpawningPoolResearchZerglingattackspeed,
  SpawningPoolResearchZerglingmovementspeed,
  SpecOpsDropshipTransport,
  SpecOpsDropshipTransportMedivacLoad,
  SpecOpsDropshipTransportMedivacUnloadAll,
  SpectreHoldFire,
  SpectreNukeCancel,
  SpectreNukeSiloArmMagazine,
  SpectreNukeSiloArmMagazineSpectreNukeArm,
  SpectreNukeSpectreNukeCalldown,
  SpectreShield,
  SpectreWeaponsFree,
  SpiderMineUnburrowRangeDummy,
  SpineCrawlerRoot,
  SpineCrawlerRootCancel,
  SpineCrawlerUproot,
  SpineCrawlerUprootCancel,
  SpireResearch,
  SpireResearchZergflyerarmor1,
  SpireResearchZergflyerarmor2,
  SpireResearchZergflyerarmor3,
  SpireResearchZergflyerattack1,
  SpireResearchZergflyerattack2,
  SpireResearchZergflyerattack3,
  SporeCrawlerRoot,
  SporeCrawlerRootCancel,
  SporeCrawlerUproot,
  SporeCrawlerUprootCancel,
  Spray,
  SprayProtoss,
  SprayTerran,
  SprayZerg,
  SsBattlecruiserHunterSeekerAttack,
  SsBattlecruiserHunterSeekerAttackSsShooting,
  SsBattlecruiserMissileAttack,
  SsBattlecruiserMissileAttackSsShooting,
  SsCarrierBossAttackLaunch,
  SsCarrierBossAttackLaunchSsShooting,
  SsCarrierBossAttackTarget,
  SsCarrierBossAttackTargetSsShooting,
  SsCarrierSpawnInterceptor,
  SsCorruptorAttack,
  SsCorruptorAttackSsShooting,
  SsFighterBomb,
  SsFighterShooting,
  SsFighterShootingSsShooting,
  SsInterceptorAttack,
  SsInterceptorAttackSsShooting,
  SsLeviathanSpawnBombs,
  SsLeviathanTentacleAttackL1,
  SsLeviathanTentacleAttackL1NoDelay,
  SsLeviathanTentacleAttackL2,
  SsLeviathanTentacleAttackL2NoDelay,
  SsLeviathanTentacleAttackR1,
  SsLeviathanTentacleAttackR1NoDelay,
  SsLeviathanTentacleAttackR2,
  SsLeviathanTentacleAttackR2NoDelay,
  SsLightningProjectorToggle,
  SsPhoenixShooting,
  SsPhoenixShootingSsShooting,
  SsPowerupMorphToBomb,
  SsPowerupMorphToHealth,
  SsPowerupMorphToSideMissiles,
  SsPowerupMorphToStrongerMissiles,
  SsScienceVesselAttack,
  SsScienceVesselAttackSsShooting,
  SsScienceVesselTeleport,
  SsScienceVesselTeleportZeratulBlink,
  SsScoutAttack,
  SsScoutAttackSsShooting,
  SsSwarmGuardianAttack,
  SsSwarmGuardianAttackSsShooting,
  SsTerraTronBeamAttack,
  SsTerraTronSawAttack,
  SsWraithAttack,
  SsWraithAttackSsShooting,
  StargateTrain,
  StargateTrainCarrier,
  StargateTrainOracle,
  StargateTrainPhoenix,
  StargateTrainTempest,
  StargateTrainVoidRay,
  StarportAddOns,
  StarportAddOnsBuildTechLabStarport,
  StarportAddOnsCancel,
  StarportAddOnsReactor,
  StarportLand,
  StarportLiftOff,
  StarportReactorMorph,
  StarportTechLabMorph,
  StarportTechLabMorphTechLabStarport,
  StarportTechLabResearch,
  StarportTechLabResearchBansheeSpeed,
  StarportTechLabResearchRavenResearchEnhancedMunitions,
  StarportTechLabResearchResearchBallisticRange,
  StarportTechLabResearchResearchBansheeCloak,
  StarportTechLabResearchResearchDurableMaterials,
  StarportTechLabResearchResearchHighCapacityFuelTanks,
  StarportTechLabResearchResearchLiberatorAgMode,
  StarportTechLabResearchResearchMedivacEnergyUpgrade,
  StarportTechLabResearchResearchRapidDeployment,
  StarportTechLabResearchResearchRavenEnergyUpgrade,
  StarportTechLabResearchResearchRavenRecalibratedExplosives,
  StarportTechLabResearchResearchSeekerMissile,
  StarportTechReactorMorph,
  StarportTechReactorMorphTechLabStarport,
  StarportTrain,
  StarportTrainBanshee,
  StarportTrainBattlecruiser,
  StarportTrainLiberator,
  StarportTrainMedivac,
  StarportTrainRaven,
  StarportTrainVikingFighter,
  StartColonistVehicle,
  Stimpack,
  StimpackMarauder,
  StimpackMarauderRedirectStimRedirect,
  StimpackRedirectStimRedirect,
  Stop,
  StopCheer,
  StopDance,
  StopHoldFireSpecial,
  StopProtossBuilding,
  StopProtossBuildingCheer,
  StopProtossBuildingDance,
  StopProtossBuildingHoldFire,
  StopRedirect,
  SummonMercenaries,
  SummonMercenariesHireDevilDogs,
  SummonMercenariesHireDukesRevenge,
  SummonMercenariesHireDuskWing,
  SummonMercenariesHireHammerSecurities,
  SummonMercenariesHireHelsAngels,
  SummonMercenariesHireKelmorianMiners,
  SummonMercenariesHireSiegeBreakers,
  SummonMercenariesHireSpartanCompany,
  SummonMercenariesPh,
  SummonMercenariesPhHireKelmorianMinersPh,
  SuperWarpGateTrain,
  SuperWarpGateTrainArchon,
  SuperWarpGateTrainCarrier,
  SuperWarpGateTrainColossus,
  SuperWarpGateTrainDarkTemplar,
  SuperWarpGateTrainHighTemplar,
  SuperWarpGateTrainImmortal,
  SuperWarpGateTrainPhoenix,
  SuperWarpGateTrainSentry,
  SuperWarpGateTrainStalker,
  SuperWarpGateTrainVoidRay,
  SuperWarpGateTrainWarpInMohandar,
  SuperWarpGateTrainWarpInScout,
  SuperWarpGateTrainWarpInSelendis,
  SuperWarpGateTrainWarpInUrun,
  SuperWarpGateTrainWarpInZeratul,
  SuperWarpGateTrainWarpPrism,
  SuperWarpGateTrainZealot,
  SupplyDepotLower,
  SupplyDepotRaise,
  SupplyDrop,
  SwarmHostSpawnLocusts,
  SwarmHostSpawnLocustsLocustMp,
}
impl S {
    pub fn from_id(id: u16) -> Option<Self> {
      match id {
      2008 => Some(Self::Sacrifice),
      2009 => Some(Self::Sacrifice),
      1903 => Some(Self::SalvageBaneling),
      1904 => Some(Self::SalvageBaneling),
      1901 => Some(Self::SalvageBanelingRefund),
      1902 => Some(Self::SalvageBanelingRefund),
      1626 => Some(Self::SalvageBunker),
      1627 => Some(Self::SalvageBunker),
      1624 => Some(Self::SalvageBunkerRefund),
      1625 => Some(Self::SalvageBunkerRefund),
      1887 => Some(Self::SalvageDrone),
      1888 => Some(Self::SalvageDrone),
      1885 => Some(Self::SalvageDroneRefund),
      1886 => Some(Self::SalvageDroneRefund),
      1907 => Some(Self::SalvageHydralisk),
      1908 => Some(Self::SalvageHydralisk),
      1905 => Some(Self::SalvageHydraliskRefund),
      1906 => Some(Self::SalvageHydraliskRefund),
      1911 => Some(Self::SalvageInfestor),
      1912 => Some(Self::SalvageInfestor),
      1909 => Some(Self::SalvageInfestorRefund),
      1910 => Some(Self::SalvageInfestorRefund),
      1895 => Some(Self::SalvageQueen),
      1896 => Some(Self::SalvageQueen),
      1893 => Some(Self::SalvageQueenRefund),
      1894 => Some(Self::SalvageQueenRefund),
      1899 => Some(Self::SalvageRoach),
      1900 => Some(Self::SalvageRoach),
      1897 => Some(Self::SalvageRoachRefund),
      1898 => Some(Self::SalvageRoachRefund),
      32 => Some(Self::SalvageShared),
      33 => Some(Self::SalvageShared),
      1915 => Some(Self::SalvageSwarmHost),
      1916 => Some(Self::SalvageSwarmHost),
      1913 => Some(Self::SalvageSwarmHostRefund),
      1914 => Some(Self::SalvageSwarmHostRefund),
      1919 => Some(Self::SalvageUltralisk),
      1920 => Some(Self::SalvageUltralisk),
      1917 => Some(Self::SalvageUltraliskRefund),
      1918 => Some(Self::SalvageUltraliskRefund),
      1891 => Some(Self::SalvageZergling),
      1892 => Some(Self::SalvageZergling),
      1889 => Some(Self::SalvageZerglingRefund),
      1890 => Some(Self::SalvageZerglingRefund),
      245 => Some(Self::SapStructure),
      246 => Some(Self::SapStructure),
      399 => Some(Self::ScannerSweep),
      400 => Some(Self::ScannerSweep),
      1982 => Some(Self::Scryer),
      1983 => Some(Self::Scryer),
      297 => Some(Self::ScvHarvest),
      295 => Some(Self::ScvHarvestGather),
      296 => Some(Self::ScvHarvestReturnCargo),
      3606 => Some(Self::SecurityGateDiagonalBlur),
      3605 => Some(Self::SecurityGateDiagonalBlurGateClose),
      3594 => Some(Self::SecurityGateDiagonalBlurLowered),
      3593 => Some(Self::SecurityGateDiagonalBlurLoweredGateOpen),
      3608 => Some(Self::SecurityGateDiagonalUlbr),
      3607 => Some(Self::SecurityGateDiagonalUlbrGateClose),
      3596 => Some(Self::SecurityGateDiagonalUlbrLowered),
      3595 => Some(Self::SecurityGateDiagonalUlbrLoweredGateOpen),
      3612 => Some(Self::SecurityGateStraightHorizontal),
      3610 => Some(Self::SecurityGateStraightHorizontalBf),
      3609 => Some(Self::SecurityGateStraightHorizontalBfGateClose),
      3598 => Some(Self::SecurityGateStraightHorizontalBfLowered),
      3597 => Some(Self::SecurityGateStraightHorizontalBfLoweredGateOpen),
      3611 => Some(Self::SecurityGateStraightHorizontalGateClose),
      3600 => Some(Self::SecurityGateStraightHorizontalLowered),
      3599 => Some(Self::SecurityGateStraightHorizontalLoweredGateOpen),
      3616 => Some(Self::SecurityGateStraightVertical),
      3615 => Some(Self::SecurityGateStraightVerticalGateClose),
      3614 => Some(Self::SecurityGateStraightVerticalLf),
      3613 => Some(Self::SecurityGateStraightVerticalLfGateClose),
      3602 => Some(Self::SecurityGateStraightVerticalLfLowered),
      3601 => Some(Self::SecurityGateStraightVerticalLfLoweredGateOpen),
      3604 => Some(Self::SecurityGateStraightVerticalLowered),
      3603 => Some(Self::SecurityGateStraightVerticalLoweredGateOpen),
      2395 => Some(Self::SeekerDummyChannel),
      2396 => Some(Self::SeekerDummyChannel),
      170 => Some(Self::SeekerMissile),
      169 => Some(Self::SeekerMissileHunterSeekerMissile),
      2968 => Some(Self::SelendisHangar),
      2969 => Some(Self::SelendisHangar),
      2970 => Some(Self::SelendisHangar),
      2971 => Some(Self::SelendisHangar),
      2972 => Some(Self::SelendisHangar),
      2973 => Some(Self::SelendisHangar),
      2974 => Some(Self::SelendisHangar),
      2975 => Some(Self::SelendisHangar),
      2976 => Some(Self::SelendisHangar),
      2977 => Some(Self::SelendisHangar),
      2978 => Some(Self::SelendisHangar),
      2979 => Some(Self::SelendisHangar),
      2980 => Some(Self::SelendisHangar),
      2981 => Some(Self::SelendisHangar),
      2982 => Some(Self::SelendisHangar),
      2983 => Some(Self::SelendisHangar),
      2984 => Some(Self::SelendisHangar),
      2985 => Some(Self::SelendisHangar),
      2986 => Some(Self::SelendisHangar),
      2967 => Some(Self::SelendisHangarInterceptor),
      2538 => Some(Self::SelfRepair),
      2539 => Some(Self::SelfRepairCancel),
      3000 => Some(Self::SentryGunBurrow),
      2999 => Some(Self::SentryGunBurrowBurrowTurret),
      3002 => Some(Self::SentryGunUnburrow),
      3001 => Some(Self::SentryGunUnburrowUnburrowTurret),
      2452 => Some(Self::ShakurasLightBridgeNe10),
      2451 => Some(Self::ShakurasLightBridgeNe10BridgeRetract),
      2450 => Some(Self::ShakurasLightBridgeNe10Out),
      2449 => Some(Self::ShakurasLightBridgeNe10OutBridgeExtend),
      2456 => Some(Self::ShakurasLightBridgeNe12),
      2455 => Some(Self::ShakurasLightBridgeNe12BridgeRetract),
      2454 => Some(Self::ShakurasLightBridgeNe12Out),
      2453 => Some(Self::ShakurasLightBridgeNe12OutBridgeExtend),
      2448 => Some(Self::ShakurasLightBridgeNe8),
      2447 => Some(Self::ShakurasLightBridgeNe8BridgeRetract),
      2446 => Some(Self::ShakurasLightBridgeNe8Out),
      2445 => Some(Self::ShakurasLightBridgeNe8OutBridgeExtend),
      2464 => Some(Self::ShakurasLightBridgeNw10),
      2463 => Some(Self::ShakurasLightBridgeNw10BridgeRetract),
      2462 => Some(Self::ShakurasLightBridgeNw10Out),
      2461 => Some(Self::ShakurasLightBridgeNw10OutBridgeExtend),
      2468 => Some(Self::ShakurasLightBridgeNw12),
      2467 => Some(Self::ShakurasLightBridgeNw12BridgeRetract),
      2466 => Some(Self::ShakurasLightBridgeNw12Out),
      2465 => Some(Self::ShakurasLightBridgeNw12OutBridgeExtend),
      2460 => Some(Self::ShakurasLightBridgeNw8),
      2459 => Some(Self::ShakurasLightBridgeNw8BridgeRetract),
      2458 => Some(Self::ShakurasLightBridgeNw8Out),
      2457 => Some(Self::ShakurasLightBridgeNw8OutBridgeExtend),
      1827 => Some(Self::Shatter),
      1828 => Some(Self::Shatter),
      3765 => Some(Self::ShieldBatteryRechargeChanneled),
      3766 => Some(Self::ShieldBatteryRechargeChanneled),
      2990 => Some(Self::SiegeBreakerSiege),
      2989 => Some(Self::SiegeBreakerSiegeSiegeMode),
      2991 => Some(Self::SiegeBreakerUnsiege),
      2992 => Some(Self::SiegeBreakerUnsiege),
      388 => Some(Self::SiegeMode),
      389 => Some(Self::SiegeMode),
      1976 => Some(Self::SingleRecall),
      1977 => Some(Self::SingleRecall),
      177 => Some(Self::Siphon),
      178 => Some(Self::SiphonCancel),
      2598 => Some(Self::SlaynElementalGrab),
      2599 => Some(Self::SlaynElementalGrab),
      1 => Some(Self::Smart),
      384 => Some(Self::Snipe),
      385 => Some(Self::Snipe),
      1988 => Some(Self::SnipeDoT),
      1989 => Some(Self::SnipeDoT),
      2257 => Some(Self::SnowRefineryTerranExtendingBridgeNeShort10),
      2256 => Some(Self::SnowRefineryTerranExtendingBridgeNeShort10BridgeRetract),
      2255 => Some(Self::SnowRefineryTerranExtendingBridgeNeShort10Out),
      2254 => Some(Self::SnowRefineryTerranExtendingBridgeNeShort10OutBridgeExtend),
      2249 => Some(Self::SnowRefineryTerranExtendingBridgeNeShort8),
      2248 => Some(Self::SnowRefineryTerranExtendingBridgeNeShort8BridgeRetract),
      2247 => Some(Self::SnowRefineryTerranExtendingBridgeNeShort8Out),
      2246 => Some(Self::SnowRefineryTerranExtendingBridgeNeShort8OutBridgeExtend),
      2261 => Some(Self::SnowRefineryTerranExtendingBridgeNwShort10),
      2260 => Some(Self::SnowRefineryTerranExtendingBridgeNwShort10BridgeRetract),
      2259 => Some(Self::SnowRefineryTerranExtendingBridgeNwShort10Out),
      2258 => Some(Self::SnowRefineryTerranExtendingBridgeNwShort10OutBridgeExtend),
      2253 => Some(Self::SnowRefineryTerranExtendingBridgeNwShort8),
      2252 => Some(Self::SnowRefineryTerranExtendingBridgeNwShort8BridgeRetract),
      2251 => Some(Self::SnowRefineryTerranExtendingBridgeNwShort8Out),
      2250 => Some(Self::SnowRefineryTerranExtendingBridgeNwShort8OutBridgeExtend),
      2993 => Some(Self::SoulChannel),
      2994 => Some(Self::SoulChannelCancel),
      181 => Some(Self::SpawnChangeling),
      182 => Some(Self::SpawnChangeling),
      1934 => Some(Self::SpawnChangelingTarget),
      1935 => Some(Self::SpawnChangelingTarget),
      2019 => Some(Self::SpawnInfestedTerran),
      2020 => Some(Self::SpawnInfestedTerran),
      2021 => Some(Self::SpawnInfestedTerran),
      2022 => Some(Self::SpawnInfestedTerran),
      2023 => Some(Self::SpawnInfestedTerran),
      2024 => Some(Self::SpawnInfestedTerran),
      2025 => Some(Self::SpawnInfestedTerran),
      2026 => Some(Self::SpawnInfestedTerran),
      2027 => Some(Self::SpawnInfestedTerran),
      2028 => Some(Self::SpawnInfestedTerran),
      2029 => Some(Self::SpawnInfestedTerran),
      2030 => Some(Self::SpawnInfestedTerran),
      2031 => Some(Self::SpawnInfestedTerran),
      2032 => Some(Self::SpawnInfestedTerran),
      2033 => Some(Self::SpawnInfestedTerran),
      2034 => Some(Self::SpawnInfestedTerran),
      2035 => Some(Self::SpawnInfestedTerran),
      2036 => Some(Self::SpawnInfestedTerran),
      2037 => Some(Self::SpawnInfestedTerran),
      2038 => Some(Self::SpawnInfestedTerran),
      2039 => Some(Self::SpawnInfestedTerran),
      2040 => Some(Self::SpawnInfestedTerran),
      2041 => Some(Self::SpawnInfestedTerran),
      2042 => Some(Self::SpawnInfestedTerran),
      2043 => Some(Self::SpawnInfestedTerran),
      2044 => Some(Self::SpawnInfestedTerran),
      2045 => Some(Self::SpawnInfestedTerran),
      2046 => Some(Self::SpawnInfestedTerran),
      2047 => Some(Self::SpawnInfestedTerran),
      2018 => Some(Self::SpawnInfestedTerranLocustMp),
      252 => Some(Self::SpawnLarva),
      251 => Some(Self::SpawnLarvaMorphMorphalisk),
      2705 => Some(Self::SpawnLocustsTargeted),
      2704 => Some(Self::SpawnLocustsTargetedSwarmHost),
      1254 => Some(Self::SpawningPoolResearch),
      1255 => Some(Self::SpawningPoolResearch),
      1256 => Some(Self::SpawningPoolResearch),
      1257 => Some(Self::SpawningPoolResearch),
      1258 => Some(Self::SpawningPoolResearch),
      1259 => Some(Self::SpawningPoolResearch),
      1260 => Some(Self::SpawningPoolResearch),
      1261 => Some(Self::SpawningPoolResearch),
      1262 => Some(Self::SpawningPoolResearch),
      1263 => Some(Self::SpawningPoolResearch),
      1264 => Some(Self::SpawningPoolResearch),
      1265 => Some(Self::SpawningPoolResearch),
      1266 => Some(Self::SpawningPoolResearch),
      1267 => Some(Self::SpawningPoolResearch),
      1268 => Some(Self::SpawningPoolResearch),
      1269 => Some(Self::SpawningPoolResearch),
      1270 => Some(Self::SpawningPoolResearch),
      1271 => Some(Self::SpawningPoolResearch),
      1272 => Some(Self::SpawningPoolResearch),
      1273 => Some(Self::SpawningPoolResearch),
      1274 => Some(Self::SpawningPoolResearch),
      1275 => Some(Self::SpawningPoolResearch),
      1276 => Some(Self::SpawningPoolResearch),
      1277 => Some(Self::SpawningPoolResearch),
      1278 => Some(Self::SpawningPoolResearch),
      1279 => Some(Self::SpawningPoolResearch),
      1280 => Some(Self::SpawningPoolResearch),
      1281 => Some(Self::SpawningPoolResearch),
      1252 => Some(Self::SpawningPoolResearchZerglingattackspeed),
      1253 => Some(Self::SpawningPoolResearchZerglingmovementspeed),
      2948 => Some(Self::SpecOpsDropshipTransport),
      2950 => Some(Self::SpecOpsDropshipTransport),
      2951 => Some(Self::SpecOpsDropshipTransport),
      2947 => Some(Self::SpecOpsDropshipTransportMedivacLoad),
      2949 => Some(Self::SpecOpsDropshipTransportMedivacUnloadAll),
      3621 => Some(Self::SpectreHoldFire),
      3622 => Some(Self::SpectreHoldFire),
      3325 => Some(Self::SpectreNukeCancel),
      3326 => Some(Self::SpectreNukeSiloArmMagazine),
      3328 => Some(Self::SpectreNukeSiloArmMagazine),
      3329 => Some(Self::SpectreNukeSiloArmMagazine),
      3330 => Some(Self::SpectreNukeSiloArmMagazine),
      3331 => Some(Self::SpectreNukeSiloArmMagazine),
      3332 => Some(Self::SpectreNukeSiloArmMagazine),
      3333 => Some(Self::SpectreNukeSiloArmMagazine),
      3334 => Some(Self::SpectreNukeSiloArmMagazine),
      3335 => Some(Self::SpectreNukeSiloArmMagazine),
      3336 => Some(Self::SpectreNukeSiloArmMagazine),
      3337 => Some(Self::SpectreNukeSiloArmMagazine),
      3338 => Some(Self::SpectreNukeSiloArmMagazine),
      3339 => Some(Self::SpectreNukeSiloArmMagazine),
      3340 => Some(Self::SpectreNukeSiloArmMagazine),
      3341 => Some(Self::SpectreNukeSiloArmMagazine),
      3342 => Some(Self::SpectreNukeSiloArmMagazine),
      3343 => Some(Self::SpectreNukeSiloArmMagazine),
      3344 => Some(Self::SpectreNukeSiloArmMagazine),
      3345 => Some(Self::SpectreNukeSiloArmMagazine),
      3327 => Some(Self::SpectreNukeSiloArmMagazineSpectreNukeArm),
      3324 => Some(Self::SpectreNukeSpectreNukeCalldown),
      1926 => Some(Self::SpectreShield),
      1927 => Some(Self::SpectreShield),
      3623 => Some(Self::SpectreWeaponsFree),
      3624 => Some(Self::SpectreWeaponsFree),
      3003 => Some(Self::SpiderMineUnburrowRangeDummy),
      3004 => Some(Self::SpiderMineUnburrowRangeDummy),
      1729 => Some(Self::SpineCrawlerRoot),
      1730 => Some(Self::SpineCrawlerRootCancel),
      1725 => Some(Self::SpineCrawlerUproot),
      1726 => Some(Self::SpineCrawlerUprootCancel),
      1318 => Some(Self::SpireResearch),
      1319 => Some(Self::SpireResearch),
      1320 => Some(Self::SpireResearch),
      1321 => Some(Self::SpireResearch),
      1322 => Some(Self::SpireResearch),
      1323 => Some(Self::SpireResearch),
      1324 => Some(Self::SpireResearch),
      1325 => Some(Self::SpireResearch),
      1326 => Some(Self::SpireResearch),
      1327 => Some(Self::SpireResearch),
      1328 => Some(Self::SpireResearch),
      1329 => Some(Self::SpireResearch),
      1330 => Some(Self::SpireResearch),
      1331 => Some(Self::SpireResearch),
      1332 => Some(Self::SpireResearch),
      1333 => Some(Self::SpireResearch),
      1334 => Some(Self::SpireResearch),
      1335 => Some(Self::SpireResearch),
      1336 => Some(Self::SpireResearch),
      1337 => Some(Self::SpireResearch),
      1338 => Some(Self::SpireResearch),
      1339 => Some(Self::SpireResearch),
      1340 => Some(Self::SpireResearch),
      1341 => Some(Self::SpireResearch),
      1315 => Some(Self::SpireResearchZergflyerarmor1),
      1316 => Some(Self::SpireResearchZergflyerarmor2),
      1317 => Some(Self::SpireResearchZergflyerarmor3),
      1312 => Some(Self::SpireResearchZergflyerattack1),
      1313 => Some(Self::SpireResearchZergflyerattack2),
      1314 => Some(Self::SpireResearchZergflyerattack3),
      1731 => Some(Self::SporeCrawlerRoot),
      1732 => Some(Self::SporeCrawlerRootCancel),
      1727 => Some(Self::SporeCrawlerUproot),
      1728 => Some(Self::SporeCrawlerUprootCancel),
      3684 => Some(Self::Spray),
      30 => Some(Self::SprayProtoss),
      31 => Some(Self::SprayProtoss),
      26 => Some(Self::SprayTerran),
      27 => Some(Self::SprayTerran),
      28 => Some(Self::SprayZerg),
      29 => Some(Self::SprayZerg),
      2794 => Some(Self::SsBattlecruiserHunterSeekerAttack),
      2793 => Some(Self::SsBattlecruiserHunterSeekerAttackSsShooting),
      2790 => Some(Self::SsBattlecruiserMissileAttack),
      2789 => Some(Self::SsBattlecruiserMissileAttackSsShooting),
      2776 => Some(Self::SsCarrierBossAttackLaunch),
      2775 => Some(Self::SsCarrierBossAttackLaunchSsShooting),
      2780 => Some(Self::SsCarrierBossAttackTarget),
      2779 => Some(Self::SsCarrierBossAttackTargetSsShooting),
      2777 => Some(Self::SsCarrierSpawnInterceptor),
      2778 => Some(Self::SsCarrierSpawnInterceptor),
      2824 => Some(Self::SsCorruptorAttack),
      2823 => Some(Self::SsCorruptorAttackSsShooting),
      2781 => Some(Self::SsFighterBomb),
      2782 => Some(Self::SsFighterBomb),
      3311 => Some(Self::SsFighterShooting),
      3310 => Some(Self::SsFighterShootingSsShooting),
      2822 => Some(Self::SsInterceptorAttack),
      2821 => Some(Self::SsInterceptorAttackSsShooting),
      2791 => Some(Self::SsLeviathanSpawnBombs),
      2792 => Some(Self::SsLeviathanSpawnBombs),
      2829 => Some(Self::SsLeviathanTentacleAttackL1),
      2830 => Some(Self::SsLeviathanTentacleAttackL1),
      2797 => Some(Self::SsLeviathanTentacleAttackL1NoDelay),
      2798 => Some(Self::SsLeviathanTentacleAttackL1NoDelay),
      2825 => Some(Self::SsLeviathanTentacleAttackL2),
      2826 => Some(Self::SsLeviathanTentacleAttackL2),
      2799 => Some(Self::SsLeviathanTentacleAttackL2NoDelay),
      2800 => Some(Self::SsLeviathanTentacleAttackL2NoDelay),
      2827 => Some(Self::SsLeviathanTentacleAttackR1),
      2828 => Some(Self::SsLeviathanTentacleAttackR1),
      2801 => Some(Self::SsLeviathanTentacleAttackR1NoDelay),
      2802 => Some(Self::SsLeviathanTentacleAttackR1NoDelay),
      2831 => Some(Self::SsLeviathanTentacleAttackR2),
      2832 => Some(Self::SsLeviathanTentacleAttackR2),
      2803 => Some(Self::SsLeviathanTentacleAttackR2NoDelay),
      2804 => Some(Self::SsLeviathanTentacleAttackR2NoDelay),
      2783 => Some(Self::SsLightningProjectorToggle),
      2784 => Some(Self::SsLightningProjectorToggle),
      2786 => Some(Self::SsPhoenixShooting),
      2785 => Some(Self::SsPhoenixShootingSsShooting),
      2787 => Some(Self::SsPowerupMorphToBomb),
      2788 => Some(Self::SsPowerupMorphToBomb),
      2795 => Some(Self::SsPowerupMorphToHealth),
      2796 => Some(Self::SsPowerupMorphToHealth),
      2815 => Some(Self::SsPowerupMorphToSideMissiles),
      2816 => Some(Self::SsPowerupMorphToSideMissiles),
      2817 => Some(Self::SsPowerupMorphToStrongerMissiles),
      2818 => Some(Self::SsPowerupMorphToStrongerMissiles),
      2834 => Some(Self::SsScienceVesselAttack),
      2833 => Some(Self::SsScienceVesselAttackSsShooting),
      2806 => Some(Self::SsScienceVesselTeleport),
      2805 => Some(Self::SsScienceVesselTeleportZeratulBlink),
      2820 => Some(Self::SsScoutAttack),
      2819 => Some(Self::SsScoutAttackSsShooting),
      2814 => Some(Self::SsSwarmGuardianAttack),
      2813 => Some(Self::SsSwarmGuardianAttackSsShooting),
      2807 => Some(Self::SsTerraTronBeamAttack),
      2808 => Some(Self::SsTerraTronBeamAttack),
      2809 => Some(Self::SsTerraTronSawAttack),
      2810 => Some(Self::SsTerraTronSawAttack),
      2812 => Some(Self::SsWraithAttack),
      2811 => Some(Self::SsWraithAttackSsShooting),
      947 => Some(Self::StargateTrain),
      949 => Some(Self::StargateTrain),
      951 => Some(Self::StargateTrain),
      952 => Some(Self::StargateTrain),
      953 => Some(Self::StargateTrain),
      956 => Some(Self::StargateTrain),
      957 => Some(Self::StargateTrain),
      958 => Some(Self::StargateTrain),
      959 => Some(Self::StargateTrain),
      960 => Some(Self::StargateTrain),
      961 => Some(Self::StargateTrain),
      962 => Some(Self::StargateTrain),
      963 => Some(Self::StargateTrain),
      964 => Some(Self::StargateTrain),
      965 => Some(Self::StargateTrain),
      966 => Some(Self::StargateTrain),
      967 => Some(Self::StargateTrain),
      968 => Some(Self::StargateTrain),
      969 => Some(Self::StargateTrain),
      970 => Some(Self::StargateTrain),
      971 => Some(Self::StargateTrain),
      972 => Some(Self::StargateTrain),
      973 => Some(Self::StargateTrain),
      974 => Some(Self::StargateTrain),
      975 => Some(Self::StargateTrain),
      948 => Some(Self::StargateTrainCarrier),
      954 => Some(Self::StargateTrainOracle),
      946 => Some(Self::StargateTrainPhoenix),
      955 => Some(Self::StargateTrainTempest),
      950 => Some(Self::StargateTrainVoidRay),
      489 => Some(Self::StarportAddOns),
      490 => Some(Self::StarportAddOns),
      491 => Some(Self::StarportAddOns),
      492 => Some(Self::StarportAddOns),
      493 => Some(Self::StarportAddOns),
      494 => Some(Self::StarportAddOns),
      495 => Some(Self::StarportAddOns),
      496 => Some(Self::StarportAddOns),
      497 => Some(Self::StarportAddOns),
      498 => Some(Self::StarportAddOns),
      499 => Some(Self::StarportAddOns),
      500 => Some(Self::StarportAddOns),
      501 => Some(Self::StarportAddOns),
      502 => Some(Self::StarportAddOns),
      503 => Some(Self::StarportAddOns),
      504 => Some(Self::StarportAddOns),
      505 => Some(Self::StarportAddOns),
      506 => Some(Self::StarportAddOns),
      507 => Some(Self::StarportAddOns),
      508 => Some(Self::StarportAddOns),
      509 => Some(Self::StarportAddOns),
      510 => Some(Self::StarportAddOns),
      511 => Some(Self::StarportAddOns),
      512 => Some(Self::StarportAddOns),
      513 => Some(Self::StarportAddOns),
      514 => Some(Self::StarportAddOns),
      515 => Some(Self::StarportAddOns),
      516 => Some(Self::StarportAddOns),
      487 => Some(Self::StarportAddOnsBuildTechLabStarport),
      517 => Some(Self::StarportAddOnsCancel),
      488 => Some(Self::StarportAddOnsReactor),
      522 => Some(Self::StarportLand),
      523 => Some(Self::StarportLand),
      518 => Some(Self::StarportLiftOff),
      519 => Some(Self::StarportLiftOff),
      1680 => Some(Self::StarportReactorMorph),
      1681 => Some(Self::StarportReactorMorph),
      1673 => Some(Self::StarportTechLabMorph),
      1672 => Some(Self::StarportTechLabMorphTechLabStarport),
      791 => Some(Self::StarportTechLabResearch),
      794 => Some(Self::StarportTechLabResearch),
      795 => Some(Self::StarportTechLabResearch),
      798 => Some(Self::StarportTechLabResearch),
      801 => Some(Self::StarportTechLabResearch),
      807 => Some(Self::StarportTechLabResearch),
      808 => Some(Self::StarportTechLabResearch),
      809 => Some(Self::StarportTechLabResearch),
      810 => Some(Self::StarportTechLabResearch),
      811 => Some(Self::StarportTechLabResearch),
      812 => Some(Self::StarportTechLabResearch),
      813 => Some(Self::StarportTechLabResearch),
      814 => Some(Self::StarportTechLabResearch),
      815 => Some(Self::StarportTechLabResearch),
      816 => Some(Self::StarportTechLabResearch),
      817 => Some(Self::StarportTechLabResearch),
      818 => Some(Self::StarportTechLabResearch),
      819 => Some(Self::StarportTechLabResearch),
      799 => Some(Self::StarportTechLabResearchBansheeSpeed),
      806 => Some(Self::StarportTechLabResearchRavenResearchEnhancedMunitions),
      805 => Some(Self::StarportTechLabResearchResearchBallisticRange),
      790 => Some(Self::StarportTechLabResearchResearchBansheeCloak),
      797 => Some(Self::StarportTechLabResearchResearchDurableMaterials),
      804 => Some(Self::StarportTechLabResearchResearchHighCapacityFuelTanks),
      800 => Some(Self::StarportTechLabResearchResearchLiberatorAgMode),
      792 => Some(Self::StarportTechLabResearchResearchMedivacEnergyUpgrade),
      802 => Some(Self::StarportTechLabResearchResearchRapidDeployment),
      793 => Some(Self::StarportTechLabResearchResearchRavenEnergyUpgrade),
      803 => Some(Self::StarportTechLabResearchResearchRavenRecalibratedExplosives),
      796 => Some(Self::StarportTechLabResearchResearchSeekerMissile),
      3309 => Some(Self::StarportTechReactorMorph),
      3308 => Some(Self::StarportTechReactorMorphTechLabStarport),
      625 => Some(Self::StarportTrain),
      627 => Some(Self::StarportTrain),
      628 => Some(Self::StarportTrain),
      629 => Some(Self::StarportTrain),
      630 => Some(Self::StarportTrain),
      631 => Some(Self::StarportTrain),
      632 => Some(Self::StarportTrain),
      633 => Some(Self::StarportTrain),
      634 => Some(Self::StarportTrain),
      635 => Some(Self::StarportTrain),
      636 => Some(Self::StarportTrain),
      637 => Some(Self::StarportTrain),
      638 => Some(Self::StarportTrain),
      639 => Some(Self::StarportTrain),
      640 => Some(Self::StarportTrain),
      641 => Some(Self::StarportTrain),
      642 => Some(Self::StarportTrain),
      643 => Some(Self::StarportTrain),
      644 => Some(Self::StarportTrain),
      645 => Some(Self::StarportTrain),
      646 => Some(Self::StarportTrain),
      647 => Some(Self::StarportTrain),
      648 => Some(Self::StarportTrain),
      649 => Some(Self::StarportTrain),
      621 => Some(Self::StarportTrainBanshee),
      623 => Some(Self::StarportTrainBattlecruiser),
      626 => Some(Self::StarportTrainLiberator),
      620 => Some(Self::StarportTrainMedivac),
      622 => Some(Self::StarportTrainRaven),
      624 => Some(Self::StarportTrainVikingFighter),
      2905 => Some(Self::StartColonistVehicle),
      2906 => Some(Self::StartColonistVehicle),
      380 => Some(Self::Stimpack),
      381 => Some(Self::Stimpack),
      253 => Some(Self::StimpackMarauder),
      254 => Some(Self::StimpackMarauder),
      1684 => Some(Self::StimpackMarauderRedirectStimRedirect),
      1683 => Some(Self::StimpackRedirectStimRedirect),
      4 => Some(Self::Stop),
      8 => Some(Self::Stop),
      9 => Some(Self::Stop),
      3665 => Some(Self::Stop),
      6 => Some(Self::StopCheer),
      7 => Some(Self::StopDance),
      5 => Some(Self::StopHoldFireSpecial),
      2057 => Some(Self::StopProtossBuilding),
      2061 => Some(Self::StopProtossBuilding),
      2062 => Some(Self::StopProtossBuilding),
      2059 => Some(Self::StopProtossBuildingCheer),
      2060 => Some(Self::StopProtossBuildingDance),
      2058 => Some(Self::StopProtossBuildingHoldFire),
      1691 => Some(Self::StopRedirect),
      3023 => Some(Self::SummonMercenaries),
      3024 => Some(Self::SummonMercenaries),
      3025 => Some(Self::SummonMercenaries),
      3026 => Some(Self::SummonMercenaries),
      3027 => Some(Self::SummonMercenaries),
      3028 => Some(Self::SummonMercenaries),
      3029 => Some(Self::SummonMercenaries),
      3030 => Some(Self::SummonMercenaries),
      3031 => Some(Self::SummonMercenaries),
      3032 => Some(Self::SummonMercenaries),
      3033 => Some(Self::SummonMercenaries),
      3034 => Some(Self::SummonMercenaries),
      3035 => Some(Self::SummonMercenaries),
      3036 => Some(Self::SummonMercenaries),
      3037 => Some(Self::SummonMercenaries),
      3038 => Some(Self::SummonMercenaries),
      3039 => Some(Self::SummonMercenaries),
      3040 => Some(Self::SummonMercenaries),
      3041 => Some(Self::SummonMercenaries),
      3042 => Some(Self::SummonMercenaries),
      3043 => Some(Self::SummonMercenaries),
      3044 => Some(Self::SummonMercenaries),
      3016 => Some(Self::SummonMercenariesHireDevilDogs),
      3022 => Some(Self::SummonMercenariesHireDukesRevenge),
      3021 => Some(Self::SummonMercenariesHireDuskWing),
      3018 => Some(Self::SummonMercenariesHireHammerSecurities),
      3020 => Some(Self::SummonMercenariesHireHelsAngels),
      3015 => Some(Self::SummonMercenariesHireKelmorianMiners),
      3019 => Some(Self::SummonMercenariesHireSiegeBreakers),
      3017 => Some(Self::SummonMercenariesHireSpartanCompany),
      3046 => Some(Self::SummonMercenariesPh),
      3047 => Some(Self::SummonMercenariesPh),
      3048 => Some(Self::SummonMercenariesPh),
      3049 => Some(Self::SummonMercenariesPh),
      3050 => Some(Self::SummonMercenariesPh),
      3051 => Some(Self::SummonMercenariesPh),
      3052 => Some(Self::SummonMercenariesPh),
      3053 => Some(Self::SummonMercenariesPh),
      3054 => Some(Self::SummonMercenariesPh),
      3055 => Some(Self::SummonMercenariesPh),
      3056 => Some(Self::SummonMercenariesPh),
      3057 => Some(Self::SummonMercenariesPh),
      3058 => Some(Self::SummonMercenariesPh),
      3059 => Some(Self::SummonMercenariesPh),
      3060 => Some(Self::SummonMercenariesPh),
      3061 => Some(Self::SummonMercenariesPh),
      3062 => Some(Self::SummonMercenariesPh),
      3063 => Some(Self::SummonMercenariesPh),
      3064 => Some(Self::SummonMercenariesPh),
      3065 => Some(Self::SummonMercenariesPh),
      3066 => Some(Self::SummonMercenariesPh),
      3067 => Some(Self::SummonMercenariesPh),
      3068 => Some(Self::SummonMercenariesPh),
      3069 => Some(Self::SummonMercenariesPh),
      3070 => Some(Self::SummonMercenariesPh),
      3071 => Some(Self::SummonMercenariesPh),
      3072 => Some(Self::SummonMercenariesPh),
      3073 => Some(Self::SummonMercenariesPh),
      3074 => Some(Self::SummonMercenariesPh),
      3045 => Some(Self::SummonMercenariesPhHireKelmorianMinersPh),
      3217 => Some(Self::SuperWarpGateTrain),
      3218 => Some(Self::SuperWarpGateTrain),
      3219 => Some(Self::SuperWarpGateTrain),
      3209 => Some(Self::SuperWarpGateTrainArchon),
      3206 => Some(Self::SuperWarpGateTrainCarrier),
      3215 => Some(Self::SuperWarpGateTrainColossus),
      3204 => Some(Self::SuperWarpGateTrainDarkTemplar),
      3203 => Some(Self::SuperWarpGateTrainHighTemplar),
      3202 => Some(Self::SuperWarpGateTrainImmortal),
      3207 => Some(Self::SuperWarpGateTrainPhoenix),
      3205 => Some(Self::SuperWarpGateTrainSentry),
      3201 => Some(Self::SuperWarpGateTrainStalker),
      3208 => Some(Self::SuperWarpGateTrainVoidRay),
      3212 => Some(Self::SuperWarpGateTrainWarpInMohandar),
      3214 => Some(Self::SuperWarpGateTrainWarpInScout),
      3213 => Some(Self::SuperWarpGateTrainWarpInSelendis),
      3211 => Some(Self::SuperWarpGateTrainWarpInUrun),
      3210 => Some(Self::SuperWarpGateTrainWarpInZeratul),
      3216 => Some(Self::SuperWarpGateTrainWarpPrism),
      3200 => Some(Self::SuperWarpGateTrainZealot),
      556 => Some(Self::SupplyDepotLower),
      557 => Some(Self::SupplyDepotLower),
      558 => Some(Self::SupplyDepotRaise),
      559 => Some(Self::SupplyDepotRaise),
      255 => Some(Self::SupplyDrop),
      256 => Some(Self::SupplyDrop),
      1991 => Some(Self::SwarmHostSpawnLocusts),
      1990 => Some(Self::SwarmHostSpawnLocustsLocustMp),
      _ => None,
    }
  }
}
