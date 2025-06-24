//! Cmd Events in a flat row for Arrow usage

#[cfg(feature = "arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::game_events::GameSCmdEvent;
use crate::game_events::{
    GameSCmdData,
    // GameSCmdDataTargetUnit,
    GameSMapCoord3D,
    GameTPlayerId,
    GameTUnitLink,
    GameTUnitTag,
};
use serde::{Deserialize, Serialize};

/// Arrow compatible Cmd Event for Target Point
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct CmdTargetPointEventFlatRow {
    pub user_id: i64,
    pub m_cmd_flags: i64,
    pub m_abil: String,
    pub target_point: GameSMapCoord3D,
    pub m_sequence: i64,
    pub m_other_unit: Option<u32>,
    pub m_unit_group: Option<u32>,
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_replay_sha256: String,
}

impl CmdTargetPointEventFlatRow {
    /// Create a new CmdEventFlatRow from a GameSCmdEvent and the fields from the Details MPQ sector
    pub fn new(
        details: &crate::details::Details,
        event: GameSCmdEvent,
        game_loop: i64,
        user_id: i64,
        _change_hint: crate::UnitChangeHint,
    ) -> Self {
        let map_coord = match event.m_data {
            GameSCmdData::TargetPoint(map_coord) => map_coord.clone(),
            _ => unreachable!(),
        };
        let ext_replay_seconds = crate::convert_game_loop_to_seconds(game_loop);
        Self {
            user_id,
            m_cmd_flags: event.m_cmd_flags,
            m_abil: match event.m_abil {
                Some(abil) => abil.ability.clone(),
                None => String::new(),
            },
            target_point: map_coord.clone(),
            m_sequence: event.m_sequence,
            m_other_unit: event.m_other_unit,
            m_unit_group: event.m_unit_group,
            ext_replay_loop: game_loop,
            ext_replay_seconds,
            ext_fs_replay_sha256: details.ext_fs_replay_sha256.clone(),
        }
    }
}

/// Arrow compatible Cmd Event for Target Unit
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct CmdTargetUnitEventFlatRow {
    pub user_id: i64,
    pub m_cmd_flags: i64,
    pub m_abil: String,
    pub m_target_unit_flags: u16,
    pub m_timer: u8,
    pub m_tag: GameTUnitTag,
    pub m_snapshot_unit_link: GameTUnitLink,
    pub m_snapshot_control_player_id: Option<GameTPlayerId>,
    pub m_snapshot_upkeep_player_id: Option<GameTPlayerId>,
    pub m_snapshot_point: GameSMapCoord3D,
    pub m_sequence: i64,
    pub m_other_unit: Option<GameTUnitTag>,
    pub m_unit_group: Option<u32>,
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_replay_sha256: String,
}

impl CmdTargetUnitEventFlatRow {
    /// Create a new CmdEventFlatRow from a GameSCmdEvent and the fields from the Details MPQ sector
    pub fn new(
        details: &crate::details::Details,
        event: GameSCmdEvent,
        game_loop: i64,
        user_id: i64,
        _change_hint: crate::UnitChangeHint,
    ) -> Self {
        let unit = match event.m_data {
            GameSCmdData::TargetUnit(unit) => unit.clone(),
            _ => unreachable!(),
        };
        let ext_replay_seconds = crate::convert_game_loop_to_seconds(game_loop);
        Self {
            user_id,
            m_cmd_flags: event.m_cmd_flags,
            m_abil: match event.m_abil {
                Some(abil) => abil.ability.clone(),
                None => String::new(),
            },
            m_target_unit_flags: unit.m_target_unit_flags,
            m_timer: unit.m_timer,
            m_tag: unit.m_tag,
            m_snapshot_unit_link: unit.m_snapshot_unit_link,
            m_snapshot_control_player_id: unit.m_snapshot_control_player_id,
            m_snapshot_upkeep_player_id: unit.m_snapshot_upkeep_player_id,
            m_snapshot_point: unit.m_snapshot_point,
            m_sequence: event.m_sequence,
            m_other_unit: event.m_other_unit,
            m_unit_group: event.m_unit_group,
            ext_replay_loop: game_loop,
            ext_replay_seconds,
            ext_fs_replay_sha256: details.ext_fs_replay_sha256.clone(),
        }
    }
}
