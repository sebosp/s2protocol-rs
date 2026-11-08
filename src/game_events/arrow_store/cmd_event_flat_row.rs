//! Cmd Events in a flat row for Arrow usage

#[cfg(feature = "dep_arrow")]
use arrow::datatypes::Schema;

use crate::game_events::GameSCmdData;
use crate::game_events::GameSCmdEvent;
use serde::{Deserialize, Serialize};

/// The different unit types of a selection in the order they were found, separated by commas.
fn unit_type_names<'a>(names: impl Iterator<Item = &'a str>) -> String {
    let mut types: Vec<&str> = vec![];
    for name in names {
        if !types.contains(&name) {
            types.push(name);
        }
    }
    types.join(",")
}

/// Arrow compatible Cmd Event for Target Point: a row per command, whatever the size of the
/// selection.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CmdTargetPointEventFlatRow {
    pub user_id: i64,
    pub player_name: Option<String>,
    pub cmd_flags: i64,
    pub abil_link: Option<u16>,
    pub abil_cmd_index: Option<i64>,
    pub ability: String,
    pub target_point_x: i64,
    pub target_point_y: i64,
    pub target_point_z: i32,
    pub sequence: i64,
    pub unit_group: Option<u32>,
    /// The first unit of the selection that received the command, empty when the selection is
    /// not known.
    pub unit_name: String,
    /// The unit types of the selection, in the order they were found, separated by commas.
    pub unit_names: String,
    /// The units of the selection that received the command.
    pub selection_size: u32,
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_id: u64,
}

impl CmdTargetPointEventFlatRow {
    /// Create a new CmdEventFlatRow from a GameSCmdEvent and the fields from the Details MPQ sector
    pub fn new(
        ext_fs_id: u64,
        event: GameSCmdEvent,
        game_loop: i64,
        user_id: i64,
        player_name: Option<String>,
        change_hint: crate::UnitChangeHint,
    ) -> Vec<Self> {
        let mut res = vec![];
        let map_coord = match event.m_data {
            GameSCmdData::TargetPoint(map_coord) => map_coord.clone(),
            _ => return res,
        };
        let ext_replay_seconds = crate::convert_game_loop_to_seconds(game_loop);
        // A command whose selection the state does not know has an empty unit name.
        if let crate::UnitChangeHint::Abilities { units, .. } = change_hint {
            let ability: String = event
                .m_abil
                .as_ref()
                .map(|abil| abil.ability.clone())
                .unwrap_or_default();
            let unit_name = units
                .first()
                .map(|unit| unit.name.clone())
                .unwrap_or_default();
            res.push(Self {
                user_id,
                player_name: player_name.clone(),
                cmd_flags: event.m_cmd_flags,
                abil_link: event.m_abil.as_ref().map(|abil| abil.m_abil_link),
                ability: ability.clone(),
                abil_cmd_index: event.m_abil.as_ref().map(|abil| abil.m_abil_cmd_index),
                target_point_x: map_coord.x,
                target_point_y: map_coord.y,
                target_point_z: map_coord.z,
                sequence: event.m_sequence,
                unit_group: event.m_unit_group,
                unit_name,
                unit_names: unit_type_names(units.iter().map(|unit| unit.name.as_str())),
                selection_size: units.len() as u32,
                ext_replay_loop: game_loop,
                ext_replay_seconds,
                ext_fs_id,
            });
        }
        res
    }

    pub fn schema() -> Schema {
        <Self as crate::arrow_store::rows::ArrowRow>::schema()
    }
}

/// Arrow compatible Cmd Event for Target Unit: a row per command, whatever the size of the
/// selection.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CmdTargetUnitEventFlatRow {
    pub user_id: i64,
    pub player_name: Option<String>,
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
    /// The first unit of the selection that received the command, empty when the selection is
    /// not known.
    pub unit_name: String,
    /// The unit types of the selection, in the order they were found, separated by commas.
    pub unit_names: String,
    /// The units of the selection that received the command.
    pub selection_size: u32,
    pub target_unit_name: String,
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_id: u64,
}

impl CmdTargetUnitEventFlatRow {
    /// Create a new CmdEventFlatRow from a GameSCmdEvent and the fields from the Details MPQ sector
    pub fn new(
        ext_fs_id: u64,
        event: GameSCmdEvent,
        game_loop: i64,
        user_id: i64,
        player_name: Option<String>,
        change_hint: crate::UnitChangeHint,
    ) -> Vec<Self> {
        let mut res = vec![];
        let unit = match event.m_data {
            GameSCmdData::TargetUnit(unit) => unit.clone(),
            _ => return res,
        };
        let ext_replay_seconds = crate::convert_game_loop_to_seconds(game_loop);
        // A command whose selection the state does not know has an empty unit name.
        if let crate::UnitChangeHint::Abilities { units, target, .. } = change_hint {
            let unit_name = units
                .first()
                .map(|unit| unit.name.clone())
                .unwrap_or_default();
            let target_unit_name = units
                .first()
                .and_then(|first| first.cmd.other_unit_name.clone())
                .or_else(|| target.map(|target| target.name.clone()))
                .unwrap_or_default();
            res.push(Self {
                user_id,
                player_name: player_name.clone(),
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
                unit_names: unit_type_names(units.iter().map(|unit| unit.name.as_str())),
                selection_size: units.len() as u32,
                target_unit_name,
                ext_replay_loop: game_loop,
                ext_replay_seconds,
                ext_fs_id,
            });
        }
        res
    }

    pub fn schema() -> Schema {
        <Self as crate::arrow_store::rows::ArrowRow>::schema()
    }
}

#[cfg(feature = "dep_arrow")]
crate::impl_arrow_row!(CmdTargetPointEventFlatRow);

#[cfg(feature = "dep_arrow")]
crate::impl_arrow_row!(CmdTargetUnitEventFlatRow);

/// Arrow compatible Cmd Event without a target: train, research, morph, stop, hold position,
/// burrow... A row per command, whatever the size of the selection.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CmdNoTargetEventFlatRow {
    pub user_id: i64,
    pub player_name: Option<String>,
    pub cmd_flags: i64,
    pub abil_link: Option<u16>,
    pub abil_cmd_index: Option<i64>,
    pub ability: String,
    /// The value of the commands that carry a number instead of a target, rare.
    pub data: Option<u32>,
    pub sequence: i64,
    pub unit_group: Option<u32>,
    /// The first unit of the selection that received the command, empty when the selection is
    /// not known.
    pub unit_name: String,
    /// The unit types of the selection, in the order they were found, separated by commas.
    pub unit_names: String,
    /// The units of the selection that received the command.
    pub selection_size: u32,
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
    pub ext_fs_id: u64,
}

impl CmdNoTargetEventFlatRow {
    /// The row of a command without a target point or unit, None for the others.
    pub fn new(
        ext_fs_id: u64,
        event: GameSCmdEvent,
        game_loop: i64,
        user_id: i64,
        player_name: Option<String>,
        change_hint: crate::UnitChangeHint,
    ) -> Option<Self> {
        let data = match event.m_data {
            GameSCmdData::None => None,
            GameSCmdData::Data(data) => Some(data),
            _ => return None,
        };
        let crate::UnitChangeHint::Abilities { units, .. } = change_hint else {
            return None;
        };
        Some(Self {
            user_id,
            player_name,
            cmd_flags: event.m_cmd_flags,
            abil_link: event.m_abil.as_ref().map(|abil| abil.m_abil_link),
            abil_cmd_index: event.m_abil.as_ref().map(|abil| abil.m_abil_cmd_index),
            ability: event
                .m_abil
                .as_ref()
                .map(|abil| abil.ability.clone())
                .unwrap_or_default(),
            data,
            sequence: event.m_sequence,
            unit_group: event.m_unit_group,
            unit_name: units
                .first()
                .map(|unit| unit.name.clone())
                .unwrap_or_default(),
            unit_names: unit_type_names(units.iter().map(|unit| unit.name.as_str())),
            selection_size: units.len() as u32,
            ext_replay_loop: game_loop,
            ext_replay_seconds: crate::convert_game_loop_to_seconds(game_loop),
            ext_fs_id,
        })
    }

    pub fn schema() -> Schema {
        <Self as crate::arrow_store::rows::ArrowRow>::schema()
    }
}

#[cfg(feature = "dep_arrow")]
crate::impl_arrow_row!(CmdNoTargetEventFlatRow);
