//! Handles the state change of units for TrackerEvents

use super::*;
use crate::*;

/// Handles the state management for unit init messages
#[tracing::instrument(level = "debug", skip(sc2_replay))]
pub fn handle_unit_init(
    sc2_replay: &mut SC2ReplayState,
    game_loop: i64,
    unit_init: &UnitInitEvent,
) -> Option<u32> {
    let unit_name_filter = &sc2_replay.filters.unit_name;
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
    if let Some(unit) = sc2_replay.units.get(&unit_init.unit_tag_index) {
        // Hmm no idea if this is normal.
        tracing::warn!("Re-initializing unit: {:?}", unit);
    }
    sc2_replay.units.insert(unit_init.unit_tag_index, sc2_unit);
    Some(unit_init.unit_tag_index)
}

/// Handles the state management for unit born messages
#[tracing::instrument(level = "debug", skip(sc2_replay))]
pub fn handle_unit_born(
    sc2_replay: &mut SC2ReplayState,
    game_loop: i64,
    unit_born: &UnitBornEvent,
) -> Option<u32> {
    let unit_name_filter = &sc2_replay.filters.unit_name;
    if let Some(unit_name_filter) = unit_name_filter {
        if unit_name_filter != &unit_born.unit_type_name {
            return None;
        }
    }
    if let Some(ref mut sc2_unit) = sc2_replay.units.get_mut(&unit_born.unit_tag_index) {
        sc2_unit.creator_ability_name = unit_born.creator_ability_name.clone();
        sc2_unit.last_game_loop = game_loop;
    } else {
        let sc2_unit = SC2Unit {
            last_game_loop: game_loop,
            user_id: Some(unit_born.control_player_id),
            name: unit_born.unit_type_name.clone(),
            pos: Vec3D::new(unit_born.x as f32, -1. * unit_born.y as f32, 0.),
            init_game_loop: game_loop,
            radius: unit_size,
            ..Default::default()
        };
        sc2_replay.units.insert(unit_born.unit_tag_index, sc2_unit);
    }
    Ok(())
}
