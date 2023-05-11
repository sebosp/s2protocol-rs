//! Handles the state change of units for TrackerEvents

use super::*;
use crate::*;

/// Handles the state management for unit init messages
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_unit_init(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_init: &UnitInitEvent,
) -> Option<u32> {
    let unit_name_filter = &sc2_state.filters.unit_name;
    if let Some(unit_name_filter) = unit_name_filter {
        if unit_name_filter != &unit_init.unit_type_name {
            return None;
        }
    }
    let sc2_unit = SC2Unit {
        last_game_loop: game_loop,
        user_id: Some(unit_init.control_player_id),
        name: unit_init.unit_type_name.clone(),
        pos: Vec3D::new(unit_init.x as i64, -1i64 * unit_init.y as i64, 0i64),
        init_game_loop: game_loop,
        ..Default::default()
    };
    tracing::info!("Initializing unit: {:?}", sc2_unit);
    if let Some(unit) = sc2_state.units.get(&unit_init.unit_tag_index) {
        // Hmm no idea if this is normal.
        tracing::warn!("Re-initializing unit: {:?}", unit);
    }
    sc2_state.units.insert(unit_init.unit_tag_index, sc2_unit);
    Some(unit_init.unit_tag_index)
}

/// Handles the state management for unit born messages
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_unit_born(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_born: &UnitBornEvent,
) -> Option<u32> {
    let unit_name_filter = &sc2_state.filters.unit_name;
    if let Some(unit_name_filter) = unit_name_filter {
        if unit_name_filter != &unit_born.unit_type_name {
            return None;
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
            pos: Vec3D::new(unit_born.x as i64, -1i64 * unit_born.y as i64, 0i64),
            init_game_loop: game_loop,
            ..Default::default()
        };
        sc2_state.units.insert(unit_born.unit_tag_index, sc2_unit);
    }
    Some(unit_born.unit_tag_index)
}

#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_unit_position(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_pos: &UnitPositionsEvent,
) -> Option<u32> {
    let unit_positions = unit_pos.to_unit_positions_vec();
    for unit_pos_item in &unit_positions {
        if let Some(ref mut sc2_unit) = sc2_state.units.get_mut(&unit_pos_item.tag) {
            sc2_unit.pos = Vec3D::new(
                unit_pos_item.x as i64 / 4i64,
                -1i64 * unit_pos_item.y as i64 / 4i64,
                0i64,
            );
            sc2_unit.last_game_loop = game_loop;
        } else {
            tracing::error!(
                "Unit {} did not exist but position registered.",
                unit_pos_item.tag
            );
        }
    }
    if unit_positions.len() > 0 {
        // XXX: Return the first unit tag, this is ugly, just to follow the previous return Option<u32>
        Some(unit_positions[0].tag)
    } else {
        None
    }
}

/// Handles a tracker event as it steps through the SC2 State
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_tracker_event(
    mut sc2_state: &mut SC2ReplayState,
    tracker_loop: i64,
    evt: &ReplayTrackerEvent,
) -> Option<u32> {
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
        ReplayTrackerEvent::PlayerStats(player_stats) => {
            handle_player_stats(sc2_state, tracker_loop, player_stats)?
        }
        ReplayTrackerEvent::PlayerSetup(player_setup) => {
            let user_id = player_setup.user_id?;
            sc2_state
                .user_state
                .insert(user_id as i64, SC2UserState::new());
            Some(user_id)
        }
        _ => unimplemented!(),
    }
}
