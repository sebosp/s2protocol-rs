pub mod byte_aligned {
    //! Generated code from source: ../s2protocol/json/protocol75689.json
    //! All byte aligned types are compatible with protocol87702

    pub(crate) use crate::versions::protocol87702::byte_aligned::GameSDetails;

    pub(crate) use crate::versions::protocol87702::byte_aligned::ReplayTrackerEEventId;
}
pub mod bit_packed {
    //! Generated code from source: ../s2protocol/json/protocol75689.json
    use crate::game_events::ReplayGameEvent;
    use crate::versions::GameEvent;

    use crate::versions::protocol87702::bit_packed::CCommanderHandle;

    use crate::versions::protocol87702::bit_packed::CHeroHandle;
    use crate::versions::protocol87702::bit_packed::CMountHandle;
    use crate::versions::protocol87702::bit_packed::CSkinHandle;
    use crate::versions::protocol87702::bit_packed::CToonHandle;

    use crate::versions::protocol87702::bit_packed::EObserve;

    use crate::versions::protocol87702::bit_packed::GameCArtifactArray;

    use crate::versions::protocol87702::bit_packed::GameCCommanderMasteryTalentArray;

    use crate::versions::protocol87702::bit_packed::GameCRewardArray;

    use crate::versions::protocol87702::bit_packed::GameCRewardOverrideArray;

    pub(crate) use crate::versions::protocol87702::bit_packed::GameEMessageId;

    use crate::versions::protocol87702::bit_packed::GameSAchievementAwardedEvent;
    use crate::versions::protocol87702::bit_packed::GameSAddAbsoluteGameSpeedEvent;
    use crate::versions::protocol87702::bit_packed::GameSAddGameSpeedEvent;
    use crate::versions::protocol87702::bit_packed::GameSAllianceEvent;
    use crate::versions::protocol87702::bit_packed::GameSBankFileEvent;
    use crate::versions::protocol87702::bit_packed::GameSBankKeyEvent;
    use crate::versions::protocol87702::bit_packed::GameSBankSectionEvent;
    use crate::versions::protocol87702::bit_packed::GameSBankSignatureEvent;
    use crate::versions::protocol87702::bit_packed::GameSBankValueEvent;
    use crate::versions::protocol87702::bit_packed::GameSBroadcastCheatEvent;
    use crate::versions::protocol87702::bit_packed::GameSCameraSaveEvent;
    use crate::versions::protocol87702::bit_packed::GameSCameraUpdateEvent;
    use crate::versions::protocol87702::bit_packed::GameSCatalogModifyEvent;

    use crate::versions::protocol87702::bit_packed::GameSCmdAbil;
    use crate::versions::protocol87702::bit_packed::GameSCmdData;

    use crate::versions::protocol87702::bit_packed::GameSCmdUpdateTargetPointEvent;
    use crate::versions::protocol87702::bit_packed::GameSCmdUpdateTargetUnitEvent;

    use crate::versions::protocol87702::bit_packed::GameSCommandManagerResetEvent;
    use crate::versions::protocol87702::bit_packed::GameSCommandManagerStateEvent;
    use crate::versions::protocol87702::bit_packed::GameSControlGroupUpdateEvent;
    use crate::versions::protocol87702::bit_packed::GameSConvertToReplaySessionEvent;
    use crate::versions::protocol87702::bit_packed::GameSDecrementGameTimeRemainingEvent;

    use crate::versions::protocol87702::bit_packed::GameSDropOurselvesEvent;
    use crate::versions::protocol87702::bit_packed::GameSDropUserEvent;
    use crate::versions::protocol87702::bit_packed::GameSGameCheatEvent;

    use crate::versions::protocol87702::bit_packed::GameSGameUserJoinEvent;
    use crate::versions::protocol87702::bit_packed::GameSGameUserLeaveEvent;
    use crate::versions::protocol87702::bit_packed::GameSHeroTalentTreeSelectedEvent;
    use crate::versions::protocol87702::bit_packed::GameSHeroTalentTreeSelectionPanelToggledEvent;
    use crate::versions::protocol87702::bit_packed::GameSHijackReplayGameEvent;

    use crate::versions::protocol87702::bit_packed::GameSHijackReplaySessionEvent;

    use crate::versions::protocol87702::bit_packed::GameSLoadGameDoneEvent;

    use crate::versions::protocol87702::bit_packed::GameSMuteChatEvent;
    use crate::versions::protocol87702::bit_packed::GameSPauseGameEvent;
    use crate::versions::protocol87702::bit_packed::GameSPeerSetSyncLoadingTimeEvent;
    use crate::versions::protocol87702::bit_packed::GameSPeerSetSyncPlayingTimeEvent;
    use crate::versions::protocol87702::bit_packed::GameSPickMapTagEvent;

    use crate::versions::protocol87702::bit_packed::GameSReplayJumpEvent;
    use crate::versions::protocol87702::bit_packed::GameSResourceRequestCancelEvent;
    use crate::versions::protocol87702::bit_packed::GameSResourceRequestEvent;
    use crate::versions::protocol87702::bit_packed::GameSResourceRequestFulfillEvent;
    use crate::versions::protocol87702::bit_packed::GameSResourceTradeEvent;
    use crate::versions::protocol87702::bit_packed::GameSSaveGameDoneEvent;
    use crate::versions::protocol87702::bit_packed::GameSSaveGameEvent;

    use crate::versions::protocol87702::bit_packed::GameSSelectionDeltaEvent;

    use crate::versions::protocol87702::bit_packed::GameSSelectionSyncCheckEvent;

    use crate::versions::protocol87702::bit_packed::GameSSessionCheatEvent;

    use crate::versions::protocol87702::bit_packed::GameSSetAbsoluteGameSpeedEvent;
    use crate::versions::protocol87702::bit_packed::GameSSetGameDurationEvent;
    use crate::versions::protocol87702::bit_packed::GameSSetGameSpeedEvent;
    use crate::versions::protocol87702::bit_packed::GameSSetLobbySlotEvent;
    use crate::versions::protocol87702::bit_packed::GameSSetSyncLoadingTimeEvent;
    use crate::versions::protocol87702::bit_packed::GameSSetSyncPlayingTimeEvent;
    use crate::versions::protocol87702::bit_packed::GameSSingleStepGameEvent;

    use crate::versions::protocol87702::bit_packed::GameSStartGameEvent;

    use crate::versions::protocol87702::bit_packed::GameSTriggerAbortMissionEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerAnimLengthQueryByNameEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerAnimLengthQueryByPropsEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerAnimOffsetEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerBattleReportPanelExitEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerBattleReportPanelPlayMissionEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerBattleReportPanelPlaySceneEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerBattleReportPanelSelectionChangedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerButtonPressedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerChatMessageEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerCommandErrorEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerConversationSkippedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerCustomDialogDismissedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerCutsceneBookmarkFiredEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerCutsceneConversationLineEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerCutsceneConversationLineMissingEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerCutsceneEndSceneFiredEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerDialogControlEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerGameCreditsFinishedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerGameMenuItemSelectedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerHotkeyPressedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerKeyPressedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerMercenaryPanelExitEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerMercenaryPanelPurchaseEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerMercenaryPanelSelectionChangedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerMouseClickedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerMouseMovedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerMouseWheelEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerMovieFinishedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerMovieFunctionEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerMovieStartedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerPingEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerPlanetMissionLaunchedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerPlanetMissionSelectedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerPlanetPanelBirthCompleteEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerPlanetPanelCanceledEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerPlanetPanelDeathCompleteEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerPlanetPanelReplayEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerPortraitLoadedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerProfilerLoggingFinishedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerPurchaseExitEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerPurchaseMadeEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerPurchasePanelSelectedPurchaseCategoryChangedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerPurchasePanelSelectedPurchaseItemChangedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerReplySelectedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerResearchPanelExitEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerResearchPanelPurchaseEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerResearchPanelSelectionChangedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerSkippedEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerSoundLengthQueryEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerSoundLengthSyncEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerSoundOffsetEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerSoundtrackDoneEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerTargetModeUpdateEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerTransmissionCompleteEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerTransmissionOffsetEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerVictoryPanelExitEvent;
    use crate::versions::protocol87702::bit_packed::GameSTriggerVictoryPanelPlayMissionAgainEvent;
    use crate::versions::protocol87702::bit_packed::GameSTurnEvent;
    use crate::versions::protocol87702::bit_packed::GameSUnitClickEvent;
    use crate::versions::protocol87702::bit_packed::GameSUnitHighlightEvent;
    use crate::versions::protocol87702::bit_packed::GameSUnpauseGameEvent;
    use crate::versions::protocol87702::bit_packed::GameSUserFinishedLoadingEvent;
    use crate::versions::protocol87702::bit_packed::GameSUserFinishedLoadingSyncEvent;
    use crate::versions::protocol87702::bit_packed::GameSUserOptionsEvent;
    use crate::versions::protocol87702::bit_packed::GameSaiCommunicateEvent;

    use crate::versions::protocol87702::bit_packed::GameTColorPreference;

    use crate::versions::protocol87702::bit_packed::GameTControlId;
    use crate::versions::protocol87702::bit_packed::GameTDifficulty;

    use crate::versions::protocol87702::bit_packed::GameTLicense;

    use crate::versions::protocol87702::bit_packed::GameTPlayerLogoIndex;

    use crate::versions::protocol87702::bit_packed::GameTTeamId;

    use crate::versions::protocol87702::bit_packed::GameTUnitTag;
    use crate::versions::protocol87702::bit_packed::GameTaiBuild;

    use crate::versions::protocol87702::bit_packed::ReplaySGameUserId;
    pub(crate) use crate::versions::protocol87702::bit_packed::ReplaySInitData;

    use crate::versions::protocol87702::bit_packed::SVarUint32;

    use crate::versions::protocol87702::bit_packed::TRacePreference;

    use crate::versions::protocol87702::bit_packed::TUserId;

    use crate::versions::protocol87702::bit_packed::Uint8;

    use crate::versions::protocol87702::bit_packed::Uint32;
    use crate::*;
    use nom::*;

    #[derive(Debug, PartialEq, Clone)]
    pub(crate) enum GameEEventId {
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
        pub(crate) fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
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
        pub(crate) fn parse_event_triplet(
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
    pub(crate) struct GameSCmdEvent {
        pub m_cmd_flags: i64,
        pub m_abil: Option<GameSCmdAbil>,
        pub m_data: GameSCmdData,
        pub m_sequence: i64,
        pub m_other_unit: Option<GameTUnitTag>,
        pub m_unit_group: Option<Uint32>,
    }
    impl GameSCmdEvent {
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub(crate) fn parse_m_cmd_flags(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), i64> {
            let (tail, m_cmd_flags) = parse_packed_int(input, 0, 26usize)?;
            tracing::trace!("res: {:?}", m_cmd_flags);
            Ok((tail, m_cmd_flags))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub(crate) fn parse_m_abil(
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
        pub(crate) fn parse_m_data(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSCmdData> {
            let (tail, m_data) = GameSCmdData::parse(input)?;
            tracing::trace!("res: {:?}", m_data);
            Ok((tail, m_data))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub(crate) fn parse_m_sequence(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), i64> {
            let (tail, m_sequence) = parse_packed_int(input, 1, 32usize)?;
            tracing::trace!("res: {:?}", m_sequence);
            Ok((tail, m_sequence))
        }
        #[tracing::instrument(level = "trace", skip(input), fields(peek = peek_bits(input)))]
        pub(crate) fn parse_m_other_unit(
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
        pub(crate) fn parse_m_unit_group(
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
        pub(crate) fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
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
}
