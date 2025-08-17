//! Handles the Cmd related events

use super::*;
use crate::game_events::ReplayGameEvent;

impl From<GameSCmdEvent> for ReplayGameEvent {
    fn from(source: GameSCmdEvent) -> ReplayGameEvent {
        let m_abil = source.m_abil.map(|val| val.into());
        ReplayGameEvent::Cmd(crate::game_events::GameSCmdEvent {
            m_cmd_flags: source.m_cmd_flags,
            m_abil,
            m_data: source.m_data.into(),
            m_sequence: source.m_sequence,
            m_other_unit: source.m_other_unit.map(|u| u.value.value as u32),
            m_unit_group: source.m_unit_group.map(|g| g.value as u32),
        })
    }
}

impl From<GameSCmdAbil> for crate::game_events::GameSCmdAbil {
    fn from(source: GameSCmdAbil) -> crate::game_events::GameSCmdAbil {
        crate::game_events::GameSCmdAbil {
            m_abil_link: source.m_abil_link.value.clone().into(),
            ability: String::new(),
            m_abil_cmd_index: source.m_abil_cmd_index,
            m_abil_cmd_data: source.m_abil_cmd_data.map(|d| d.value as u8),
        }
    }
}

impl From<GameSCmdData> for crate::game_events::GameSCmdData {
    fn from(source: GameSCmdData) -> crate::game_events::GameSCmdData {
        match source {
            GameSCmdData::None(()) => crate::game_events::GameSCmdData::None,
            GameSCmdData::TargetPoint(val) => {
                crate::game_events::GameSCmdData::TargetPoint(val.into())
            }
            GameSCmdData::TargetUnit(val) => {
                crate::game_events::GameSCmdData::TargetUnit(val.into())
            }
            GameSCmdData::Data(val) => crate::game_events::GameSCmdData::Data(val.into()),
        }
    }
}

impl From<GameSCmdDataTargetUnit> for crate::game_events::GameSCmdDataTargetUnit {
    fn from(source: GameSCmdDataTargetUnit) -> crate::game_events::GameSCmdDataTargetUnit {
        crate::game_events::GameSCmdDataTargetUnit {
            m_target_unit_flags: source.m_target_unit_flags.value as u16,
            m_timer: source.m_timer.value as u8,
            m_tag: source.m_tag.value.into(),
            m_snapshot_unit_link: source.m_snapshot_unit_link.value.into(),
            m_snapshot_control_player_id: source.m_snapshot_control_player_id.map(|p| p.value),
            m_snapshot_upkeep_player_id: source.m_snapshot_upkeep_player_id.map(|p| p.value),
            m_snapshot_point: source.m_snapshot_point.into(),
        }
    }
}

impl From<GameSCmdUpdateTargetPointEvent> for ReplayGameEvent {
    fn from(source: GameSCmdUpdateTargetPointEvent) -> ReplayGameEvent {
        ReplayGameEvent::CmdUpdateTargetPoint(crate::game_events::GameSCmdUpdateTargetPointEvent {
            m_target: source.m_target.into(),
        })
    }
}

impl From<GameSCmdUpdateTargetUnitEvent> for ReplayGameEvent {
    fn from(source: GameSCmdUpdateTargetUnitEvent) -> ReplayGameEvent {
        ReplayGameEvent::CmdUpdateTargetUnit(crate::game_events::GameSCmdUpdateTargetUnitEvent {
            m_target: source.m_target.into(),
        })
    }
}

impl From<GameSCommandManagerStateEvent> for ReplayGameEvent {
    fn from(source: GameSCommandManagerStateEvent) -> ReplayGameEvent {
        ReplayGameEvent::CommandManagerState(crate::game_events::GameSCommandManagerStateEvent {
            m_state: source.m_state.into(),
            m_sequence: source.m_sequence,
        })
    }
}

impl From<GameECommandManagerState> for crate::game_events::GameECommandManagerState {
    fn from(source: GameECommandManagerState) -> crate::game_events::GameECommandManagerState {
        match source {
            GameECommandManagerState::EFireDone => {
                crate::game_events::GameECommandManagerState::EFireDone
            }
            GameECommandManagerState::EFireOnce => {
                crate::game_events::GameECommandManagerState::EFireOnce
            }
            GameECommandManagerState::EFireMany => {
                crate::game_events::GameECommandManagerState::EFireMany
            }
        }
    }
}
