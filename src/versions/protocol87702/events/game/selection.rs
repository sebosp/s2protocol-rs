//! Selection Group changes

use super::*;

impl TryFrom<GameSSelectionDeltaEvent> for game_events::ReplayGameEvent {
    type Error = S2ProtocolError;
    fn try_from(source: GameSSelectionDeltaEvent) -> Result<Self, Self::Error> {
        Ok(ReplayGameEvent::SelectionDelta(
            game_events::GameSSelectionDeltaEvent {
                m_control_group_id: source.m_control_group_id.into(),
                m_delta: source.m_delta.try_into()?,
            },
        ))
    }
}
impl TryFrom<GameSelectionMaskType> for game_events::GameSelectionMaskType {
    type Error = S2ProtocolError;
    fn try_from(source: GameSelectionMaskType) -> Result<Self, Self::Error> {
        Ok(Self {
            value: str::from_utf8(&source.value)?.to_string(),
        })
    }
}

impl TryFrom<GameSSelectionDelta> for game_events::GameSSelectionDelta {
    type Error = S2ProtocolError;
    fn try_from(source: GameSSelectionDelta) -> Result<Self, Self::Error> {
        let mut m_add_subgroups = vec![];
        for group in source.m_add_subgroups {
            m_add_subgroups.push(group.into());
        }
        let mut m_add_unit_tags = vec![];
        for unit in source.m_add_unit_tags {
            m_add_unit_tags.push(unit.into());
        }
        Ok(Self {
            m_subgroup_index: source.m_subgroup_index.into(),
            m_remove_mask: source.m_remove_mask.try_into()?,
            m_add_subgroups,
            m_add_unit_tags,
        })
    }
}

impl TryFrom<GameSSelectionMask> for game_events::GameSSelectionMask {
    type Error = S2ProtocolError;
    fn try_from(source: GameSSelectionMask) -> Result<Self, Self::Error> {
        match source {
            GameSSelectionMask::None(()) => Ok(game_events::GameSSelectionMask::None),
            GameSSelectionMask::Mask(val) => {
                Ok(game_events::GameSSelectionMask::Mask(val.try_into()?))
            }
            GameSSelectionMask::OneIndices(val) => {
                Ok(game_events::GameSSelectionMask::OneIndices(val.into()))
            }
            GameSSelectionMask::ZeroIndices(val) => {
                Ok(game_events::GameSSelectionMask::ZeroIndices(val.into()))
            }
        }
    }
}

impl From<GameTControlGroupId> for game_events::GameTControlGroupId {
    fn from(source: GameTControlGroupId) -> game_events::GameTControlGroupId {
        source.value as u8
    }
}

impl From<GameTSubgroupIndex> for game_events::GameTSubgroupIndex {
    fn from(source: GameTSubgroupIndex) -> game_events::GameTSubgroupIndex {
        source.value as u16
    }
}

impl From<GameTSubgroupPriority> for game_events::GameTSubgroupPriority {
    fn from(source: GameTSubgroupPriority) -> game_events::GameTSubgroupPriority {
        source.value as u8
    }
}

impl From<GameTSelectionCount> for game_events::GameTSelectionCount {
    fn from(source: GameTSelectionCount) -> game_events::GameTSelectionCount {
        source.value as u16
    }
}

impl From<GameTSubgroupCount> for game_events::GameTSubgroupCount {
    fn from(source: GameTSubgroupCount) -> game_events::GameTSubgroupCount {
        source.value as u16
    }
}

impl From<GameSSelectionDeltaSubgroup> for game_events::GameSSelectionDeltaSubgroup {
    fn from(source: GameSSelectionDeltaSubgroup) -> game_events::GameSSelectionDeltaSubgroup {
        game_events::GameSSelectionDeltaSubgroup {
            m_unit_link: source.m_unit_link.into(),
            m_subgroup_priority: source.m_subgroup_priority.into(),
            m_intra_subgroup_priority: source.m_intra_subgroup_priority.into(),
            m_count: source.m_count.into(),
        }
    }
}

impl From<GameSelectionIndexArrayType> for game_events::GameSelectionIndexArrayType {
    fn from(source: GameSelectionIndexArrayType) -> game_events::GameSelectionIndexArrayType {
        let mut res = vec![];
        for val in source.value {
            res.push(val.into());
        }
        game_events::GameSelectionIndexArrayType { value: res }
    }
}

impl From<GameTSyncChecksum> for game_events::GameTSyncChecksum {
    fn from(source: GameTSyncChecksum) -> game_events::GameTSyncChecksum {
        source.value.into()
    }
}

impl From<GameSSelectionSyncCheckEvent> for game_events::ReplayGameEvent {
    fn from(source: GameSSelectionSyncCheckEvent) -> game_events::ReplayGameEvent {
        game_events::ReplayGameEvent::SelectionSyncCheck(
            game_events::GameSSelectionSyncCheckEvent {
                m_control_group_id: source.m_control_group_id.into(),
                m_selection_sync_data: source.m_selection_sync_data.into(),
            },
        )
    }
}

impl From<GameSSelectionSyncData> for game_events::GameSSelectionSyncData {
    fn from(source: GameSSelectionSyncData) -> game_events::GameSSelectionSyncData {
        game_events::GameSSelectionSyncData {
            m_count: source.m_count.into(),
            m_subgroup_count: source.m_subgroup_count.into(),
            m_active_subgroup_index: source.m_active_subgroup_index.into(),
            m_unit_tags_checksum: source.m_unit_tags_checksum.value.into(),
            m_subgroup_indices_checksum: source.m_subgroup_indices_checksum.value.into(),
            m_subgroups_checksum: source.m_subgroups_checksum.value.into(),
        }
    }
}
