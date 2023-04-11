//! Handles the Unit Click/Highlight Events
//!
use super::*;
use crate::game_events::ReplayGameEvent;

impl From<GameSUnitClickEvent> for ReplayGameEvent {
    fn from(source: GameSUnitClickEvent) -> ReplayGameEvent {
        ReplayGameEvent::UnitClick(crate::game_events::GameSUnitClickEvent {
            m_unit_tag: source.m_unit_tag.value.into(),
        })
    }
}

impl From<GameSUnitHighlightEvent> for ReplayGameEvent {
    fn from(source: GameSUnitHighlightEvent) -> ReplayGameEvent {
        ReplayGameEvent::UnitHighlight(crate::game_events::GameSUnitHighlightEvent {
            m_unit_tag: source.m_unit_tag.value.into(),
            m_flags: source.m_flags.into(),
        })
    }
}
