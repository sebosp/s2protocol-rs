//! Cmd Events in a flat row for Arrow usage

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::game_events::GameSCmdEvent;
use crate::game_events::{GameSCmdAbil, GameSCmdData, GameTUnitTag};
use serde::{Deserialize, Serialize};

/// Arrow compatible Cmd Event
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct CmdEventFlatRow {
    pub user_id: i64,
    pub m_cmd_flags: i64,
    pub m_abil: Option<GameSCmdAbil>,
    pub m_data: GameSCmdData,
    pub m_sequence: i64,
    pub m_other_unit: Option<GameTUnitTag>,
    pub m_unit_group: Option<u32>,
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_replay_sha256: String,
}

impl CmdEventFlatRow {
    /// Create a new UpgradeEventFlatRow from a UpgradeEvent and the fields from the Details MPQ sector
    pub fn new(
        details: &crate::details::Details,
        event: GameSCmdEvent,
        game_loop: i64,
        user_id: i64,
        _change_hint: crate::UnitChangeHint,
    ) -> Self {
        let ext_replay_seconds = crate::convert_game_loop_to_seconds(game_loop);
        Self {
            user_id,
            m_cmd_flags: event.m_cmd_flags,
            m_abil: event.m_abil,
            m_data: event.m_data,
            m_sequence: event.m_sequence,
            m_other_unit: event.m_other_unit,
            m_unit_group: event.m_unit_group,
            ext_replay_loop: game_loop,
            ext_replay_seconds,
            ext_fs_replay_sha256: details.ext_fs_replay_sha256.clone(),
        }
    }
}
