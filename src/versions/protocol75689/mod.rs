pub use crate::versions::protocol87702::events::*;
pub mod byte_aligned {
    //! Generated code from source: ../s2protocol/json/protocol75689.json
    //! All byte aligned types are compatible with protocol87702
    pub use crate::versions::protocol87702::byte_aligned::EObserve;
    pub use crate::versions::protocol87702::byte_aligned::GameCPlayerDetailsArray;
    pub use crate::versions::protocol87702::byte_aligned::GameEGameSpeed;
    pub use crate::versions::protocol87702::byte_aligned::GameEResultDetails;
    pub use crate::versions::protocol87702::byte_aligned::GameSColor;
    pub use crate::versions::protocol87702::byte_aligned::GameSDetails;
    pub use crate::versions::protocol87702::byte_aligned::GameSPlayerDetails;
    pub use crate::versions::protocol87702::byte_aligned::GameSThumbnail;
    pub use crate::versions::protocol87702::byte_aligned::GameSToonNameDetails;
    pub use crate::versions::protocol87702::byte_aligned::ReplaySHeader;
    pub use crate::versions::protocol87702::byte_aligned::ReplayTrackerEEventId;
    pub use crate::versions::protocol87702::byte_aligned::ReplayTrackerSPlayerSetupEvent;
    pub use crate::versions::protocol87702::byte_aligned::ReplayTrackerSPlayerStats;
    pub use crate::versions::protocol87702::byte_aligned::ReplayTrackerSPlayerStatsEvent;
    pub use crate::versions::protocol87702::byte_aligned::ReplayTrackerSUnitBornEvent;
    pub use crate::versions::protocol87702::byte_aligned::ReplayTrackerSUnitDiedEvent;
    pub use crate::versions::protocol87702::byte_aligned::ReplayTrackerSUnitDoneEvent;
    pub use crate::versions::protocol87702::byte_aligned::ReplayTrackerSUnitInitEvent;
    pub use crate::versions::protocol87702::byte_aligned::ReplayTrackerSUnitOwnerChangeEvent;
    pub use crate::versions::protocol87702::byte_aligned::ReplayTrackerSUnitPositionsEvent;
    pub use crate::versions::protocol87702::byte_aligned::ReplayTrackerSUnitTypeChangeEvent;
    pub use crate::versions::protocol87702::byte_aligned::ReplayTrackerSUpgradeEvent;
    pub use crate::versions::protocol87702::byte_aligned::SVarUint32;
    pub use crate::versions::protocol87702::byte_aligned::SVersion;
    pub use crate::versions::protocol87702::byte_aligned::Smd5;
}
pub mod bit_packed {
    //! Generated code from source: ../s2protocol/json/protocol75689.json
    use crate::game_events::ReplayGameEvent;
    use crate::versions::GameEvent;
    pub use crate::versions::protocol87702::bit_packed::CAllowedObserveTypes;
    pub use crate::versions::protocol87702::bit_packed::CAllowedRaces;
    pub use crate::versions::protocol87702::bit_packed::CArtifactHandle;
    pub use crate::versions::protocol87702::bit_packed::CCacheHandle;
    pub use crate::versions::protocol87702::bit_packed::CClanTag;
    pub use crate::versions::protocol87702::bit_packed::CCommanderHandle;
    pub use crate::versions::protocol87702::bit_packed::CFilePath;
    pub use crate::versions::protocol87702::bit_packed::CHeroHandle;
    pub use crate::versions::protocol87702::bit_packed::CMountHandle;
    pub use crate::versions::protocol87702::bit_packed::CSkinHandle;
    pub use crate::versions::protocol87702::bit_packed::CToonHandle;
    pub use crate::versions::protocol87702::bit_packed::CUserInitialDataArray;
    pub use crate::versions::protocol87702::bit_packed::CUserName;
    pub use crate::versions::protocol87702::bit_packed::ELeaveReason;
    pub use crate::versions::protocol87702::bit_packed::EObserve;
    pub use crate::versions::protocol87702::bit_packed::EReconnectStatus;
    pub use crate::versions::protocol87702::bit_packed::GameCAllowedAiBuild;
    pub use crate::versions::protocol87702::bit_packed::GameCAllowedColors;
    pub use crate::versions::protocol87702::bit_packed::GameCAllowedControls;
    pub use crate::versions::protocol87702::bit_packed::GameCAllowedDifficulty;
    pub use crate::versions::protocol87702::bit_packed::GameCArtifactArray;
    pub use crate::versions::protocol87702::bit_packed::GameCAuthorName;
    pub use crate::versions::protocol87702::bit_packed::GameCCacheHandle;
    pub use crate::versions::protocol87702::bit_packed::GameCCacheHandles;
    pub use crate::versions::protocol87702::bit_packed::GameCChatString;
    pub use crate::versions::protocol87702::bit_packed::GameCCheatString;
    pub use crate::versions::protocol87702::bit_packed::GameCCommanderMasteryTalentArray;
    pub use crate::versions::protocol87702::bit_packed::GameCGameCacheName;
    pub use crate::versions::protocol87702::bit_packed::GameCLobbySlotArray;
    pub use crate::versions::protocol87702::bit_packed::GameCModPaths;
    pub use crate::versions::protocol87702::bit_packed::GameCPlayerDetailsArray;
    pub use crate::versions::protocol87702::bit_packed::GameCRewardArray;
    pub use crate::versions::protocol87702::bit_packed::GameCRewardOverride;
    pub use crate::versions::protocol87702::bit_packed::GameCRewardOverrideArray;
    pub use crate::versions::protocol87702::bit_packed::GameCTriggerChatMessageString;
    pub use crate::versions::protocol87702::bit_packed::GameEClientDebugFlags;
    pub use crate::versions::protocol87702::bit_packed::GameECommandManagerState;
    pub use crate::versions::protocol87702::bit_packed::GameEControl;
    pub use crate::versions::protocol87702::bit_packed::GameEControlGroupUpdate;
    pub use crate::versions::protocol87702::bit_packed::GameEConversationSkip;
    pub use crate::versions::protocol87702::bit_packed::GameEDebug;
    pub use crate::versions::protocol87702::bit_packed::GameEGameLaunch;
    pub use crate::versions::protocol87702::bit_packed::GameEGameSpeed;
    pub use crate::versions::protocol87702::bit_packed::GameEGameType;
    pub use crate::versions::protocol87702::bit_packed::GameEHijackMethod;
    pub use crate::versions::protocol87702::bit_packed::GameEMessageId;
    pub use crate::versions::protocol87702::bit_packed::GameEMessageRecipient;
    pub use crate::versions::protocol87702::bit_packed::GameEOptionFog;
    pub use crate::versions::protocol87702::bit_packed::GameEOptionObservers;
    pub use crate::versions::protocol87702::bit_packed::GameEOptionUserDifficulty;
    pub use crate::versions::protocol87702::bit_packed::GameEPhase;
    pub use crate::versions::protocol87702::bit_packed::GameEResultCode;
    pub use crate::versions::protocol87702::bit_packed::GameEResultDetails;
    pub use crate::versions::protocol87702::bit_packed::GameESynchronous;
    pub use crate::versions::protocol87702::bit_packed::GameESynthesized;
    pub use crate::versions::protocol87702::bit_packed::GameSAchievementAwardedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSAddAbsoluteGameSpeedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSAddGameSpeedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSAllianceEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSBankFileEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSBankKeyEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSBankSectionEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSBankSignatureEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSBankValueEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSBroadcastCheatEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSCameraSaveEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSCameraUpdateEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSCatalogModifyEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSChatMessage;
    pub use crate::versions::protocol87702::bit_packed::GameSCheatEventData;
    pub use crate::versions::protocol87702::bit_packed::GameSCmdAbil;
    pub use crate::versions::protocol87702::bit_packed::GameSCmdData;
    pub use crate::versions::protocol87702::bit_packed::GameSCmdDataTargetUnit;
    pub use crate::versions::protocol87702::bit_packed::GameSCmdUpdateTargetPointEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSCmdUpdateTargetUnitEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSColor;
    pub use crate::versions::protocol87702::bit_packed::GameSCommandManagerResetEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSCommandManagerStateEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSControlGroupUpdateEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSConvertToReplaySessionEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSDecrementGameTimeRemainingEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSDetails;
    pub use crate::versions::protocol87702::bit_packed::GameSDropOurselvesEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSDropUserEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSGameCheatEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSGameDescription;
    pub use crate::versions::protocol87702::bit_packed::GameSGameOptions;
    pub use crate::versions::protocol87702::bit_packed::GameSGameSyncInfo;
    pub use crate::versions::protocol87702::bit_packed::GameSGameUserJoinEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSGameUserLeaveEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSHeroTalentTreeSelectedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSHeroTalentTreeSelectionPanelToggledEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSHijackReplayGameEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSHijackReplayGameUserInfo;
    pub use crate::versions::protocol87702::bit_packed::GameSHijackReplaySessionEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSHijackReplaySessionUserInfo;
    pub use crate::versions::protocol87702::bit_packed::GameSLoadGameDoneEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSLoadingProgressMessage;
    pub use crate::versions::protocol87702::bit_packed::GameSLobbyState;
    pub use crate::versions::protocol87702::bit_packed::GameSLobbySyncState;
    pub use crate::versions::protocol87702::bit_packed::GameSMapCoord;
    pub use crate::versions::protocol87702::bit_packed::GameSMapCoord3D;
    pub use crate::versions::protocol87702::bit_packed::GameSMuteChatEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSPauseGameEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSPeerSetSyncLoadingTimeEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSPeerSetSyncPlayingTimeEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSPickMapTagEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSPingMessage;
    pub use crate::versions::protocol87702::bit_packed::GameSPlayerDetails;
    pub use crate::versions::protocol87702::bit_packed::GameSPoint;
    pub use crate::versions::protocol87702::bit_packed::GameSPoint3;
    pub use crate::versions::protocol87702::bit_packed::GameSPointMini;
    pub use crate::versions::protocol87702::bit_packed::GameSReconnectNotifyMessage;
    pub use crate::versions::protocol87702::bit_packed::GameSReplayJumpEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSResourceRequestCancelEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSResourceRequestEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSResourceRequestFulfillEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSResourceTradeEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSaveGameDoneEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSaveGameEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSelectionDelta;
    pub use crate::versions::protocol87702::bit_packed::GameSSelectionDeltaEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSelectionDeltaSubgroup;
    pub use crate::versions::protocol87702::bit_packed::GameSSelectionMask;
    pub use crate::versions::protocol87702::bit_packed::GameSSelectionSyncCheckEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSelectionSyncData;
    pub use crate::versions::protocol87702::bit_packed::GameSServerPingMessage;
    pub use crate::versions::protocol87702::bit_packed::GameSSessionCheatEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSessionSyncInfo;
    pub use crate::versions::protocol87702::bit_packed::GameSSetAbsoluteGameSpeedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSetGameDurationEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSetGameSpeedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSetLobbySlotEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSetSyncLoadingTimeEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSetSyncPlayingTimeEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSingleStepGameEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSlotDescription;
    pub use crate::versions::protocol87702::bit_packed::GameSSlotDescriptions;
    pub use crate::versions::protocol87702::bit_packed::GameSStartGameEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSSyncSoundLength;
    pub use crate::versions::protocol87702::bit_packed::GameSThumbnail;
    pub use crate::versions::protocol87702::bit_packed::GameSToonNameDetails;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerAbortMissionEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerAnimLengthQueryByNameEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerAnimLengthQueryByPropsEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerAnimOffsetEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerBattleReportPanelExitEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerBattleReportPanelPlayMissionEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerBattleReportPanelPlaySceneEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerBattleReportPanelSelectionChangedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerButtonPressedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerChatMessageEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerCommandErrorEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerConversationSkippedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerCustomDialogDismissedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerCutsceneBookmarkFiredEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerCutsceneConversationLineEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerCutsceneConversationLineMissingEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerCutsceneEndSceneFiredEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerDialogControlEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerGameCreditsFinishedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerGameMenuItemSelectedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerHotkeyPressedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerKeyPressedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerMercenaryPanelExitEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerMercenaryPanelPurchaseEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerMercenaryPanelSelectionChangedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerMouseClickedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerMouseMovedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerMouseWheelEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerMovieFinishedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerMovieFunctionEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerMovieStartedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerPingEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerPlanetMissionLaunchedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerPlanetMissionSelectedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerPlanetPanelBirthCompleteEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerPlanetPanelCanceledEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerPlanetPanelDeathCompleteEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerPlanetPanelReplayEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerPortraitLoadedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerProfilerLoggingFinishedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerPurchaseExitEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerPurchaseMadeEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerPurchasePanelSelectedPurchaseCategoryChangedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerPurchasePanelSelectedPurchaseItemChangedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerReplySelectedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerResearchPanelExitEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerResearchPanelPurchaseEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerResearchPanelSelectionChangedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerSkippedEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerSoundLengthQueryEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerSoundLengthSyncEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerSoundOffsetEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerSoundtrackDoneEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerTargetModeUpdateEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerTransmissionCompleteEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerTransmissionOffsetEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerVictoryPanelExitEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTriggerVictoryPanelPlayMissionAgainEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSTurnEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSUnitClickEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSUnitHighlightEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSUnpauseGameEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSUserFinishedLoadingEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSUserFinishedLoadingSyncEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSUserOptionsEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSaiCommunicateEvent;
    pub use crate::versions::protocol87702::bit_packed::GameSelectionIndexArrayType;
    pub use crate::versions::protocol87702::bit_packed::GameSelectionMaskType;
    pub use crate::versions::protocol87702::bit_packed::GameSuiCoord;
    pub use crate::versions::protocol87702::bit_packed::GameTAbilLink;
    pub use crate::versions::protocol87702::bit_packed::GameTAchievementLink;
    pub use crate::versions::protocol87702::bit_packed::GameTAchievementTermLink;
    pub use crate::versions::protocol87702::bit_packed::GameTButtonLink;
    pub use crate::versions::protocol87702::bit_packed::GameTColorCount;
    pub use crate::versions::protocol87702::bit_packed::GameTColorId;
    pub use crate::versions::protocol87702::bit_packed::GameTColorPreference;
    pub use crate::versions::protocol87702::bit_packed::GameTControlCount;
    pub use crate::versions::protocol87702::bit_packed::GameTControlGroupCount;
    pub use crate::versions::protocol87702::bit_packed::GameTControlGroupId;
    pub use crate::versions::protocol87702::bit_packed::GameTControlGroupIndex;
    pub use crate::versions::protocol87702::bit_packed::GameTControlId;
    pub use crate::versions::protocol87702::bit_packed::GameTDifficulty;
    pub use crate::versions::protocol87702::bit_packed::GameTFixedBits;
    pub use crate::versions::protocol87702::bit_packed::GameTFixedInt;
    pub use crate::versions::protocol87702::bit_packed::GameTFixedMiniBitsSigned;
    pub use crate::versions::protocol87702::bit_packed::GameTFixedMiniBitsUnsigned;
    pub use crate::versions::protocol87702::bit_packed::GameTFixedUInt;
    pub use crate::versions::protocol87702::bit_packed::GameTFlexLicenseAttributeName;
    pub use crate::versions::protocol87702::bit_packed::GameTFlexLicenseAttributeValue;
    pub use crate::versions::protocol87702::bit_packed::GameTFlexLicenseName;
    pub use crate::versions::protocol87702::bit_packed::GameTHeroLink;
    pub use crate::versions::protocol87702::bit_packed::GameTLicense;
    pub use crate::versions::protocol87702::bit_packed::GameTLobbySlotCount;
    pub use crate::versions::protocol87702::bit_packed::GameTLobbySlotId;
    pub use crate::versions::protocol87702::bit_packed::GameTMapCoordFixedBits;
    pub use crate::versions::protocol87702::bit_packed::GameTPlayerCount;
    pub use crate::versions::protocol87702::bit_packed::GameTPlayerId;
    pub use crate::versions::protocol87702::bit_packed::GameTPlayerLogoIndex;
    pub use crate::versions::protocol87702::bit_packed::GameTQueryId;
    pub use crate::versions::protocol87702::bit_packed::GameTReward;
    pub use crate::versions::protocol87702::bit_packed::GameTSelectionCount;
    pub use crate::versions::protocol87702::bit_packed::GameTSelectionIndex;
    pub use crate::versions::protocol87702::bit_packed::GameTSubgroupCount;
    pub use crate::versions::protocol87702::bit_packed::GameTSubgroupIndex;
    pub use crate::versions::protocol87702::bit_packed::GameTSubgroupPriority;
    pub use crate::versions::protocol87702::bit_packed::GameTSyncChecksum;
    pub use crate::versions::protocol87702::bit_packed::GameTSyncValue;
    pub use crate::versions::protocol87702::bit_packed::GameTTeamCount;
    pub use crate::versions::protocol87702::bit_packed::GameTTeamId;
    pub use crate::versions::protocol87702::bit_packed::GameTTriggerSoundTag;
    pub use crate::versions::protocol87702::bit_packed::GameTTriggerThreadTag;
    pub use crate::versions::protocol87702::bit_packed::GameTUnitLink;
    pub use crate::versions::protocol87702::bit_packed::GameTUnitTag;
    pub use crate::versions::protocol87702::bit_packed::GameTaiBuild;
    pub use crate::versions::protocol87702::bit_packed::GameTuiCoordX;
    pub use crate::versions::protocol87702::bit_packed::GameTuiCoordY;
    pub use crate::versions::protocol87702::bit_packed::Int8;
    pub use crate::versions::protocol87702::bit_packed::Int16;
    pub use crate::versions::protocol87702::bit_packed::Int32;
    pub use crate::versions::protocol87702::bit_packed::Int64;
    pub use crate::versions::protocol87702::bit_packed::MEventData;
    pub use crate::versions::protocol87702::bit_packed::ReplaySGameUserId;
    pub use crate::versions::protocol87702::bit_packed::ReplaySInitData;
    pub use crate::versions::protocol87702::bit_packed::SUserInitialData;
    pub use crate::versions::protocol87702::bit_packed::SVarUint32;
    pub use crate::versions::protocol87702::bit_packed::SVersion;
    pub use crate::versions::protocol87702::bit_packed::Smd5;
    pub use crate::versions::protocol87702::bit_packed::TRaceCount;
    pub use crate::versions::protocol87702::bit_packed::TRaceId;
    pub use crate::versions::protocol87702::bit_packed::TRacePreference;
    pub use crate::versions::protocol87702::bit_packed::TTeamPreference;
    pub use crate::versions::protocol87702::bit_packed::TUserCount;
    pub use crate::versions::protocol87702::bit_packed::TUserId;
    pub use crate::versions::protocol87702::bit_packed::Uint6;
    pub use crate::versions::protocol87702::bit_packed::Uint8;
    pub use crate::versions::protocol87702::bit_packed::Uint14;
    pub use crate::versions::protocol87702::bit_packed::Uint16;
    pub use crate::versions::protocol87702::bit_packed::Uint22;
    pub use crate::versions::protocol87702::bit_packed::Uint32;
    pub use crate::versions::protocol87702::bit_packed::Uint64;
    use crate::*;
    use nom::*;

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEEventId {
        ESetLobbySlot(GameSSetLobbySlotEvent),
        EDropUser(GameSDropUserEvent),
        EStartGame(GameSStartGameEvent),
        EDropOurselves(GameSDropOurselvesEvent),
        EUserFinishedLoading(GameSUserFinishedLoadingEvent),
        EUserFinishedLoadingSync(GameSUserFinishedLoadingSyncEvent),
        ESetGameDuration(GameSSetGameDurationEvent),
        EUserOptions(GameSUserOptionsEvent),
        EPickMapTag(GameSPickMapTagEvent),
        ETurn(GameSTurnEvent),
        EBankFile(GameSBankFileEvent),
        EBankSection(GameSBankSectionEvent),
        EBankKey(GameSBankKeyEvent),
        EBankValue(GameSBankValueEvent),
        EBankSignature(GameSBankSignatureEvent),
        ECameraSave(GameSCameraSaveEvent),
        EPauseGame(GameSPauseGameEvent),
        EUnpauseGame(GameSUnpauseGameEvent),
        ESingleStepGame(GameSSingleStepGameEvent),
        ESetGameSpeed(GameSSetGameSpeedEvent),
        EAddGameSpeed(GameSAddGameSpeedEvent),
        EReplayJump(GameSReplayJumpEvent),
        ESaveGame(GameSSaveGameEvent),
        ESaveGameDone(GameSSaveGameDoneEvent),
        ELoadGameDone(GameSLoadGameDoneEvent),
        ESessionCheat(GameSSessionCheatEvent),
        ECommandManagerReset(GameSCommandManagerResetEvent),
        EGameCheat(GameSGameCheatEvent),
        ECmd(GameSCmdEvent),
        ESelectionDelta(GameSSelectionDeltaEvent),
        EControlGroupUpdate(GameSControlGroupUpdateEvent),
        ESelectionSyncCheck(GameSSelectionSyncCheckEvent),
        EResourceTrade(GameSResourceTradeEvent),
        ETriggerChatMessage(GameSTriggerChatMessageEvent),
        EAiCommunicate(GameSaiCommunicateEvent),
        ESetAbsoluteGameSpeed(GameSSetAbsoluteGameSpeedEvent),
        EAddAbsoluteGameSpeed(GameSAddAbsoluteGameSpeedEvent),
        ETriggerPing(GameSTriggerPingEvent),
        EBroadcastCheat(GameSBroadcastCheatEvent),
        EAlliance(GameSAllianceEvent),
        EUnitClick(GameSUnitClickEvent),
        EUnitHighlight(GameSUnitHighlightEvent),
        ETriggerReplySelected(GameSTriggerReplySelectedEvent),
        EHijackReplaySession(GameSHijackReplaySessionEvent),
        EHijackReplayGame(GameSHijackReplayGameEvent),
        ETriggerSkipped(GameSTriggerSkippedEvent),
        ETriggerSoundLengthQuery(GameSTriggerSoundLengthQueryEvent),
        ETriggerSoundOffset(GameSTriggerSoundOffsetEvent),
        ETriggerTransmissionOffset(GameSTriggerTransmissionOffsetEvent),
        ETriggerTransmissionComplete(GameSTriggerTransmissionCompleteEvent),
        ECameraUpdate(GameSCameraUpdateEvent),
        ETriggerAbortMission(GameSTriggerAbortMissionEvent),
        ETriggerPurchaseMade(GameSTriggerPurchaseMadeEvent),
        ETriggerPurchaseExit(GameSTriggerPurchaseExitEvent),
        ETriggerPlanetMissionLaunched(GameSTriggerPlanetMissionLaunchedEvent),
        ETriggerPlanetPanelCanceled(GameSTriggerPlanetPanelCanceledEvent),
        ETriggerDialogControl(GameSTriggerDialogControlEvent),
        ETriggerSoundLengthSync(GameSTriggerSoundLengthSyncEvent),
        ETriggerConversationSkipped(GameSTriggerConversationSkippedEvent),
        ETriggerMouseClicked(GameSTriggerMouseClickedEvent),
        ETriggerMouseMoved(GameSTriggerMouseMovedEvent),
        EAchievementAwarded(GameSAchievementAwardedEvent),
        ETriggerHotkeyPressed(GameSTriggerHotkeyPressedEvent),
        ETriggerTargetModeUpdate(GameSTriggerTargetModeUpdateEvent),
        ETriggerPlanetPanelPanelReplay(GameSTriggerPlanetPanelReplayEvent),
        ETriggerSoundtrackDone(GameSTriggerSoundtrackDoneEvent),
        ETriggerPlanetMissionSelected(GameSTriggerPlanetMissionSelectedEvent),
        ETriggerKeyPressed(GameSTriggerKeyPressedEvent),
        ETriggerMovieFunction(GameSTriggerMovieFunctionEvent),
        ETriggerPlanetPanelPanelBirthComplete(GameSTriggerPlanetPanelBirthCompleteEvent),
        ETriggerPlanetPanelPanelDeathComplete(GameSTriggerPlanetPanelDeathCompleteEvent),
        EResourceRequest(GameSResourceRequestEvent),
        EResourceRequestFulfill(GameSResourceRequestFulfillEvent),
        EResourceRequestCancel(GameSResourceRequestCancelEvent),
        ETriggerResearchPanelExit(GameSTriggerResearchPanelExitEvent),
        ETriggerResearchPanelPurchase(GameSTriggerResearchPanelPurchaseEvent),
        ETriggerResearchPanelSelectionChanged(GameSTriggerResearchPanelSelectionChangedEvent),
        ETriggerCommandError(GameSTriggerCommandErrorEvent),
        ETriggerMercenaryPanelExit(GameSTriggerMercenaryPanelExitEvent),
        ETriggerMercenaryPanelPurchase(GameSTriggerMercenaryPanelPurchaseEvent),
        ETriggerMercenaryPanelSelectionChanged(GameSTriggerMercenaryPanelSelectionChangedEvent),
        ETriggerVictoryPanelExit(GameSTriggerVictoryPanelExitEvent),
        ETriggerBattleReportPanelExit(GameSTriggerBattleReportPanelExitEvent),
        ETriggerBattleReportPanelPlayMission(GameSTriggerBattleReportPanelPlayMissionEvent),
        ETriggerBattleReportPanelPlayScene(GameSTriggerBattleReportPanelPlaySceneEvent),
        ETriggerBattleReportSelectionChanged(GameSTriggerBattleReportPanelSelectionChangedEvent),
        ETriggerVictoryPanelPlayMissionAgain(GameSTriggerVictoryPanelPlayMissionAgainEvent),
        ETriggerMovieStarted(GameSTriggerMovieStartedEvent),
        ETriggerMovieFinished(GameSTriggerMovieFinishedEvent),
        EDecrementGameTimeRemaining(GameSDecrementGameTimeRemainingEvent),
        ETriggerPortraitLoaded(GameSTriggerPortraitLoadedEvent),
        ETriggerQueryDialogDismissed(GameSTriggerCustomDialogDismissedEvent),
        ETriggerGameMenuItemSelected(GameSTriggerGameMenuItemSelectedEvent),
        ETriggerMouseWheel(GameSTriggerMouseWheelEvent),
        ETriggerPurchasePanelSelectedPurchaseItemChanged(
            GameSTriggerPurchasePanelSelectedPurchaseItemChangedEvent,
        ),
        ETriggerPurchasePanelSelectedPurchaseCategoryChanged(
            GameSTriggerPurchasePanelSelectedPurchaseCategoryChangedEvent,
        ),
        ETriggerButtonPressed(GameSTriggerButtonPressedEvent),
        ETriggerGameCreditsFinished(GameSTriggerGameCreditsFinishedEvent),
        ETriggerCutsceneBookmarkFired(GameSTriggerCutsceneBookmarkFiredEvent),
        ETriggerCutsceneEndSceneFired(GameSTriggerCutsceneEndSceneFiredEvent),
        ETriggerCutsceneConversationLine(GameSTriggerCutsceneConversationLineEvent),
        ETriggerCutsceneConversationLineMissing(GameSTriggerCutsceneConversationLineMissingEvent),
        EGameUserLeave(GameSGameUserLeaveEvent),
        EGameUserJoin(GameSGameUserJoinEvent),
        ECommandManagerState(GameSCommandManagerStateEvent),
        ECmdUpdateTargetPoint(GameSCmdUpdateTargetPointEvent),
        ECmdUpdateTargetUnit(GameSCmdUpdateTargetUnitEvent),
        ETriggerAnimLengthQueryByName(GameSTriggerAnimLengthQueryByNameEvent),
        ETriggerAnimLengthQueryByProps(GameSTriggerAnimLengthQueryByPropsEvent),
        ETriggerAnimOffset(GameSTriggerAnimOffsetEvent),
        ECatalogModify(GameSCatalogModifyEvent),
        EHeroTalentTreeSelected(GameSHeroTalentTreeSelectedEvent),
        ETriggerProfilerLoggingFinished(GameSTriggerProfilerLoggingFinishedEvent),
        EHeroTalentTreeSelectionPanelToggled(GameSHeroTalentTreeSelectionPanelToggledEvent),
        EMuteUserChanged(GameSMuteChatEvent),
        EConvertToReplaySession(GameSConvertToReplaySessionEvent),
        ESetSyncLoadingTime(GameSSetSyncLoadingTimeEvent),
        ESetSyncPlayingTime(GameSSetSyncPlayingTimeEvent),
        EPeerSetSyncLoadingTime(GameSPeerSetSyncLoadingTimeEvent),
        EPeerSetSyncPlayingTime(GameSPeerSetSyncPlayingTimeEvent),
    }

    impl GameEEventId {
        #[tracing::instrument(name="75689::GameEEventId::Parse", level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 120
            let num_bits: usize = 7;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ESetLobbySlot for value '0'");

                    let (tail, res) = GameSSetLobbySlotEvent::parse(tail)?;
                    Ok((tail, Self::ESetLobbySlot(res)))
                }
                1 => {
                    tracing::debug!("Variant EDropUser for value '1'");

                    let (tail, res) = GameSDropUserEvent::parse(tail)?;
                    Ok((tail, Self::EDropUser(res)))
                }
                2 => {
                    tracing::debug!("Variant EStartGame for value '2'");

                    let (tail, res) = GameSStartGameEvent::parse(tail)?;
                    Ok((tail, Self::EStartGame(res)))
                }
                3 => {
                    tracing::debug!("Variant EDropOurselves for value '3'");

                    let (tail, res) = GameSDropOurselvesEvent::parse(tail)?;
                    Ok((tail, Self::EDropOurselves(res)))
                }
                4 => {
                    tracing::debug!("Variant EUserFinishedLoading for value '4'");

                    let (tail, res) = GameSUserFinishedLoadingEvent::parse(tail)?;
                    Ok((tail, Self::EUserFinishedLoading(res)))
                }
                5 => {
                    tracing::debug!("Variant EUserFinishedLoadingSync for value '5'");

                    let (tail, res) = GameSUserFinishedLoadingSyncEvent::parse(tail)?;
                    Ok((tail, Self::EUserFinishedLoadingSync(res)))
                }
                6 => {
                    tracing::debug!("Variant ESetGameDuration for value '6'");

                    let (tail, res) = GameSSetGameDurationEvent::parse(tail)?;
                    Ok((tail, Self::ESetGameDuration(res)))
                }
                7 => {
                    tracing::debug!("Variant EUserOptions for value '7'");

                    let (tail, res) = GameSUserOptionsEvent::parse(tail)?;
                    Ok((tail, Self::EUserOptions(res)))
                }
                114 => {
                    tracing::debug!("Variant EPickMapTag for value '114'");

                    let (tail, res) = GameSPickMapTagEvent::parse(tail)?;
                    Ok((tail, Self::EPickMapTag(res)))
                }
                8 => {
                    tracing::debug!("Variant ETurn for value '8'");

                    let (tail, res) = GameSTurnEvent::parse(tail)?;
                    Ok((tail, Self::ETurn(res)))
                }
                9 => {
                    tracing::debug!("Variant EBankFile for value '9'");

                    let (tail, res) = GameSBankFileEvent::parse(tail)?;
                    Ok((tail, Self::EBankFile(res)))
                }
                10 => {
                    tracing::debug!("Variant EBankSection for value '10'");

                    let (tail, res) = GameSBankSectionEvent::parse(tail)?;
                    Ok((tail, Self::EBankSection(res)))
                }
                11 => {
                    tracing::debug!("Variant EBankKey for value '11'");

                    let (tail, res) = GameSBankKeyEvent::parse(tail)?;
                    Ok((tail, Self::EBankKey(res)))
                }
                12 => {
                    tracing::debug!("Variant EBankValue for value '12'");

                    let (tail, res) = GameSBankValueEvent::parse(tail)?;
                    Ok((tail, Self::EBankValue(res)))
                }
                13 => {
                    tracing::debug!("Variant EBankSignature for value '13'");

                    let (tail, res) = GameSBankSignatureEvent::parse(tail)?;
                    Ok((tail, Self::EBankSignature(res)))
                }
                14 => {
                    tracing::debug!("Variant ECameraSave for value '14'");

                    let (tail, res) = GameSCameraSaveEvent::parse(tail)?;
                    Ok((tail, Self::ECameraSave(res)))
                }
                15 => {
                    tracing::debug!("Variant EPauseGame for value '15'");

                    let (tail, res) = GameSPauseGameEvent::parse(tail)?;
                    Ok((tail, Self::EPauseGame(res)))
                }
                16 => {
                    tracing::debug!("Variant EUnpauseGame for value '16'");

                    let (tail, res) = GameSUnpauseGameEvent::parse(tail)?;
                    Ok((tail, Self::EUnpauseGame(res)))
                }
                17 => {
                    tracing::debug!("Variant ESingleStepGame for value '17'");

                    let (tail, res) = GameSSingleStepGameEvent::parse(tail)?;
                    Ok((tail, Self::ESingleStepGame(res)))
                }
                18 => {
                    tracing::debug!("Variant ESetGameSpeed for value '18'");

                    let (tail, res) = GameSSetGameSpeedEvent::parse(tail)?;
                    Ok((tail, Self::ESetGameSpeed(res)))
                }
                19 => {
                    tracing::debug!("Variant EAddGameSpeed for value '19'");

                    let (tail, res) = GameSAddGameSpeedEvent::parse(tail)?;
                    Ok((tail, Self::EAddGameSpeed(res)))
                }
                20 => {
                    tracing::debug!("Variant EReplayJump for value '20'");

                    let (tail, res) = GameSReplayJumpEvent::parse(tail)?;
                    Ok((tail, Self::EReplayJump(res)))
                }
                21 => {
                    tracing::debug!("Variant ESaveGame for value '21'");

                    let (tail, res) = GameSSaveGameEvent::parse(tail)?;
                    Ok((tail, Self::ESaveGame(res)))
                }
                22 => {
                    tracing::debug!("Variant ESaveGameDone for value '22'");

                    let (tail, res) = GameSSaveGameDoneEvent::parse(tail)?;
                    Ok((tail, Self::ESaveGameDone(res)))
                }
                23 => {
                    tracing::debug!("Variant ELoadGameDone for value '23'");

                    let (tail, res) = GameSLoadGameDoneEvent::parse(tail)?;
                    Ok((tail, Self::ELoadGameDone(res)))
                }
                24 => {
                    tracing::debug!("Variant ESessionCheat for value '24'");

                    let (tail, res) = GameSSessionCheatEvent::parse(tail)?;
                    Ok((tail, Self::ESessionCheat(res)))
                }
                25 => {
                    tracing::debug!("Variant ECommandManagerReset for value '25'");

                    let (tail, res) = GameSCommandManagerResetEvent::parse(tail)?;
                    Ok((tail, Self::ECommandManagerReset(res)))
                }
                26 => {
                    tracing::debug!("Variant EGameCheat for value '26'");

                    let (tail, res) = GameSGameCheatEvent::parse(tail)?;
                    Ok((tail, Self::EGameCheat(res)))
                }
                27 => {
                    tracing::debug!("Variant ECmd for value '27'");

                    let (tail, res) = GameSCmdEvent::parse(tail)?;
                    Ok((tail, Self::ECmd(res)))
                }
                28 => {
                    tracing::debug!("Variant ESelectionDelta for value '28'");

                    let (tail, res) = GameSSelectionDeltaEvent::parse(tail)?;
                    Ok((tail, Self::ESelectionDelta(res)))
                }
                29 => {
                    tracing::debug!("Variant EControlGroupUpdate for value '29'");

                    let (tail, res) = GameSControlGroupUpdateEvent::parse(tail)?;
                    Ok((tail, Self::EControlGroupUpdate(res)))
                }
                30 => {
                    tracing::debug!("Variant ESelectionSyncCheck for value '30'");

                    let (tail, res) = GameSSelectionSyncCheckEvent::parse(tail)?;
                    Ok((tail, Self::ESelectionSyncCheck(res)))
                }
                31 => {
                    tracing::debug!("Variant EResourceTrade for value '31'");

                    let (tail, res) = GameSResourceTradeEvent::parse(tail)?;
                    Ok((tail, Self::EResourceTrade(res)))
                }
                32 => {
                    tracing::debug!("Variant ETriggerChatMessage for value '32'");

                    let (tail, res) = GameSTriggerChatMessageEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerChatMessage(res)))
                }
                33 => {
                    tracing::debug!("Variant EAiCommunicate for value '33'");

                    let (tail, res) = GameSaiCommunicateEvent::parse(tail)?;
                    Ok((tail, Self::EAiCommunicate(res)))
                }
                34 => {
                    tracing::debug!("Variant ESetAbsoluteGameSpeed for value '34'");

                    let (tail, res) = GameSSetAbsoluteGameSpeedEvent::parse(tail)?;
                    Ok((tail, Self::ESetAbsoluteGameSpeed(res)))
                }
                35 => {
                    tracing::debug!("Variant EAddAbsoluteGameSpeed for value '35'");

                    let (tail, res) = GameSAddAbsoluteGameSpeedEvent::parse(tail)?;
                    Ok((tail, Self::EAddAbsoluteGameSpeed(res)))
                }
                36 => {
                    tracing::debug!("Variant ETriggerPing for value '36'");

                    let (tail, res) = GameSTriggerPingEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPing(res)))
                }
                37 => {
                    tracing::debug!("Variant EBroadcastCheat for value '37'");

                    let (tail, res) = GameSBroadcastCheatEvent::parse(tail)?;
                    Ok((tail, Self::EBroadcastCheat(res)))
                }
                38 => {
                    tracing::debug!("Variant EAlliance for value '38'");

                    let (tail, res) = GameSAllianceEvent::parse(tail)?;
                    Ok((tail, Self::EAlliance(res)))
                }
                39 => {
                    tracing::debug!("Variant EUnitClick for value '39'");

                    let (tail, res) = GameSUnitClickEvent::parse(tail)?;
                    Ok((tail, Self::EUnitClick(res)))
                }
                40 => {
                    tracing::debug!("Variant EUnitHighlight for value '40'");

                    let (tail, res) = GameSUnitHighlightEvent::parse(tail)?;
                    Ok((tail, Self::EUnitHighlight(res)))
                }
                41 => {
                    tracing::debug!("Variant ETriggerReplySelected for value '41'");

                    let (tail, res) = GameSTriggerReplySelectedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerReplySelected(res)))
                }
                42 => {
                    tracing::debug!("Variant EHijackReplaySession for value '42'");

                    let (tail, res) = GameSHijackReplaySessionEvent::parse(tail)?;
                    Ok((tail, Self::EHijackReplaySession(res)))
                }
                43 => {
                    tracing::debug!("Variant EHijackReplayGame for value '43'");

                    let (tail, res) = GameSHijackReplayGameEvent::parse(tail)?;
                    Ok((tail, Self::EHijackReplayGame(res)))
                }
                44 => {
                    tracing::debug!("Variant ETriggerSkipped for value '44'");

                    let (tail, res) = GameSTriggerSkippedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerSkipped(res)))
                }
                45 => {
                    tracing::debug!("Variant ETriggerSoundLengthQuery for value '45'");

                    let (tail, res) = GameSTriggerSoundLengthQueryEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerSoundLengthQuery(res)))
                }
                46 => {
                    tracing::debug!("Variant ETriggerSoundOffset for value '46'");

                    let (tail, res) = GameSTriggerSoundOffsetEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerSoundOffset(res)))
                }
                47 => {
                    tracing::debug!("Variant ETriggerTransmissionOffset for value '47'");

                    let (tail, res) = GameSTriggerTransmissionOffsetEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerTransmissionOffset(res)))
                }
                48 => {
                    tracing::debug!("Variant ETriggerTransmissionComplete for value '48'");

                    let (tail, res) = GameSTriggerTransmissionCompleteEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerTransmissionComplete(res)))
                }
                49 => {
                    tracing::debug!("Variant ECameraUpdate for value '49'");

                    let (tail, res) = GameSCameraUpdateEvent::parse(tail)?;
                    Ok((tail, Self::ECameraUpdate(res)))
                }
                50 => {
                    tracing::debug!("Variant ETriggerAbortMission for value '50'");

                    let (tail, res) = GameSTriggerAbortMissionEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerAbortMission(res)))
                }
                51 => {
                    tracing::debug!("Variant ETriggerPurchaseMade for value '51'");

                    let (tail, res) = GameSTriggerPurchaseMadeEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPurchaseMade(res)))
                }
                52 => {
                    tracing::debug!("Variant ETriggerPurchaseExit for value '52'");

                    let (tail, res) = GameSTriggerPurchaseExitEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPurchaseExit(res)))
                }
                53 => {
                    tracing::debug!("Variant ETriggerPlanetMissionLaunched for value '53'");

                    let (tail, res) = GameSTriggerPlanetMissionLaunchedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPlanetMissionLaunched(res)))
                }
                54 => {
                    tracing::debug!("Variant ETriggerPlanetPanelCanceled for value '54'");

                    let (tail, res) = GameSTriggerPlanetPanelCanceledEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPlanetPanelCanceled(res)))
                }
                55 => {
                    tracing::debug!("Variant ETriggerDialogControl for value '55'");

                    let (tail, res) = GameSTriggerDialogControlEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerDialogControl(res)))
                }
                56 => {
                    tracing::debug!("Variant ETriggerSoundLengthSync for value '56'");

                    let (tail, res) = GameSTriggerSoundLengthSyncEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerSoundLengthSync(res)))
                }
                57 => {
                    tracing::debug!("Variant ETriggerConversationSkipped for value '57'");

                    let (tail, res) = GameSTriggerConversationSkippedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerConversationSkipped(res)))
                }
                58 => {
                    tracing::debug!("Variant ETriggerMouseClicked for value '58'");

                    let (tail, res) = GameSTriggerMouseClickedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMouseClicked(res)))
                }
                59 => {
                    tracing::debug!("Variant ETriggerMouseMoved for value '59'");

                    let (tail, res) = GameSTriggerMouseMovedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMouseMoved(res)))
                }
                60 => {
                    tracing::debug!("Variant EAchievementAwarded for value '60'");

                    let (tail, res) = GameSAchievementAwardedEvent::parse(tail)?;
                    Ok((tail, Self::EAchievementAwarded(res)))
                }
                61 => {
                    tracing::debug!("Variant ETriggerHotkeyPressed for value '61'");

                    let (tail, res) = GameSTriggerHotkeyPressedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerHotkeyPressed(res)))
                }
                62 => {
                    tracing::debug!("Variant ETriggerTargetModeUpdate for value '62'");

                    let (tail, res) = GameSTriggerTargetModeUpdateEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerTargetModeUpdate(res)))
                }
                63 => {
                    tracing::debug!("Variant ETriggerPlanetPanelPanelReplay for value '63'");

                    let (tail, res) = GameSTriggerPlanetPanelReplayEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPlanetPanelPanelReplay(res)))
                }
                64 => {
                    tracing::debug!("Variant ETriggerSoundtrackDone for value '64'");

                    let (tail, res) = GameSTriggerSoundtrackDoneEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerSoundtrackDone(res)))
                }
                65 => {
                    tracing::debug!("Variant ETriggerPlanetMissionSelected for value '65'");

                    let (tail, res) = GameSTriggerPlanetMissionSelectedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPlanetMissionSelected(res)))
                }
                66 => {
                    tracing::debug!("Variant ETriggerKeyPressed for value '66'");

                    let (tail, res) = GameSTriggerKeyPressedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerKeyPressed(res)))
                }
                67 => {
                    tracing::debug!("Variant ETriggerMovieFunction for value '67'");

                    let (tail, res) = GameSTriggerMovieFunctionEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMovieFunction(res)))
                }
                68 => {
                    tracing::debug!("Variant ETriggerPlanetPanelPanelBirthComplete for value '68'");

                    let (tail, res) = GameSTriggerPlanetPanelBirthCompleteEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPlanetPanelPanelBirthComplete(res)))
                }
                69 => {
                    tracing::debug!("Variant ETriggerPlanetPanelPanelDeathComplete for value '69'");

                    let (tail, res) = GameSTriggerPlanetPanelDeathCompleteEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPlanetPanelPanelDeathComplete(res)))
                }
                70 => {
                    tracing::debug!("Variant EResourceRequest for value '70'");

                    let (tail, res) = GameSResourceRequestEvent::parse(tail)?;
                    Ok((tail, Self::EResourceRequest(res)))
                }
                71 => {
                    tracing::debug!("Variant EResourceRequestFulfill for value '71'");

                    let (tail, res) = GameSResourceRequestFulfillEvent::parse(tail)?;
                    Ok((tail, Self::EResourceRequestFulfill(res)))
                }
                72 => {
                    tracing::debug!("Variant EResourceRequestCancel for value '72'");

                    let (tail, res) = GameSResourceRequestCancelEvent::parse(tail)?;
                    Ok((tail, Self::EResourceRequestCancel(res)))
                }
                73 => {
                    tracing::debug!("Variant ETriggerResearchPanelExit for value '73'");

                    let (tail, res) = GameSTriggerResearchPanelExitEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerResearchPanelExit(res)))
                }
                74 => {
                    tracing::debug!("Variant ETriggerResearchPanelPurchase for value '74'");

                    let (tail, res) = GameSTriggerResearchPanelPurchaseEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerResearchPanelPurchase(res)))
                }
                75 => {
                    tracing::debug!("Variant ETriggerResearchPanelSelectionChanged for value '75'");

                    let (tail, res) = GameSTriggerResearchPanelSelectionChangedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerResearchPanelSelectionChanged(res)))
                }
                76 => {
                    tracing::debug!("Variant ETriggerCommandError for value '76'");

                    let (tail, res) = GameSTriggerCommandErrorEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerCommandError(res)))
                }
                77 => {
                    tracing::debug!("Variant ETriggerMercenaryPanelExit for value '77'");

                    let (tail, res) = GameSTriggerMercenaryPanelExitEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMercenaryPanelExit(res)))
                }
                78 => {
                    tracing::debug!("Variant ETriggerMercenaryPanelPurchase for value '78'");

                    let (tail, res) = GameSTriggerMercenaryPanelPurchaseEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMercenaryPanelPurchase(res)))
                }
                79 => {
                    tracing::debug!(
                        "Variant ETriggerMercenaryPanelSelectionChanged for value '79'"
                    );

                    let (tail, res) = GameSTriggerMercenaryPanelSelectionChangedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMercenaryPanelSelectionChanged(res)))
                }
                80 => {
                    tracing::debug!("Variant ETriggerVictoryPanelExit for value '80'");

                    let (tail, res) = GameSTriggerVictoryPanelExitEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerVictoryPanelExit(res)))
                }
                81 => {
                    tracing::debug!("Variant ETriggerBattleReportPanelExit for value '81'");

                    let (tail, res) = GameSTriggerBattleReportPanelExitEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerBattleReportPanelExit(res)))
                }
                82 => {
                    tracing::debug!("Variant ETriggerBattleReportPanelPlayMission for value '82'");

                    let (tail, res) = GameSTriggerBattleReportPanelPlayMissionEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerBattleReportPanelPlayMission(res)))
                }
                83 => {
                    tracing::debug!("Variant ETriggerBattleReportPanelPlayScene for value '83'");

                    let (tail, res) = GameSTriggerBattleReportPanelPlaySceneEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerBattleReportPanelPlayScene(res)))
                }
                84 => {
                    tracing::debug!("Variant ETriggerBattleReportSelectionChanged for value '84'");

                    let (tail, res) =
                        GameSTriggerBattleReportPanelSelectionChangedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerBattleReportSelectionChanged(res)))
                }
                85 => {
                    tracing::debug!("Variant ETriggerVictoryPanelPlayMissionAgain for value '85'");

                    let (tail, res) = GameSTriggerVictoryPanelPlayMissionAgainEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerVictoryPanelPlayMissionAgain(res)))
                }
                86 => {
                    tracing::debug!("Variant ETriggerMovieStarted for value '86'");

                    let (tail, res) = GameSTriggerMovieStartedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMovieStarted(res)))
                }
                87 => {
                    tracing::debug!("Variant ETriggerMovieFinished for value '87'");

                    let (tail, res) = GameSTriggerMovieFinishedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMovieFinished(res)))
                }
                88 => {
                    tracing::debug!("Variant EDecrementGameTimeRemaining for value '88'");

                    let (tail, res) = GameSDecrementGameTimeRemainingEvent::parse(tail)?;
                    Ok((tail, Self::EDecrementGameTimeRemaining(res)))
                }
                89 => {
                    tracing::debug!("Variant ETriggerPortraitLoaded for value '89'");

                    let (tail, res) = GameSTriggerPortraitLoadedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPortraitLoaded(res)))
                }
                90 => {
                    tracing::debug!("Variant ETriggerQueryDialogDismissed for value '90'");

                    let (tail, res) = GameSTriggerCustomDialogDismissedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerQueryDialogDismissed(res)))
                }
                91 => {
                    tracing::debug!("Variant ETriggerGameMenuItemSelected for value '91'");

                    let (tail, res) = GameSTriggerGameMenuItemSelectedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerGameMenuItemSelected(res)))
                }
                92 => {
                    tracing::debug!("Variant ETriggerMouseWheel for value '92'");

                    let (tail, res) = GameSTriggerMouseWheelEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMouseWheel(res)))
                }
                93 => {
                    tracing::debug!(
                        "Variant ETriggerPurchasePanelSelectedPurchaseItemChanged for value '93'"
                    );

                    let (tail, res) =
                        GameSTriggerPurchasePanelSelectedPurchaseItemChangedEvent::parse(tail)?;
                    Ok((
                        tail,
                        Self::ETriggerPurchasePanelSelectedPurchaseItemChanged(res),
                    ))
                }
                94 => {
                    tracing::debug!(
                        "Variant ETriggerPurchasePanelSelectedPurchaseCategoryChanged for value '94'"
                    );

                    let (tail, res) =
                        GameSTriggerPurchasePanelSelectedPurchaseCategoryChangedEvent::parse(tail)?;
                    Ok((
                        tail,
                        Self::ETriggerPurchasePanelSelectedPurchaseCategoryChanged(res),
                    ))
                }
                95 => {
                    tracing::debug!("Variant ETriggerButtonPressed for value '95'");

                    let (tail, res) = GameSTriggerButtonPressedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerButtonPressed(res)))
                }
                96 => {
                    tracing::debug!("Variant ETriggerGameCreditsFinished for value '96'");

                    let (tail, res) = GameSTriggerGameCreditsFinishedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerGameCreditsFinished(res)))
                }
                97 => {
                    tracing::debug!("Variant ETriggerCutsceneBookmarkFired for value '97'");

                    let (tail, res) = GameSTriggerCutsceneBookmarkFiredEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerCutsceneBookmarkFired(res)))
                }
                98 => {
                    tracing::debug!("Variant ETriggerCutsceneEndSceneFired for value '98'");

                    let (tail, res) = GameSTriggerCutsceneEndSceneFiredEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerCutsceneEndSceneFired(res)))
                }
                99 => {
                    tracing::debug!("Variant ETriggerCutsceneConversationLine for value '99'");

                    let (tail, res) = GameSTriggerCutsceneConversationLineEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerCutsceneConversationLine(res)))
                }
                100 => {
                    tracing::debug!(
                        "Variant ETriggerCutsceneConversationLineMissing for value '100'"
                    );

                    let (tail, res) =
                        GameSTriggerCutsceneConversationLineMissingEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerCutsceneConversationLineMissing(res)))
                }
                101 => {
                    tracing::debug!("Variant EGameUserLeave for value '101'");

                    let (tail, res) = GameSGameUserLeaveEvent::parse(tail)?;
                    Ok((tail, Self::EGameUserLeave(res)))
                }
                102 => {
                    tracing::debug!("Variant EGameUserJoin for value '102'");

                    let (tail, res) = GameSGameUserJoinEvent::parse(tail)?;
                    Ok((tail, Self::EGameUserJoin(res)))
                }
                103 => {
                    tracing::debug!("Variant ECommandManagerState for value '103'");

                    let (tail, res) = GameSCommandManagerStateEvent::parse(tail)?;
                    Ok((tail, Self::ECommandManagerState(res)))
                }
                104 => {
                    tracing::debug!("Variant ECmdUpdateTargetPoint for value '104'");

                    let (tail, res) = GameSCmdUpdateTargetPointEvent::parse(tail)?;
                    Ok((tail, Self::ECmdUpdateTargetPoint(res)))
                }
                105 => {
                    tracing::debug!("Variant ECmdUpdateTargetUnit for value '105'");

                    let (tail, res) = GameSCmdUpdateTargetUnitEvent::parse(tail)?;
                    Ok((tail, Self::ECmdUpdateTargetUnit(res)))
                }
                106 => {
                    tracing::debug!("Variant ETriggerAnimLengthQueryByName for value '106'");

                    let (tail, res) = GameSTriggerAnimLengthQueryByNameEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerAnimLengthQueryByName(res)))
                }
                107 => {
                    tracing::debug!("Variant ETriggerAnimLengthQueryByProps for value '107'");

                    let (tail, res) = GameSTriggerAnimLengthQueryByPropsEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerAnimLengthQueryByProps(res)))
                }
                108 => {
                    tracing::debug!("Variant ETriggerAnimOffset for value '108'");

                    let (tail, res) = GameSTriggerAnimOffsetEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerAnimOffset(res)))
                }
                109 => {
                    tracing::debug!("Variant ECatalogModify for value '109'");

                    let (tail, res) = GameSCatalogModifyEvent::parse(tail)?;
                    Ok((tail, Self::ECatalogModify(res)))
                }
                110 => {
                    tracing::debug!("Variant EHeroTalentTreeSelected for value '110'");

                    let (tail, res) = GameSHeroTalentTreeSelectedEvent::parse(tail)?;
                    Ok((tail, Self::EHeroTalentTreeSelected(res)))
                }
                111 => {
                    tracing::debug!("Variant ETriggerProfilerLoggingFinished for value '111'");

                    let (tail, res) = GameSTriggerProfilerLoggingFinishedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerProfilerLoggingFinished(res)))
                }
                112 => {
                    tracing::debug!("Variant EHeroTalentTreeSelectionPanelToggled for value '112'");

                    let (tail, res) = GameSHeroTalentTreeSelectionPanelToggledEvent::parse(tail)?;
                    Ok((tail, Self::EHeroTalentTreeSelectionPanelToggled(res)))
                }
                113 => {
                    tracing::debug!("Variant EMuteUserChanged for value '113'");

                    let (tail, res) = GameSMuteChatEvent::parse(tail)?;
                    Ok((tail, Self::EMuteUserChanged(res)))
                }
                115 => {
                    tracing::debug!("Variant EConvertToReplaySession for value '115'");

                    let (tail, res) = GameSConvertToReplaySessionEvent::parse(tail)?;
                    Ok((tail, Self::EConvertToReplaySession(res)))
                }
                116 => {
                    tracing::debug!("Variant ESetSyncLoadingTime for value '116'");

                    let (tail, res) = GameSSetSyncLoadingTimeEvent::parse(tail)?;
                    Ok((tail, Self::ESetSyncLoadingTime(res)))
                }
                117 => {
                    tracing::debug!("Variant ESetSyncPlayingTime for value '117'");

                    let (tail, res) = GameSSetSyncPlayingTimeEvent::parse(tail)?;
                    Ok((tail, Self::ESetSyncPlayingTime(res)))
                }
                118 => {
                    tracing::debug!("Variant EPeerSetSyncLoadingTime for value '118'");

                    let (tail, res) = GameSPeerSetSyncLoadingTimeEvent::parse(tail)?;
                    Ok((tail, Self::EPeerSetSyncLoadingTime(res)))
                }
                119 => {
                    tracing::debug!("Variant EPeerSetSyncPlayingTime for value '119'");

                    let (tail, res) = GameSPeerSetSyncPlayingTimeEvent::parse(tail)?;
                    Ok((tail, Self::EPeerSetSyncPlayingTime(res)))
                }

                _ => {
                    tracing::debug!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
        /// Reads a delta, GameEvent set
        #[tracing::instrument(name="GameEvents::parse_events", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_event_triplet(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), (i64, i64, GameEEventId)> {
            let (tail, delta) = SVarUint32::parse(input)?;
            let (tail, user_id) = ReplaySGameUserId::parse(tail)?;
            let (tail, event) = GameEEventId::parse(tail)?;
            let delta = match delta {
                SVarUint32::MUint6(val) => val.value,
                SVarUint32::MUint14(val) => val.value,
                SVarUint32::MUint22(val) => val.value,
                SVarUint32::MUint32(val) => val.value,
            };
            // The next event is byte aligned
            let (tail, _) = byte_align(tail)?;
            Ok((tail, (delta, user_id.m_user_id, event)))
        }

        /// Read the Tracker Events
        pub fn read_events(
            mpq: &MPQ,
            file_contents: &[u8],
        ) -> Result<Vec<GameEvent>, S2ProtocolError> {
            let (_event_tail, game_events) =
                mpq.read_mpq_file_sector("replay.game.events", false, file_contents)?;
            let mut res = vec![];
            let mut count = 1usize;
            let mut event_tail: (&[u8], usize) = (&game_events, 0usize);
            loop {
                tracing::debug!("-----------------------------------------------");
                tracing::debug!("Event number: {}", count);
                let (new_event_tail, (delta, user_id, event)) =
                    Self::parse_event_triplet(event_tail)?;
                count += 1;
                event_tail = new_event_tail;
                match event.try_into() {
                    Ok(val) => res.push(GameEvent {
                        delta,
                        user_id,
                        event: val,
                    }),
                    Err(err) => {
                        tracing::debug!("Skipping event: {:?}", err);
                    }
                };
                if event_tail.0.input_len() == 0 {
                    break;
                }
            }
            Ok(res)
        }
    }

    impl TryFrom<GameEEventId> for ReplayGameEvent {
        type Error = S2ProtocolError;
        fn try_from(value: GameEEventId) -> Result<Self, Self::Error> {
            match value {
                GameEEventId::EDropUser(e) => Ok(e.into()),
                GameEEventId::ECameraSave(e) => Ok(e.into()),
                GameEEventId::ECmd(e) => Ok(e.into()),
                GameEEventId::ESelectionDelta(e) => Ok(e.try_into()?),
                GameEEventId::EControlGroupUpdate(e) => Ok(e.try_into()?),
                GameEEventId::ESelectionSyncCheck(e) => Ok(e.into()),
                GameEEventId::ETriggerChatMessage(e) => Ok(e.into()),
                GameEEventId::EUnitClick(e) => Ok(e.into()),
                GameEEventId::EUnitHighlight(e) => Ok(e.into()),
                GameEEventId::ETriggerReplySelected(e) => Ok(e.into()),
                GameEEventId::ECameraUpdate(e) => Ok(e.into()),
                GameEEventId::ETriggerMouseClicked(e) => Ok(e.into()),
                GameEEventId::ETriggerMouseMoved(e) => Ok(e.into()),
                GameEEventId::ETriggerHotkeyPressed(e) => Ok(e.into()),
                GameEEventId::ETriggerTargetModeUpdate(e) => Ok(e.into()),
                GameEEventId::ETriggerKeyPressed(e) => Ok(e.into()),
                GameEEventId::ETriggerMouseWheel(e) => Ok(e.into()),
                GameEEventId::ETriggerButtonPressed(e) => Ok(e.into()),
                GameEEventId::ECommandManagerState(e) => Ok(e.into()),
                GameEEventId::ECmdUpdateTargetPoint(e) => Ok(e.into()),
                GameEEventId::ECmdUpdateTargetUnit(e) => Ok(e.into()),
                _ => Err(S2ProtocolError::UnsupportedEventType),
            }
        }
    }

    impl From<GameSCmdEvent> for ReplayGameEvent {
        fn from(source: GameSCmdEvent) -> ReplayGameEvent {
            let m_abil = source.m_abil.map(|val| val.into());
            ReplayGameEvent::Cmd(crate::game_events::GameSCmdEvent {
                m_cmd_flags: source.m_cmd_flags,
                m_abil,
                m_data: source.m_data.into(),
                m_sequence: source.m_sequence,
                m_other_unit: source.m_other_unit.map(|u| u.value.value as u32),
                m_unit_group: source.m_unit_group.map(|g| g.value as u32),
            })
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSCmdEvent {
        pub m_cmd_flags: i64,
        pub m_abil: Option<GameSCmdAbil>,
        pub m_data: GameSCmdData,
        pub m_sequence: i64,
        pub m_other_unit: Option<GameTUnitTag>,
        pub m_unit_group: Option<Uint32>,
    }
    impl GameSCmdEvent {
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_cmd_flags(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), i64> {
            let (tail, m_cmd_flags) = parse_packed_int(input, 0, 26usize)?;
            tracing::trace!("res: {:?}", m_cmd_flags);
            Ok((tail, m_cmd_flags))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_abil(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameSCmdAbil>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_abil) = if is_provided {
                let (tail, res) = GameSCmdAbil::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::trace!("res: {:?}", m_abil);
            Ok((tail, m_abil))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_data(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameSCmdData> {
            let (tail, m_data) = GameSCmdData::parse(input)?;
            tracing::trace!("res: {:?}", m_data);
            Ok((tail, m_data))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sequence(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), i64> {
            let (tail, m_sequence) = parse_packed_int(input, 1, 32usize)?;
            tracing::trace!("res: {:?}", m_sequence);
            Ok((tail, m_sequence))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_other_unit(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameTUnitTag>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_other_unit) = if is_provided {
                let (tail, res) = GameTUnitTag::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::trace!("res: {:?}", m_other_unit);
            Ok((tail, m_other_unit))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_unit_group(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Uint32>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_unit_group) = if is_provided {
                let (tail, res) = Uint32::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::trace!("res: {:?}", m_unit_group);
            Ok((tail, m_unit_group))
        }
        #[tracing::instrument(name="75689::bit_packed::GameSCmdEvent::Parse", level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_cmd_flags: Option<i64> = None;
            let mut m_abil: Option<Option<GameSCmdAbil>> = Some(None);
            let mut m_data: Option<GameSCmdData> = None;
            let mut m_sequence: Option<i64> = None;
            let mut m_other_unit: Option<Option<GameTUnitTag>> = Some(None);
            let mut m_unit_group: Option<Option<Uint32>> = Some(None);
            if m_cmd_flags.is_none() {
                let (new_tail, parsed_m_cmd_flags) = Self::parse_m_cmd_flags(tail)?;
                tail = new_tail;
                m_cmd_flags = Some(parsed_m_cmd_flags);
            }
            if let Some(None) = m_abil {
                let (new_tail, parsed_m_abil) = Self::parse_m_abil(tail)?;
                tail = new_tail;
                m_abil = Some(parsed_m_abil);
            }
            if m_data.is_none() {
                let (new_tail, parsed_m_data) = Self::parse_m_data(tail)?;
                tail = new_tail;
                m_data = Some(parsed_m_data);
            }
            if m_sequence.is_none() {
                let (new_tail, parsed_m_sequence) = Self::parse_m_sequence(tail)?;
                tail = new_tail;
                m_sequence = Some(parsed_m_sequence);
            }
            if let Some(None) = m_other_unit {
                let (new_tail, parsed_m_other_unit) = Self::parse_m_other_unit(tail)?;
                tail = new_tail;
                m_other_unit = Some(parsed_m_other_unit);
            }
            if let Some(None) = m_unit_group {
                let (new_tail, parsed_m_unit_group) = Self::parse_m_unit_group(tail)?;
                tail = new_tail;
                m_unit_group = Some(parsed_m_unit_group);
            }
            Ok((
                tail,
                Self {
                    m_cmd_flags: m_cmd_flags.expect("Missing m_cmd_flags from struct"),
                    m_abil: m_abil.expect("Missing m_abil from struct"),
                    m_data: m_data.expect("Missing m_data from struct"),
                    m_sequence: m_sequence.expect("Missing m_sequence from struct"),
                    m_other_unit: m_other_unit.expect("Missing m_other_unit from struct"),
                    m_unit_group: m_unit_group.expect("Missing m_unit_group from struct"),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTHandicap {
        pub value: i64,
    }
    impl GameTHandicap {
        #[tracing::instrument(name="75689::GameTHandicap::IntType::Parse::MinMaxConstraint", level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 7;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCLicenseArray {
        pub value: Vec<GameTLicense>,
    }
    impl GameCLicenseArray {
        #[tracing::instrument(name="75689::GameCLicenseArray::ArrayType::Parse", level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 13;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(GameTLicense::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<GameTLicense>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = GameTLicense::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSLobbySlot {
        pub m_control: GameTControlId,
        pub m_user_id: Option<TUserId>,
        pub m_team_id: GameTTeamId,
        pub m_color_pref: GameTColorPreference,
        pub m_race_pref: TRacePreference,
        pub m_difficulty: GameTDifficulty,
        pub m_ai_build: GameTaiBuild,
        pub m_handicap: GameTHandicap,
        pub m_observe: EObserve,
        pub m_logo_index: GameTPlayerLogoIndex,
        pub m_hero: CHeroHandle,
        pub m_skin: CSkinHandle,
        pub m_mount: CMountHandle,
        pub m_artifacts: GameCArtifactArray,
        pub m_working_set_slot_id: Option<Uint8>,
        pub m_rewards: GameCRewardArray,
        pub m_toon_handle: CToonHandle,
        pub m_licenses: GameCLicenseArray,
        pub m_tandem_leader_id: Option<TUserId>,
        pub m_commander: CCommanderHandle,
        pub m_commander_level: Uint32,
        pub m_has_silence_penalty: bool,
        pub m_tandem_id: Option<TUserId>,
        pub m_commander_mastery_level: Uint32,
        pub m_commander_mastery_talents: GameCCommanderMasteryTalentArray,
        pub m_trophy_id: Uint32,
        pub m_reward_overrides: GameCRewardOverrideArray,
    }
    impl GameSLobbySlot {
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_control(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTControlId> {
            let (tail, m_control) = GameTControlId::parse(input)?;
            tracing::trace!("res: {:?}", m_control);
            Ok((tail, m_control))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_user_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<TUserId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_user_id) = if is_provided {
                let (tail, res) = TUserId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::trace!("res: {:?}", m_user_id);
            Ok((tail, m_user_id))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_team_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTTeamId> {
            let (tail, m_team_id) = GameTTeamId::parse(input)?;
            tracing::trace!("res: {:?}", m_team_id);
            Ok((tail, m_team_id))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_color_pref(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTColorPreference> {
            let (tail, m_color_pref) = GameTColorPreference::parse(input)?;
            tracing::trace!("res: {:?}", m_color_pref);
            Ok((tail, m_color_pref))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_race_pref(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TRacePreference> {
            let (tail, m_race_pref) = TRacePreference::parse(input)?;
            tracing::trace!("res: {:?}", m_race_pref);
            Ok((tail, m_race_pref))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_difficulty(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTDifficulty> {
            let (tail, m_difficulty) = GameTDifficulty::parse(input)?;
            tracing::trace!("res: {:?}", m_difficulty);
            Ok((tail, m_difficulty))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_ai_build(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTaiBuild> {
            let (tail, m_ai_build) = GameTaiBuild::parse(input)?;
            tracing::trace!("res: {:?}", m_ai_build);
            Ok((tail, m_ai_build))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_handicap(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTHandicap> {
            let (tail, m_handicap) = GameTHandicap::parse(input)?;
            tracing::trace!("res: {:?}", m_handicap);
            Ok((tail, m_handicap))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_observe(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), EObserve> {
            let (tail, m_observe) = EObserve::parse(input)?;
            tracing::trace!("res: {:?}", m_observe);
            Ok((tail, m_observe))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_logo_index(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTPlayerLogoIndex> {
            let (tail, m_logo_index) = GameTPlayerLogoIndex::parse(input)?;
            tracing::trace!("res: {:?}", m_logo_index);
            Ok((tail, m_logo_index))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_hero(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CHeroHandle> {
            let (tail, m_hero) = CHeroHandle::parse(input)?;
            tracing::trace!("res: {:?}", m_hero);
            Ok((tail, m_hero))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_skin(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CSkinHandle> {
            let (tail, m_skin) = CSkinHandle::parse(input)?;
            tracing::trace!("res: {:?}", m_skin);
            Ok((tail, m_skin))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_mount(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CMountHandle> {
            let (tail, m_mount) = CMountHandle::parse(input)?;
            tracing::trace!("res: {:?}", m_mount);
            Ok((tail, m_mount))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_artifacts(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCArtifactArray> {
            let (tail, m_artifacts) = GameCArtifactArray::parse(input)?;
            tracing::trace!("res: {:?}", m_artifacts);
            Ok((tail, m_artifacts))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_working_set_slot_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Uint8>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_working_set_slot_id) = if is_provided {
                let (tail, res) = Uint8::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::trace!("res: {:?}", m_working_set_slot_id);
            Ok((tail, m_working_set_slot_id))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_rewards(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCRewardArray> {
            let (tail, m_rewards) = GameCRewardArray::parse(input)?;
            tracing::trace!("res: {:?}", m_rewards);
            Ok((tail, m_rewards))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_toon_handle(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CToonHandle> {
            let (tail, m_toon_handle) = CToonHandle::parse(input)?;
            tracing::trace!("res: {:?}", m_toon_handle);
            Ok((tail, m_toon_handle))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_licenses(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCLicenseArray> {
            let (tail, m_licenses) = GameCLicenseArray::parse(input)?;
            tracing::trace!("res: {:?}", m_licenses);
            Ok((tail, m_licenses))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_tandem_leader_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<TUserId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_tandem_leader_id) = if is_provided {
                let (tail, res) = TUserId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::trace!("res: {:?}", m_tandem_leader_id);
            Ok((tail, m_tandem_leader_id))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_commander(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CCommanderHandle> {
            let (tail, m_commander) = CCommanderHandle::parse(input)?;
            tracing::trace!("res: {:?}", m_commander);
            Ok((tail, m_commander))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_commander_level(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_commander_level) = Uint32::parse(input)?;
            tracing::trace!("res: {:?}", m_commander_level);
            Ok((tail, m_commander_level))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_has_silence_penalty(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_has_silence_penalty) = parse_bool(input)?;
            tracing::trace!("res: {:?}", m_has_silence_penalty);
            Ok((tail, m_has_silence_penalty))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_tandem_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<TUserId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_tandem_id) = if is_provided {
                let (tail, res) = TUserId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::trace!("res: {:?}", m_tandem_id);
            Ok((tail, m_tandem_id))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_commander_mastery_level(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_commander_mastery_level) = Uint32::parse(input)?;
            tracing::trace!("res: {:?}", m_commander_mastery_level);
            Ok((tail, m_commander_mastery_level))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_commander_mastery_talents(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCCommanderMasteryTalentArray> {
            let (tail, m_commander_mastery_talents) =
                GameCCommanderMasteryTalentArray::parse(input)?;
            tracing::trace!("res: {:?}", m_commander_mastery_talents);
            Ok((tail, m_commander_mastery_talents))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_trophy_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_trophy_id) = Uint32::parse(input)?;
            tracing::trace!("res: {:?}", m_trophy_id);
            Ok((tail, m_trophy_id))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_reward_overrides(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCRewardOverrideArray> {
            let (tail, m_reward_overrides) = GameCRewardOverrideArray::parse(input)?;
            tracing::trace!("res: {:?}", m_reward_overrides);
            Ok((tail, m_reward_overrides))
        }
        #[tracing::instrument(name="75689::bit_packed::GameSLobbySlot::Parse", level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_control: Option<GameTControlId> = None;
            let mut m_user_id: Option<Option<TUserId>> = Some(None);
            let mut m_team_id: Option<GameTTeamId> = None;
            let mut m_color_pref: Option<GameTColorPreference> = None;
            let mut m_race_pref: Option<TRacePreference> = None;
            let mut m_difficulty: Option<GameTDifficulty> = None;
            let mut m_ai_build: Option<GameTaiBuild> = None;
            let mut m_handicap: Option<GameTHandicap> = None;
            let mut m_observe: Option<EObserve> = None;
            let mut m_logo_index: Option<GameTPlayerLogoIndex> = None;
            let mut m_hero: Option<CHeroHandle> = None;
            let mut m_skin: Option<CSkinHandle> = None;
            let mut m_mount: Option<CMountHandle> = None;
            let mut m_artifacts: Option<GameCArtifactArray> = None;
            let mut m_working_set_slot_id: Option<Option<Uint8>> = Some(None);
            let mut m_rewards: Option<GameCRewardArray> = None;
            let mut m_toon_handle: Option<CToonHandle> = None;
            let mut m_licenses: Option<GameCLicenseArray> = None;
            let mut m_tandem_leader_id: Option<Option<TUserId>> = Some(None);
            let mut m_commander: Option<CCommanderHandle> = None;
            let mut m_commander_level: Option<Uint32> = None;
            let mut m_has_silence_penalty: Option<bool> = None;
            let mut m_tandem_id: Option<Option<TUserId>> = Some(None);
            let mut m_commander_mastery_level: Option<Uint32> = None;
            let mut m_commander_mastery_talents: Option<GameCCommanderMasteryTalentArray> = None;
            let mut m_trophy_id: Option<Uint32> = None;
            let mut m_reward_overrides: Option<GameCRewardOverrideArray> = None;
            if m_control.is_none() {
                let (new_tail, parsed_m_control) = Self::parse_m_control(tail)?;
                tail = new_tail;
                m_control = Some(parsed_m_control);
            }
            if let Some(None) = m_user_id {
                let (new_tail, parsed_m_user_id) = Self::parse_m_user_id(tail)?;
                tail = new_tail;
                m_user_id = Some(parsed_m_user_id);
            }
            if m_team_id.is_none() {
                let (new_tail, parsed_m_team_id) = Self::parse_m_team_id(tail)?;
                tail = new_tail;
                m_team_id = Some(parsed_m_team_id);
            }
            if m_color_pref.is_none() {
                let (new_tail, parsed_m_color_pref) = Self::parse_m_color_pref(tail)?;
                tail = new_tail;
                m_color_pref = Some(parsed_m_color_pref);
            }
            if m_race_pref.is_none() {
                let (new_tail, parsed_m_race_pref) = Self::parse_m_race_pref(tail)?;
                tail = new_tail;
                m_race_pref = Some(parsed_m_race_pref);
            }
            if m_difficulty.is_none() {
                let (new_tail, parsed_m_difficulty) = Self::parse_m_difficulty(tail)?;
                tail = new_tail;
                m_difficulty = Some(parsed_m_difficulty);
            }
            if m_ai_build.is_none() {
                let (new_tail, parsed_m_ai_build) = Self::parse_m_ai_build(tail)?;
                tail = new_tail;
                m_ai_build = Some(parsed_m_ai_build);
            }
            if m_handicap.is_none() {
                let (new_tail, parsed_m_handicap) = Self::parse_m_handicap(tail)?;
                tail = new_tail;
                m_handicap = Some(parsed_m_handicap);
            }
            if m_observe.is_none() {
                let (new_tail, parsed_m_observe) = Self::parse_m_observe(tail)?;
                tail = new_tail;
                m_observe = Some(parsed_m_observe);
            }
            if m_logo_index.is_none() {
                let (new_tail, parsed_m_logo_index) = Self::parse_m_logo_index(tail)?;
                tail = new_tail;
                m_logo_index = Some(parsed_m_logo_index);
            }
            if m_hero.is_none() {
                let (new_tail, parsed_m_hero) = Self::parse_m_hero(tail)?;
                tail = new_tail;
                m_hero = Some(parsed_m_hero);
            }
            if m_skin.is_none() {
                let (new_tail, parsed_m_skin) = Self::parse_m_skin(tail)?;
                tail = new_tail;
                m_skin = Some(parsed_m_skin);
            }
            if m_mount.is_none() {
                let (new_tail, parsed_m_mount) = Self::parse_m_mount(tail)?;
                tail = new_tail;
                m_mount = Some(parsed_m_mount);
            }
            if m_artifacts.is_none() {
                let (new_tail, parsed_m_artifacts) = Self::parse_m_artifacts(tail)?;
                tail = new_tail;
                m_artifacts = Some(parsed_m_artifacts);
            }
            if let Some(None) = m_working_set_slot_id {
                let (new_tail, parsed_m_working_set_slot_id) =
                    Self::parse_m_working_set_slot_id(tail)?;
                tail = new_tail;
                m_working_set_slot_id = Some(parsed_m_working_set_slot_id);
            }
            if m_rewards.is_none() {
                let (new_tail, parsed_m_rewards) = Self::parse_m_rewards(tail)?;
                tail = new_tail;
                m_rewards = Some(parsed_m_rewards);
            }
            if m_toon_handle.is_none() {
                let (new_tail, parsed_m_toon_handle) = Self::parse_m_toon_handle(tail)?;
                tail = new_tail;
                m_toon_handle = Some(parsed_m_toon_handle);
            }
            if m_licenses.is_none() {
                let (new_tail, parsed_m_licenses) = Self::parse_m_licenses(tail)?;
                tail = new_tail;
                m_licenses = Some(parsed_m_licenses);
            }
            if let Some(None) = m_tandem_leader_id {
                let (new_tail, parsed_m_tandem_leader_id) = Self::parse_m_tandem_leader_id(tail)?;
                tail = new_tail;
                m_tandem_leader_id = Some(parsed_m_tandem_leader_id);
            }
            if m_commander.is_none() {
                let (new_tail, parsed_m_commander) = Self::parse_m_commander(tail)?;
                tail = new_tail;
                m_commander = Some(parsed_m_commander);
            }
            if m_commander_level.is_none() {
                let (new_tail, parsed_m_commander_level) = Self::parse_m_commander_level(tail)?;
                tail = new_tail;
                m_commander_level = Some(parsed_m_commander_level);
            }
            if m_has_silence_penalty.is_none() {
                let (new_tail, parsed_m_has_silence_penalty) =
                    Self::parse_m_has_silence_penalty(tail)?;
                tail = new_tail;
                m_has_silence_penalty = Some(parsed_m_has_silence_penalty);
            }
            if let Some(None) = m_tandem_id {
                let (new_tail, parsed_m_tandem_id) = Self::parse_m_tandem_id(tail)?;
                tail = new_tail;
                m_tandem_id = Some(parsed_m_tandem_id);
            }
            if m_commander_mastery_level.is_none() {
                let (new_tail, parsed_m_commander_mastery_level) =
                    Self::parse_m_commander_mastery_level(tail)?;
                tail = new_tail;
                m_commander_mastery_level = Some(parsed_m_commander_mastery_level);
            }
            if m_commander_mastery_talents.is_none() {
                let (new_tail, parsed_m_commander_mastery_talents) =
                    Self::parse_m_commander_mastery_talents(tail)?;
                tail = new_tail;
                m_commander_mastery_talents = Some(parsed_m_commander_mastery_talents);
            }
            if m_trophy_id.is_none() {
                let (new_tail, parsed_m_trophy_id) = Self::parse_m_trophy_id(tail)?;
                tail = new_tail;
                m_trophy_id = Some(parsed_m_trophy_id);
            }
            if m_reward_overrides.is_none() {
                let (new_tail, parsed_m_reward_overrides) = Self::parse_m_reward_overrides(tail)?;
                tail = new_tail;
                m_reward_overrides = Some(parsed_m_reward_overrides);
            }
            Ok((
                tail,
                Self {
                    m_control: m_control.expect("Missing m_control from struct"),
                    m_user_id: m_user_id.expect("Missing m_user_id from struct"),
                    m_team_id: m_team_id.expect("Missing m_team_id from struct"),
                    m_color_pref: m_color_pref.expect("Missing m_color_pref from struct"),
                    m_race_pref: m_race_pref.expect("Missing m_race_pref from struct"),
                    m_difficulty: m_difficulty.expect("Missing m_difficulty from struct"),
                    m_ai_build: m_ai_build.expect("Missing m_ai_build from struct"),
                    m_handicap: m_handicap.expect("Missing m_handicap from struct"),
                    m_observe: m_observe.expect("Missing m_observe from struct"),
                    m_logo_index: m_logo_index.expect("Missing m_logo_index from struct"),
                    m_hero: m_hero.expect("Missing m_hero from struct"),
                    m_skin: m_skin.expect("Missing m_skin from struct"),
                    m_mount: m_mount.expect("Missing m_mount from struct"),
                    m_artifacts: m_artifacts.expect("Missing m_artifacts from struct"),
                    m_working_set_slot_id: m_working_set_slot_id
                        .expect("Missing m_working_set_slot_id from struct"),
                    m_rewards: m_rewards.expect("Missing m_rewards from struct"),
                    m_toon_handle: m_toon_handle.expect("Missing m_toon_handle from struct"),
                    m_licenses: m_licenses.expect("Missing m_licenses from struct"),
                    m_tandem_leader_id: m_tandem_leader_id
                        .expect("Missing m_tandem_leader_id from struct"),
                    m_commander: m_commander.expect("Missing m_commander from struct"),
                    m_commander_level: m_commander_level
                        .expect("Missing m_commander_level from struct"),
                    m_has_silence_penalty: m_has_silence_penalty
                        .expect("Missing m_has_silence_penalty from struct"),
                    m_tandem_id: m_tandem_id.expect("Missing m_tandem_id from struct"),
                    m_commander_mastery_level: m_commander_mastery_level
                        .expect("Missing m_commander_mastery_level from struct"),
                    m_commander_mastery_talents: m_commander_mastery_talents
                        .expect("Missing m_commander_mastery_talents from struct"),
                    m_trophy_id: m_trophy_id.expect("Missing m_trophy_id from struct"),
                    m_reward_overrides: m_reward_overrides
                        .expect("Missing m_reward_overrides from struct"),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameSLobbySlotChange {
        MControl(GameTControlId),
        MUserId(Option<TUserId>),
        MTeamId(GameTTeamId),
        MColorPref(GameTColorPreference),
        MRacePref(TRacePreference),
        MDifficulty(GameTDifficulty),
        MAiBuild(GameTaiBuild),
        MHandicap(GameTHandicap),
        MObserve(EObserve),
        MLogoIndex(GameTPlayerLogoIndex),
        MHero(CHeroHandle),
        MSkin(CSkinHandle),
        MMount(CMountHandle),
        MLicenses(GameCLicenseArray),
        MTandemLeaderId(Option<TUserId>),
        MCommander(CCommanderHandle),
        MCommanderLevel(Uint32),
        MHasSilencePenalty(bool),
        MTandemId(Option<TUserId>),
        MCommanderMasteryLevel(Uint32),
    }
    impl GameSLobbySlotChange {
        #[tracing::instrument(name="75689::GameSLobbySlotChange::ChoiceType::parse", level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // ChoiceType:
            // Use the number of elements in the json .fields to calculate how many
            // bits to have unique tags.
            // let num_fields: usize = 20;
            let offset = 0i64;
            let num_bits: usize = 5;
            let (tail, variant_tag) = parse_packed_int(input, offset, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::trace!("Variant tagged '0' for MControl");
                    let (tail, res) = GameTControlId::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MControl(res)))
                }
                1 => {
                    tracing::trace!("Variant tagged '1' for MUserId");
                    let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(tail)?;
                    if is_provided {
                        let (tail, res) = TUserId::parse(tail)?;
                        tracing::trace!("res: {:?}", res);
                        Ok((tail, Self::MUserId(Some(res))))
                    } else {
                        Ok((tail, Self::MUserId(None)))
                    }
                }
                2 => {
                    tracing::trace!("Variant tagged '2' for MTeamId");
                    let (tail, res) = GameTTeamId::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MTeamId(res)))
                }
                3 => {
                    tracing::trace!("Variant tagged '3' for MColorPref");
                    let (tail, res) = GameTColorPreference::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MColorPref(res)))
                }
                4 => {
                    tracing::trace!("Variant tagged '4' for MRacePref");
                    let (tail, res) = TRacePreference::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MRacePref(res)))
                }
                5 => {
                    tracing::trace!("Variant tagged '5' for MDifficulty");
                    let (tail, res) = GameTDifficulty::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MDifficulty(res)))
                }
                6 => {
                    tracing::trace!("Variant tagged '6' for MAiBuild");
                    let (tail, res) = GameTaiBuild::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MAiBuild(res)))
                }
                7 => {
                    tracing::trace!("Variant tagged '7' for MHandicap");
                    let (tail, res) = GameTHandicap::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MHandicap(res)))
                }
                8 => {
                    tracing::trace!("Variant tagged '8' for MObserve");
                    let (tail, res) = EObserve::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MObserve(res)))
                }
                9 => {
                    tracing::trace!("Variant tagged '9' for MLogoIndex");
                    let (tail, res) = GameTPlayerLogoIndex::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MLogoIndex(res)))
                }
                10 => {
                    tracing::trace!("Variant tagged '10' for MHero");
                    let (tail, res) = CHeroHandle::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MHero(res)))
                }
                11 => {
                    tracing::trace!("Variant tagged '11' for MSkin");
                    let (tail, res) = CSkinHandle::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MSkin(res)))
                }
                12 => {
                    tracing::trace!("Variant tagged '12' for MMount");
                    let (tail, res) = CMountHandle::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MMount(res)))
                }
                13 => {
                    tracing::trace!("Variant tagged '13' for MLicenses");
                    let (tail, res) = GameCLicenseArray::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MLicenses(res)))
                }
                14 => {
                    tracing::trace!("Variant tagged '14' for MTandemLeaderId");
                    let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(tail)?;
                    if is_provided {
                        let (tail, res) = TUserId::parse(tail)?;
                        tracing::trace!("res: {:?}", res);
                        Ok((tail, Self::MTandemLeaderId(Some(res))))
                    } else {
                        Ok((tail, Self::MTandemLeaderId(None)))
                    }
                }
                15 => {
                    tracing::trace!("Variant tagged '15' for MCommander");
                    let (tail, res) = CCommanderHandle::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MCommander(res)))
                }
                16 => {
                    tracing::trace!("Variant tagged '16' for MCommanderLevel");
                    let (tail, res) = Uint32::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MCommanderLevel(res)))
                }
                17 => {
                    tracing::trace!("Variant tagged '17' for MHasSilencePenalty");
                    let (tail, res) = parse_bool(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MHasSilencePenalty(res)))
                }
                18 => {
                    tracing::trace!("Variant tagged '18' for MTandemId");
                    let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(tail)?;
                    if is_provided {
                        let (tail, res) = TUserId::parse(tail)?;
                        tracing::trace!("res: {:?}", res);
                        Ok((tail, Self::MTandemId(Some(res))))
                    } else {
                        Ok((tail, Self::MTandemId(None)))
                    }
                }
                19 => {
                    tracing::trace!("Variant tagged '19' for MCommanderMasteryLevel");
                    let (tail, res) = Uint32::parse(tail)?;
                    tracing::trace!("res: {:?}", res);
                    Ok((tail, Self::MCommanderMasteryLevel(res)))
                }

                _ => {
                    tracing::debug!("Unknown variant for tag {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }
}
