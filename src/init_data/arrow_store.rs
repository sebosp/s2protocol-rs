//! Provides  the flat row versions of the InitData.

#[cfg(feature = "dep_arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use super::InitData;

use serde::{Deserialize, Serialize};

// TODO: So far only the "name" seems interesting, maybe we should drop this struct.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UserInitDataFlatRow {
    pub ext_fs_id: u64,
    pub ext_fs_sha256: String,
    pub ext_fs_file_name: String,
    pub name: String,
    pub clan_tag: String,
    pub scaled_rating: Option<i32>,
}

impl From<&InitData> for Vec<UserInitDataFlatRow> {
    fn from(init_data: &InitData) -> Self {
        init_data
            .sync_lobby_state
            .user_initial_data
            .clone()
            .into_iter()
            .map(|user| UserInitDataFlatRow {
                ext_fs_id: init_data.ext_fs_id,
                ext_fs_sha256: init_data.ext_fs_sha256.clone(),
                ext_fs_file_name: init_data.ext_fs_file_name.clone(),
                name: user.name,
                clan_tag: user.clan_tag.unwrap_or_default(),
                scaled_rating: user.scaled_rating,
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct LobbySlotInitDataFlatRow {
    pub ext_fs_id: u64,
    pub ext_fs_sha256: String,
    pub ext_fs_file_name: String,
    pub control: i64,
    pub user_id: Option<i64>,
    pub team_id: i64,
    pub observe: u8,
    pub working_set_slot_id: Option<u8>,
    pub map_size_x: u8,
    pub map_size_y: u8,
}

impl From<&InitData> for Vec<LobbySlotInitDataFlatRow> {
    fn from(init_data: &InitData) -> Self {
        init_data
            .sync_lobby_state
            .lobby_state
            .slots
            .clone()
            .into_iter()
            .map(|user| LobbySlotInitDataFlatRow {
                ext_fs_id: init_data.ext_fs_id,
                ext_fs_sha256: init_data.ext_fs_sha256.clone(),
                ext_fs_file_name: init_data.ext_fs_file_name.clone(),
                control: user.control,
                user_id: user.user_id,
                team_id: user.team_id,
                observe: user.observe,
                working_set_slot_id: user.working_set_slot_id,
                map_size_x: init_data.sync_lobby_state.game_description.map_size_x,
                map_size_y: init_data.sync_lobby_state.game_description.map_size_y,
            })
            .collect()
    }
}
