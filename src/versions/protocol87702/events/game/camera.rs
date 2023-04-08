//! Handles the Camera related events

use super::*;
use crate::game_events::{CameraSaveEvent, CameraUpdateEvent, ReplayGameEvent};

impl From<GameSCameraSaveEvent> for ReplayGameEvent {
    fn from(source: GameSCameraSaveEvent) -> ReplayGameEvent {
        ReplayGameEvent::CameraSave(CameraSaveEvent {
            m_which: source.m_which,
            m_target: source.m_target.into(),
        })
    }
}

impl From<GameSCameraUpdateEvent> for ReplayGameEvent {
    fn from(source: GameSCameraUpdateEvent) -> ReplayGameEvent {
        ReplayGameEvent::CameraUpdate(CameraUpdateEvent {
            m_target: source.m_target.map(|x| x.into()),
            m_distance: source.m_distance.map(|x| x.into()),
            m_pitch: source.m_pitch.map(|x| x.into()),
            m_yaw: source.m_yaw.map(|x| x.into()),
            m_reason: source.m_reason.map(|x| x.into()),
            m_follow: source.m_follow,
        })
    }
}
