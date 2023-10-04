//! Handles the Mouse related events

use super::*;
use crate::game_events::ReplayGameEvent;

impl From<GameSTriggerMouseClickedEvent> for ReplayGameEvent {
    fn from(source: GameSTriggerMouseClickedEvent) -> ReplayGameEvent {
        ReplayGameEvent::TriggerMouseClicked(crate::game_events::GameSTriggerMouseClickedEvent {
            m_button: source.m_button.into(),
            m_down: source.m_down,
            m_pos_ui: source.m_pos_ui.into(),
            m_pos_world: source.m_pos_world.into(),
            m_flags: source.m_flags.into(),
        })
    }
}

impl From<GameSTriggerMouseMovedEvent> for ReplayGameEvent {
    fn from(source: GameSTriggerMouseMovedEvent) -> ReplayGameEvent {
        ReplayGameEvent::TriggerMouseMoved(crate::game_events::GameSTriggerMouseMovedEvent {
            m_pos_ui: source.m_pos_ui.into(),
            m_pos_world: source.m_pos_world.into(),
            m_flags: source.m_flags.into(),
        })
    }
}

impl From<GameSTriggerMouseWheelEvent> for ReplayGameEvent {
    fn from(source: GameSTriggerMouseWheelEvent) -> ReplayGameEvent {
        ReplayGameEvent::TriggerMouseWheel(crate::game_events::GameSTriggerMouseWheelEvent {
            m_wheel_spin: source.m_wheel_spin.into(),
            m_flags: source.m_flags.into(),
        })
    }
}
