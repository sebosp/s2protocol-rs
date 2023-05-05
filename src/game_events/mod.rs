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

pub type GameTUnitTag = u32;
pub type GameTUnitLink = u16;
pub type GameTPlayerId = i64;
pub type GameTMapCoordFixedBits = i64;
pub type GameTFixedBits = i32;
pub type GameTAbilLink = i32;
pub type GameTFixedMiniBitsSigned = i16;
pub type GameTFixedMiniBitsUnsigned = i64;
pub type GameTControlGroupId = u8;
pub type GameTSubgroupIndex = u16;
pub type GameTSelectionIndex = u16;
pub type GameTSubgroupPriority = u8;
pub type GameTSelectionCount = u16;

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
    SelectionDelta(GameSSelectionDeltaEvent),
    /*ControlGroupUpdate(GameSControlGroupUpdateEvent),
    SelectionSyncCheck(GameSSelectionSyncCheckEvent),
    TriggerChatMessage(GameSTriggerChatMessageEvent),*/
    UnitClick(GameSUnitClickEvent),
    UnitHighlight(GameSUnitHighlightEvent),
    /*TriggerReplySelected(GameSTriggerReplySelectedEvent),*/
    CameraUpdate(CameraUpdateEvent),
    TriggerMouseClicked(GameSTriggerMouseClickedEvent),
    TriggerMouseMoved(GameSTriggerMouseMovedEvent),
    /*TriggerHotkeyPressed(GameSTriggerHotkeyPressedEvent),
    TriggerTargetModeUpdate(GameSTriggerTargetModeUpdateEvent),
    TriggerKeyPressed(GameSTriggerKeyPressedEvent),*/
    TriggerMouseWheel(GameSTriggerMouseWheelEvent),
    /*TriggerButtonPressed(GameSTriggerButtonPressedEvent),
    GameUserLeave(GameSGameUserLeaveEvent),
    GameUserJoin(GameSGameUserJoinEvent),
    CommandManagerState(GameSCommandManagerStateEvent),*/
    CmdUpdateTargetPoint(GameSCmdUpdateTargetPointEvent),
    CmdUpdateTargetUnit(GameSCmdUpdateTargetUnitEvent),
    /*TriggerAnimLengthQueryByName(GameSTriggerAnimLengthQueryByNameEvent),
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

#[derive(Debug, PartialEq, Clone)]
pub struct GameSCmdUpdateTargetPointEvent {
    pub m_target: GameSMapCoord3D,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameSCmdUpdateTargetUnitEvent {
    pub m_target: GameSCmdDataTargetUnit,
}
#[derive(Debug, PartialEq, Clone)]
pub struct GameSTriggerMouseClickedEvent {
    pub m_button: u32,
    pub m_down: bool,
    pub m_pos_ui: GameSuiCoord,
    pub m_pos_world: GameSMapCoord3D,
    pub m_flags: i8,
}
#[derive(Debug, PartialEq, Clone)]
pub struct GameSTriggerMouseMovedEvent {
    pub m_pos_ui: GameSuiCoord,
    pub m_pos_world: GameSMapCoord3D,
    pub m_flags: i8,
}
#[derive(Debug, PartialEq, Clone)]
pub struct GameSuiCoord {
    pub x: u16,
    pub y: u16,
}
#[derive(Debug, PartialEq, Clone)]
pub struct GameSTriggerMouseWheelEvent {
    pub m_wheel_spin: GameTFixedMiniBitsSigned,
    pub m_flags: i8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameSUnitClickEvent {
    pub m_unit_tag: GameTUnitTag,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameSUnitHighlightEvent {
    pub m_unit_tag: GameTUnitTag,
    pub m_flags: u8,
}
#[derive(Debug, PartialEq, Clone)]
pub struct GameSSelectionDeltaEvent {
    pub m_control_group_id: GameTControlGroupId,
    pub m_delta: GameSSelectionDelta,
}
#[derive(Debug, PartialEq, Clone)]
pub struct GameSSelectionDelta {
    pub m_subgroup_index: GameTSubgroupIndex,
    pub m_remove_mask: GameSSelectionMask,
    pub m_add_subgroups: Vec<GameSSelectionDeltaSubgroup>,
    pub m_add_unit_tags: Vec<GameTUnitTag>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameSSelectionDeltaSubgroup {
    pub m_unit_link: GameTUnitLink,
    pub m_subgroup_priority: GameTSubgroupPriority,
    pub m_intra_subgroup_priority: GameTSubgroupPriority,
    pub m_count: GameTSelectionCount,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameSSelectionMask {
    None,
    Mask(GameSelectionMaskType),
    OneIndices(GameSelectionIndexArrayType),
    ZeroIndices(GameSelectionIndexArrayType),
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameSelectionMaskType {
    // Maybe needs to be Vec<u8>, trying as String first
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameSelectionIndexArrayType {
    pub value: Vec<GameTSelectionIndex>,
}