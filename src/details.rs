//! Decodes the Details.

use std::path::PathBuf;

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};
use nom_mpq::MPQ;

use crate::error::S2ProtocolError;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct Details {
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
    pub game_speed: u8,
    pub default_difficulty: u32,
    pub mod_paths: Vec<String>,
    pub ext_fs_replay_file_name: String,
    pub ext_fs_replay_sha256: String,
    pub ext_datetime: chrono::NaiveDateTime,
}

impl Details {
    /// Calls the per-protocol parser for the Details and sets the metadadata.
    pub fn new(file_name: &str, mpq: &MPQ, file_contents: &[u8]) -> Result<Self, S2ProtocolError> {
        let details = match crate::versions::read_details(file_name, mpq, file_contents) {
            Ok(details) => details,
            Err(err) => {
                tracing::error!("Error reading details: {:?}", err);
                return Err(err);
            }
        };
        Ok(details.set_metadata(file_name, file_contents))
    }

    /// Sets the metadata related to the filesystem entry and the replay time
    pub fn set_metadata(mut self, file_name: &str, file_contents: &[u8]) -> Self {
        self.ext_fs_replay_file_name = file_name.to_string();
        self.ext_fs_replay_sha256 = sha256::digest(file_contents);
        self.ext_datetime = crate::transform_to_naivetime(self.time_utc, self.time_local_offset)
            .unwrap_or_default();
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

impl TryFrom<PathBuf> for Details {
    type Error = S2ProtocolError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let file_contents = crate::read_file(&path)?;
        let (_input, mpq) = crate::parser::parse(&file_contents)?;
        match Self::new(path.to_str().unwrap_or_default(), &mpq, &file_contents) {
            Ok(details) => {
                Ok(details.set_metadata(path.to_str().unwrap_or_default(), &file_contents))
            }
            Err(err) => {
                tracing::error!("Error reading details: {:?}", err);
                Err(err)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
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
    pub result: u8,
    pub working_set_slot_id: Option<u8>,
    pub hero: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct ToonNameDetails {
    pub region: u8,
    pub program_id: u32,
    pub realm: u32,
    pub id: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct Thumbnail {
    pub file: String,
}
