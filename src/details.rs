//! Decodes the Details.

use std::path::PathBuf;

#[cfg(feature = "dep_arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};
use nom_mpq::MPQ;

use crate::{InitData, error::S2ProtocolError};
use serde::{Deserialize, Serialize};

/* Removed fields:
* player_handicap is always 100 in this dataset
* difficulty is always empty
* restart_as_transition map is always false
* disable_recover_game is veeeeeeery rarely true, only 69 out 32K user-map-details.
* description always empty
* image_file_path always empty
* campaign_index always 0
* map_file_name always empty
* mini_save always false
*/

/// A Flat row of PlayerDetails for Arrow usage.
/// without the cache_handles and mod_paths.
/// because I haven't seen what they are used for yet.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct PlayerDetailsFlatRow {
    pub player_name: String,
    pub player_toon_region: u8,
    pub player_toon_program_id: u32,
    pub player_toon_realm: u32,
    pub player_toon_id: u64,
    pub player_race: String,
    pub player_color_a: u8,
    pub player_color_r: u8,
    pub player_color_g: u8,
    pub player_color_b: u8,
    pub player_control: u8,
    pub player_team_id: u8,
    pub player_observe: u8,
    pub player_result: String,
    pub player_working_set_slot_id: Option<u8>,
    pub player_hero: String,
    pub title: String,
    pub is_blizzard_map: bool,
    pub time_utc: i64,
    pub time_local_offset: i64,
    pub ext_fs_id: u64,
    pub ext_datetime: chrono::NaiveDateTime,
}

impl From<Details> for Vec<PlayerDetailsFlatRow> {
    fn from(details: Details) -> Self {
        details
            .player_list
            .into_iter()
            .map(|player| PlayerDetailsFlatRow {
                ext_fs_id: details.ext_fs_id,
                ext_datetime: details.ext_datetime,
                player_name: player.name,
                player_toon_region: player.toon.region,
                player_toon_program_id: player.toon.program_id,
                player_toon_realm: player.toon.realm,
                player_toon_id: player.toon.id,
                player_race: player.race,
                player_color_a: player.color.a,
                player_color_r: player.color.r,
                player_color_g: player.color.g,
                player_color_b: player.color.b,
                player_control: player.control,
                player_team_id: player.team_id,
                player_observe: player.observe,
                player_result: player.result,
                player_working_set_slot_id: player.working_set_slot_id,
                player_hero: player.hero,
                title: details.title.clone(),
                is_blizzard_map: details.is_blizzard_map,
                time_utc: details.time_utc,
                time_local_offset: details.time_local_offset,
            })
            .collect()
    }
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct Details {
    pub ext_fs_id: u64,
    pub ext_datetime: chrono::NaiveDateTime,
    pub player_list: Vec<PlayerDetails>,
    pub title: String,
    pub difficulty: String,
    pub thumbnail: Thumbnail,
    pub is_blizzard_map: bool,
    pub time_utc: i64,
    pub time_local_offset: i64,
    pub restart_as_transition_map: Option<bool>,
    pub disable_recover_game: bool,
    pub description: String,
    pub image_file_path: String,
    pub campaign_index: u8,
    pub map_file_name: String,
    pub cache_handles: Vec<String>,
    pub mini_save: bool,
    pub game_speed: String,
    pub default_difficulty: u32,
    pub mod_paths: Vec<String>,
}

impl Details {
    /// Calls the per-protocol parser for the Details and sets the metadadata.
    pub fn new(
        file_name: &str,
        ext_fs_id: u64,
        mpq: &MPQ,
        file_contents: &[u8],
    ) -> Result<Self, S2ProtocolError> {
        let details = match crate::versions::read_details(file_name, mpq, file_contents) {
            Ok(details) => details,
            Err(err) => {
                tracing::error!("Error reading details: {:?}", err);
                return Err(err);
            }
        };
        Ok(details.set_metadata(ext_fs_id))
    }

    /// Sets the metadata related to the filesystem entry and the replay time
    pub fn set_metadata(mut self, ext_fs_id: u64) -> Self {
        self.ext_datetime = crate::transform_to_naivetime(self.time_utc, self.time_local_offset)
            .unwrap_or_default();
        self.ext_fs_id = ext_fs_id;
        self
    }

    /// Attempts to find the player id from the player_list vector.
    /// The player_id in this vector is off by one on the player_id in the tracker events, or is
    /// it? We should check this.
    pub fn get_player_name(&self, event_player_id: u8) -> String {
        self.player_list
            .iter()
            .find(|player| {
                let adjusted_player_id = match player.working_set_slot_id {
                    // NOTE: The working_set_slot_id is 0-based
                    // while the incoming event_player_id is 1-based
                    Some(val) => val + 1,
                    _ => return false,
                };
                adjusted_player_id == event_player_id
            })
            .map(|player| {
                // The player name may be prepend by its clan.
                // The clan seems to be URL encoded like "&lt&CLAN&gt<sp/>PLAYERNAME"
                // Remove up to <sp/> if it exists from the player name
                // This should be a different field and maybe we can consider removing
                // it, this is because we change player names/tags through time.
                let player_name = player.name.split("<sp/>").last().unwrap_or_default();
                format!(
                    "{}-{}-{}-{}",
                    player.toon.region, player.toon.realm, player.toon.id, player_name
                )
            })
            .unwrap_or("".to_string())
    }
}

impl TryFrom<&InitData> for Details {
    type Error = S2ProtocolError;

    fn try_from(init: &InitData) -> Result<Self, Self::Error> {
        let path = PathBuf::from(init.ext_fs_file_name.clone());
        let file_contents = crate::read_file(&path)?;
        let (_input, mpq) = crate::parser::parse(&file_contents)?;
        Self::new(
            path.to_str().unwrap_or_default(),
            init.ext_fs_id,
            &mpq,
            &file_contents,
        )
    }
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct PlayerDetails {
    pub name: String,
    pub toon: ToonNameDetails,
    pub race: String,
    pub color: Color,
    pub control: u8,
    pub team_id: u8,
    pub handicap: u32,
    pub observe: u8,
    pub result: String,
    pub working_set_slot_id: Option<u8>,
    pub hero: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct ToonNameDetails {
    pub region: u8,
    pub program_id: u32,
    pub realm: u32,
    pub id: u64,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct Thumbnail {
    pub file: String,
}
