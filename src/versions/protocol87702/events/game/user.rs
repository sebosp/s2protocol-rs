//! User specific events such as user dropped, joined, etc.
//!
use super::*;
use crate::game_events::ReplayGameEvent;

impl From<GameSDropUserEvent> for ReplayGameEvent {
    fn from(source: GameSDropUserEvent) -> ReplayGameEvent {
        ReplayGameEvent::DropUser(crate::game_events::DropUserEvent {
            m_drop_session_user_id: source.m_drop_session_user_id.into(),
            m_reason: source.m_reason.into(),
        })
    }
}

impl From<ELeaveReason> for crate::game_events::ELeaveReason {
    fn from(source: ELeaveReason) -> crate::game_events::ELeaveReason {
        match source {
            ELeaveReason::EUserLeft => crate::game_events::ELeaveReason::UserLeft,
            ELeaveReason::EUserDropped => crate::game_events::ELeaveReason::UserDropped,
            ELeaveReason::EUserBanned => crate::game_events::ELeaveReason::UserBanned,
            ELeaveReason::EUserVictory => crate::game_events::ELeaveReason::UserVictory,
            ELeaveReason::EUserDefeat => crate::game_events::ELeaveReason::UserDefeat,
            ELeaveReason::EUserTied => crate::game_events::ELeaveReason::UserTied,
            ELeaveReason::EUserDesynced => crate::game_events::ELeaveReason::UserDesynced,
            ELeaveReason::EUserOutOfTime => crate::game_events::ELeaveReason::UserOutOfTime,
            ELeaveReason::EWeWereUnresponsive => {
                crate::game_events::ELeaveReason::WeWereUnresponsive
            }
            ELeaveReason::EWeContinuedAlone => crate::game_events::ELeaveReason::WeContinuedAlone,
            ELeaveReason::EReplayDesynced => crate::game_events::ELeaveReason::ReplayDesynced,
            ELeaveReason::EUserTimeout => crate::game_events::ELeaveReason::UserTimeout,
            ELeaveReason::EUserDisconnected => crate::game_events::ELeaveReason::UserDisconnected,
            ELeaveReason::EUnrecoverable => crate::game_events::ELeaveReason::Unrecoverable,
            ELeaveReason::EUserCatchupDesynced => {
                crate::game_events::ELeaveReason::UserCatchupDesynced
            }
            ELeaveReason::ETakeCommandDropped => {
                crate::game_events::ELeaveReason::TakeCommandDropped
            }
        }
    }
}
