//! Handles the state change of units for GameEvents

use super::*;
use crate::tracker_events::unit_tag_index;
use crate::*;

/// The game events seem to be at this ratio when compared to Tracker Events.
pub const GAME_EVENT_POS_RATIO: f32 = 4096f32;

/// There are multiple camera positions that can be hotkeyed, I think 8, maybe 10.
/// m_which is the index of the camera
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_camera_save(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    camera_update: &CameraSaveEvent,
) -> UnitChangeHint {
    if let Some(ref mut user_state) = sc2_state.user_state.get_mut(&user_id) {
        user_state.camera_pos.x = camera_update.m_target.x;
        user_state.camera_pos.y = camera_update.m_target.y;
    }
    UnitChangeHint::None
}

/// Unsure  why the camera target may be None, but for now let's just handle the Some()
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_camera_update(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    camera_update: &CameraUpdateEvent,
) -> UnitChangeHint {
    if let Some(target) = &camera_update.m_target
        && let Some(ref mut user_state) = sc2_state.user_state.get_mut(&user_id)
    {
        user_state.camera_pos.x = target.x;
        user_state.camera_pos.y = target.y;
    }
    UnitChangeHint::None
}

/// The selected units for a specific players are given a specific command.
/// That is, a selected group, the ACTIVE_UNITS_GROUP_IDX, and a HotKey is pressed.
/// The hotkey belongs to a command, and the command may have an index of specific action.
/// At this point, the target unit or the target point is not known.
/// A future event may set the target point or target unit.
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_cmd(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    // The cmd is mut as we will enrich the ability field with the String value.
    mut cmd: GameSCmdEvent,
    balance_data_units: &HashMap<String, VersionedBalanceUnit>,
) -> (ReplayGameEvent, UnitChangeHint) {
    // TODO:: Many commands, such as "Stop", should clear the unit.cmd.other_unit.
    // For now this is not implemented and a unit maybe have a target unit, even tho the active
    // command is Stop.
    let target = if let GameSCmdData::TargetUnit(tu) = &cmd.m_data {
        sc2_state.units.get(&tu.m_tag).cloned()
    } else {
        None
    };
    let mut user_selected_unit_ids: Vec<u32> = vec![];
    let mut user_selected_units: Vec<SC2Unit> = vec![];
    if let Some(state) = sc2_state.user_state.get(&user_id) {
        user_selected_unit_ids = state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone();
    }
    for selected_unit in &user_selected_unit_ids {
        let unit_index = unit_tag_index(*selected_unit as i64);
        if let Some(ref mut registered_unit) = sc2_state.units.get_mut(&unit_index) {
            match cmd.m_abil {
                None => continue,
                Some(ref mut val) => {
                    val.ability = get_indexed_ability_command_name(
                        balance_data_units,
                        registered_unit.name.as_str(),
                        val.m_abil_link,
                        val.m_abil_cmd_index,
                    );
                }
            };
            tracing::debug!(
                "Unit {} executing command: {:?} on target: {:?}",
                registered_unit.name,
                cmd.m_abil,
                target,
            );
            registered_unit.cmd = cmd.clone().into();
            registered_unit.cmd.other_unit_name = target.as_ref().map(|u| u.name.clone());
            registered_unit.last_game_loop = game_loop;
            user_selected_units.push(registered_unit.clone());
        }
    }
    // TODO: Maybe we can avoid returning the event `cmd` twice? Or is it because of an old
    // consumer/test?
    (
        ReplayGameEvent::Cmd(cmd.clone()),
        UnitChangeHint::Abilities {
            units: user_selected_units,
            event: cmd,
            target,
        },
    )
}

/// The selected units for a specific players are given a specific target point to move towards.
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_update_target_point(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    target_point: GameSCmdUpdateTargetPointEvent,
) -> (ReplayGameEvent, UnitChangeHint) {
    // The TargetPoint does not contain the ability id or command index.
    let mut user_selected_unit_ids: Vec<u32> = vec![];
    let mut user_selected_units: Vec<SC2Unit> = vec![];
    if let Some(state) = sc2_state.user_state.get(&user_id) {
        user_selected_unit_ids = state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone();
    }
    for selected_unit in &user_selected_unit_ids {
        let unit_index = unit_tag_index(*selected_unit as i64);
        if let Some(ref mut registered_unit) = sc2_state.units.get_mut(&unit_index) {
            registered_unit
                .cmd
                .set_data_target_point(target_point.m_target.clone());
            // Unset any previous ability vs a target unit.
            registered_unit.cmd.other_unit = None;
            registered_unit.last_game_loop = game_loop;
            user_selected_units.push(registered_unit.clone());
        }
    }
    (
        ReplayGameEvent::CmdUpdateTargetPoint(target_point),
        UnitChangeHint::TargetPoints(user_selected_units),
    )
}

/// Handles the change of target for the currently selected units.
/// The unit is previously selected and is part of the ACTIVE_UNITS_GROUP_IDX,
/// then a TargetUnit event is emitted.
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_update_target_unit(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    target_unit: GameSCmdUpdateTargetUnitEvent,
) -> (ReplayGameEvent, UnitChangeHint) {
    // The TargetPoint does not contain the ability id or command index.
    let registered_target_unit = match sc2_state.units.get(&target_unit.m_target.m_tag) {
        Some(x) => x.clone(),
        None => {
            match sc2_state
                .units
                .get(&unit_tag_index(target_unit.m_target.m_tag.into()))
            {
                Some(x) => x.clone(),
                None => {
                    tracing::warn!(
                        "Unit not found for target unit: {}",
                        target_unit.m_target.m_tag
                    );
                    return (
                        ReplayGameEvent::CmdUpdateTargetUnit(target_unit),
                        UnitChangeHint::None,
                    );
                }
            }
        }
    };
    let mut user_selected_unit_ids: Vec<u32> = vec![];
    let mut user_selected_units: Vec<SC2Unit> = vec![];
    if let Some(state) = sc2_state.user_state.get(&user_id) {
        user_selected_unit_ids = state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone();
    }
    for selected_unit in &user_selected_unit_ids {
        let unit_index = unit_tag_index(*selected_unit as i64);
        if let Some(ref mut registered_unit) = sc2_state.units.get_mut(&unit_index) {
            registered_unit
                .cmd
                .set_data_target_unit(target_unit.m_target.clone().into());
            registered_unit.last_game_loop = game_loop;
            user_selected_units.push(registered_unit.clone());
        }
    }
    (
        ReplayGameEvent::CmdUpdateTargetUnit(target_unit),
        UnitChangeHint::TargetUnits {
            units: user_selected_units,
            target: Box::new(registered_target_unit),
        },
    )
}

/// Removes the changes to the units that signify they are selected.
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn unmark_previously_selected_units(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
) -> UnitChangeHint {
    let mut updated_unit_ids = vec![];
    if let Some(state) = sc2_state.user_state.get_mut(&user_id) {
        for prev_unit in &state.control_groups[ACTIVE_UNITS_GROUP_IDX] {
            let unit_index = unit_tag_index(*prev_unit as i64);
            if let Some(ref mut unit) = sc2_state.units.get_mut(&unit_index) {
                if unit.is_selected {
                    unit.is_selected = false;
                    unit.radius *= 0.5;
                    updated_unit_ids.push(unit_index);
                }
                unit.last_game_loop = game_loop;
            }
        }
    }
    let updated_units = updated_unit_ids
        .iter()
        .filter_map(|id| sc2_state.units.get(id))
        .cloned()
        .collect();
    UnitChangeHint::Selection(updated_units)
}

/// Marks a group of units as selected by increasing the radius.
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn mark_selected_units(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    _user_id: i64,
    selected_units: &[u32],
) -> UnitChangeHint {
    let mut updated_unit_ids = vec![];
    for new_selected_unit in selected_units {
        let unit_index = unit_tag_index(*new_selected_unit as i64);
        if let Some(ref mut unit) = sc2_state.units.get_mut(&unit_index) {
            if !unit.is_selected {
                unit.is_selected = true;
                unit.radius *= 2.0;
                updated_unit_ids.push(unit_index);
            }
            unit.last_game_loop = game_loop;
        }
    }
    let updated_units = updated_unit_ids
        .iter()
        .filter_map(|id| sc2_state.units.get(id))
        .cloned()
        .collect();
    UnitChangeHint::Selection(updated_units)
}

/// Registers units as being selected.
/// The previous selected units radius is decreased to its normal state.
/// The new selected units radius is increased.
/// The event could be for a non-selected group, for example, a unit in a group may have died
/// and that would trigger a selection delta. Same if a unit as Larva is part of a group and
/// then it is born into another unit which triggers a selection delta.
/// In the rust version we are matching the ACTIVE_UNITS_GROUP_IDX to 10, the last item in the
/// array of selceted units which seems to match the blizzard UI so far.
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_selection_delta(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    selection_delta: &GameSSelectionDeltaEvent,
) -> UnitChangeHint {
    let mut updated_units = vec![];
    if selection_delta.m_control_group_id == ACTIVE_UNITS_GROUP_IDX as u8 {
        let mut unmarked_units =
            match unmark_previously_selected_units(sc2_state, game_loop, user_id) {
                UnitChangeHint::Selection(units) => units,
                _ => unreachable!(),
            };
        updated_units.append(&mut unmarked_units);
        let mut marked_units = match mark_selected_units(
            sc2_state,
            game_loop,
            user_id,
            &selection_delta.m_delta.m_add_unit_tags,
        ) {
            UnitChangeHint::Selection(units) => units,
            _ => unreachable!(),
        };
        updated_units.append(&mut marked_units);
    }
    if let Some(ref mut state) = sc2_state.user_state.get_mut(&user_id) {
        state.control_groups[selection_delta.m_control_group_id as usize] =
            selection_delta.m_delta.m_add_unit_tags.clone();
    }
    updated_units.sort_unstable();
    updated_units.dedup();
    UnitChangeHint::Selection(updated_units)
}

/// Handles control group update events
/// These may be initializing or recalled
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn update_control_group(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    ctrl_group_evt: &GameSControlGroupUpdateEvent,
) -> UnitChangeHint {
    let mut updated_units = match unmark_previously_selected_units(sc2_state, game_loop, user_id) {
        UnitChangeHint::Selection(units) => units,
        _ => unreachable!(),
    };
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
            let mut curr_units =
                match mark_selected_units(sc2_state, game_loop, user_id, &current_selected_units) {
                    UnitChangeHint::Selection(units) => units,
                    _ => unreachable!(),
                };
            updated_units.append(&mut curr_units);
        }
    }
    UnitChangeHint::Selection(updated_units)
}

/// Handles a game event as it steps through the SC2 State.
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_game_event(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    evt: ReplayGameEvent,
    balance_data_units: &HashMap<String, VersionedBalanceUnit>,
) -> (ReplayGameEvent, UnitChangeHint) {
    // NOTE: There are special cases that enrich the event, when there are abilities:
    // - Cmd
    // - CmdUpdateTargetPoint
    // - CmdUpdateTargetUnit
    match evt.clone() {
        ReplayGameEvent::CameraSave(camera_save) => (
            evt,
            handle_camera_save(sc2_state, game_loop, user_id, &camera_save),
        ),
        ReplayGameEvent::CameraUpdate(camera_update) => (
            evt,
            handle_camera_update(sc2_state, game_loop, user_id, &camera_update),
        ),
        ReplayGameEvent::Cmd(game_cmd) => handle_cmd(
            sc2_state,
            game_loop,
            user_id,
            game_cmd.clone(),
            balance_data_units,
        ),
        ReplayGameEvent::CmdUpdateTargetPoint(target_point) => {
            handle_update_target_point(sc2_state, game_loop, user_id, target_point)
        }
        ReplayGameEvent::CmdUpdateTargetUnit(target_unit) => {
            handle_update_target_unit(sc2_state, game_loop, user_id, target_unit)
        }
        ReplayGameEvent::SelectionDelta(selection_delta) => (
            evt,
            handle_selection_delta(sc2_state, game_loop, user_id, &selection_delta),
        ),
        ReplayGameEvent::ControlGroupUpdate(ctrl_group) => (
            evt,
            update_control_group(sc2_state, game_loop, user_id, &ctrl_group),
        ),
        _ => (evt, UnitChangeHint::None),
    }
}
