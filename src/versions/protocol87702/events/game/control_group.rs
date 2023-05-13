//! Control Group supported

use super::*;
use crate::game_events::ReplayGameEvent;

impl TryFrom<GameSControlGroupUpdateEvent> for ReplayGameEvent {
    type Error = GameEventError;
    fn try_from(source: GameSControlGroupUpdateEvent) -> Result<Self, Self::Error> {
        Ok(ReplayGameEvent::ControlGroupUpdate(
            crate::game_events::GameSControlGroupUpdateEvent {
                m_control_group_index: source.m_control_group_index.into(),
                m_control_group_update: source.m_control_group_update.into(),
                m_mask: source.m_mask.try_into()?,
            },
        ))
    }
}

impl From<GameEControlGroupUpdate> for crate::game_events::GameEControlGroupUpdate {
    fn from(source: GameEControlGroupUpdate) -> crate::game_events::GameEControlGroupUpdate {
        match source {
            GameEControlGroupUpdate::ESet => crate::game_events::GameEControlGroupUpdate::ESet,
            GameEControlGroupUpdate::EAppend => {
                crate::game_events::GameEControlGroupUpdate::EAppend
            }
            GameEControlGroupUpdate::ERecall => {
                crate::game_events::GameEControlGroupUpdate::ERecall
            }
            GameEControlGroupUpdate::EClear => crate::game_events::GameEControlGroupUpdate::EClear,
            GameEControlGroupUpdate::ESetAndSteal => {
                crate::game_events::GameEControlGroupUpdate::ESetAndSteal
            }
            GameEControlGroupUpdate::EAppendAndSteal => {
                crate::game_events::GameEControlGroupUpdate::EAppendAndSteal
            }
        }
    }
}

impl From<GameTControlGroupIndex> for game_events::GameTControlGroupIndex {
    fn from(source: GameTControlGroupIndex) -> game_events::GameTControlGroupIndex {
        source.value as u8
    }
}
