pub mod init_data;

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

    use crate::versions::protocol87702::bit_packed::GameTUnitTag;

    use crate::versions::protocol87702::bit_packed::ReplaySGameUserId;

    use crate::versions::protocol87702::bit_packed::SVarUint32;

    use crate::versions::protocol87702::bit_packed::Uint32;
    use crate::*;
    use nom::*;

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(choice, bits = 7)]
    pub(crate) enum GameEEventId {
        #[mpq(tag = 0)]
        ESetLobbySlot(GameSSetLobbySlotEvent),
        #[mpq(tag = 1)]
        EDropUser(GameSDropUserEvent),
        #[mpq(tag = 2)]
        EStartGame(GameSStartGameEvent),
        #[mpq(tag = 3)]
        EDropOurselves(GameSDropOurselvesEvent),
        #[mpq(tag = 4)]
        EUserFinishedLoading(GameSUserFinishedLoadingEvent),
        #[mpq(tag = 5)]
        EUserFinishedLoadingSync(GameSUserFinishedLoadingSyncEvent),
        #[mpq(tag = 6)]
        ESetGameDuration(GameSSetGameDurationEvent),
        #[mpq(tag = 7)]
        EUserOptions(GameSUserOptionsEvent),
        #[mpq(tag = 114)]
        EPickMapTag(GameSPickMapTagEvent),
        #[mpq(tag = 8)]
        ETurn(GameSTurnEvent),
        #[mpq(tag = 9)]
        EBankFile(GameSBankFileEvent),
        #[mpq(tag = 10)]
        EBankSection(GameSBankSectionEvent),
        #[mpq(tag = 11)]
        EBankKey(GameSBankKeyEvent),
        #[mpq(tag = 12)]
        EBankValue(GameSBankValueEvent),
        #[mpq(tag = 13)]
        EBankSignature(GameSBankSignatureEvent),
        #[mpq(tag = 14)]
        ECameraSave(GameSCameraSaveEvent),
        #[mpq(tag = 15)]
        EPauseGame(GameSPauseGameEvent),
        #[mpq(tag = 16)]
        EUnpauseGame(GameSUnpauseGameEvent),
        #[mpq(tag = 17)]
        ESingleStepGame(GameSSingleStepGameEvent),
        #[mpq(tag = 18)]
        ESetGameSpeed(GameSSetGameSpeedEvent),
        #[mpq(tag = 19)]
        EAddGameSpeed(GameSAddGameSpeedEvent),
        #[mpq(tag = 20)]
        EReplayJump(GameSReplayJumpEvent),
        #[mpq(tag = 21)]
        ESaveGame(GameSSaveGameEvent),
        #[mpq(tag = 22)]
        ESaveGameDone(GameSSaveGameDoneEvent),
        #[mpq(tag = 23)]
        ELoadGameDone(GameSLoadGameDoneEvent),
        #[mpq(tag = 24)]
        ESessionCheat(GameSSessionCheatEvent),
        #[mpq(tag = 25)]
        ECommandManagerReset(GameSCommandManagerResetEvent),
        #[mpq(tag = 26)]
        EGameCheat(GameSGameCheatEvent),
        #[mpq(tag = 27)]
        ECmd(GameSCmdEvent),
        #[mpq(tag = 28)]
        ESelectionDelta(GameSSelectionDeltaEvent),
        #[mpq(tag = 29)]
        EControlGroupUpdate(GameSControlGroupUpdateEvent),
        #[mpq(tag = 30)]
        ESelectionSyncCheck(GameSSelectionSyncCheckEvent),
        #[mpq(tag = 31)]
        EResourceTrade(GameSResourceTradeEvent),
        #[mpq(tag = 32)]
        ETriggerChatMessage(GameSTriggerChatMessageEvent),
        #[mpq(tag = 33)]
        EAiCommunicate(GameSaiCommunicateEvent),
        #[mpq(tag = 34)]
        ESetAbsoluteGameSpeed(GameSSetAbsoluteGameSpeedEvent),
        #[mpq(tag = 35)]
        EAddAbsoluteGameSpeed(GameSAddAbsoluteGameSpeedEvent),
        #[mpq(tag = 36)]
        ETriggerPing(GameSTriggerPingEvent),
        #[mpq(tag = 37)]
        EBroadcastCheat(GameSBroadcastCheatEvent),
        #[mpq(tag = 38)]
        EAlliance(GameSAllianceEvent),
        #[mpq(tag = 39)]
        EUnitClick(GameSUnitClickEvent),
        #[mpq(tag = 40)]
        EUnitHighlight(GameSUnitHighlightEvent),
        #[mpq(tag = 41)]
        ETriggerReplySelected(GameSTriggerReplySelectedEvent),
        #[mpq(tag = 42)]
        EHijackReplaySession(GameSHijackReplaySessionEvent),
        #[mpq(tag = 43)]
        EHijackReplayGame(GameSHijackReplayGameEvent),
        #[mpq(tag = 44)]
        ETriggerSkipped(GameSTriggerSkippedEvent),
        #[mpq(tag = 45)]
        ETriggerSoundLengthQuery(GameSTriggerSoundLengthQueryEvent),
        #[mpq(tag = 46)]
        ETriggerSoundOffset(GameSTriggerSoundOffsetEvent),
        #[mpq(tag = 47)]
        ETriggerTransmissionOffset(GameSTriggerTransmissionOffsetEvent),
        #[mpq(tag = 48)]
        ETriggerTransmissionComplete(GameSTriggerTransmissionCompleteEvent),
        #[mpq(tag = 49)]
        ECameraUpdate(GameSCameraUpdateEvent),
        #[mpq(tag = 50)]
        ETriggerAbortMission(GameSTriggerAbortMissionEvent),
        #[mpq(tag = 51)]
        ETriggerPurchaseMade(GameSTriggerPurchaseMadeEvent),
        #[mpq(tag = 52)]
        ETriggerPurchaseExit(GameSTriggerPurchaseExitEvent),
        #[mpq(tag = 53)]
        ETriggerPlanetMissionLaunched(GameSTriggerPlanetMissionLaunchedEvent),
        #[mpq(tag = 54)]
        ETriggerPlanetPanelCanceled(GameSTriggerPlanetPanelCanceledEvent),
        #[mpq(tag = 55)]
        ETriggerDialogControl(GameSTriggerDialogControlEvent),
        #[mpq(tag = 56)]
        ETriggerSoundLengthSync(GameSTriggerSoundLengthSyncEvent),
        #[mpq(tag = 57)]
        ETriggerConversationSkipped(GameSTriggerConversationSkippedEvent),
        #[mpq(tag = 58)]
        ETriggerMouseClicked(GameSTriggerMouseClickedEvent),
        #[mpq(tag = 59)]
        ETriggerMouseMoved(GameSTriggerMouseMovedEvent),
        #[mpq(tag = 60)]
        EAchievementAwarded(GameSAchievementAwardedEvent),
        #[mpq(tag = 61)]
        ETriggerHotkeyPressed(GameSTriggerHotkeyPressedEvent),
        #[mpq(tag = 62)]
        ETriggerTargetModeUpdate(GameSTriggerTargetModeUpdateEvent),
        #[mpq(tag = 63)]
        ETriggerPlanetPanelPanelReplay(GameSTriggerPlanetPanelReplayEvent),
        #[mpq(tag = 64)]
        ETriggerSoundtrackDone(GameSTriggerSoundtrackDoneEvent),
        #[mpq(tag = 65)]
        ETriggerPlanetMissionSelected(GameSTriggerPlanetMissionSelectedEvent),
        #[mpq(tag = 66)]
        ETriggerKeyPressed(GameSTriggerKeyPressedEvent),
        #[mpq(tag = 67)]
        ETriggerMovieFunction(GameSTriggerMovieFunctionEvent),
        #[mpq(tag = 68)]
        ETriggerPlanetPanelPanelBirthComplete(GameSTriggerPlanetPanelBirthCompleteEvent),
        #[mpq(tag = 69)]
        ETriggerPlanetPanelPanelDeathComplete(GameSTriggerPlanetPanelDeathCompleteEvent),
        #[mpq(tag = 70)]
        EResourceRequest(GameSResourceRequestEvent),
        #[mpq(tag = 71)]
        EResourceRequestFulfill(GameSResourceRequestFulfillEvent),
        #[mpq(tag = 72)]
        EResourceRequestCancel(GameSResourceRequestCancelEvent),
        #[mpq(tag = 73)]
        ETriggerResearchPanelExit(GameSTriggerResearchPanelExitEvent),
        #[mpq(tag = 74)]
        ETriggerResearchPanelPurchase(GameSTriggerResearchPanelPurchaseEvent),
        #[mpq(tag = 75)]
        ETriggerResearchPanelSelectionChanged(GameSTriggerResearchPanelSelectionChangedEvent),
        #[mpq(tag = 76)]
        ETriggerCommandError(GameSTriggerCommandErrorEvent),
        #[mpq(tag = 77)]
        ETriggerMercenaryPanelExit(GameSTriggerMercenaryPanelExitEvent),
        #[mpq(tag = 78)]
        ETriggerMercenaryPanelPurchase(GameSTriggerMercenaryPanelPurchaseEvent),
        #[mpq(tag = 79)]
        ETriggerMercenaryPanelSelectionChanged(GameSTriggerMercenaryPanelSelectionChangedEvent),
        #[mpq(tag = 80)]
        ETriggerVictoryPanelExit(GameSTriggerVictoryPanelExitEvent),
        #[mpq(tag = 81)]
        ETriggerBattleReportPanelExit(GameSTriggerBattleReportPanelExitEvent),
        #[mpq(tag = 82)]
        ETriggerBattleReportPanelPlayMission(GameSTriggerBattleReportPanelPlayMissionEvent),
        #[mpq(tag = 83)]
        ETriggerBattleReportPanelPlayScene(GameSTriggerBattleReportPanelPlaySceneEvent),
        #[mpq(tag = 84)]
        ETriggerBattleReportSelectionChanged(GameSTriggerBattleReportPanelSelectionChangedEvent),
        #[mpq(tag = 85)]
        ETriggerVictoryPanelPlayMissionAgain(GameSTriggerVictoryPanelPlayMissionAgainEvent),
        #[mpq(tag = 86)]
        ETriggerMovieStarted(GameSTriggerMovieStartedEvent),
        #[mpq(tag = 87)]
        ETriggerMovieFinished(GameSTriggerMovieFinishedEvent),
        #[mpq(tag = 88)]
        EDecrementGameTimeRemaining(GameSDecrementGameTimeRemainingEvent),
        #[mpq(tag = 89)]
        ETriggerPortraitLoaded(GameSTriggerPortraitLoadedEvent),
        #[mpq(tag = 90)]
        ETriggerQueryDialogDismissed(GameSTriggerCustomDialogDismissedEvent),
        #[mpq(tag = 91)]
        ETriggerGameMenuItemSelected(GameSTriggerGameMenuItemSelectedEvent),
        #[mpq(tag = 92)]
        ETriggerMouseWheel(GameSTriggerMouseWheelEvent),
        #[mpq(tag = 93)]
        ETriggerPurchasePanelSelectedPurchaseItemChanged(
            GameSTriggerPurchasePanelSelectedPurchaseItemChangedEvent,
        ),
        #[mpq(tag = 94)]
        ETriggerPurchasePanelSelectedPurchaseCategoryChanged(
            GameSTriggerPurchasePanelSelectedPurchaseCategoryChangedEvent,
        ),
        #[mpq(tag = 95)]
        ETriggerButtonPressed(GameSTriggerButtonPressedEvent),
        #[mpq(tag = 96)]
        ETriggerGameCreditsFinished(GameSTriggerGameCreditsFinishedEvent),
        #[mpq(tag = 97)]
        ETriggerCutsceneBookmarkFired(GameSTriggerCutsceneBookmarkFiredEvent),
        #[mpq(tag = 98)]
        ETriggerCutsceneEndSceneFired(GameSTriggerCutsceneEndSceneFiredEvent),
        #[mpq(tag = 99)]
        ETriggerCutsceneConversationLine(GameSTriggerCutsceneConversationLineEvent),
        #[mpq(tag = 100)]
        ETriggerCutsceneConversationLineMissing(GameSTriggerCutsceneConversationLineMissingEvent),
        #[mpq(tag = 101)]
        EGameUserLeave(GameSGameUserLeaveEvent),
        #[mpq(tag = 102)]
        EGameUserJoin(GameSGameUserJoinEvent),
        #[mpq(tag = 103)]
        ECommandManagerState(GameSCommandManagerStateEvent),
        #[mpq(tag = 104)]
        ECmdUpdateTargetPoint(GameSCmdUpdateTargetPointEvent),
        #[mpq(tag = 105)]
        ECmdUpdateTargetUnit(GameSCmdUpdateTargetUnitEvent),
        #[mpq(tag = 106)]
        ETriggerAnimLengthQueryByName(GameSTriggerAnimLengthQueryByNameEvent),
        #[mpq(tag = 107)]
        ETriggerAnimLengthQueryByProps(GameSTriggerAnimLengthQueryByPropsEvent),
        #[mpq(tag = 108)]
        ETriggerAnimOffset(GameSTriggerAnimOffsetEvent),
        #[mpq(tag = 109)]
        ECatalogModify(GameSCatalogModifyEvent),
        #[mpq(tag = 110)]
        EHeroTalentTreeSelected(GameSHeroTalentTreeSelectedEvent),
        #[mpq(tag = 111)]
        ETriggerProfilerLoggingFinished(GameSTriggerProfilerLoggingFinishedEvent),
        #[mpq(tag = 112)]
        EHeroTalentTreeSelectionPanelToggled(GameSHeroTalentTreeSelectionPanelToggledEvent),
        #[mpq(tag = 113)]
        EMuteUserChanged(GameSMuteChatEvent),
        #[mpq(tag = 115)]
        EConvertToReplaySession(GameSConvertToReplaySessionEvent),
        #[mpq(tag = 116)]
        ESetSyncLoadingTime(GameSSetSyncLoadingTimeEvent),
        #[mpq(tag = 117)]
        ESetSyncPlayingTime(GameSSetSyncPlayingTimeEvent),
        #[mpq(tag = 118)]
        EPeerSetSyncLoadingTime(GameSPeerSetSyncLoadingTimeEvent),
        #[mpq(tag = 119)]
        EPeerSetSyncPlayingTime(GameSPeerSetSyncPlayingTimeEvent),
    }

    impl GameEEventId {
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

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSCmdEvent {
        #[mpq(bits = 26, offset = 0)]
        pub m_cmd_flags: i64,
        pub m_abil: Option<GameSCmdAbil>,
        pub m_data: GameSCmdData,
        #[mpq(bits = 32, offset = 1)]
        pub m_sequence: i64,
        pub m_other_unit: Option<GameTUnitTag>,
        pub m_unit_group: Option<Uint32>,
    }
}
