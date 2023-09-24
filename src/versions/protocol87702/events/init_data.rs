//! Returns the players data at the initial state of the game.

use super::bit_packed::*;
use crate::common::*;
use crate::init_data::*;
use crate::*;
use nom_mpq::MPQ;

impl ReplaySInitData {
    /// Read the Init Data
    pub fn read_init_data(mpq: &MPQ, file_contents: &[u8]) -> Result<InitData, S2ProtocolError> {
        let (_, init_data_sector) =
            mpq.read_mpq_file_sector("replay.initData", false, file_contents)?;
        let (_, replay_sinitdata) = ReplaySInitData::parse((&init_data_sector, 0usize))?;
        replay_sinitdata
            .try_into()
            .map_err(|err: S2ProtocolError| err.into())
    }
}

impl TryFrom<ReplaySInitData> for InitData {
    type Error = S2ProtocolError;
    fn try_from(source: ReplaySInitData) -> Result<Self, Self::Error> {
        let sync_lobby_state = source.m_sync_lobby_state.try_into()?;
        Ok(InitData {
            sync_lobby_state,
            sha256: "".to_string(),
            file_name: "".to_string(),
        })
    }
}

impl TryFrom<GameSLobbySyncState> for LobbySyncState {
    type Error = S2ProtocolError;
    fn try_from(source: GameSLobbySyncState) -> Result<Self, Self::Error> {
        let mut user_initial_data = vec![];
        for data in source.m_user_initial_data.value {
            user_initial_data.push(data.try_into()?);
        }
        let game_description = source.m_game_description.try_into()?;
        let lobby_state = source.m_lobby_state.try_into()?;
        Ok(LobbySyncState {
            user_initial_data,
            game_description,
            lobby_state,
        })
    }
}

impl TryFrom<SUserInitialData> for UserInitialData {
    type Error = S2ProtocolError;
    #[tracing::instrument(level = "debug")]
    fn try_from(source: SUserInitialData) -> Result<Self, Self::Error> {
        // tranforms from SUserInitialData into UserInitialData casting into internal types
        let name = str::from_utf8(&source.m_name.value)?.to_string();
        let clan_tag = match source.m_clan_tag {
            Some(clan_tag) => Some(str::from_utf8(&clan_tag.value)?.to_string()),
            None => None,
        };
        // Found some instances of non valid UTF8
        let clan_logo = match source.m_clan_logo {
            Some(clan_logo) => Some(clan_logo.value),
            None => None,
        };
        let highest_league = source.m_highest_league.map(|v| v.into());
        let combined_race_levels = source.m_combined_race_levels.map(|v| v.into());
        let random_seed = source.m_random_seed.into();
        let race_preference = source.m_race_preference.m_race.map(|v| v.into());
        let team_preference = source.m_team_preference.m_team.map(|v| v.into());
        let test_map = source.m_test_map;
        let test_auto = source.m_test_auto;
        let examine = source.m_examine;
        let custom_interface = source.m_custom_interface;
        let test_type = source.m_test_type.into();
        let observe = source.m_observe.into();
        let hero = str::from_utf8(&source.m_hero.value)?.to_string();
        let skin = str::from_utf8(&source.m_skin.value)?.to_string();
        let mount = str::from_utf8(&source.m_mount.value)?.to_string();
        let toon_handle = str::from_utf8(&source.m_toon_handle.value)?.to_string();
        let scaled_rating = source.m_scaled_rating.map(|v| v.into());
        Ok(Self {
            name,
            clan_tag,
            clan_logo,
            highest_league,
            combined_race_levels,
            random_seed,
            race_preference,
            team_preference,
            test_map,
            test_auto,
            examine,
            custom_interface,
            test_type,
            observe,
            hero,
            skin,
            mount,
            toon_handle,
            scaled_rating,
        })
    }
}

impl From<TRaceId> for i64 {
    fn from(value: TRaceId) -> Self {
        value.value
    }
}

impl From<super::bit_packed::GameEGameType> for crate::init_data::GameType {
    fn from(value: super::bit_packed::GameEGameType) -> Self {
        match value {
            super::bit_packed::GameEGameType::EMelee => Self::EMelee,
            super::bit_packed::GameEGameType::EFreeForAll => Self::EFreeForAll,
            super::bit_packed::GameEGameType::EUseSettings => Self::EUseSettings,
            super::bit_packed::GameEGameType::EOneOnOne => Self::EOneOnOne,
            super::bit_packed::GameEGameType::ETwoTeamPlay => Self::ETwoTeamPlay,
            super::bit_packed::GameEGameType::EThreeTeamPlay => Self::EThreeTeamPlay,
            super::bit_packed::GameEGameType::EFourTeamPlay => Self::EFourTeamPlay,
        }
    }
}

impl TryFrom<super::bit_packed::GameSGameDescription> for crate::init_data::GameDescription {
    type Error = S2ProtocolError;
    #[tracing::instrument(level = "debug")]
    fn try_from(source: super::bit_packed::GameSGameDescription) -> Result<Self, Self::Error> {
        let random_value = source.m_random_value.into();
        let game_cache_name = str::from_utf8(&source.m_game_cache_name.value)?.to_string();
        let game_options = source.m_game_options.into();
        let game_speed = source.m_game_speed.into();
        let game_type = source.m_game_type.into();
        let max_users = source.m_max_users.value;
        let max_observers = source.m_max_observers.value;
        let max_players = source.m_max_players.value;
        let max_teams = source.m_max_teams.value;
        let max_colors = source.m_max_colors.value;
        let max_races = source.m_max_races.value;
        let max_controls = source.m_max_controls.value;
        let map_size_x = source.m_map_size_x.value as u8;
        let map_size_y = source.m_map_size_y.value as u8;
        let map_file_sync_checksum = source.m_map_file_sync_checksum.value.into();
        let map_file_name = str::from_utf8(&source.m_map_file_name.value)?.to_string();
        let map_author_name = str::from_utf8(&source.m_map_author_name.value)?.to_string();
        let mod_file_sync_checksum = source.m_mod_file_sync_checksum.value.into();
        let slot_descriptions = source
            .m_slot_descriptions
            .value
            .into_iter()
            .map(|v| v.into())
            .collect();
        let default_difficulty = source.m_default_difficulty.value;
        let default_ai_build = source.m_default_ai_build.value;
        let cache_handles = source
            .m_cache_handles
            .value
            .into_iter()
            .map(|v| v.value.clone())
            .collect();
        let has_extension_mod = source.m_has_extension_mod;
        let has_non_blizzard_extension_mod = source.m_has_non_blizzard_extension_mod;
        let is_blizzard_map = source.m_is_blizzard_map;
        let is_premade_ffa = source.m_is_premade_ffa;
        let is_coop_mode = source.m_is_coop_mode;
        let is_realtime_mode = source.m_is_realtime_mode;

        Ok(Self {
            random_value,
            game_cache_name,
            game_options,
            game_speed,
            game_type,
            max_users,
            max_observers,
            max_players,
            max_teams,
            max_colors,
            max_races,
            max_controls,
            map_size_x,
            map_size_y,
            map_file_sync_checksum,
            map_file_name,
            map_author_name,
            mod_file_sync_checksum,
            slot_descriptions,
            default_difficulty,
            default_ai_build,
            cache_handles,
            has_extension_mod,
            has_non_blizzard_extension_mod,
            is_blizzard_map,
            is_premade_ffa,
            is_coop_mode,
            is_realtime_mode,
        })
    }
}

impl From<super::bit_packed::GameSSlotDescription> for crate::init_data::SlotDescription {
    fn from(value: super::bit_packed::GameSSlotDescription) -> Self {
        let allowed_colors = value.m_allowed_colors.value;
        let allowed_races = value.m_allowed_races.value;
        let allowed_difficulty = value.m_allowed_difficulty.value;
        let allowed_controls = value.m_allowed_controls.value.clone();
        let allowed_observe_types = value.m_allowed_observe_types.value;
        let allowed_ai_builds = value.m_allowed_ai_builds.value.clone();
        Self {
            allowed_colors,
            allowed_races,
            allowed_difficulty,
            allowed_controls,
            allowed_observe_types,
            allowed_ai_builds,
        }
    }
}

impl From<super::bit_packed::GameEOptionFog> for crate::init_data::OptionFog {
    fn from(value: super::bit_packed::GameEOptionFog) -> Self {
        match value {
            super::bit_packed::GameEOptionFog::EDefault => Self::EDefault,
            super::bit_packed::GameEOptionFog::EHideTerrain => Self::EHideTerrain,
            super::bit_packed::GameEOptionFog::EMapExplored => Self::EMapExplored,
            super::bit_packed::GameEOptionFog::EAlwaysVisible => Self::EAlwaysVisible,
        }
    }
}

impl From<super::bit_packed::GameEOptionObservers> for crate::init_data::OptionObservers {
    fn from(value: super::bit_packed::GameEOptionObservers) -> Self {
        match value {
            super::bit_packed::GameEOptionObservers::ENone => Self::ENone,
            super::bit_packed::GameEOptionObservers::EOnJoin => Self::EOnJoin,
            super::bit_packed::GameEOptionObservers::EOnJoinAndDefeat => Self::EOnJoinAndDefeat,
            super::bit_packed::GameEOptionObservers::ERefereesOnJoin => Self::ERefereesOnJoin,
        }
    }
}

impl From<super::bit_packed::GameEOptionUserDifficulty> for crate::init_data::OptionUserDifficulty {
    fn from(value: super::bit_packed::GameEOptionUserDifficulty) -> Self {
        match value {
            super::bit_packed::GameEOptionUserDifficulty::ENone => Self::ENone,
            super::bit_packed::GameEOptionUserDifficulty::EGlobal => Self::EGlobal,
            super::bit_packed::GameEOptionUserDifficulty::EIndividual => Self::EIndividual,
        }
    }
}

impl From<super::bit_packed::GameSGameOptions> for GameOptions {
    fn from(value: super::bit_packed::GameSGameOptions) -> Self {
        let lock_teams = value.m_lock_teams;
        let teams_together = value.m_teams_together;
        let advanced_shared_control = value.m_advanced_shared_control;
        let random_races = value.m_random_races;
        let battle_net = value.m_battle_net;
        let amm = value.m_amm;
        let competitive = value.m_competitive;
        let practice = value.m_practice;
        let cooperative = value.m_cooperative;
        let no_victory_or_defeat = value.m_no_victory_or_defeat;
        let hero_duplicates_allowed = value.m_hero_duplicates_allowed;
        let fog = value.m_fog.into();
        let observers = value.m_observers.into();
        let user_difficulty = value.m_user_difficulty.into();
        let client_debug_flags = value.m_client_debug_flags.value;
        let build_coach_enabled = value.m_build_coach_enabled;
        Self {
            lock_teams,
            teams_together,
            advanced_shared_control,
            random_races,
            battle_net,
            amm,
            competitive,
            practice,
            cooperative,
            no_victory_or_defeat,
            hero_duplicates_allowed,
            fog,
            observers,
            user_difficulty,
            client_debug_flags,
            build_coach_enabled,
        }
    }
}

impl From<super::bit_packed::GameEGameSpeed> for GameSpeed {
    fn from(value: super::bit_packed::GameEGameSpeed) -> Self {
        match value {
            GameEGameSpeed::ESlower => Self::ESlower,
            GameEGameSpeed::ESlow => Self::ESlow,
            GameEGameSpeed::ENormal => Self::ENormal,
            GameEGameSpeed::EFast => Self::EFast,
            GameEGameSpeed::EFaster => Self::EFaster,
        }
    }
}

impl TryFrom<super::bit_packed::GameSLobbyState> for LobbyState {
    type Error = S2ProtocolError;
    fn try_from(value: super::bit_packed::GameSLobbyState) -> Result<Self, Self::Error> {
        let phase = value.m_phase.into();
        let max_users = value.m_max_users.value;
        let max_observers = value.m_max_observers.value;
        let slots = value
            .m_slots
            .value
            .into_iter()
            .map(|v| v.try_into())
            .collect::<Result<Vec<_>, _>>()?;
        let random_seed = value.m_random_seed.into();
        let host_user_id = value.m_host_user_id.map(|v| v.value);
        let is_single_player = value.m_is_single_player;
        let picked_map_tag = value.m_picked_map_tag.value as u8;
        let game_duration = value.m_game_duration.value as u32;
        let default_difficulty = value.m_default_difficulty.value;
        let default_ai_build = value.m_default_ai_build.value;
        Ok(Self {
            phase,
            max_users,
            max_observers,
            slots,
            random_seed,
            host_user_id,
            is_single_player,
            picked_map_tag,
            game_duration,
            default_difficulty,
            default_ai_build,
        })
    }
}

impl From<super::bit_packed::GameEPhase> for GamePhase {
    fn from(value: super::bit_packed::GameEPhase) -> Self {
        match value {
            GameEPhase::EInitializing => Self::EInitializing,
            GameEPhase::ELobby => Self::ELobby,
            GameEPhase::EClosed => Self::EClosed,
            GameEPhase::ELoading => Self::ELoading,
            GameEPhase::EPlaying => Self::EPlaying,
            GameEPhase::EGameover => Self::EGameover,
        }
    }
}

impl TryFrom<super::bit_packed::GameSLobbySlot> for LobbySlot {
    type Error = S2ProtocolError;
    #[tracing::instrument(level = "debug")]
    fn try_from(value: super::bit_packed::GameSLobbySlot) -> Result<Self, Self::Error> {
        let control = value.m_control.value;
        let user_id = value.m_user_id.map(|v| v.value);
        let team_id = value.m_team_id.value;
        let color_pref = value.m_color_pref.m_color.map(|v| v.value);
        let race_pref = value.m_race_pref.m_race.map(|v| v.value);
        let difficulty = value.m_difficulty.value;
        let ai_build = value.m_ai_build.value;
        let handicap = value.m_handicap.value.into();
        let observe = value.m_observe.into();
        let logo_index = value.m_logo_index.value.into();
        let hero = str::from_utf8(&value.m_hero.value)?.to_string();
        let skin = str::from_utf8(&value.m_skin.value)?.to_string();
        let mount = str::from_utf8(&value.m_mount.value)?.to_string();
        let artifacts = value
            .m_artifacts
            .value
            .into_iter()
            .map(|v| str::from_utf8(&v.value).map(|v| v.to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        let working_set_slot_id = value.m_working_set_slot_id.map(|v| v.into());
        let rewards = value
            .m_rewards
            .value
            .into_iter()
            .map(|v| v.value.into())
            .collect();
        let toon_handle = str::from_utf8(&value.m_toon_handle.value)?.to_string();
        let licenses = value
            .m_licenses
            .value
            .into_iter()
            .map(|v| v.value.into())
            .collect();
        let tandem_leader_id = value.m_tandem_leader_id.map(|v| v.value);
        let commander = str::from_utf8(&value.m_commander.value)?.to_string();
        let commander_level = value.m_commander_level.into();
        let has_silence_penalty = value.m_has_silence_penalty;
        let tandem_id = value.m_tandem_id.map(|v| v.value);
        let commander_mastery_level = value.m_commander_mastery_level.into();
        let commander_mastery_talents = value
            .m_commander_mastery_talents
            .value
            .into_iter()
            .map(|v| v.into())
            .collect();
        let trophy_id = value.m_trophy_id.into();
        let reward_overrides = value
            .m_reward_overrides
            .value
            .into_iter()
            .map(|v| v.into())
            .collect();
        let brutal_plus_difficulty = value.m_brutal_plus_difficulty.into();
        let retry_mutation_indexes = value
            .m_retry_mutation_indexes
            .value
            .into_iter()
            .map(|v| v.into())
            .collect();
        let a_c_enemy_race = value.m_a_c_enemy_race.into();
        let a_c_enemy_wave_type = value.m_a_c_enemy_wave_type.into();
        let selected_commander_prestige = value.m_selected_commander_prestige.into();

        Ok(Self {
            control,
            user_id,
            team_id,
            color_pref,
            race_pref,
            difficulty,
            ai_build,
            handicap,
            observe,
            logo_index,
            hero,
            skin,
            mount,
            artifacts,
            working_set_slot_id,
            rewards,
            toon_handle,
            licenses,
            tandem_leader_id,
            commander,
            commander_level,
            has_silence_penalty,
            tandem_id,
            commander_mastery_level,
            commander_mastery_talents,
            trophy_id,
            reward_overrides,
            brutal_plus_difficulty,
            retry_mutation_indexes,
            a_c_enemy_race,
            a_c_enemy_wave_type,
            selected_commander_prestige,
        })
    }
}

impl From<super::bit_packed::GameCRewardOverride> for RewardOverride {
    fn from(value: super::bit_packed::GameCRewardOverride) -> Self {
        let key = value.m_key.into();
        let rewards = value
            .m_rewards
            .value
            .into_iter()
            .map(|v| v.value.into())
            .collect();
        Self { key, rewards }
    }
}
