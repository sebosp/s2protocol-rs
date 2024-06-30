//! Cmd Events in a flat row for Arrow usage

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::game_events::GameSCmdEvent;
use crate::game_events::{GameSCmdData, GameSCmdDataTargetUnit, GameSMapCoord3D, GameTUnitTag};
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
    pub unit: GameSCmdDataTargetUnit,
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
            unit,
            m_sequence: event.m_sequence,
            m_other_unit: event.m_other_unit,
            m_unit_group: event.m_unit_group,
            ext_replay_loop: game_loop,
            ext_replay_seconds,
            ext_fs_replay_sha256: details.ext_fs_replay_sha256.clone(),
        }
    }
}
