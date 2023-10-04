//! Decodes the Game Events.
//! These are stored in an embedded file in the MPQ file called 'replay.game.events'

use serde::{Deserialize, Serialize};
pub mod state;
pub use state::*;
pub mod iterator;
pub use iterator::*;

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use crate::SC2ReplayFilters;

pub type TUserId = u8;
pub type GameTUnitTag = u32;
pub type GameTUnitLink = u16;
pub type GameTPlayerId = i64;
pub type GameTMapCoordFixedBits = i64;
pub type GameTFixedBits = i32;
pub type GameTAbilLink = u16;
pub type GameTFixedMiniBitsSigned = i16;
pub type GameTFixedMiniBitsUnsigned = i64;
pub type GameTControlGroupId = u8;
pub type GameTSubgroupIndex = u16;
pub type GameTSelectionIndex = u16;
pub type GameTSubgroupPriority = u8;
pub type GameTSelectionCount = u16;
pub type GameTSubgroupCount = u16;
pub type GameTControlGroupIndex = u8;
pub type GameTControlGroupCount = u8;
pub type GameTSyncChecksum = u32;
pub type GameTButtonLink = u16;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameEvent {
    pub delta: i64,
    pub user_id: i64,
    pub event: ReplayGameEvent,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum ReplayGameEvent {
    DropUser(DropUserEvent),
    CameraSave(CameraSaveEvent),
    Cmd(GameSCmdEvent),
    SelectionDelta(GameSSelectionDeltaEvent),
    ControlGroupUpdate(GameSControlGroupUpdateEvent),
    SelectionSyncCheck(GameSSelectionSyncCheckEvent),
    TriggerChatMessage(GameSTriggerChatMessageEvent),
    UnitClick(GameSUnitClickEvent),
    UnitHighlight(GameSUnitHighlightEvent),
    TriggerReplySelected(GameSTriggerReplySelectedEvent),
    CameraUpdate(CameraUpdateEvent),
    TriggerMouseClicked(GameSTriggerMouseClickedEvent),
    TriggerMouseMoved(GameSTriggerMouseMovedEvent),
    TriggerHotkeyPressed(GameSTriggerHotkeyPressedEvent),
    TriggerTargetModeUpdate(GameSTriggerTargetModeUpdateEvent),
    TriggerKeyPressed(GameSTriggerKeyPressedEvent),
    TriggerMouseWheel(GameSTriggerMouseWheelEvent),
    TriggerButtonPressed(GameSTriggerButtonPressedEvent),
    /*GameUserLeave(GameSGameUserLeaveEvent),
    GameUserJoin(GameSGameUserJoinEvent),*/
    CommandManagerState(GameSCommandManagerStateEvent),
    CmdUpdateTargetPoint(GameSCmdUpdateTargetPointEvent),
    CmdUpdateTargetUnit(GameSCmdUpdateTargetUnitEvent),
    /*TriggerAnimLengthQueryByName(GameSTriggerAnimLengthQueryByNameEvent),
    TriggerAnimLengthQueryByProps(GameSTriggerAnimLengthQueryByPropsEvent),*/
}

impl ReplayGameEvent {
    pub fn should_skip(&self, _filters: &SC2ReplayFilters) -> bool {
        // for now we do not filter GameEvents
        false
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum ELeaveReason {
    UserLeft,
    UserDropped,
    UserBanned,
    UserVictory,
    UserDefeat,
    UserTied,
    UserDesynced,
    UserOutOfTime,
    WeWereUnresponsive,
    WeContinuedAlone,
    ReplayDesynced,
    UserTimeout,
    UserDisconnected,
    Unrecoverable,
    UserCatchupDesynced,
    TakeCommandDropped,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct DropUserEvent {
    pub m_drop_session_user_id: TUserId,
    pub m_reason: ELeaveReason,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct CameraSaveEvent {
    pub m_which: i64,
    pub m_target: GameSPointMini,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct CameraUpdateEvent {
    pub m_target: Option<GameSPointMini>,
    pub m_distance: Option<GameTFixedMiniBitsUnsigned>,
    pub m_pitch: Option<GameTFixedMiniBitsUnsigned>,
    pub m_yaw: Option<GameTFixedMiniBitsUnsigned>,
    pub m_reason: Option<i8>,
    pub m_follow: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSPointMini {
    pub x: GameTFixedMiniBitsUnsigned,
    pub y: GameTFixedMiniBitsUnsigned,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSCmdEvent {
    pub m_cmd_flags: i64,
    pub m_abil: Option<GameSCmdAbil>,
    pub m_data: GameSCmdData,
    pub m_sequence: i64,
    pub m_other_unit: Option<GameTUnitTag>,
    pub m_unit_group: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSCmdAbil {
    pub m_abil_link: GameTAbilLink,
    pub m_abil_cmd_index: i64,
    pub m_abil_cmd_data: Option<u8>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum GameSCmdData {
    None,
    TargetPoint(GameSMapCoord3D),
    TargetUnit(GameSCmdDataTargetUnit),
    Data(u32),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSMapCoord3D {
    pub x: GameTMapCoordFixedBits,
    pub y: GameTMapCoordFixedBits,
    pub z: GameTFixedBits,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSCmdDataTargetUnit {
    pub m_target_unit_flags: u16,
    pub m_timer: u8,
    pub m_tag: GameTUnitTag,
    pub m_snapshot_unit_link: GameTUnitLink,
    pub m_snapshot_control_player_id: Option<GameTPlayerId>,
    pub m_snapshot_upkeep_player_id: Option<GameTPlayerId>,
    pub m_snapshot_point: GameSMapCoord3D,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSCmdUpdateTargetPointEvent {
    pub m_target: GameSMapCoord3D,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSCmdUpdateTargetUnitEvent {
    pub m_target: GameSCmdDataTargetUnit,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSTriggerMouseClickedEvent {
    pub m_button: u32,
    pub m_down: bool,
    pub m_pos_ui: GameSuiCoord,
    pub m_pos_world: GameSMapCoord3D,
    pub m_flags: i8,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSTriggerMouseMovedEvent {
    pub m_pos_ui: GameSuiCoord,
    pub m_pos_world: GameSMapCoord3D,
    pub m_flags: i8,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSuiCoord {
    pub x: u16,
    pub y: u16,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSTriggerMouseWheelEvent {
    pub m_wheel_spin: GameTFixedMiniBitsSigned,
    pub m_flags: i8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSUnitClickEvent {
    pub m_unit_tag: GameTUnitTag,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSUnitHighlightEvent {
    pub m_unit_tag: GameTUnitTag,
    pub m_flags: u8,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSSelectionDeltaEvent {
    pub m_control_group_id: GameTControlGroupId,
    pub m_delta: GameSSelectionDelta,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSSelectionDelta {
    pub m_subgroup_index: GameTSubgroupIndex,
    pub m_remove_mask: GameSSelectionMask,
    pub m_add_subgroups: Vec<GameSSelectionDeltaSubgroup>,
    pub m_add_unit_tags: Vec<GameTUnitTag>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSSelectionDeltaSubgroup {
    pub m_unit_link: GameTUnitLink,
    pub m_subgroup_priority: GameTSubgroupPriority,
    pub m_intra_subgroup_priority: GameTSubgroupPriority,
    pub m_count: GameTSelectionCount,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum GameSSelectionMask {
    None,
    Mask(GameSelectionMaskType),
    OneIndices(GameSelectionIndexArrayType),
    ZeroIndices(GameSelectionIndexArrayType),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSelectionMaskType {
    pub value: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSelectionIndexArrayType {
    pub value: Vec<GameTSelectionIndex>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSSelectionSyncCheckEvent {
    pub m_control_group_id: GameTControlGroupId,
    pub m_selection_sync_data: GameSSelectionSyncData,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSSelectionSyncData {
    pub m_count: GameTSelectionCount,
    pub m_subgroup_count: GameTSubgroupCount,
    pub m_active_subgroup_index: GameTSubgroupIndex,
    pub m_unit_tags_checksum: GameTSyncChecksum,
    pub m_subgroup_indices_checksum: GameTSyncChecksum,
    pub m_subgroups_checksum: GameTSyncChecksum,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSControlGroupUpdateEvent {
    pub m_control_group_index: GameTControlGroupIndex,
    pub m_control_group_update: GameEControlGroupUpdate,
    pub m_mask: GameSSelectionMask,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum GameEControlGroupUpdate {
    ESet,
    EAppend,
    ERecall,
    EClear,
    ESetAndSteal,
    EAppendAndSteal,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSTriggerChatMessageEvent {
    pub m_chat_message: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSTriggerReplySelectedEvent {
    pub m_conversation_id: i32,
    pub m_reply_id: i32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSTriggerHotkeyPressedEvent {
    pub m_hotkey: u32,
    pub m_down: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSTriggerTargetModeUpdateEvent {
    pub m_abil_link: GameTAbilLink,
    pub m_abil_cmd_index: i64,
    pub m_state: i8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSTriggerKeyPressedEvent {
    pub m_key: i8,
    pub m_flags: i8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSTriggerButtonPressedEvent {
    pub m_button: GameTButtonLink,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
pub struct GameSCommandManagerStateEvent {
    pub m_state: GameECommandManagerState,
    pub m_sequence: Option<i64>,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum GameECommandManagerState {
    EFireDone,
    EFireOnce,
    EFireMany,
}
