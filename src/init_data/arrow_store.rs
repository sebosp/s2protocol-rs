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
    /// The index in the array, maybe this correlates with user_id in other places.
    pub user_index: u32,
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
            .enumerate()
            .map(|(idx, user)| UserInitDataFlatRow {
                ext_fs_id: init_data.ext_fs_id,
                ext_fs_sha256: init_data.ext_fs_sha256.clone(),
                ext_fs_file_name: init_data.ext_fs_file_name.clone(),
                user_index: idx as u32,
                name: user.name,
                clan_tag: user.clan_tag.unwrap_or_default(),
                scaled_rating: user.scaled_rating,
            })
            .collect()
    }
}
