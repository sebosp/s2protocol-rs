//! Cmd Events in a flat row for Arrow usage

#[cfg(feature = "dep_arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::game_events::GameSCmdData;
use crate::game_events::GameSCmdEvent;
use serde::{Deserialize, Serialize};

/// Arrow compatible Cmd Event for Target Point
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct CmdTargetPointEventFlatRow {
    pub user_id: i64,
    pub cmd_flags: i64,
    pub abil_link: Option<u16>,
    pub abil_cmd_index: Option<i64>,
    pub ability: String,
    pub target_point_x: i64,
    pub target_point_y: i64,
    pub target_point_z: i32,
    pub sequence: i64,
    pub unit_group: Option<u32>,
    pub unit_name: String,
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_id: u64,
}

impl CmdTargetPointEventFlatRow {
    /// Create a new CmdEventFlatRow from a GameSCmdEvent and the fields from the Details MPQ sector
    pub fn new(
        details: &crate::details::Details,
        event: GameSCmdEvent,
        game_loop: i64,
        user_id: i64,
        change_hint: crate::UnitChangeHint,
    ) -> Vec<Self> {
        let mut res = vec![];
        let map_coord = match event.m_data {
            GameSCmdData::TargetPoint(map_coord) => map_coord.clone(),
            _ => return res,
        };
        let ext_replay_seconds = crate::convert_game_loop_to_seconds(game_loop);
        if let crate::UnitChangeHint::Abilities { units, .. } = change_hint {
            for unit in units {
                let unit_name = unit.name.clone();
                res.push(Self {
                    user_id,
                    cmd_flags: event.m_cmd_flags,
                    abil_link: event.m_abil.as_ref().map(|abil| abil.m_abil_link),
                    ability: event
                        .m_abil
                        .as_ref()
                        .map(|abil| abil.ability.clone())
                        .unwrap_or_default(),
                    abil_cmd_index: event.m_abil.as_ref().map(|abil| abil.m_abil_cmd_index),
                    target_point_x: map_coord.x,
                    target_point_y: map_coord.y,
                    target_point_z: map_coord.z,
                    sequence: event.m_sequence,
                    unit_group: event.m_unit_group,
                    unit_name,
                    ext_replay_loop: game_loop,
                    ext_replay_seconds,
                    ext_fs_id: details.ext_fs_id,
                });
            }
        }
        res
    }
}

/// Arrow compatible Cmd Event for Target Unit
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "dep_arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct CmdTargetUnitEventFlatRow {
    pub user_id: i64,
    pub cmd_flags: i64,
    pub abil_link: Option<u16>,
    pub abil_cmd_index: Option<i64>,
    pub ability: String,
    pub target_unit_flags: u16,
    pub tag: u32,
    pub snapshot_unit_link: u16,
    pub snapshot_control_player_id: Option<i64>,
    pub snapshot_upkeep_player_id: Option<i64>,
    pub snapshot_point_x: i64,
    pub snapshot_point_y: i64,
    pub snapshot_point_z: i32,
    pub sequence: i64,
    pub unit_name: String,
    pub target_unit_name: String,
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_id: u64,
}

impl CmdTargetUnitEventFlatRow {
    /// Create a new CmdEventFlatRow from a GameSCmdEvent and the fields from the Details MPQ sector
    pub fn new(
        details: &crate::details::Details,
        event: GameSCmdEvent,
        game_loop: i64,
        user_id: i64,
        change_hint: crate::UnitChangeHint,
    ) -> Vec<Self> {
        let mut res = vec![];
        let unit = match event.m_data {
            GameSCmdData::TargetUnit(unit) => unit.clone(),
            _ => return res,
        };
        let ext_replay_seconds = crate::convert_game_loop_to_seconds(game_loop);
        if let crate::UnitChangeHint::Abilities {
            units,
            event: _,
            target: _,
        } = change_hint
        {
            for unit_hint in units {
                let unit_name = unit_hint.name.clone();
                let target_unit_name = match unit_hint.cmd.other_unit_name {
                    Some(name) => name.clone(),
                    None => "".to_string(),
                };
                res.push(Self {
                    user_id,
                    cmd_flags: event.m_cmd_flags,
                    abil_link: event.m_abil.as_ref().map(|abil| abil.m_abil_link),
                    abil_cmd_index: event.m_abil.as_ref().map(|abil| abil.m_abil_cmd_index),
                    ability: event
                        .m_abil
                        .as_ref()
                        .map(|abil| abil.ability.clone())
                        .unwrap_or_default(),
                    target_unit_flags: unit.m_target_unit_flags,
                    tag: unit.m_tag,
                    snapshot_unit_link: unit.m_snapshot_unit_link,
                    snapshot_control_player_id: unit.m_snapshot_control_player_id,
                    snapshot_upkeep_player_id: unit.m_snapshot_upkeep_player_id,
                    snapshot_point_x: unit.m_snapshot_point.x,
                    snapshot_point_y: unit.m_snapshot_point.y,
                    snapshot_point_z: unit.m_snapshot_point.z,
                    sequence: event.m_sequence,
                    unit_name,
                    target_unit_name,
                    ext_replay_loop: game_loop,
                    ext_replay_seconds,
                    ext_fs_id: details.ext_fs_id,
                });
            }
        }
        res
    }
}
