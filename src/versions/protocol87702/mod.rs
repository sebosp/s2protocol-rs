pub mod events;
pub use events::*;
pub mod byte_aligned {
    //! Generated code from source: ../s2protocol/json/protocol87702.json
    use crate::*;

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    #[mpq(choice)]
    pub(crate) enum SVarUint32 {
        #[mpq(tag = 0)]
        MUint6(u8),
        #[mpq(tag = 1)]
        MUint14(u32),
        #[mpq(tag = 2)]
        MUint22(u32),
        #[mpq(tag = 3)]
        MUint32(u32),
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    #[mpq(int_enum)]
    pub(crate) enum EObserve {
        #[mpq(tag = 0)]
        ENone,
        #[mpq(tag = 1)]
        ESpectator,
        #[mpq(tag = 2)]
        EReferee,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub struct SVersion {
        #[mpq(tag = 0)]
        pub m_flags: u8,
        #[mpq(tag = 1)]
        pub m_major: u8,
        #[mpq(tag = 2)]
        pub m_minor: u8,
        #[mpq(tag = 3)]
        pub m_revision: u8,
        #[mpq(tag = 4)]
        pub m_build: u32,
        #[mpq(tag = 5)]
        pub m_base_build: u32,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub struct Smd5 {
        #[mpq(tag = 0)]
        pub m_data_deprecated: Option<Vec<u8>>,
        #[mpq(tag = 1, blob)]
        pub m_data: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    #[mpq(int_enum)]
    pub(crate) enum GameEGameSpeed {
        #[mpq(tag = 0)]
        ESlower,
        #[mpq(tag = 1)]
        ESlow,
        #[mpq(tag = 2)]
        ENormal,
        #[mpq(tag = 3)]
        EFast,
        #[mpq(tag = 4)]
        EFaster,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct GameSThumbnail {
        #[mpq(tag = 0, blob)]
        pub m_file: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct GameSColor {
        #[mpq(tag = 0)]
        pub m_a: u8,
        #[mpq(tag = 1)]
        pub m_r: u8,
        #[mpq(tag = 2)]
        pub m_g: u8,
        #[mpq(tag = 3)]
        pub m_b: u8,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    #[mpq(int_enum)]
    pub(crate) enum GameEResultDetails {
        #[mpq(tag = 0)]
        EUndecided,
        #[mpq(tag = 1)]
        EWin,
        #[mpq(tag = 2)]
        ELoss,
        #[mpq(tag = 3)]
        ETie,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct GameSToonNameDetails {
        #[mpq(tag = 0)]
        pub m_region: u8,
        #[mpq(tag = 1, fourcc)]
        pub m_program_id: u32,
        #[mpq(tag = 2)]
        pub m_realm: u32,
        #[mpq(tag = 3, blob, default)]
        pub m_name: Vec<u8>,
        #[mpq(tag = 4)]
        pub m_id: u64,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct GameSPlayerDetails {
        #[mpq(tag = 0, blob)]
        pub m_name: Vec<u8>,
        #[mpq(tag = 1)]
        pub m_toon: GameSToonNameDetails,
        #[mpq(tag = 2, blob)]
        pub m_race: Vec<u8>,
        #[mpq(tag = 3)]
        pub m_color: GameSColor,
        #[mpq(tag = 4)]
        pub m_control: u8,
        #[mpq(tag = 5)]
        pub m_team_id: u8,
        #[mpq(tag = 6)]
        pub m_handicap: u32,
        #[mpq(tag = 7)]
        pub m_observe: EObserve,
        #[mpq(tag = 8)]
        pub m_result: GameEResultDetails,
        #[mpq(tag = 9)]
        pub m_working_set_slot_id: Option<u8>,
        #[mpq(tag = 10, blob)]
        pub m_hero: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct GameSDetails {
        #[mpq(tag = 0)]
        pub m_player_list: Option<Vec<GameSPlayerDetails>>,
        #[mpq(tag = 1, blob)]
        pub m_title: Vec<u8>,
        #[mpq(tag = 2, blob)]
        pub m_difficulty: Vec<u8>,
        #[mpq(tag = 3)]
        pub m_thumbnail: GameSThumbnail,
        #[mpq(tag = 4)]
        pub m_is_blizzard_map: bool,
        #[mpq(tag = 5)]
        pub m_time_utc: i64,
        #[mpq(tag = 6)]
        pub m_time_local_offset: i64,
        #[mpq(tag = 16)]
        pub m_restart_as_transition_map: Option<bool>,
        #[mpq(tag = 17)]
        pub m_disable_recover_game: bool,
        #[mpq(tag = 7, blob)]
        pub m_description: Vec<u8>,
        #[mpq(tag = 8, blob)]
        pub m_image_file_path: Vec<u8>,
        #[mpq(tag = 15)]
        pub m_campaign_index: u8,
        #[mpq(tag = 9, blob)]
        pub m_map_file_name: Vec<u8>,
        #[mpq(tag = 10, blob)]
        pub m_cache_handles: Option<Vec<Vec<u8>>>,
        #[mpq(tag = 11)]
        pub m_mini_save: bool,
        #[mpq(tag = 12)]
        pub m_game_speed: GameEGameSpeed,
        #[mpq(tag = 13)]
        pub m_default_difficulty: u32,
        #[mpq(tag = 14, blob)]
        pub m_mod_paths: Option<Vec<Vec<u8>>>,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub struct ReplaySHeader {
        #[mpq(tag = 0, blob)]
        pub m_signature: Vec<u8>,
        #[mpq(tag = 1)]
        pub m_version: SVersion,
        #[mpq(tag = 2)]
        pub m_type: u8,
        #[mpq(tag = 3)]
        pub m_elapsed_game_loops: u32,
        #[mpq(tag = 4)]
        pub m_use_scaled_time: bool,
        #[mpq(tag = 5)]
        pub m_ngdp_root_key: Smd5,
        #[mpq(tag = 6)]
        pub m_data_build_num: u32,
        #[mpq(tag = 7)]
        pub m_replay_compatibility_hash: Smd5,
        #[mpq(tag = 8)]
        pub m_ngdp_root_key_is_dev_data: bool,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    #[mpq(int_enum)]
    pub(crate) enum ReplayTrackerEEventId {
        #[mpq(tag = 0)]
        EPlayerStats(ReplayTrackerSPlayerStatsEvent),
        #[mpq(tag = 1)]
        EUnitBorn(ReplayTrackerSUnitBornEvent),
        #[mpq(tag = 2)]
        EUnitDied(ReplayTrackerSUnitDiedEvent),
        #[mpq(tag = 3)]
        EUnitOwnerChange(ReplayTrackerSUnitOwnerChangeEvent),
        #[mpq(tag = 4)]
        EUnitTypeChange(ReplayTrackerSUnitTypeChangeEvent),
        #[mpq(tag = 5)]
        EUpgrade(ReplayTrackerSUpgradeEvent),
        #[mpq(tag = 6)]
        EUnitInit(ReplayTrackerSUnitInitEvent),
        #[mpq(tag = 7)]
        EUnitDone(ReplayTrackerSUnitDoneEvent),
        #[mpq(tag = 8)]
        EUnitPosition(ReplayTrackerSUnitPositionsEvent),
        #[mpq(tag = 9)]
        EPlayerSetup(ReplayTrackerSPlayerSetupEvent),
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct ReplayTrackerSPlayerStats {
        #[mpq(tag = 0)]
        pub m_score_value_minerals_current: i32,
        #[mpq(tag = 1)]
        pub m_score_value_vespene_current: i32,
        #[mpq(tag = 2)]
        pub m_score_value_minerals_collection_rate: i32,
        #[mpq(tag = 3)]
        pub m_score_value_vespene_collection_rate: i32,
        #[mpq(tag = 4)]
        pub m_score_value_workers_active_count: i32,
        #[mpq(tag = 5)]
        pub m_score_value_minerals_used_in_progress_army: i32,
        #[mpq(tag = 6)]
        pub m_score_value_minerals_used_in_progress_economy: i32,
        #[mpq(tag = 7)]
        pub m_score_value_minerals_used_in_progress_technology: i32,
        #[mpq(tag = 8)]
        pub m_score_value_vespene_used_in_progress_army: i32,
        #[mpq(tag = 9)]
        pub m_score_value_vespene_used_in_progress_economy: i32,
        #[mpq(tag = 10)]
        pub m_score_value_vespene_used_in_progress_technology: i32,
        #[mpq(tag = 11)]
        pub m_score_value_minerals_used_current_army: i32,
        #[mpq(tag = 12)]
        pub m_score_value_minerals_used_current_economy: i32,
        #[mpq(tag = 13)]
        pub m_score_value_minerals_used_current_technology: i32,
        #[mpq(tag = 14)]
        pub m_score_value_vespene_used_current_army: i32,
        #[mpq(tag = 15)]
        pub m_score_value_vespene_used_current_economy: i32,
        #[mpq(tag = 16)]
        pub m_score_value_vespene_used_current_technology: i32,
        #[mpq(tag = 17)]
        pub m_score_value_minerals_lost_army: i32,
        #[mpq(tag = 18)]
        pub m_score_value_minerals_lost_economy: i32,
        #[mpq(tag = 19)]
        pub m_score_value_minerals_lost_technology: i32,
        #[mpq(tag = 20)]
        pub m_score_value_vespene_lost_army: i32,
        #[mpq(tag = 21)]
        pub m_score_value_vespene_lost_economy: i32,
        #[mpq(tag = 22)]
        pub m_score_value_vespene_lost_technology: i32,
        #[mpq(tag = 23)]
        pub m_score_value_minerals_killed_army: i32,
        #[mpq(tag = 24)]
        pub m_score_value_minerals_killed_economy: i32,
        #[mpq(tag = 25)]
        pub m_score_value_minerals_killed_technology: i32,
        #[mpq(tag = 26)]
        pub m_score_value_vespene_killed_army: i32,
        #[mpq(tag = 27)]
        pub m_score_value_vespene_killed_economy: i32,
        #[mpq(tag = 28)]
        pub m_score_value_vespene_killed_technology: i32,
        #[mpq(tag = 29)]
        pub m_score_value_food_used: i32,
        #[mpq(tag = 30)]
        pub m_score_value_food_made: i32,
        #[mpq(tag = 31)]
        pub m_score_value_minerals_used_active_forces: i32,
        #[mpq(tag = 32)]
        pub m_score_value_vespene_used_active_forces: i32,
        #[mpq(tag = 33)]
        pub m_score_value_minerals_friendly_fire_army: i32,
        #[mpq(tag = 34)]
        pub m_score_value_minerals_friendly_fire_economy: i32,
        #[mpq(tag = 35)]
        pub m_score_value_minerals_friendly_fire_technology: i32,
        #[mpq(tag = 36)]
        pub m_score_value_vespene_friendly_fire_army: i32,
        #[mpq(tag = 37)]
        pub m_score_value_vespene_friendly_fire_economy: i32,
        #[mpq(tag = 38)]
        pub m_score_value_vespene_friendly_fire_technology: i32,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct ReplayTrackerSPlayerStatsEvent {
        #[mpq(tag = 0)]
        pub m_player_id: u8,
        #[mpq(tag = 1)]
        pub m_stats: ReplayTrackerSPlayerStats,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct ReplayTrackerSUnitBornEvent {
        #[mpq(tag = 0)]
        pub m_unit_tag_index: u32,
        #[mpq(tag = 1)]
        pub m_unit_tag_recycle: u32,
        #[mpq(tag = 2, blob)]
        pub m_unit_type_name: Vec<u8>,
        #[mpq(tag = 3)]
        pub m_control_player_id: u8,
        #[mpq(tag = 4)]
        pub m_upkeep_player_id: u8,
        #[mpq(tag = 5)]
        pub m_x: u8,
        #[mpq(tag = 6)]
        pub m_y: u8,
        #[mpq(tag = 7)]
        pub m_creator_unit_tag_index: Option<u32>,
        #[mpq(tag = 8)]
        pub m_creator_unit_tag_recycle: Option<u32>,
        #[mpq(tag = 9, blob)]
        pub m_creator_ability_name: Option<Vec<u8>>,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct ReplayTrackerSUnitDiedEvent {
        #[mpq(tag = 0)]
        pub m_unit_tag_index: u32,
        #[mpq(tag = 1)]
        pub m_unit_tag_recycle: u32,
        #[mpq(tag = 2)]
        pub m_killer_player_id: Option<u8>,
        #[mpq(tag = 3)]
        pub m_x: u8,
        #[mpq(tag = 4)]
        pub m_y: u8,
        #[mpq(tag = 5)]
        pub m_killer_unit_tag_index: Option<u32>,
        #[mpq(tag = 6)]
        pub m_killer_unit_tag_recycle: Option<u32>,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct ReplayTrackerSUnitOwnerChangeEvent {
        #[mpq(tag = 0)]
        pub m_unit_tag_index: u32,
        #[mpq(tag = 1)]
        pub m_unit_tag_recycle: u32,
        #[mpq(tag = 2)]
        pub m_control_player_id: u8,
        #[mpq(tag = 3)]
        pub m_upkeep_player_id: u8,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct ReplayTrackerSUnitTypeChangeEvent {
        #[mpq(tag = 0)]
        pub m_unit_tag_index: u32,
        #[mpq(tag = 1)]
        pub m_unit_tag_recycle: u32,
        #[mpq(tag = 2, blob)]
        pub m_unit_type_name: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct ReplayTrackerSUpgradeEvent {
        #[mpq(tag = 0)]
        pub m_player_id: u8,
        #[mpq(tag = 1, blob)]
        pub m_upgrade_type_name: Vec<u8>,
        #[mpq(tag = 2)]
        pub m_count: i32,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct ReplayTrackerSUnitInitEvent {
        #[mpq(tag = 0)]
        pub m_unit_tag_index: u32,
        #[mpq(tag = 1)]
        pub m_unit_tag_recycle: u32,
        #[mpq(tag = 2, blob)]
        pub m_unit_type_name: Vec<u8>,
        #[mpq(tag = 3)]
        pub m_control_player_id: u8,
        #[mpq(tag = 4)]
        pub m_upkeep_player_id: u8,
        #[mpq(tag = 5)]
        pub m_x: u8,
        #[mpq(tag = 6)]
        pub m_y: u8,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct ReplayTrackerSUnitDoneEvent {
        #[mpq(tag = 0)]
        pub m_unit_tag_index: u32,
        #[mpq(tag = 1)]
        pub m_unit_tag_recycle: u32,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct ReplayTrackerSUnitPositionsEvent {
        #[mpq(tag = 0)]
        pub m_first_unit_index: u32,
        #[mpq(tag = 1)]
        pub m_items: Vec<i32>,
    }

    #[derive(Debug, PartialEq, Clone, MPQByteAlignedDeserialize, MPQByteAlignedSerialize)]
    pub(crate) struct ReplayTrackerSPlayerSetupEvent {
        #[mpq(tag = 0)]
        pub m_player_id: u8,
        #[mpq(tag = 1)]
        pub m_type: u32,
        #[mpq(tag = 2)]
        pub m_user_id: Option<u32>,
        #[mpq(tag = 3)]
        pub m_slot_id: Option<u32>,
    }
}
pub mod bit_packed {
    //! Generated code from source: ../s2protocol/json/protocol87702.json
    use crate::*;

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CFilePath {
        #[mpq(string, len_bits = 11)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct TRaceId {
        #[mpq(bits = 8, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct TRaceCount {
        #[mpq(bits = 8, offset = 1)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct TRacePreference {
        pub m_race: Option<TRaceId>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CAllowedRaces {
        #[mpq(bitarray, len_bits = 8)]
        pub value: BitArray,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct Int8 {
        #[mpq(bits = 8, offset = -128)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct Int16 {
        #[mpq(bits = 16, offset = -32768)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct Int32 {
        #[mpq(bits = 32, offset = -2147483648)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub struct Int64 {
        #[mpq(bits = 64, offset = -9223372036854775808)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub struct Uint8 {
        #[mpq(bits = 8, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct Uint16 {
        #[mpq(bits = 16, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub struct Uint32 {
        #[mpq(bits = 32, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub struct Uint64 {
        #[mpq(bits = 64, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct Uint6 {
        #[mpq(bits = 6, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct Uint14 {
        #[mpq(bits = 14, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct Uint22 {
        #[mpq(bits = 22, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(choice, bits = 2)]
    pub(crate) enum SVarUint32 {
        #[mpq(tag = 0)]
        MUint6(Uint6),
        #[mpq(tag = 1)]
        MUint14(Uint14),
        #[mpq(tag = 2)]
        MUint22(Uint22),
        #[mpq(tag = 3)]
        MUint32(Uint32),
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct TUserId {
        #[mpq(bits = 4, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct TUserCount {
        #[mpq(bits = 5, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CCacheHandle {
        #[mpq(blob, bytes = 40)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CUserName {
        #[mpq(string, len_bits = 8)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CClanTag {
        #[mpq(string, len_bits = 8)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CHeroHandle {
        #[mpq(string, len_bits = 9)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CSkinHandle {
        #[mpq(string, len_bits = 9)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CMountHandle {
        #[mpq(string, len_bits = 9)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CArtifactHandle {
        #[mpq(string, len_bits = 9)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CToonHandle {
        #[mpq(string, len_bits = 7)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CCommanderHandle {
        #[mpq(string, len_bits = 9)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 2)]
    pub(crate) enum EObserve {
        #[mpq(tag = 0)]
        ENone,
        #[mpq(tag = 1)]
        ESpectator,
        #[mpq(tag = 2)]
        EReferee,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CAllowedObserveTypes {
        #[mpq(bitarray, len_bits = 2)]
        pub value: BitArray,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct TTeamPreference {
        pub m_team: Option<Uint8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct SUserInitialData {
        pub m_name: CUserName,
        pub m_clan_tag: Option<CClanTag>,
        pub m_clan_logo: Option<CCacheHandle>,
        pub m_highest_league: Option<Uint8>,
        pub m_combined_race_levels: Option<Uint32>,
        pub m_random_seed: Uint32,
        pub m_race_preference: TRacePreference,
        pub m_team_preference: TTeamPreference,
        pub m_test_map: bool,
        pub m_test_auto: bool,
        pub m_examine: bool,
        pub m_custom_interface: bool,
        pub m_test_type: Uint32,
        pub m_observe: EObserve,
        pub m_hero: CHeroHandle,
        pub m_skin: CSkinHandle,
        pub m_mount: CMountHandle,
        pub m_toon_handle: CToonHandle,
        pub m_scaled_rating: Option<Int32>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct CUserInitialDataArray {
        #[mpq(len_bits = 5)]
        pub value: Vec<SUserInitialData>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 4)]
    pub(crate) enum ELeaveReason {
        #[mpq(tag = 0)]
        EUserLeft,
        #[mpq(tag = 1)]
        EUserDropped,
        #[mpq(tag = 2)]
        EUserBanned,
        #[mpq(tag = 3)]
        EUserVictory,
        #[mpq(tag = 4)]
        EUserDefeat,
        #[mpq(tag = 5)]
        EUserTied,
        #[mpq(tag = 6)]
        EUserDesynced,
        #[mpq(tag = 7)]
        EUserOutOfTime,
        #[mpq(tag = 8)]
        EWeWereUnresponsive,
        #[mpq(tag = 9)]
        EWeContinuedAlone,
        #[mpq(tag = 10)]
        EReplayDesynced,
        #[mpq(tag = 11)]
        EUserTimeout,
        #[mpq(tag = 12)]
        EUserDisconnected,
        #[mpq(tag = 13)]
        EUnrecoverable,
        #[mpq(tag = 14)]
        EUserCatchupDesynced,
        #[mpq(tag = 15)]
        ETakeCommandDropped,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 2)]
    pub(crate) enum EReconnectStatus {
        #[mpq(tag = 0)]
        EConnected,
        #[mpq(tag = 1)]
        EReconnected,
        #[mpq(tag = 2)]
        EDisconnected,
        #[mpq(tag = 3)]
        EUnrecoverable,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub struct SVersion {
        pub m_flags: Uint8,
        pub m_major: Uint8,
        pub m_minor: Uint8,
        pub m_revision: Uint8,
        pub m_build: Uint32,
        pub m_base_build: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTColorId {
        #[mpq(bits = 5, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTColorCount {
        #[mpq(bits = 6, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameTColorPreference {
        pub m_color: Option<GameTColorId>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCAllowedColors {
        #[mpq(bitarray, len_bits = 6)]
        pub value: BitArray,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 1)]
    pub(crate) enum GameEHijackMethod {
        #[mpq(tag = 0)]
        ERecover,
        #[mpq(tag = 1)]
        ETakeCommand,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTQueryId {
        pub value: Uint16,
    }

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

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSCmdAbil {
        pub m_abil_link: GameTAbilLink,
        #[mpq(bits = 5, offset = 0)]
        pub m_abil_cmd_index: i64,
        pub m_abil_cmd_data: Option<Uint8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSCmdDataTargetUnit {
        pub m_target_unit_flags: Uint16,
        pub m_timer: Uint8,
        pub m_tag: GameTUnitTag,
        pub m_snapshot_unit_link: GameTUnitLink,
        pub m_snapshot_control_player_id: Option<GameTPlayerId>,
        pub m_snapshot_upkeep_player_id: Option<GameTPlayerId>,
        pub m_snapshot_point: GameSMapCoord3D,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(choice, bits = 2)]
    pub(crate) enum GameSCmdData {
        #[mpq(tag = 0)]
        None(()),
        #[mpq(tag = 1)]
        TargetPoint(GameSMapCoord3D),
        #[mpq(tag = 2)]
        TargetUnit(GameSCmdDataTargetUnit),
        #[mpq(tag = 3)]
        Data(Uint32),
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSetLobbySlotEvent {
        pub m_slot_id: GameTLobbySlotId,
        pub m_slot_change: GameSLobbySlotChange,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSDropUserEvent {
        pub m_drop_session_user_id: TUserId,
        pub m_reason: ELeaveReason,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSStartGameEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSDropOurselvesEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSBankFileEvent {
        // Blizzard: a blob of 7 length bits, byte aligned (was read as 7 bits of unaligned bytes).
        #[mpq(string, len_bits = 7)]
        pub m_name: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSBankSectionEvent {
        // Blizzard: a blob of 6 length bits, byte aligned (was read as 6 bits of unaligned bytes).
        #[mpq(string, len_bits = 6)]
        pub m_name: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSBankKeyEvent {
        // Blizzard: a blob of 6 length bits, byte aligned (was read as 6 bits of unaligned bytes).
        #[mpq(string, len_bits = 6)]
        pub m_name: Vec<u8>,
        pub m_type: Uint32,
        // Blizzard: a blob of 7 length bits, byte aligned (was read as 5 bits of unaligned bytes).
        #[mpq(string, len_bits = 7)]
        pub m_data: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSBankValueEvent {
        pub m_type: Uint32,
        // Blizzard: a blob of 6 length bits, byte aligned (was read as 6 bits of unaligned bytes).
        #[mpq(string, len_bits = 6)]
        pub m_name: Vec<u8>,
        // Blizzard: a blob of 12 length bits, byte aligned (was read as 10 bits of unaligned bytes).
        #[mpq(string, len_bits = 12)]
        pub m_data: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSBankSignatureEvent {
        #[mpq(len_bits = 5)]
        pub m_signature: Vec<Uint8>,
        pub m_toon_handle: CToonHandle,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSUserOptionsEvent {
        pub m_game_fully_downloaded: bool,
        pub m_development_cheats_enabled: bool,
        pub m_test_cheats_enabled: bool,
        pub m_multiplayer_cheats_enabled: bool,
        pub m_sync_checksumming_enabled: bool,
        pub m_is_map_to_map_transition: bool,
        pub m_debug_pause_enabled: bool,
        pub m_use_galaxy_asserts: bool,
        pub m_platform_mac: bool,
        pub m_camera_follow: bool,
        pub m_base_build_num: Uint32,
        pub m_build_num: Uint32,
        pub m_version_flags: Uint32,
        // Blizzard: a blob of 9 length bits, byte aligned (was read as 7 bits of unaligned bytes).
        #[mpq(string, len_bits = 9)]
        pub m_hotkey_profile: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSPickMapTagEvent {
        pub m_picked_map_tag: Uint8,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSUserFinishedLoadingEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSUserFinishedLoadingSyncEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSetGameDurationEvent {
        pub m_game_duration: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTurnEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSCameraSaveEvent {
        #[mpq(bits = 3, offset = 0)]
        pub m_which: i64,
        pub m_target: GameSPointMini,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSPauseGameEvent {
        pub m_pause_type_index: Uint8,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSUnpauseGameEvent {
        pub m_pause_type_index: Uint8,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSingleStepGameEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSetGameSpeedEvent {
        pub m_speed: GameEGameSpeed,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSAddGameSpeedEvent {
        pub m_delta: Int8,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSReplayJumpEvent {
        pub m_replay_jump_game_loop: Option<Uint32>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSaveGameEvent {
        pub m_file_name: CFilePath,
        pub m_automatic: bool,
        pub m_overwrite: bool,
        // Blizzard: a blob of 8 length bits, byte aligned (was read as 6 bits of unaligned bytes).
        #[mpq(string, len_bits = 8)]
        pub m_name: Vec<u8>,
        // Blizzard: a blob of 10 length bits, byte aligned (was read as 9 bits of unaligned bytes).
        #[mpq(string, len_bits = 10)]
        pub m_description: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSaveGameDoneEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSLoadGameDoneEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSCheatEventData {
        pub m_point: GameSPoint,
        pub m_time: Int32,
        pub m_verb: GameCCheatString,
        pub m_arguments: GameCCheatString,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSessionCheatEvent {
        pub m_data: GameSCheatEventData,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSCommandManagerResetEvent {
        pub m_sequence: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSGameCheatEvent {
        pub m_data: GameSCheatEventData,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSCmdEvent {
        #[mpq(bits = 27, offset = 0)]
        pub m_cmd_flags: i64,
        pub m_abil: Option<GameSCmdAbil>,
        pub m_data: GameSCmdData,
        #[mpq(bits = 32, offset = 1)]
        pub m_sequence: i64,
        pub m_other_unit: Option<GameTUnitTag>,
        pub m_unit_group: Option<Uint32>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSelectionDeltaEvent {
        pub m_control_group_id: GameTControlGroupId,
        pub m_delta: GameSSelectionDelta,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSControlGroupUpdateEvent {
        pub m_control_group_index: GameTControlGroupIndex,
        pub m_control_group_update: GameEControlGroupUpdate,
        pub m_mask: GameSSelectionMask,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSelectionSyncCheckEvent {
        pub m_control_group_id: GameTControlGroupId,
        pub m_selection_sync_data: GameSSelectionSyncData,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSResourceTradeEvent {
        pub m_recipient_id: GameTPlayerId,
        #[mpq(len_bits = 3)]
        pub m_resources: Vec<Int32>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerChatMessageEvent {
        pub m_chat_message: GameCTriggerChatMessageString,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSaiCommunicateEvent {
        pub m_beacon: Int8,
        pub m_ally: Int8,
        pub m_flags: Int8,
        pub m_build: Int8,
        pub m_target_unit_tag: GameTUnitTag,
        pub m_target_unit_snapshot_unit_link: GameTUnitLink,
        pub m_target_unit_snapshot_upkeep_player_id: Int8,
        pub m_target_unit_snapshot_control_player_id: Int8,
        pub m_target_point: GameSPoint3,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSetAbsoluteGameSpeedEvent {
        pub m_speed: GameEGameSpeed,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSAddAbsoluteGameSpeedEvent {
        pub m_delta: Int8,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerPingEvent {
        pub m_point: GameSPoint,
        pub m_unit: GameTUnitTag,
        pub m_unit_link: GameTUnitLink,
        pub m_unit_control_player_id: Option<GameTPlayerId>,
        pub m_unit_upkeep_player_id: Option<GameTPlayerId>,
        pub m_unit_position: GameSMapCoord3D,
        pub m_unit_is_under_construction: bool,
        pub m_pinged_minimap: bool,
        pub m_option: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSBroadcastCheatEvent {
        pub m_verb: GameCCheatString,
        pub m_arguments: GameCCheatString,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSAllianceEvent {
        pub m_alliance: Uint32,
        pub m_control: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSUnitClickEvent {
        pub m_unit_tag: GameTUnitTag,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSUnitHighlightEvent {
        pub m_unit_tag: GameTUnitTag,
        pub m_flags: Uint8,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerReplySelectedEvent {
        pub m_conversation_id: Int32,
        pub m_reply_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSHijackReplaySessionUserInfo {
        pub m_session_user_id: TUserId,
        pub m_add_new_game_user: bool,
        pub m_game_user_id: TUserId,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSHijackReplaySessionEvent {
        #[mpq(len_bits = 5)]
        pub m_user_infos: Vec<GameSHijackReplaySessionUserInfo>,
        pub m_method: GameEHijackMethod,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSHijackReplayGameUserInfo {
        pub m_game_user_id: TUserId,
        pub m_observe: EObserve,
        pub m_name: CUserName,
        pub m_toon_handle: Option<CToonHandle>,
        pub m_clan_tag: Option<CClanTag>,
        pub m_clan_logo: Option<GameCCacheHandle>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSHijackReplayGameEvent {
        #[mpq(len_bits = 5)]
        pub m_user_infos: Vec<GameSHijackReplayGameUserInfo>,
        pub m_method: GameEHijackMethod,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerAbortMissionEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerPurchaseMadeEvent {
        pub m_purchase_item_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerPurchaseExitEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerPlanetMissionLaunchedEvent {
        pub m_difficulty_level: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerPlanetPanelCanceledEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerDialogControlEvent {
        pub m_control_id: Int32,
        pub m_event_type: Int32,
        pub m_event_data: MEventData,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(choice, bits = 3)]
    pub(crate) enum MEventData {
        #[mpq(tag = 0)]
        None(()),
        #[mpq(tag = 1)]
        Checked(bool),
        #[mpq(tag = 2)]
        ValueChanged(Uint32),
        #[mpq(tag = 3)]
        SelectionChanged(Int32),
        #[mpq(tag = 4)]
        TextChanged(GameCChatString),
        #[mpq(tag = 5)]
        MouseButton(Uint32),
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerSkippedEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerSoundLengthQueryEvent {
        pub m_sound_hash: Uint32,
        pub m_length: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerSoundLengthSyncEvent {
        pub m_sync_info: GameSSyncSoundLength,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerAnimLengthQueryByNameEvent {
        pub m_query_id: GameTQueryId,
        pub m_length_ms: Uint32,
        pub m_finish_game_loop: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerAnimLengthQueryByPropsEvent {
        pub m_query_id: GameTQueryId,
        pub m_length_ms: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerAnimOffsetEvent {
        pub m_anim_wait_query_id: GameTQueryId,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerSoundOffsetEvent {
        pub m_sound: GameTTriggerSoundTag,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerTransmissionOffsetEvent {
        pub m_transmission_id: Int32,
        pub m_thread: GameTTriggerThreadTag,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerTransmissionCompleteEvent {
        pub m_transmission_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSCameraUpdateEvent {
        pub m_target: Option<GameSPointMini>,
        pub m_distance: Option<GameTFixedMiniBitsUnsigned>,
        pub m_pitch: Option<GameTFixedMiniBitsUnsigned>,
        pub m_yaw: Option<GameTFixedMiniBitsUnsigned>,
        pub m_reason: Option<Int8>,
        pub m_follow: bool,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerConversationSkippedEvent {
        pub m_skip_type: GameEConversationSkip,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerMouseClickedEvent {
        pub m_button: Uint32,
        pub m_down: bool,
        pub m_pos_ui: GameSuiCoord,
        pub m_pos_world: GameSMapCoord3D,
        pub m_flags: Int8,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerMouseMovedEvent {
        pub m_pos_ui: GameSuiCoord,
        pub m_pos_world: GameSMapCoord3D,
        pub m_flags: Int8,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSAchievementAwardedEvent {
        pub m_achievement_link: GameTAchievementLink,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerHotkeyPressedEvent {
        pub m_hotkey: Uint32,
        pub m_down: bool,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerTargetModeUpdateEvent {
        pub m_abil_link: GameTAbilLink,
        #[mpq(bits = 5, offset = 0)]
        pub m_abil_cmd_index: i64,
        pub m_state: Int8,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerPlanetPanelReplayEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerSoundtrackDoneEvent {
        pub m_soundtrack: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerPlanetMissionSelectedEvent {
        pub m_planet_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerKeyPressedEvent {
        pub m_key: Int8,
        pub m_flags: Int8,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerPlanetPanelBirthCompleteEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerPlanetPanelDeathCompleteEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSResourceRequestEvent {
        #[mpq(len_bits = 3)]
        pub m_resources: Vec<Int32>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSResourceRequestFulfillEvent {
        pub m_fulfill_request_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSResourceRequestCancelEvent {
        pub m_cancel_request_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerResearchPanelExitEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerResearchPanelPurchaseEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerCommandErrorEvent {
        pub m_error: Int32,
        pub m_abil: Option<GameSCmdAbil>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerResearchPanelSelectionChangedEvent {
        pub m_research_item_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerMercenaryPanelExitEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerMercenaryPanelPurchaseEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerMercenaryPanelSelectionChangedEvent {
        pub m_mercenary_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerVictoryPanelExitEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerBattleReportPanelExitEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerBattleReportPanelPlayMissionEvent {
        pub m_battle_report_id: Int32,
        pub m_difficulty_level: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerBattleReportPanelPlaySceneEvent {
        pub m_battle_report_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerBattleReportPanelSelectionChangedEvent {
        pub m_battle_report_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerVictoryPanelPlayMissionAgainEvent {
        pub m_difficulty_level: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerMovieStartedEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerMovieFinishedEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSDecrementGameTimeRemainingEvent {
        pub m_decrement_seconds: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerPortraitLoadedEvent {
        pub m_portrait_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerMovieFunctionEvent {
        // Blizzard: a blob of 7 length bits, byte aligned (was read as 7 bits of unaligned bytes).
        #[mpq(string, len_bits = 7)]
        pub m_function_name: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerCustomDialogDismissedEvent {
        pub m_result: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerGameMenuItemSelectedEvent {
        pub m_game_menu_item_index: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerMouseWheelEvent {
        pub m_wheel_spin: GameTFixedMiniBitsSigned,
        pub m_flags: Int8,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerPurchasePanelSelectedPurchaseItemChangedEvent {
        pub m_purchase_item_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerPurchasePanelSelectedPurchaseCategoryChangedEvent {
        pub m_purchase_category_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerButtonPressedEvent {
        pub m_button: GameTButtonLink,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerGameCreditsFinishedEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerCutsceneBookmarkFiredEvent {
        pub m_cutscene_id: Int32,
        // Blizzard: a blob of 7 length bits, byte aligned (was read as 7 bits of unaligned bytes).
        #[mpq(string, len_bits = 7)]
        pub m_bookmark_name: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerCutsceneEndSceneFiredEvent {
        pub m_cutscene_id: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerCutsceneConversationLineEvent {
        pub m_cutscene_id: Int32,
        // Blizzard: a blob of 7 length bits, byte aligned (was read as 7 bits of unaligned bytes).
        #[mpq(string, len_bits = 7)]
        pub m_conversation_line: Vec<u8>,
        // Blizzard: a blob of 7 length bits, byte aligned (was read as 7 bits of unaligned bytes).
        #[mpq(string, len_bits = 7)]
        pub m_alt_conversation_line: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerCutsceneConversationLineMissingEvent {
        pub m_cutscene_id: Int32,
        // Blizzard: a blob of 7 length bits, byte aligned (was read as 7 bits of unaligned bytes).
        #[mpq(string, len_bits = 7)]
        pub m_conversation_line: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSGameUserLeaveEvent {
        pub m_leave_reason: ELeaveReason,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSGameUserJoinEvent {
        pub m_observe: EObserve,
        pub m_name: CUserName,
        pub m_toon_handle: Option<CToonHandle>,
        pub m_clan_tag: Option<CClanTag>,
        pub m_clan_logo: Option<GameCCacheHandle>,
        pub m_hijack: bool,
        pub m_hijack_clone_game_user_id: Option<TUserId>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 2)]
    pub(crate) enum GameECommandManagerState {
        #[mpq(tag = 0)]
        EFireDone,
        #[mpq(tag = 1)]
        EFireOnce,
        #[mpq(tag = 2)]
        EFireMany,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSCommandManagerStateEvent {
        pub m_state: GameECommandManagerState,
        // FIXED: the generated code read the value from `input`, before the presence bit
        #[mpq(bits = 32, offset = 1)]
        pub m_sequence: Option<i64>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSCmdUpdateTargetPointEvent {
        pub m_target: GameSMapCoord3D,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSCmdUpdateTargetUnitEvent {
        pub m_target: GameSCmdDataTargetUnit,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSCatalogModifyEvent {
        pub m_catalog: Uint8,
        pub m_entry: Uint16,
        // Blizzard: a blob of 8 length bits, byte aligned (was read as 9 bits of unaligned bytes).
        #[mpq(string, len_bits = 8)]
        pub m_field: Vec<u8>,
        // Blizzard: a blob of 8 length bits, byte aligned (was read as 9 bits of unaligned bytes).
        #[mpq(string, len_bits = 8)]
        pub m_value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSHeroTalentTreeSelectedEvent {
        pub m_index: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSTriggerProfilerLoggingFinishedEvent {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSHeroTalentTreeSelectionPanelToggledEvent {
        pub m_shown: bool,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSMuteChatEvent {
        pub m_target_user_id: TUserId,
        pub m_muted: bool,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSConvertToReplaySessionEvent {
        pub m_replay_jump_game_loop: Option<Int32>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSetSyncLoadingTimeEvent {
        pub m_sync_time: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSetSyncPlayingTimeEvent {
        pub m_sync_time: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSPeerSetSyncLoadingTimeEvent {
        pub m_sync_time: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSPeerSetSyncPlayingTimeEvent {
        pub m_sync_time: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 3)]
    pub(crate) enum GameEGameSpeed {
        #[mpq(tag = 0)]
        ESlower,
        #[mpq(tag = 1)]
        ESlow,
        #[mpq(tag = 2)]
        ENormal,
        #[mpq(tag = 3)]
        EFast,
        #[mpq(tag = 4)]
        EFaster,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 3)]
    pub(crate) enum GameEPhase {
        #[mpq(tag = 0)]
        EInitializing,
        #[mpq(tag = 1)]
        ELobby,
        #[mpq(tag = 2)]
        EClosed,
        #[mpq(tag = 3)]
        ELoading,
        #[mpq(tag = 4)]
        EPlaying,
        #[mpq(tag = 5)]
        EGameover,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 1)]
    pub(crate) enum GameEConversationSkip {
        #[mpq(tag = 0)]
        ESkipOneLine,
        #[mpq(tag = 1)]
        ESkipAllLines,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCCheatString {
        // Blizzard: 10 length bits, the generator had 11.
        #[mpq(string, len_bits = 10)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCTriggerChatMessageString {
        // Blizzard: 10 length bits, the generator had 11.
        #[mpq(string, len_bits = 10)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTAchievementLink {
        pub value: Uint16,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTButtonLink {
        pub value: Uint16,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTUnitLink {
        pub value: Uint16,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTUnitTag {
        pub value: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTTriggerThreadTag {
        pub value: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTTriggerSoundTag {
        pub value: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTAbilLink {
        pub value: Uint16,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTFixedBits {
        pub value: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTFixedMiniBitsUnsigned {
        pub value: Uint16,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTFixedMiniBitsSigned {
        pub value: Int16,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTPlayerLogoIndex {
        pub value: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTMapCoordFixedBits {
        #[mpq(bits = 20, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTuiCoordX {
        #[mpq(bits = 11, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTuiCoordY {
        #[mpq(bits = 11, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSPoint {
        pub x: GameTFixedBits,
        pub y: GameTFixedBits,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSPoint3 {
        pub x: GameTFixedBits,
        pub y: GameTFixedBits,
        pub z: GameTFixedBits,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSPointMini {
        pub x: GameTFixedMiniBitsUnsigned,
        pub y: GameTFixedMiniBitsUnsigned,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSMapCoord3D {
        pub x: GameTMapCoordFixedBits,
        pub y: GameTMapCoordFixedBits,
        pub z: GameTFixedBits,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSuiCoord {
        pub x: GameTuiCoordX,
        pub y: GameTuiCoordY,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTHandicap {
        pub value: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTDifficulty {
        #[mpq(bits = 6, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCAllowedDifficulty {
        #[mpq(bitarray, len_bits = 6)]
        pub value: BitArray,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTaiBuild {
        #[mpq(bits = 8, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCAllowedAiBuild {
        #[mpq(bitarray, len_bits = 8)]
        pub value: BitArray,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSyncSoundLength {
        #[mpq(len_bits = 7)]
        pub m_sound_hash: Vec<Uint32>,
        #[mpq(len_bits = 7)]
        pub m_length: Vec<Uint32>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 2)]
    pub(crate) enum GameEOptionFog {
        #[mpq(tag = 0)]
        EDefault,
        #[mpq(tag = 1)]
        EHideTerrain,
        #[mpq(tag = 2)]
        EMapExplored,
        #[mpq(tag = 3)]
        EAlwaysVisible,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 2)]
    pub(crate) enum GameEOptionObservers {
        #[mpq(tag = 0)]
        ENone,
        #[mpq(tag = 1)]
        EOnJoin,
        #[mpq(tag = 2)]
        EOnJoinAndDefeat,
        #[mpq(tag = 3)]
        ERefereesOnJoin,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 2)]
    pub(crate) enum GameEOptionUserDifficulty {
        #[mpq(tag = 0)]
        ENone,
        #[mpq(tag = 1)]
        EGlobal,
        #[mpq(tag = 2)]
        EIndividual,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameEClientDebugFlags {
        #[mpq(bits = 64, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSGameOptions {
        pub m_lock_teams: bool,
        pub m_teams_together: bool,
        pub m_advanced_shared_control: bool,
        pub m_random_races: bool,
        pub m_battle_net: bool,
        pub m_amm: bool,
        pub m_competitive: bool,
        pub m_practice: bool,
        pub m_cooperative: bool,
        pub m_no_victory_or_defeat: bool,
        pub m_hero_duplicates_allowed: bool,
        pub m_fog: GameEOptionFog,
        pub m_observers: GameEOptionObservers,
        pub m_user_difficulty: GameEOptionUserDifficulty,
        pub m_client_debug_flags: GameEClientDebugFlags,
        pub m_build_coach_enabled: bool,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 3)]
    pub(crate) enum GameEGameType {
        #[mpq(tag = 0)]
        EMelee,
        #[mpq(tag = 1)]
        EFreeForAll,
        #[mpq(tag = 2)]
        EUseSettings,
        #[mpq(tag = 3)]
        EOneOnOne,
        #[mpq(tag = 4)]
        ETwoTeamPlay,
        #[mpq(tag = 5)]
        EThreeTeamPlay,
        #[mpq(tag = 6)]
        EFourTeamPlay,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTControlId {
        #[mpq(bits = 8, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTControlCount {
        #[mpq(bits = 8, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCAllowedControls {
        #[mpq(bitarray, len_bits = 8)]
        pub value: BitArray,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSlotDescription {
        pub m_allowed_colors: GameCAllowedColors,
        pub m_allowed_races: CAllowedRaces,
        pub m_allowed_difficulty: GameCAllowedDifficulty,
        pub m_allowed_controls: GameCAllowedControls,
        pub m_allowed_observe_types: CAllowedObserveTypes,
        pub m_allowed_ai_builds: GameCAllowedAiBuild,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCCacheHandle {
        #[mpq(blob, bytes = 40)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCCacheHandles {
        #[mpq(len_bits = 6)]
        pub value: Vec<GameCCacheHandle>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCGameCacheName {
        // Blizzard: 10 length bits, the generator had 11.
        #[mpq(string, len_bits = 10)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCAuthorName {
        #[mpq(string, len_bits = 8)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameSSlotDescriptions {
        #[mpq(len_bits = 5)]
        pub value: Vec<GameSSlotDescription>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSGameDescription {
        pub m_random_value: Uint32,
        pub m_game_cache_name: GameCGameCacheName,
        pub m_game_options: GameSGameOptions,
        pub m_game_speed: GameEGameSpeed,
        pub m_game_type: GameEGameType,
        pub m_max_users: TUserCount,
        pub m_max_observers: TUserCount,
        pub m_max_players: GameTPlayerCount,
        pub m_max_teams: GameTTeamCount,
        pub m_max_colors: GameTColorCount,
        pub m_max_races: TRaceCount,
        pub m_max_controls: GameTControlCount,
        pub m_map_size_x: Uint8,
        pub m_map_size_y: Uint8,
        pub m_map_file_sync_checksum: GameTSyncChecksum,
        pub m_map_file_name: CFilePath,
        pub m_map_author_name: GameCAuthorName,
        pub m_mod_file_sync_checksum: GameTSyncChecksum,
        pub m_slot_descriptions: GameSSlotDescriptions,
        pub m_default_difficulty: GameTDifficulty,
        pub m_default_ai_build: GameTaiBuild,
        pub m_cache_handles: GameCCacheHandles,
        pub m_has_extension_mod: bool,
        pub m_has_non_blizzard_extension_mod: bool,
        pub m_is_blizzard_map: bool,
        pub m_is_premade_ffa: bool,
        pub m_is_coop_mode: bool,
        pub m_is_realtime_mode: bool,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTLobbySlotId {
        #[mpq(bits = 4, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCArtifactArray {
        #[mpq(len_bits = 4)]
        pub value: Vec<CArtifactHandle>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCCommanderMasteryTalentArray {
        #[mpq(len_bits = 3)]
        pub value: Vec<Uint32>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCRetryMutationIndexArray {
        #[mpq(len_bits = 3)]
        pub value: Vec<Uint32>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTReward {
        pub value: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCRewardArray {
        #[mpq(len_bits = 17)]
        pub value: Vec<GameTReward>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameCRewardOverride {
        pub m_key: Uint32,
        pub m_rewards: GameCRewardArray,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCRewardOverrideArray {
        #[mpq(len_bits = 17)]
        pub value: Vec<GameCRewardOverride>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTLicense {
        pub value: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCLicenseArray {
        #[mpq(len_bits = 16)]
        pub value: Vec<GameTLicense>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSLobbySlot {
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
        pub m_brutal_plus_difficulty: Uint32,
        pub m_retry_mutation_indexes: GameCRetryMutationIndexArray,
        pub m_a_c_enemy_race: Uint32,
        pub m_a_c_enemy_wave_type: Uint32,
        pub m_selected_commander_prestige: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCLobbySlotArray {
        #[mpq(len_bits = 5)]
        pub value: Vec<GameSLobbySlot>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(choice, bits = 5)]
    pub(crate) enum GameSLobbySlotChange {
        #[mpq(tag = 0)]
        MControl(GameTControlId),
        #[mpq(tag = 1)]
        MUserId(Option<TUserId>),
        #[mpq(tag = 2)]
        MTeamId(GameTTeamId),
        #[mpq(tag = 3)]
        MColorPref(GameTColorPreference),
        #[mpq(tag = 4)]
        MRacePref(TRacePreference),
        #[mpq(tag = 5)]
        MDifficulty(GameTDifficulty),
        #[mpq(tag = 6)]
        MAiBuild(GameTaiBuild),
        #[mpq(tag = 7)]
        MHandicap(GameTHandicap),
        #[mpq(tag = 8)]
        MObserve(EObserve),
        #[mpq(tag = 9)]
        MLogoIndex(GameTPlayerLogoIndex),
        #[mpq(tag = 10)]
        MHero(CHeroHandle),
        #[mpq(tag = 11)]
        MSkin(CSkinHandle),
        #[mpq(tag = 12)]
        MMount(CMountHandle),
        #[mpq(tag = 13)]
        MLicenses(GameCLicenseArray),
        #[mpq(tag = 14)]
        MTandemLeaderId(Option<TUserId>),
        #[mpq(tag = 15)]
        MCommander(CCommanderHandle),
        #[mpq(tag = 16)]
        MCommanderLevel(Uint32),
        #[mpq(tag = 17)]
        MHasSilencePenalty(bool),
        #[mpq(tag = 18)]
        MTandemId(Option<TUserId>),
        #[mpq(tag = 19)]
        MCommanderMasteryLevel(Uint32),
        #[mpq(tag = 20)]
        MBrutalPlusDifficulty(Uint32),
        #[mpq(tag = 21)]
        MRetryMutationIndexes(GameCRetryMutationIndexArray),
        #[mpq(tag = 22)]
        MACEnemyRace(Uint32),
        #[mpq(tag = 23)]
        MACEnemyWaveType(Uint32),
        #[mpq(tag = 24)]
        MSelectedCommanderPrestige(Uint32),
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSLobbyState {
        pub m_phase: GameEPhase,
        pub m_max_users: TUserCount,
        pub m_max_observers: TUserCount,
        pub m_slots: GameCLobbySlotArray,
        pub m_random_seed: Uint32,
        pub m_host_user_id: Option<TUserId>,
        pub m_is_single_player: bool,
        pub m_picked_map_tag: Uint8,
        pub m_game_duration: Uint32,
        pub m_default_difficulty: GameTDifficulty,
        pub m_default_ai_build: GameTaiBuild,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSLobbySyncState {
        pub m_user_initial_data: CUserInitialDataArray,
        pub m_game_description: GameSGameDescription,
        pub m_lobby_state: GameSLobbyState,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 3)]
    pub(crate) enum GameEMessageRecipient {
        #[mpq(tag = 0)]
        EAll,
        #[mpq(tag = 1)]
        EAllies,
        #[mpq(tag = 2)]
        EIndividual,
        #[mpq(tag = 3)]
        EBattlenet,
        #[mpq(tag = 4)]
        EObservers,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameCChatString {
        #[mpq(string, len_bits = 11)]
        pub value: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(choice, bits = 4)]
    pub(crate) enum GameEMessageId {
        #[mpq(tag = 0)]
        EChat(GameSChatMessage),
        #[mpq(tag = 1)]
        EPing(GameSPingMessage),
        #[mpq(tag = 2)]
        ELoadingProgress(GameSLoadingProgressMessage),
        #[mpq(tag = 3)]
        EServerPing(GameSServerPingMessage),
        #[mpq(tag = 4)]
        EReconnectNotify(GameSReconnectNotifyMessage),
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSChatMessage {
        pub m_recipient: GameEMessageRecipient,
        pub m_string: GameCChatString,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSPingMessage {
        pub m_recipient: GameEMessageRecipient,
        pub m_point: GameSPoint,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSLoadingProgressMessage {
        pub m_progress: Int32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSServerPingMessage {}

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSReconnectNotifyMessage {
        pub m_status: EReconnectStatus,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTPlayerId {
        #[mpq(bits = 4, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTPlayerCount {
        #[mpq(bits = 5, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(int_enum, bits = 3)]
    pub(crate) enum GameEControlGroupUpdate {
        #[mpq(tag = 0)]
        ESet,
        #[mpq(tag = 1)]
        EAppend,
        #[mpq(tag = 2)]
        ERecall,
        #[mpq(tag = 3)]
        EClear,
        #[mpq(tag = 4)]
        ESetAndSteal,
        #[mpq(tag = 5)]
        EAppendAndSteal,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTSelectionCount {
        #[mpq(bits = 9, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTSelectionIndex {
        #[mpq(bits = 9, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTSubgroupPriority {
        #[mpq(bits = 8, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTSubgroupCount {
        #[mpq(bits = 9, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTSubgroupIndex {
        #[mpq(bits = 9, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTControlGroupIndex {
        #[mpq(bits = 4, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTControlGroupId {
        #[mpq(bits = 4, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSelectionDeltaSubgroup {
        pub m_unit_link: GameTUnitLink,
        pub m_subgroup_priority: GameTSubgroupPriority,
        pub m_intra_subgroup_priority: GameTSubgroupPriority,
        pub m_count: GameTSelectionCount,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameSelectionIndexArrayType {
        #[mpq(len_bits = 9)]
        pub value: Vec<GameTSelectionIndex>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameSelectionMaskType {
        #[mpq(bitarray, len_bits = 9)]
        pub value: BitArray,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(choice, bits = 2)]
    pub(crate) enum GameSSelectionMask {
        #[mpq(tag = 0)]
        None(()),
        #[mpq(tag = 1)]
        Mask(GameSelectionMaskType),
        #[mpq(tag = 2)]
        OneIndices(GameSelectionIndexArrayType),
        #[mpq(tag = 3)]
        ZeroIndices(GameSelectionIndexArrayType),
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSelectionDelta {
        pub m_subgroup_index: GameTSubgroupIndex,
        pub m_remove_mask: GameSSelectionMask,
        #[mpq(len_bits = 9)]
        pub m_add_subgroups: Vec<GameSSelectionDeltaSubgroup>,
        #[mpq(len_bits = 9)]
        pub m_add_unit_tags: Vec<GameTUnitTag>,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct GameSSelectionSyncData {
        pub m_count: GameTSelectionCount,
        pub m_subgroup_count: GameTSubgroupCount,
        pub m_active_subgroup_index: GameTSubgroupIndex,
        pub m_unit_tags_checksum: GameTSyncChecksum,
        pub m_subgroup_indices_checksum: GameTSyncChecksum,
        pub m_subgroups_checksum: GameTSyncChecksum,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTSyncChecksum {
        pub value: Uint32,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTTeamId {
        #[mpq(bits = 4, offset = 0)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    #[mpq(transparent)]
    pub(crate) struct GameTTeamCount {
        #[mpq(bits = 4, offset = 1)]
        pub value: i64,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct ReplaySInitData {
        pub m_sync_lobby_state: GameSLobbySyncState,
    }

    #[derive(Debug, PartialEq, Clone, MPQBitPackedDeserialize, MPQBitPackedSerialize)]
    pub(crate) struct ReplaySGameUserId {
        #[mpq(bits = 5, offset = 0)]
        pub m_user_id: i64,
    }
}
