//! Handles the state change of units for GameEvents

use super::*;
use crate::game_events::*;
use crate::tracker_events::unit_tag_index;
use crate::*;

/// The game events seem to be at this ratio when compared to Tracker Events.
pub const GAME_EVENT_POS_RATIO: f32 = 4096f32;

pub fn handle_cmd(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    game_cmd: &GameSCmdEvent,
) -> Vec<u32> {
    match &game_cmd.m_data {
        GameSCmdData::TargetPoint(target) => {
            handle_update_target_point(sc2_state, game_loop, user_id, target)
        }
        GameSCmdData::TargetUnit(target_unit) => {
            handle_update_target_unit(sc2_state, game_loop, user_id, target_unit)
        }
        GameSCmdData::Data(data) => {
            // Unknown for now.
            tracing::info!("GameSCmdData::Data: {}", data);
            vec![]
        }
        GameSCmdData::None => vec![],
    }
}

pub fn handle_update_target_point(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    target_point: &GameSMapCoord3D,
) -> Vec<u32> {
    let unit_target_pos = Vec3D::new(
        target_point.x as f32 / GAME_EVENT_POS_RATIO,
        -1. * target_point.y as f32 / GAME_EVENT_POS_RATIO,
        target_point.z as f32 / GAME_EVENT_POS_RATIO,
    );
    let mut user_selected_units: Vec<u32> = vec![];
    if let Some(state) = sc2_state.user_state.get(&user_id) {
        user_selected_units = state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone();
    }
    for selected_unit in user_selected_units {
        let unit_index = unit_tag_index(selected_unit as i64);
        if let Some(ref mut registered_unit) = sc2_state.units.get_mut(&unit_index) {
            registered_unit.target = Some(unit_target_pos);
        }
    }
    user_selected_units
}

/// Handles the change of target for a unit.
/// The unit is previously selected and is part of the ACTIVE_UNITS_GROUP_IDX,
/// then a TargetUnit event is emitted.
pub fn handle_update_target_unit(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    target_unit: &GameSCmdDataTargetUnit,
) -> Vec<u32> {
    let unit_target_pos = Vec3D::new(
        target_unit.m_snapshot_point.x as f32 / GAME_EVENT_POS_RATIO,
        -1. * target_unit.m_snapshot_point.y as f32 / GAME_EVENT_POS_RATIO,
        target_unit.m_snapshot_point.z as f32 / GAME_EVENT_POS_RATIO,
    );
    let mut user_selected_units: Vec<u32> = vec![];
    if let Some(state) = sc2_state.user_state.get(&user_id) {
        user_selected_units = state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone();
    }
    for selected_unit in user_selected_units {
        let unit_index = unit_tag_index(selected_unit as i64);
        if let Some(ref mut registered_unit) = sc2_state.units.get_mut(&unit_index) {
            registered_unit.target = Some(unit_target_pos);
        }
    }
    user_selected_units
}

/// Removes the changes to the units that signify they are selected.
pub fn unmark_previously_selected_units(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
) -> Vec<u32> {
    let mut updated_units = vec![];
    if let Some(state) = sc2_state.user_state.get_mut(&user_id) {
        for prev_unit in &state.control_groups[ACTIVE_UNITS_GROUP_IDX] {
            let unit_index = unit_tag_index(*prev_unit as i64);
            if let Some(ref mut unit) = sc2_state.units.get_mut(&unit_index) {
                if unit.is_selected {
                    unit.is_selected = false;
                    unit.radius = unit.radius * 0.5;
                    updated_units.push(unit_index);
                }
            }
        }
    }
    updated_units
}

/// Marks a group of units as selected by increasing the radius.
pub fn mark_selected_units(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    _user_id: i64,
    selected_units: &[u32],
) -> Vec<u32> {
    let mut updated_units = vec![];
    for new_selected_unit in selected_units {
        let unit_index = unit_tag_index(*new_selected_unit as i64);
        if let Some(ref mut unit) = sc2_state.units.get_mut(&unit_index) {
            if !unit.is_selected {
                unit.is_selected = true;
                unit.radius = unit.radius * 2.0;
                updated_units.push(unit_index);
            }
        }
    }
    updated_units
}

/// Registers units as being selected.
/// The previous selected units radius is decreased to its normal state.
/// The new selected units radius is increased.
/// The event could be for a non-selected group, for example, a unit in a group may have died
/// and that would trigger a selection delta. Same if a unit as Larva is part of a group and
/// then it is born into another unit which triggers a selection delta.
/// In the rust version we are matching the ACTIVE_UNITS_GROUP_IDX to 10, the last item in the
/// array of selceted units which seems to match the blizzard UI so far.
pub fn handle_selection_delta(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    selection_delta: &GameSSelectionDeltaEvent,
) -> Vec<u32> {
    let mut updated_units = vec![];
    if selection_delta.m_control_group_id == ACTIVE_UNITS_GROUP_IDX as u8 {
        updated_units.append(&mut unmark_previously_selected_units(
            sc2_state, game_loop, user_id,
        ));
        updated_units.append(&mut mark_selected_units(
            sc2_state,
            game_loop,
            user_id,
            &selection_delta.m_delta.m_add_unit_tags,
        ));
    }
    if let Some(ref mut state) = sc2_state.user_state.get_mut(&user_id) {
        state.control_groups[selection_delta.m_control_group_id as usize] =
            selection_delta.m_delta.m_add_unit_tags.clone();
    }
    updated_units.sort_unstable();
    updated_units.dedup();
    updated_units
}

/// Handles control group update events
/// These may be initializing or recalled
pub fn update_control_group(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    ctrl_group_evt: &GameSControlGroupUpdateEvent,
) -> Vec<u32> {
    let mut updated_units = unmark_previously_selected_units(sc2_state, game_loop, user_id);
    match ctrl_group_evt.m_control_group_update {
        GameEControlGroupUpdate::ESet => {
            if let Some(ref mut user_state) = sc2_state.user_state.get_mut(&user_id) {
                user_state.control_groups[ctrl_group_evt.m_control_group_index as usize] =
                    user_state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone();
            }
        }
        GameEControlGroupUpdate::ESetAndSteal => {
            if let Some(ref mut user_state) = sc2_state.user_state.get_mut(&user_id) {
                // Remove the instances from other groups first
                let current_selected_units =
                    user_state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone();
                for group_idx in 0..9 {
                    for unit in &current_selected_units {
                        user_state.control_groups[group_idx].retain(|&x| x != *unit);
                    }
                }
                // Set the group index as the value of the current selected units.
                user_state.control_groups[ctrl_group_evt.m_control_group_index as usize] =
                    user_state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone();
            }
        }
        GameEControlGroupUpdate::EClear => {
            if let Some(ref mut user_state) = sc2_state.user_state.get_mut(&user_id) {
                user_state.control_groups[ctrl_group_evt.m_control_group_index as usize] = vec![];
            }
        }
        GameEControlGroupUpdate::EAppend => {
            if let Some(ref mut user_state) = sc2_state.user_state.get_mut(&user_id) {
                let mut current_selected_units =
                    user_state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone();
                user_state.control_groups[ctrl_group_evt.m_control_group_index as usize]
                    .append(&mut current_selected_units);
                user_state.control_groups[ctrl_group_evt.m_control_group_index as usize]
                    .sort_unstable();
                user_state.control_groups[ctrl_group_evt.m_control_group_index as usize].dedup();
            }
        }
        GameEControlGroupUpdate::EAppendAndSteal => {
            if let Some(ref mut user_state) = sc2_state.user_state.get_mut(&user_id) {
                // Remove the instances from other groups first
                let mut current_selected_units =
                    user_state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone();
                for group_idx in 0..9 {
                    for unit in &current_selected_units {
                        user_state.control_groups[group_idx].retain(|&x| x != *unit);
                    }
                }
                user_state.control_groups[ctrl_group_evt.m_control_group_index as usize]
                    .append(&mut current_selected_units);
                user_state.control_groups[ctrl_group_evt.m_control_group_index as usize]
                    .sort_unstable();
                user_state.control_groups[ctrl_group_evt.m_control_group_index as usize].dedup();
            }
        }
        GameEControlGroupUpdate::ERecall => {
            let mut current_selected_units: Vec<u32> = vec![];
            if let Some(ref mut user_state) = sc2_state.user_state.get_mut(&user_id) {
                user_state.control_groups[ACTIVE_UNITS_GROUP_IDX] = user_state.control_groups
                    [ctrl_group_evt.m_control_group_index as usize]
                    .clone();
                current_selected_units = user_state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone();
            }
            updated_units.append(&mut mark_selected_units(
                sc2_state,
                game_loop,
                user_id,
                &current_selected_units,
            ));
        }
    }
    updated_units
}

/// Handles a game event as it steps through the SC2 State.
pub fn handle_game_event(
    mut sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    evt: &ReplayGameEvent,
) -> Vec<u32> {
    match &evt {
        ReplayGameEvent::CameraUpdate(camera_update) => {
            // Unhandled for now
            // handle_camera_update(&sc2_state, game_loop, user_id, camera_update)?;
            vec![]
        }
        ReplayGameEvent::Cmd(game_cmd) => handle_cmd(&mut sc2_state, game_loop, user_id, game_cmd),
        ReplayGameEvent::CmdUpdateTargetPoint(target_point) => {
            handle_update_target_point(&mut sc2_state, game_loop, user_id, &target_point.m_target)
        }
        ReplayGameEvent::CmdUpdateTargetUnit(target_unit) => {
            handle_update_target_unit(&mut sc2_state, game_loop, user_id, &target_unit.m_target)
        }
        ReplayGameEvent::SelectionDelta(selection_delta) => {
            handle_selection_delta(&mut sc2_state, game_loop, user_id, &selection_delta)
        }
        ReplayGameEvent::ControlGroupUpdate(ctrl_group) => {
            update_control_group(&mut sc2_state, game_loop, user_id, &ctrl_group)
        }
        _ => vec![],
    }
}
