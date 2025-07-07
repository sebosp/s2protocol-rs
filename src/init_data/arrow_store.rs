//! Provides  the flat row versions of the InitData.

#[cfg(feature = "dep_arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use super::InitData;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct UserInitDataFlatRow {
    pub ext_fs_id: u64,
    pub name: String,
    pub clan_tag: String,
    pub race_preference: Option<i64>,
    pub team_preference: Option<u8>,
    pub examine: bool,
    pub observe: u8,
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
                name: user.name,
                clan_tag: user.clan_tag.unwrap_or_default(),
                race_preference: user.race_preference,
                team_preference: user.team_preference,
                examine: user.examine,
                observe: user.observe,
                scaled_rating: user.scaled_rating,
            })
            .collect()
    }
}
