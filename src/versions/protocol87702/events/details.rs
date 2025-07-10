//! Details conversion into the version-agnostic structs
//!
//!

use super::byte_aligned::*;
use crate::details::{Color, Details, PlayerDetails, Thumbnail, ToonNameDetails};
use crate::*;
use nom_mpq::MPQ;

impl GameSDetails {
    /// Read the Details MPQ sector.
    pub fn read_details(mpq: &MPQ, file_contents: &[u8]) -> Result<Details, S2ProtocolError> {
        let (_, details_sector) =
            mpq.read_mpq_file_sector("replay.details", false, file_contents)?;
        let (_, game_sdetails) = GameSDetails::parse(&details_sector)?;
        game_sdetails.try_into()
    }
}

impl TryFrom<GameSDetails> for Details {
    type Error = S2ProtocolError;
    fn try_from(source: GameSDetails) -> Result<Self, Self::Error> {
        let mut player_list = vec![];
        // Into the Optional
        if let Some(m_player_list) = source.m_player_list {
            // Into the Vec<>
            for player_detail in m_player_list {
                player_list.push(player_detail.try_into()?);
            }
        }
        let mut cache_handles = vec![];
        if let Some(m_cache_handles) = source.m_cache_handles {
            for cache_handle in m_cache_handles {
                let mut cache_str = String::from("0x");
                for cache_byte in cache_handle {
                    cache_str.push_str(&format!("{cache_byte:x}"));
                }
                cache_handles.push(cache_str);
            }
        }
        let mut mod_paths = vec![];
        if let Some(m_mod_paths) = source.m_mod_paths {
            for mod_path in m_mod_paths {
                mod_paths.push(str::from_utf8(&mod_path)?.to_string());
            }
        }
        Ok(Details {
            player_list,
            title: str::from_utf8(&source.m_title)?.to_string(),
            difficulty: str::from_utf8(&source.m_difficulty)?.to_string(),
            thumbnail: source.m_thumbnail.try_into()?,
            is_blizzard_map: source.m_is_blizzard_map,
            time_utc: source.m_time_utc,
            time_local_offset: source.m_time_local_offset,
            restart_as_transition_map: source.m_restart_as_transition_map,
            disable_recover_game: source.m_disable_recover_game,
            description: str::from_utf8(&source.m_description)?.to_string(),
            image_file_path: str::from_utf8(&source.m_image_file_path)?.to_string(),
            campaign_index: source.m_campaign_index,
            map_file_name: str::from_utf8(&source.m_map_file_name)?.to_string(),
            cache_handles,
            mini_save: source.m_mini_save,
            game_speed: source.m_game_speed.into(),
            default_difficulty: source.m_default_difficulty,
            mod_paths,
            ext_fs_id: 0u64, // The ext_fs_id is filled later by `set_metadata`
            ext_fs_replay_file_name: String::new(), // The ext_fs_replay_file_name is filled later by `set_metadata`
            ext_datetime: chrono::NaiveDateTime::default(), // The ext_datetime is filled later by `set_metadata`
        })
    }
}

impl TryFrom<GameSPlayerDetails> for PlayerDetails {
    type Error = S2ProtocolError;
    fn try_from(source: GameSPlayerDetails) -> Result<Self, Self::Error> {
        Ok(PlayerDetails {
            name: str::from_utf8(&source.m_name)?.to_string(),
            toon: source.m_toon.try_into()?,
            race: str::from_utf8(&source.m_race)?.to_string(),
            color: source.m_color.into(),
            control: source.m_control,
            team_id: source.m_team_id,
            handicap: source.m_handicap,
            observe: source.m_observe.into(),
            result: source.m_result.into(),
            working_set_slot_id: source.m_working_set_slot_id,
            hero: str::from_utf8(&source.m_hero)?.to_string(),
        })
    }
}

impl From<GameSColor> for Color {
    fn from(value: GameSColor) -> Self {
        Self {
            a: value.m_a,
            r: value.m_r,
            g: value.m_g,
            b: value.m_b,
        }
    }
}

impl From<GameEResultDetails> for u8 {
    fn from(value: GameEResultDetails) -> Self {
        match value {
            GameEResultDetails::EUndecided => crate::common::GAME_RESULT_UNDECIDED,
            GameEResultDetails::EWin => crate::common::GAME_RESULT_WIN,
            GameEResultDetails::ELoss => crate::common::GAME_RESULT_DEFEAT,
            GameEResultDetails::ETie => crate::common::GAME_RESULT_TIE,
        }
    }
}

impl TryFrom<GameSThumbnail> for Thumbnail {
    type Error = S2ProtocolError;
    fn try_from(value: GameSThumbnail) -> Result<Self, Self::Error> {
        Ok(Self {
            file: str::from_utf8(&value.m_file)?.to_string(),
        })
    }
}

impl From<GameEGameSpeed> for u8 {
    fn from(value: GameEGameSpeed) -> Self {
        match value {
            GameEGameSpeed::ESlower => crate::common::GAME_SPEED_SLOWER,
            GameEGameSpeed::ESlow => crate::common::GAME_SPEED_SLOW,
            GameEGameSpeed::ENormal => crate::common::GAME_SPEED_NORMAL,
            GameEGameSpeed::EFast => crate::common::GAME_SPEED_FAST,
            GameEGameSpeed::EFaster => crate::common::GAME_SPEED_FASTER,
        }
    }
}

impl TryFrom<GameSToonNameDetails> for ToonNameDetails {
    type Error = S2ProtocolError;
    fn try_from(value: GameSToonNameDetails) -> Result<Self, Self::Error> {
        Ok(Self {
            region: value.m_region,
            program_id: value.m_program_id,
            realm: value.m_realm,
            id: value.m_id,
        })
    }
}
