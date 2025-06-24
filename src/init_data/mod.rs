//! Decodes the initData

#[cfg(feature = "dep_arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::S2ProtocolError;
use nom_mpq::MPQ;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct InitData {
    /// The lobby state
    pub sync_lobby_state: LobbySyncState,
    /// The sha256 of the file, prevent duplicates and provide a unique identifier
    pub sha256: String,
    /// The file name
    pub file_name: String,
    /// The version of the protocol
    pub version: u32,
}

impl InitData {
    /// Calls the per-protocol parser for the InitData and sets the metadadata.
    #[tracing::instrument(level = "error", skip(file_contents, mpq))]
    pub fn new(file_name: &str, mpq: &MPQ, file_contents: &[u8]) -> Result<Self, S2ProtocolError> {
        let init_data = crate::versions::read_init_data(file_name, mpq, file_contents)?;
        Ok(init_data.set_metadata(file_name, file_contents))
    }

    #[tracing::instrument(level = "debug", skip(file_contents))]
    pub fn set_metadata(mut self, file_name: &str, file_contents: &[u8]) -> Self {
        // TODO: We need to find a way to trim the sha just like git rev-parse,
        // just so that we reduce the size of the files, but providing unicity.
        // This also means that generating of the files must be done at the same time
        // or we will have different sha references (different length) in different files.
        // OOOOOR HOW ABOUT one table of <SEQUENTIAL>:<SHA256>?
        self.sha256 = sha256::digest(file_contents);
        self.file_name = file_name.to_string();
        self
    }

    #[tracing::instrument(level = "error")]
    pub fn set_version(&mut self, version: u32) {
        self.version = version;
    }

    /// Trim the sha to n characters, this is done to reduce the size of the generated files
    /// because every sha is unique and we don't need the full sha to identify the file.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn trim_sha(&mut self, n: usize) {
        self.sha256 = self.sha256.chars().take(n).collect()
    }
}

impl TryFrom<PathBuf> for InitData {
    type Error = S2ProtocolError;

    /// Reads the file from the path and calls the per-protocol parser for the InitData.
    /// Sets the metadata if successful.
    /// Returns an error if the file cannot be read or the parser fails.
    #[tracing::instrument(level = "debug")]
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let file_contents = crate::read_file(&path)?;
        let (_input, mpq) = crate::parser::parse(&file_contents)?;
        match Self::new(path.to_str().unwrap_or_default(), &mpq, &file_contents) {
            Ok(init_data) => {
                Ok(init_data.set_metadata(path.to_str().unwrap_or_default(), &file_contents))
            }
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct LobbySyncState {
    pub user_initial_data: Vec<UserInitialData>,
    pub game_description: GameDescription,
    pub lobby_state: LobbyState,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UserInitialData {
    pub name: String,
    pub clan_tag: Option<String>,
    pub clan_logo: Option<Vec<u8>>,
    pub highest_league: Option<u8>,
    pub combined_race_levels: Option<u32>,
    pub random_seed: u32,
    pub race_preference: Option<i64>,
    pub team_preference: Option<u8>,
    pub test_map: bool,
    pub test_auto: bool,
    pub examine: bool,
    pub custom_interface: bool,
    pub test_type: u32,
    pub observe: u8,
    pub hero: String,
    pub skin: String,
    pub mount: String,
    pub toon_handle: String,
    pub scaled_rating: Option<i32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameDescription {
    pub random_value: u32,
    pub game_cache_name: String,
    pub game_options: GameOptions,
    pub game_speed: u8,
    pub game_type: GameType,
    pub max_users: i64,
    pub max_observers: i64,
    pub max_players: i64,
    pub max_teams: i64,
    pub max_colors: i64,
    pub max_races: i64,
    pub max_controls: i64,
    pub map_size_x: u8,
    pub map_size_y: u8,
    pub map_file_sync_checksum: u32,
    pub map_file_name: String,
    pub map_author_name: String,
    pub mod_file_sync_checksum: u32,
    pub slot_descriptions: Vec<SlotDescription>,
    pub default_difficulty: i64,
    pub default_ai_build: i64,
    pub cache_handles: Vec<Vec<u8>>,
    pub has_extension_mod: bool,
    pub has_non_blizzard_extension_mod: bool,
    pub is_blizzard_map: bool,
    pub is_premade_ffa: bool,
    pub is_coop_mode: bool,
    pub is_realtime_mode: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameOptions {
    pub lock_teams: bool,
    pub teams_together: bool,
    pub advanced_shared_control: bool,
    pub random_races: bool,
    pub battle_net: bool,
    pub amm: bool,
    pub competitive: bool,
    pub practice: bool,
    pub cooperative: bool,
    pub no_victory_or_defeat: bool,
    pub hero_duplicates_allowed: bool,
    pub fog: OptionFog,
    pub observers: OptionObservers,
    pub user_difficulty: OptionUserDifficulty,
    pub client_debug_flags: i64,
    pub build_coach_enabled: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "dep_arrow", arrow_field(type = "dense"))]
pub enum GameType {
    EMelee,
    EFreeForAll,
    EUseSettings,
    EOneOnOne,
    ETwoTeamPlay,
    EThreeTeamPlay,
    EFourTeamPlay,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "dep_arrow", arrow_field(type = "dense"))]
pub struct SlotDescription {
    pub allowed_colors: i64,
    pub allowed_races: i64,
    pub allowed_difficulty: i64,
    pub allowed_controls: Vec<u8>,
    pub allowed_observe_types: i64,
    pub allowed_ai_builds: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "dep_arrow", arrow_field(type = "dense"))]
pub enum OptionFog {
    EDefault,
    EHideTerrain,
    EMapExplored,
    EAlwaysVisible,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "dep_arrow", arrow_field(type = "dense"))]
pub enum OptionObservers {
    ENone,
    EOnJoin,
    EOnJoinAndDefeat,
    ERefereesOnJoin,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "dep_arrow", arrow_field(type = "dense"))]
pub enum OptionUserDifficulty {
    ENone,
    EGlobal,
    EIndividual,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "dep_arrow", arrow_field(type = "dense"))]
pub struct LobbyState {
    pub phase: GamePhase,
    pub max_users: i64,
    pub max_observers: i64,
    pub slots: Vec<LobbySlot>,
    pub random_seed: u32,
    pub host_user_id: Option<i64>,
    pub is_single_player: bool,
    pub picked_map_tag: u8,
    pub game_duration: u32,
    pub default_difficulty: i64,
    pub default_ai_build: i64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "dep_arrow", arrow_field(type = "dense"))]
pub enum GamePhase {
    EInitializing,
    ELobby,
    EClosed,
    ELoading,
    EPlaying,
    EGameover,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "dep_arrow", arrow_field(type = "dense"))]
pub struct LobbySlot {
    pub control: i64,
    pub user_id: Option<i64>,
    pub team_id: i64,
    pub color_pref: Option<i64>,
    pub race_pref: Option<i64>,
    pub difficulty: i64,
    pub ai_build: i64,
    pub handicap: u32,
    pub observe: u8,
    pub logo_index: u32,
    pub hero: String,
    pub skin: String,
    pub mount: String,
    pub artifacts: Vec<String>,
    pub working_set_slot_id: Option<u8>,
    pub rewards: Vec<u32>,
    pub toon_handle: String,
    pub licenses: Vec<u32>,
    pub tandem_leader_id: Option<i64>,
    pub commander: String,
    pub commander_level: u32,
    pub has_silence_penalty: bool,
    pub tandem_id: Option<i64>,
    pub commander_mastery_level: u32,
    pub commander_mastery_talents: Vec<u32>,
    pub trophy_id: u32,
    pub reward_overrides: Vec<RewardOverride>,
    pub brutal_plus_difficulty: u32,
    pub retry_mutation_indexes: Vec<u32>,
    pub a_c_enemy_race: u32,
    pub a_c_enemy_wave_type: u32,
    pub selected_commander_prestige: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "dep_arrow", arrow_field(type = "dense"))]
pub struct RewardOverride {
    pub key: u32,
    pub rewards: Vec<u32>,
}

// Tests
#[cfg(test)]
mod tests {
    use crate::versions::protocol87702::bit_packed::GameSLobbySlot;

    #[test_log::test]
    fn basic_tests() {
        // Part of the init data that was not being parsed properly.
        let data: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x03, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        ];
        // In the example snippet, three bits had already been consumed by a previous region.
        // Take three bits from the head, since we read in reverse we set to 3.
        let tail = (&data[..], 5usize);
        let (tail, handicap) = GameSLobbySlot::parse_m_handicap(tail).unwrap();
        assert_eq!(handicap.value.value, 100);
        let (tail, observe) = GameSLobbySlot::parse_m_observe(tail).unwrap();
        assert_eq!(u8::from(observe), crate::common::OBSERVE_NONE);
        let (tail, logo) = GameSLobbySlot::parse_m_logo_index(tail).unwrap();
        assert_eq!(logo.value.value, 0);
        let (_tail, hero) = GameSLobbySlot::parse_m_hero(tail).unwrap();
        assert_eq!(hero.value, vec![0]);
    }
}
