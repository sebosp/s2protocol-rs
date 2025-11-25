//! Decodes the Details.
//! The cache_handles can be downloaded from blizzard depo URL , sort of like:
//! The first 8 bytes of the cache_handle are the extension, for example "s2ma".
//! The other 2 characters are the region, for example "EU".
//! ```bash
//!❯ echo "73326d6100004555"|xxd -r -ps
//! s2maEU
//! ````
//! ```bash
//!./target/debug/s2protocol -v error --source 'SomeReplay' --json-balance-data-dir=/home/seb/git/s2protocol-rs/assets/BalanceData/ --quiet get details|jq '.cache_handles.[]' -r|while read cache_handle; do cache_hash=$(echo $cache_handle|sed 's/^73326d6100004555//g'); curl -L https://eu-s2-depot.classic.blizzard.com/$cache_hash.s2ma -o $cache_hash.s2ma;done
//! ```

use std::path::PathBuf;

#[cfg(feature = "dep_arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};
use nom_mpq::MPQ;

use crate::{GameDescription, InitData, LobbySlot, error::S2ProtocolError};
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

/// A Flat row of PlayerDetails with LobbySyncState data
/// without the cache_handles and mod_paths.
/// because I haven't seen what they are used for yet.
///
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct PlayerLobbyDetailsFlatRow {
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
    pub lobby_slot_user_id: Option<i64>,
    pub lobby_slot_observe: u8,
    pub lobby_slot_map_size_x: u8,
    pub lobby_slot_map_size_y: u8,
    pub ext_fs_sha256: String,
    pub ext_fs_file_name: String,
    pub ext_fs_id: u64,
    pub ext_datetime: chrono::NaiveDateTime,
}

impl From<PlayerLobbyDetails> for PlayerLobbyDetailsFlatRow {
    fn from(source: PlayerLobbyDetails) -> PlayerLobbyDetailsFlatRow {
        PlayerLobbyDetailsFlatRow {
            ext_fs_id: source.ext_fs_id,
            ext_fs_sha256: source.ext_fs_sha256,
            ext_fs_file_name: source.ext_fs_file_name,
            ext_datetime: source.ext_datetime,
            player_name: source.player_details.name,
            player_toon_region: source.player_details.toon.region,
            player_toon_program_id: source.player_details.toon.program_id,
            player_toon_realm: source.player_details.toon.realm,
            player_toon_id: source.player_details.toon.id,
            player_race: source.player_details.race,
            player_color_a: source.player_details.color.a,
            player_color_r: source.player_details.color.r,
            player_color_g: source.player_details.color.g,
            player_color_b: source.player_details.color.b,
            player_control: source.player_details.control,
            player_team_id: source.player_details.team_id,
            player_observe: source.player_details.observe,
            player_result: source.player_details.result,
            player_working_set_slot_id: source.player_details.working_set_slot_id,
            player_hero: source.player_details.hero,
            title: source.title,
            is_blizzard_map: source.game_description.is_blizzard_map,
            lobby_slot_user_id: source.lobby_slot.user_id,
            lobby_slot_observe: source.lobby_slot.observe,
            lobby_slot_map_size_x: source.game_description.map_size_x,
            lobby_slot_map_size_y: source.game_description.map_size_y,
            time_utc: source.time_utc,
            time_local_offset: source.time_local_offset,
        }
    }
}

/// A joined version of the PlayerLobbySlot contained within the InitData sector and the Details
/// sector
/// The working_set_slot_id joins the initData with the details.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlayerLobbyDetails {
    pub player_details: PlayerDetails,
    pub lobby_slot: LobbySlot,
    /// The name of the map
    pub title: String,
    pub game_description: GameDescription,
    pub time_utc: i64,
    pub time_local_offset: i64,
    pub user_init_data_name: String,
    pub user_init_data_clan_tag: String,
    // Attempt a join from the PlayerSetupEvent at the start of ReplayTrackerEvents
    pub tracker_setup_player_id: Option<u8>,
    pub tracker_setup_slot_id: Option<u32>, // Is this u32 or?
    pub ext_fs_sha256: String,
    pub ext_fs_file_name: String,
    pub ext_fs_id: u64,
    pub ext_datetime: chrono::NaiveDateTime,
}

impl TryFrom<&InitData> for Vec<PlayerLobbyDetails> {
    type Error = S2ProtocolError;

    fn try_from(init: &InitData) -> Result<Self, Self::Error> {
        let details: Details = init.try_into()?;
        let res = details
            .player_list
            .into_iter()
            .filter_map(|player| {
                let mut slot_idx = None;
                for (idx, lobby_slot) in init.sync_lobby_state.lobby_state.slots.iter().enumerate()
                {
                    if let (Some(init_slot_id), Some(details_slot_id)) =
                        (lobby_slot.working_set_slot_id, player.working_set_slot_id)
                        && init_slot_id == details_slot_id
                    {
                        slot_idx = Some(idx);
                        break;
                    }
                }
                let slot_idx = slot_idx?;
                Some(PlayerLobbyDetails {
                    ext_fs_id: details.ext_fs_id,
                    ext_fs_sha256: init.ext_fs_sha256.clone(),
                    ext_fs_file_name: init.ext_fs_file_name.clone(),
                    ext_datetime: details.ext_datetime,
                    title: details.title.clone(),
                    game_description: init.sync_lobby_state.game_description.clone(),
                    lobby_slot: init.sync_lobby_state.lobby_state.slots[slot_idx].clone(),
                    player_details: player.clone(),
                    time_utc: details.time_utc,
                    time_local_offset: details.time_local_offset,
                    user_init_data_name: init
                        .sync_lobby_state
                        .user_initial_data
                        .get(slot_idx)
                        .map_or("".to_string(), |u| u.name.clone()),
                    user_init_data_clan_tag: init
                        .sync_lobby_state
                        .user_initial_data
                        .get(slot_idx)
                        .map_or("".to_string(), |u| u.clan_tag.clone().unwrap_or_default()),
                    tracker_setup_player_id: None,
                    tracker_setup_slot_id: None,
                })
            })
            .collect();
        Ok(res)
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

    /// Returns all the players from the sync lobby state.
    /// That is the ones that are not observers.
    pub fn get_player_names(&self) -> Vec<String> {
        self.player_list
            .iter()
            .filter_map(|u| {
                if u.observe == crate::common::OBSERVE_NONE {
                    Some(u.name.clone())
                } else {
                    None
                }
            })
            .collect()
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
