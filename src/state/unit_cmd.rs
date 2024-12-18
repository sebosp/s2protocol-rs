//! The unit may have a command triggered.
//! This can be pointing to a target, casting an ability, etc.

use super::*;
use crate::game_events::{
    GameSCmdAbil, GameSCmdData, GameSCmdDataTargetUnit, GameSCmdEvent, GameSMapCoord3D,
};
use serde::{Deserialize, Serialize};

/// A unit command, this can be where the unit points to, what abilities it casts, etc.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SC2UnitCmd {
    /// Flags are unknown.
    pub cmd_flags: i64,
    /// An ability may be engaged.
    pub abil: Option<SC2UnitCmdAbil>,
    /// Potentially there is data, such as TargetPoint or TargetUnit.
    pub data: SC2UnitCmdData,
    /// Unknown
    pub sequence: i64,
    /// Potentially another unit is targeted.
    pub other_unit: Option<u32>,
    /// Unknown
    pub unit_group: Option<u32>,
}

impl SC2UnitCmd {
    /// Sets the Target Point for a unit.
    pub fn set_data_target_point(&mut self, coord: GameSMapCoord3D) {
        self.data = SC2UnitCmdData::TargetPoint(coord.into());
    }

    /// Sets the Target Point for a unit.
    pub fn set_data_target_unit(&mut self, target: SC2UnitCmdDataTargetUnit) {
        self.data = SC2UnitCmdData::TargetUnit(target);
    }
}

impl From<GameSCmdEvent> for SC2UnitCmd {
    fn from(event: GameSCmdEvent) -> Self {
        SC2UnitCmd {
            cmd_flags: event.m_cmd_flags,
            abil: event.m_abil.map(|abil| abil.into()),
            data: event.m_data.into(),
            sequence: event.m_sequence,
            other_unit: event.m_other_unit,
            unit_group: event.m_unit_group,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SC2UnitCmdAbil {
    /// Apparently related to stableid.json, but not sure yet.
    pub abil_link: u16,
    /// A translation into string, this is not stable yet.
    pub ability: String,
    /// Relates to the stableid.json but not matched properly yet.
    pub abil_cmd_index: i64,
    /// Unsure what the data is about.
    pub abil_cmd_data: Option<u8>,
}

impl From<GameSCmdAbil> for SC2UnitCmdAbil {
    fn from(event: GameSCmdAbil) -> Self {
        SC2UnitCmdAbil {
            abil_link: event.m_abil_link,
            ability: event.ability,
            abil_cmd_index: event.m_abil_cmd_index,
            abil_cmd_data: event.m_abil_cmd_data,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum SC2UnitCmdData {
    #[default]
    None,
    /// A point in the map is targeted.
    TargetPoint(Vec3D),
    /// Another unit is targeted.
    TargetUnit(SC2UnitCmdDataTargetUnit),
    /// Unknown
    Data(u32),
}

impl From<GameSCmdData> for SC2UnitCmdData {
    fn from(event: GameSCmdData) -> Self {
        match event {
            GameSCmdData::None => SC2UnitCmdData::None,
            GameSCmdData::TargetPoint(point) => SC2UnitCmdData::TargetPoint(point.into()),
            GameSCmdData::TargetUnit(unit) => SC2UnitCmdData::TargetUnit(unit.into()),
            GameSCmdData::Data(data) => SC2UnitCmdData::Data(data),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SC2UnitCmdDataTargetUnit {
    /// Unknown
    pub target_unit_flags: u16,
    /// Unknown
    pub timer: u8,
    /// The tag of the unit
    pub tag: u32,
    /// Unknown
    pub snapshot_unit_link: u16,
    /// Unknown
    pub snapshot_control_player_id: Option<i64>,
    /// Unknown
    pub snapshot_upkeep_player_id: Option<i64>,
    /// Unknown
    pub snapshot_point: Vec3D,
}

impl From<GameSCmdDataTargetUnit> for SC2UnitCmdDataTargetUnit {
    fn from(event: GameSCmdDataTargetUnit) -> Self {
        SC2UnitCmdDataTargetUnit {
            target_unit_flags: event.m_target_unit_flags,
            timer: event.m_timer,
            tag: event.m_tag,
            snapshot_unit_link: event.m_snapshot_unit_link,
            snapshot_control_player_id: event.m_snapshot_control_player_id,
            snapshot_upkeep_player_id: event.m_snapshot_upkeep_player_id,
            snapshot_point: event.m_snapshot_point.into(),
        }
    }
}
