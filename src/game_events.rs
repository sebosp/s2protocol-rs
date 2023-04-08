//! Decodes the Game Events.
//! These are stored in an embebdded file in the MPQ file called 'replay.game.events'

use std::str::Utf8Error;

/// A list of errors when handling GameEvents
#[derive(Debug, thiserror::Error)]
pub enum GameEventError {
    /// An error to be used in TryFrom, when converting from protocol-specific types into our
    /// consolidated-types
    #[error("Unsupported Event Type")]
    UnsupportedEventType,
    /// Conversion to UTF-8 failed, from the Vec<u8> _name fields in the proto fields
    #[error("Utf8 conversion error")]
    Utf8Error(#[from] Utf8Error),
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameEvent {
    pub delta: i64,
    pub user_id: i64,
    pub event: ReplayGameEvent,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReplayGameEvent {
    CameraSave(CameraSaveEvent),
    Cmd(GameSCmdEvent),
    /*SelectionDelta(GameSSelectionDeltaEvent),
    ControlGroupUpdate(GameSControlGroupUpdateEvent),
    SelectionSyncCheck(GameSSelectionSyncCheckEvent),
    TriggerChatMessage(GameSTriggerChatMessageEvent),
    UnitClick(GameSUnitClickEvent),
    UnitHighlight(GameSUnitHighlightEvent),
    TriggerReplySelected(GameSTriggerReplySelectedEvent),*/
    CameraUpdate(CameraUpdateEvent),
    /*TriggerMouseClicked(GameSTriggerMouseClickedEvent),
    TriggerMouseMoved(GameSTriggerMouseMovedEvent),
    TriggerHotkeyPressed(GameSTriggerHotkeyPressedEvent),
    TriggerTargetModeUpdate(GameSTriggerTargetModeUpdateEvent),
    TriggerKeyPressed(GameSTriggerKeyPressedEvent),
    TriggerMouseWheel(GameSTriggerMouseWheelEvent),
    TriggerButtonPressed(GameSTriggerButtonPressedEvent),
    GameUserLeave(GameSGameUserLeaveEvent),
    GameUserJoin(GameSGameUserJoinEvent),
    CommandManagerState(GameSCommandManagerStateEvent),
    CmdUpdateTargetPoint(GameSCmdUpdateTargetPointEvent),
    CmdUpdateTargetUnit(GameSCmdUpdateTargetUnitEvent),
    TriggerAnimLengthQueryByName(GameSTriggerAnimLengthQueryByNameEvent),
    TriggerAnimLengthQueryByProps(GameSTriggerAnimLengthQueryByPropsEvent),*/
}

#[derive(Debug, PartialEq, Clone)]
pub struct CameraSaveEvent {
    pub m_which: i64,
    pub m_target: GameSPointMini,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CameraUpdateEvent {
    pub m_target: Option<GameSPointMini>,
    pub m_distance: Option<GameTFixedMiniBitsUnsigned>,
    pub m_pitch: Option<GameTFixedMiniBitsUnsigned>,
    pub m_yaw: Option<GameTFixedMiniBitsUnsigned>,
    pub m_reason: Option<i8>,
    pub m_follow: bool,
}

pub type GameTFixedMiniBitsUnsigned = i64;

#[derive(Debug, PartialEq, Clone)]
pub struct GameSPointMini {
    pub x: GameTFixedMiniBitsUnsigned,
    pub y: GameTFixedMiniBitsUnsigned,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameSCmdEvent {
    pub m_cmd_flags: i64,
    pub m_abil: Option<GameSCmdAbil>,
    pub m_data: GameSCmdData,
    pub m_sequence: i64,
    pub m_other_unit: Option<GameTUnitTag>,
    pub m_unit_group: Option<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameSCmdAbil {
    pub m_abil_link: GameTAbilLink,
    pub m_abil_cmd_index: i64,
    pub m_abil_cmd_data: Option<u8>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameSCmdData {
    None,
    TargetPoint(GameSMapCoord3D),
    TargetUnit(GameSCmdDataTargetUnit),
    Data(u32),
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameSMapCoord3D {
    pub x: GameTMapCoordFixedBits,
    pub y: GameTMapCoordFixedBits,
    pub z: GameTFixedBits,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameSCmdDataTargetUnit {
    pub m_target_unit_flags: u16,
    pub m_timer: u8,
    pub m_tag: GameTUnitTag,
    pub m_snapshot_unit_link: GameTUnitLink,
    pub m_snapshot_control_player_id: Option<GameTPlayerId>,
    pub m_snapshot_upkeep_player_id: Option<GameTPlayerId>,
    pub m_snapshot_point: GameSMapCoord3D,
}

pub type GameTUnitTag = u32;
pub type GameTUnitLink = u16;
pub type GameTPlayerId = i64;
pub type GameTMapCoordFixedBits = i64;
pub type GameTFixedBits = i32;
pub type GameTAbilLink = i32;
