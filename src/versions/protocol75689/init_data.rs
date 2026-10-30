//! The InitData of protocol 75689 (4.10), i.e. the replays of the Linux client used by AI Arena.
//!
//! Its lobby slots differ from the protocol87702 ones: `m_handicap` has 7 bits instead of 32, the
//! length of `m_licenses` has 13 bits instead of 16, and the five fields added in 4.11 (from
//! `m_brutal_plus_difficulty` to `m_selected_commander_prestige`) are not there. The rest of the
//! InitData is the same. The slots are parsed as they are in 75689 and converted to the
//! protocol87702 types, so that the conversions to [`InitData`] are shared.

use crate::bit_packed_decoder::*;
use crate::error::S2ProtocolError;
use crate::init_data::InitData;
use crate::versions::protocol87702::bit_packed::{
    GameCLicenseArray, GameCLobbySlotArray, GameCRetryMutationIndexArray, GameSLobbySlot,
    GameSLobbyState, GameSLobbySyncState, GameTHandicap, GameTLicense, ReplaySInitData, Uint32,
};
use crate::*;
use nom_mpq::MPQ;

/// The bits of `m_handicap` in 75689.
const HANDICAP_BITS: usize = 7;

/// The bits of the length of `m_licenses` in 75689.
const LICENSES_LENGTH_BITS: usize = 13;

/// The bits of the length of `m_slots`, the same as in 87702.
const SLOTS_LENGTH_BITS: usize = 5;

/// Reads the InitData of a 75689 replay.
#[tracing::instrument(level = "debug", skip(mpq, file_contents))]
pub fn read_init_data(mpq: &MPQ, file_contents: &[u8]) -> Result<InitData, S2ProtocolError> {
    let (_, init_data_sector) =
        mpq.read_mpq_file_sector("replay.initData", false, file_contents)?;
    let (_, init_data) = parse_init_data((&init_data_sector, 0usize))?;
    init_data.try_into()
}

fn parse_init_data(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), ReplaySInitData> {
    let (tail, m_user_initial_data) = GameSLobbySyncState::parse_m_user_initial_data(input)?;
    let (tail, m_game_description) = GameSLobbySyncState::parse_m_game_description(tail)?;
    let (tail, m_lobby_state) = parse_lobby_state(tail)?;
    Ok((
        tail,
        ReplaySInitData {
            m_sync_lobby_state: GameSLobbySyncState {
                m_user_initial_data,
                m_game_description,
                m_lobby_state,
            },
        },
    ))
}

fn parse_lobby_state(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameSLobbyState> {
    let (tail, m_phase) = GameSLobbyState::parse_m_phase(input)?;
    let (tail, m_max_users) = GameSLobbyState::parse_m_max_users(tail)?;
    let (tail, m_max_observers) = GameSLobbyState::parse_m_max_observers(tail)?;
    let (mut tail, slots_length) = parse_packed_int(tail, 0, SLOTS_LENGTH_BITS)?;
    let mut slots = Vec::with_capacity(slots_length as usize);
    for _ in 0..slots_length {
        let (new_tail, slot) = parse_lobby_slot(tail)?;
        tail = new_tail;
        slots.push(slot);
    }
    let (tail, m_random_seed) = GameSLobbyState::parse_m_random_seed(tail)?;
    let (tail, m_host_user_id) = GameSLobbyState::parse_m_host_user_id(tail)?;
    let (tail, m_is_single_player) = GameSLobbyState::parse_m_is_single_player(tail)?;
    let (tail, m_picked_map_tag) = GameSLobbyState::parse_m_picked_map_tag(tail)?;
    let (tail, m_game_duration) = GameSLobbyState::parse_m_game_duration(tail)?;
    let (tail, m_default_difficulty) = GameSLobbyState::parse_m_default_difficulty(tail)?;
    let (tail, m_default_ai_build) = GameSLobbyState::parse_m_default_ai_build(tail)?;
    Ok((
        tail,
        GameSLobbyState {
            m_phase,
            m_max_users,
            m_max_observers,
            m_slots: GameCLobbySlotArray { value: slots },
            m_random_seed,
            m_host_user_id,
            m_is_single_player,
            m_picked_map_tag,
            m_game_duration,
            m_default_difficulty,
            m_default_ai_build,
        },
    ))
}

/// A lobby slot of 75689, the fields added in 4.11 are left empty.
fn parse_lobby_slot(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameSLobbySlot> {
    let (tail, m_control) = GameSLobbySlot::parse_m_control(input)?;
    let (tail, m_user_id) = GameSLobbySlot::parse_m_user_id(tail)?;
    let (tail, m_team_id) = GameSLobbySlot::parse_m_team_id(tail)?;
    let (tail, m_color_pref) = GameSLobbySlot::parse_m_color_pref(tail)?;
    let (tail, m_race_pref) = GameSLobbySlot::parse_m_race_pref(tail)?;
    let (tail, m_difficulty) = GameSLobbySlot::parse_m_difficulty(tail)?;
    let (tail, m_ai_build) = GameSLobbySlot::parse_m_ai_build(tail)?;
    let (tail, handicap) = parse_packed_int(tail, 0, HANDICAP_BITS)?;
    let (tail, m_observe) = GameSLobbySlot::parse_m_observe(tail)?;
    let (tail, m_logo_index) = GameSLobbySlot::parse_m_logo_index(tail)?;
    let (tail, m_hero) = GameSLobbySlot::parse_m_hero(tail)?;
    let (tail, m_skin) = GameSLobbySlot::parse_m_skin(tail)?;
    let (tail, m_mount) = GameSLobbySlot::parse_m_mount(tail)?;
    let (tail, m_artifacts) = GameSLobbySlot::parse_m_artifacts(tail)?;
    let (tail, m_working_set_slot_id) = GameSLobbySlot::parse_m_working_set_slot_id(tail)?;
    let (tail, m_rewards) = GameSLobbySlot::parse_m_rewards(tail)?;
    let (tail, m_toon_handle) = GameSLobbySlot::parse_m_toon_handle(tail)?;
    let (mut tail, licenses_length) = parse_packed_int(tail, 0, LICENSES_LENGTH_BITS)?;
    let mut licenses = Vec::with_capacity((licenses_length as usize).min(64));
    for _ in 0..licenses_length {
        let (new_tail, license) = GameTLicense::parse(tail)?;
        tail = new_tail;
        licenses.push(license);
    }
    let (tail, m_tandem_leader_id) = GameSLobbySlot::parse_m_tandem_leader_id(tail)?;
    let (tail, m_commander) = GameSLobbySlot::parse_m_commander(tail)?;
    let (tail, m_commander_level) = GameSLobbySlot::parse_m_commander_level(tail)?;
    let (tail, m_has_silence_penalty) = GameSLobbySlot::parse_m_has_silence_penalty(tail)?;
    let (tail, m_tandem_id) = GameSLobbySlot::parse_m_tandem_id(tail)?;
    let (tail, m_commander_mastery_level) = GameSLobbySlot::parse_m_commander_mastery_level(tail)?;
    let (tail, m_commander_mastery_talents) =
        GameSLobbySlot::parse_m_commander_mastery_talents(tail)?;
    let (tail, m_trophy_id) = GameSLobbySlot::parse_m_trophy_id(tail)?;
    let (tail, m_reward_overrides) = GameSLobbySlot::parse_m_reward_overrides(tail)?;
    Ok((
        tail,
        GameSLobbySlot {
            m_control,
            m_user_id,
            m_team_id,
            m_color_pref,
            m_race_pref,
            m_difficulty,
            m_ai_build,
            m_handicap: GameTHandicap {
                value: Uint32 { value: handicap },
            },
            m_observe,
            m_logo_index,
            m_hero,
            m_skin,
            m_mount,
            m_artifacts,
            m_working_set_slot_id,
            m_rewards,
            m_toon_handle,
            m_licenses: GameCLicenseArray { value: licenses },
            m_tandem_leader_id,
            m_commander,
            m_commander_level,
            m_has_silence_penalty,
            m_tandem_id,
            m_commander_mastery_level,
            m_commander_mastery_talents,
            m_trophy_id,
            m_reward_overrides,
            m_brutal_plus_difficulty: Uint32 { value: 0 },
            m_retry_mutation_indexes: GameCRetryMutationIndexArray { value: vec![] },
            m_a_c_enemy_race: Uint32 { value: 0 },
            m_a_c_enemy_wave_type: Uint32 { value: 0 },
            m_selected_commander_prestige: Uint32 { value: 0 },
        },
    ))
}
