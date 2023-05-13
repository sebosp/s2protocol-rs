//! Handles the state change of units for TrackerEvents

use super::*;
use crate::*;

/// The unit position seems to be 4 times the actual unit position to fit in the map.
/// So we have to divide the UnitPosition events by this number:
pub const UNIT_POSITION_RATIO: f32 = 4.;

/// Handles the state management for unit init messages
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_unit_init(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_init: &UnitInitEvent,
) -> Vec<u32> {
    let unit_name_filter = &sc2_state.filters.unit_name;
    if let Some(unit_name_filter) = unit_name_filter {
        if unit_name_filter != &unit_init.unit_type_name {
            return vec![];
        }
    }
    let sc2_unit = SC2Unit {
        last_game_loop: game_loop,
        user_id: Some(unit_init.control_player_id),
        name: unit_init.unit_type_name.clone(),
        pos: Vec3D::new(unit_init.x as f32, -1. * unit_init.y as f32, 0.),
        init_game_loop: game_loop,
        ..Default::default()
    };
    tracing::info!("Initializing unit: {:?}", sc2_unit);
    if let Some(unit) = sc2_state.units.get(&unit_init.unit_tag_index) {
        // Hmm no idea if this is normal.
        tracing::warn!("Re-initializing unit: {:?}", unit);
    }
    sc2_state.units.insert(unit_init.unit_tag_index, sc2_unit);
    vec![unit_init.unit_tag_index]
}

/// Handles the state management for unit born messages
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_unit_born(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_born: &UnitBornEvent,
) -> Vec<u32> {
    let unit_name_filter = &sc2_state.filters.unit_name;
    if let Some(unit_name_filter) = unit_name_filter {
        if unit_name_filter != &unit_born.unit_type_name {
            return vec![];
        }
    }
    if let Some(ref mut unit) = sc2_state.units.get_mut(&unit_born.unit_tag_index) {
        unit.creator_ability_name = unit_born.creator_ability_name.clone();
        unit.last_game_loop = game_loop;
    } else {
        let sc2_unit = SC2Unit {
            last_game_loop: game_loop,
            user_id: Some(unit_born.control_player_id),
            name: unit_born.unit_type_name.clone(),
            pos: Vec3D::new(unit_born.x as f32, -1. * unit_born.y as f32, 0.),
            init_game_loop: game_loop,
            ..Default::default()
        };
        sc2_state.units.insert(unit_born.unit_tag_index, sc2_unit);
    }
    vec![unit_born.unit_tag_index]
}

#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_unit_position(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_pos: &UnitPositionsEvent,
) -> Vec<u32> {
    let mut updated_units = vec![];
    let unit_positions = unit_pos.clone().to_unit_positions_vec();
    for unit_pos_item in &unit_positions {
        if let Some(ref mut sc2_unit) = sc2_state.units.get_mut(&unit_pos_item.tag) {
            updated_units.push(unit_pos_item.tag);
            sc2_unit.pos = Vec3D::new(
                unit_pos_item.x as f32 / UNIT_POSITION_RATIO,
                -1. * unit_pos_item.y as f32 / UNIT_POSITION_RATIO,
                0.,
            );
            sc2_unit.last_game_loop = game_loop;
        } else {
            tracing::error!(
                "Unit {} did not exist but position registered.",
                unit_pos_item.tag
            );
        }
    }
    updated_units
}
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_unit_died(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_dead: &UnitDiedEvent,
) -> Vec<u32> {
    // Clean up the unit from previous groups where it was selected.
    for (_idx, state) in sc2_state.user_state.iter_mut() {
        for group_idx in 0..10 {
            state.control_groups[group_idx].retain(|&x| x != unit_dead.unit_tag_index);
        }
    }
    match sc2_state.units.remove(&unit_dead.unit_tag_index) {
        None => {
            // This may happen when a larva is transformed to a unit in zerg. so this is normal.
            tracing::debug!(
                "Unit {} reported dead but was not registered before.",
                unit_dead.unit_tag_index
            );
            vec![]
        }
        Some(val) => {
            tracing::debug!("Unit died: {:?}", val);
            vec![unit_dead.unit_tag_index]
        }
    }
}
/// Handles a tracker event as it steps through the SC2 State
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_tracker_event(
    mut sc2_state: &mut SC2ReplayState,
    tracker_loop: i64,
    evt: &ReplayTrackerEvent,
) -> Vec<u32> {
    match &evt {
        ReplayTrackerEvent::UnitInit(unit_init) => {
            handle_unit_init(&mut sc2_state, tracker_loop, unit_init)
        }
        ReplayTrackerEvent::UnitBorn(unit_born) => {
            handle_unit_born(&mut sc2_state, tracker_loop, unit_born)
        }
        ReplayTrackerEvent::UnitDied(unit_died) => {
            handle_unit_died(&mut sc2_state, tracker_loop, unit_died)
        }
        ReplayTrackerEvent::UnitPosition(unit_pos) => {
            handle_unit_position(&mut sc2_state, tracker_loop, unit_pos)
        }
        ReplayTrackerEvent::PlayerStats(_player_stats) => {
            // For now the player stats are not recorded here.
            vec![]
        }
        ReplayTrackerEvent::PlayerSetup(player_setup) => {
            if let Some(user_id) = player_setup.user_id {
                sc2_state
                    .user_state
                    .insert(user_id as i64, SC2UserState::new());
                vec![user_id]
            } else {
                vec![]
            }
        }
        _ => unimplemented!(),
    }
}
