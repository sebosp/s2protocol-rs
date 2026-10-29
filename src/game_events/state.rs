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
    camera_save: CameraSaveEvent,
) -> (ReplayGameEvent, UnitChangeHint) {
    if let Some(ref mut user_state) = sc2_state.user_state.get_mut(&user_id) {
        user_state.camera_pos.x = camera_save.m_target.x;
        user_state.camera_pos.y = camera_save.m_target.y;
    }
    (
        ReplayGameEvent::CameraSave(camera_save),
        UnitChangeHint::None,
    )
}

/// Unsure  why the camera target may be None, but for now let's just handle the Some()
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_camera_update(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    camera_update: CameraUpdateEvent,
) -> (ReplayGameEvent, UnitChangeHint) {
    if let Some(target) = &camera_update.m_target
        && let Some(ref mut user_state) = sc2_state.user_state.get_mut(&user_id)
    {
        user_state.camera_pos.x = target.x;
        user_state.camera_pos.y = target.y;
    }
    (
        ReplayGameEvent::CameraUpdate(camera_update),
        UnitChangeHint::None,
    )
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
    // The units of a selection do not all have the ability, i.e. a Battlecruiser selected with
    // Marines moves with BattlecruiserMove and the Marines with move. The command is named after
    // the first unit that has it, each unit after its own.
    let mut command_name = String::new();
    for selected_unit in &user_selected_unit_ids {
        let unit_index = unit_tag_index(*selected_unit as i64);
        if let Some(ref mut registered_unit) = sc2_state.units.get_mut(&unit_index) {
            let mut unit_cmd = cmd.clone();
            match unit_cmd.m_abil {
                None => continue,
                Some(ref mut val) => {
                    val.ability = get_indexed_ability_command_name(
                        &sc2_state.balance_units,
                        registered_unit.name.as_str(),
                        val.m_abil_link,
                        val.m_abil_cmd_index,
                    );
                    if command_name.is_empty() {
                        command_name = val.ability.clone();
                    }
                }
            };
            tracing::debug!(
                "Unit {} executing command: {:?} on target: {:?}",
                registered_unit.name,
                unit_cmd.m_abil,
                target,
            );
            registered_unit.cmd = unit_cmd.into();
            registered_unit.cmd.other_unit_name = target.as_ref().map(|u| u.name.clone());
            registered_unit.last_game_loop = game_loop;
            user_selected_units.push(registered_unit.clone());
        }
    }
    if let Some(ref mut ability) = cmd.m_abil {
        if command_name.is_empty() {
            command_name = sc2_state
                .ability_names
                .get(&(ability.m_abil_link, ability.m_abil_cmd_index))
                .cloned()
                .unwrap_or_default();
        }
        ability.ability = command_name;
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
        } else {
            tracing::debug!(
                "{} handle_update_target_point Unable to locate selected unit {} for user_id {}",
                sc2_state.init_data.ext_fs_file_name,
                selected_unit,
                user_id
            );
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
        } else {
            tracing::debug!(
                "{} handle_update_target_unit Unable to locate selected unit {} for user_id {}",
                sc2_state.init_data.ext_fs_file_name,
                selected_unit,
                user_id
            );
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
    selection_delta: GameSSelectionDeltaEvent,
) -> (ReplayGameEvent, UnitChangeHint) {
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
        let group = &mut state.control_groups[selection_delta.m_control_group_id as usize];
        *group = apply_selection_delta(group, &selection_delta.m_delta);
    }
    updated_units.sort_unstable();
    updated_units.dedup();
    (
        ReplayGameEvent::SelectionDelta(selection_delta),
        UnitChangeHint::Selection(updated_units),
    )
}

/// Whether the unit at the index of a selection of `len` units is removed by the mask. The mask is
/// read in chunks of 8 bits and the last chunk, with the remaining bits, has the lowest indices, as
/// sc2reader reads it.
fn is_masked(mask: &[u8], len: usize, idx: usize) -> bool {
    let last_bits = match len % 8 {
        0 => 8,
        bits => bits,
    };
    let mut pos = idx;
    for (chunk_idx, chunk) in mask.iter().enumerate().rev() {
        let bits = if chunk_idx + 1 == mask.len() {
            last_bits
        } else {
            8
        };
        if pos < bits {
            return (chunk >> pos) & 1 == 1;
        }
        pos -= bits;
    }
    false
}

/// The units of a selection or control group that remain after the remove mask.
/// The game does not tell when units leave a selection by themselves, i.e. when they are loaded in
/// a transport, so the mask may not match the units tracked, the mask is then applied to the units
/// that it covers.
pub fn remove_from_selection(current: &[u32], mask: &GameSSelectionMask) -> Vec<u32> {
    let keep_by_index = |keep: &dyn Fn(usize) -> bool| -> Vec<u32> {
        current
            .iter()
            .enumerate()
            .filter(|(idx, _)| keep(*idx))
            .map(|(_, unit)| *unit)
            .collect()
    };
    match mask {
        GameSSelectionMask::None => current.to_vec(),
        GameSSelectionMask::Mask(mask) => {
            if mask.value.len() != current.len().div_ceil(8) {
                tracing::debug!(
                    "Selection mask of {} bytes for {} units",
                    mask.value.len(),
                    current.len()
                );
            }
            keep_by_index(&|idx| !is_masked(&mask.value, current.len(), idx))
        }
        GameSSelectionMask::OneIndices(indices) => {
            keep_by_index(&|idx| !indices.value.contains(&(idx as GameTSelectionIndex)))
        }
        GameSSelectionMask::ZeroIndices(indices) => indices
            .value
            .iter()
            .filter_map(|idx| current.get(*idx as usize).copied())
            .collect(),
    }
}

/// Adds units to a selection, the selection is sorted by unit tag, as sc2reader does.
fn add_to_selection(mut selection: Vec<u32>, units: &[u32]) -> Vec<u32> {
    selection.extend_from_slice(units);
    selection.sort_unstable();
    selection.dedup();
    selection
}

/// The units of a selection after a delta: the remove mask is applied to the current units, then
/// the new units are added.
pub fn apply_selection_delta(current: &[u32], delta: &GameSSelectionDelta) -> Vec<u32> {
    add_to_selection(
        remove_from_selection(current, &delta.m_remove_mask),
        &delta.m_add_unit_tags,
    )
}

/// Handles control group update events
/// These may be initializing or recalled
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn update_control_group(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    ctrl_group_evt: GameSControlGroupUpdateEvent,
) -> (ReplayGameEvent, UnitChangeHint) {
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
                for group_idx in 0..ACTIVE_UNITS_GROUP_IDX {
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
                let group_idx = ctrl_group_evt.m_control_group_index as usize;
                let group = remove_from_selection(
                    &user_state.control_groups[group_idx],
                    &ctrl_group_evt.m_mask,
                );
                user_state.control_groups[group_idx] =
                    add_to_selection(group, &user_state.control_groups[ACTIVE_UNITS_GROUP_IDX]);
            }
        }
        GameEControlGroupUpdate::EAppendAndSteal => {
            if let Some(ref mut user_state) = sc2_state.user_state.get_mut(&user_id) {
                // Remove the instances from other groups first
                let current_selected_units =
                    user_state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone();
                for group_idx in 0..ACTIVE_UNITS_GROUP_IDX {
                    for unit in &current_selected_units {
                        user_state.control_groups[group_idx].retain(|&x| x != *unit);
                    }
                }
                let group_idx = ctrl_group_evt.m_control_group_index as usize;
                let group = remove_from_selection(
                    &user_state.control_groups[group_idx],
                    &ctrl_group_evt.m_mask,
                );
                user_state.control_groups[group_idx] =
                    add_to_selection(group, &current_selected_units);
            }
        }
        GameEControlGroupUpdate::ERecall => {
            let mut current_selected_units: Vec<u32> = vec![];
            if let Some(ref mut user_state) = sc2_state.user_state.get_mut(&user_id) {
                // The mask removes the units that are no longer in the group, i.e. dead.
                let group_idx = ctrl_group_evt.m_control_group_index as usize;
                let group = remove_from_selection(
                    &user_state.control_groups[group_idx],
                    &ctrl_group_evt.m_mask,
                );
                user_state.control_groups[group_idx] = group.clone();
                user_state.control_groups[ACTIVE_UNITS_GROUP_IDX] = group.clone();
                current_selected_units = group;
            }
            let mut curr_units =
                match mark_selected_units(sc2_state, game_loop, user_id, &current_selected_units) {
                    UnitChangeHint::Selection(units) => units,
                    _ => unreachable!(),
                };
            updated_units.append(&mut curr_units);
        }
    }
    (
        ReplayGameEvent::ControlGroupUpdate(ctrl_group_evt),
        UnitChangeHint::Selection(updated_units),
    )
}

/// Handles a game event as it steps through the SC2 State.
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_game_event(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    user_id: i64,
    evt: ReplayGameEvent,
) -> (ReplayGameEvent, UnitChangeHint) {
    // NOTE: There are special cases that enrich the event, when there are abilities:
    // - Cmd
    // - CmdUpdateTargetPoint
    // - CmdUpdateTargetUnit
    match evt {
        ReplayGameEvent::CameraSave(camera_save) => {
            handle_camera_save(sc2_state, game_loop, user_id, camera_save)
        }
        ReplayGameEvent::CameraUpdate(camera_update) => {
            handle_camera_update(sc2_state, game_loop, user_id, camera_update)
        }
        ReplayGameEvent::Cmd(game_cmd) => handle_cmd(sc2_state, game_loop, user_id, game_cmd),
        ReplayGameEvent::CmdUpdateTargetPoint(target_point) => {
            handle_update_target_point(sc2_state, game_loop, user_id, target_point)
        }
        ReplayGameEvent::CmdUpdateTargetUnit(target_unit) => {
            handle_update_target_unit(sc2_state, game_loop, user_id, target_unit)
        }
        ReplayGameEvent::SelectionDelta(selection_delta) => {
            handle_selection_delta(sc2_state, game_loop, user_id, selection_delta)
        }
        ReplayGameEvent::ControlGroupUpdate(ctrl_group) => {
            update_control_group(sc2_state, game_loop, user_id, ctrl_group)
        }
        _ => (evt.clone(), UnitChangeHint::None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn delta(mask: GameSSelectionMask, add: &[u32]) -> GameSSelectionDelta {
        GameSSelectionDelta {
            m_subgroup_index: 0,
            m_remove_mask: mask,
            m_add_subgroups: vec![],
            m_add_unit_tags: add.to_vec(),
        }
    }

    #[test]
    fn it_applies_the_remove_masks_before_adding_units() {
        let current = [10, 20, 30];
        // Shift click adds a unit to the selection.
        let added = apply_selection_delta(&current, &delta(GameSSelectionMask::None, &[25]));
        assert_eq!(added, vec![10, 20, 25, 30]);
        let one = GameSSelectionMask::OneIndices(GameSelectionIndexArrayType { value: vec![1] });
        assert_eq!(
            apply_selection_delta(&current, &delta(one, &[])),
            vec![10, 30]
        );
        let zero =
            GameSSelectionMask::ZeroIndices(GameSelectionIndexArrayType { value: vec![0, 2] });
        assert_eq!(
            apply_selection_delta(&current, &delta(zero, &[40])),
            vec![10, 30, 40]
        );
        // 0b010: the second unit is removed.
        let mask = GameSSelectionMask::Mask(GameSelectionMaskType { value: vec![0b010] });
        assert_eq!(
            apply_selection_delta(&current, &delta(mask, &[])),
            vec![10, 30]
        );
    }

    #[test]
    fn it_reads_the_lowest_indices_from_the_last_chunk_of_the_mask() {
        // 10 units: a chunk of 8 bits, then a chunk of 2 bits with the indices 0 and 1.
        let mask = [0b1000_0000, 0b01];
        let masked: Vec<usize> = (0..10).filter(|idx| is_masked(&mask, 10, *idx)).collect();
        assert_eq!(masked, vec![0, 9]);
    }
}
