//! Handles the state change of units for TrackerEvents

use super::*;
use crate::*;

/// The unit position seems to be 4 times the actual unit position to fit in the map.
/// So we have to divide the UnitPosition events by this number:
pub const UNIT_POSITION_RATIO: f32 = 4.;

/// Handles the state management for unit init messages
#[tracing::instrument(
    level = "debug",
    skip(sc2_state),
    fields(filename = "sc2_state.filename")
)]
pub fn handle_unit_init(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_init: &UnitInitEvent,
) -> UnitChangeHint {
    let mut sc2_unit = SC2Unit {
        last_game_loop: game_loop,
        user_id: Some(unit_init.control_player_id),
        name: unit_init.unit_type_name.clone(),
        pos: Vec3D::new(unit_init.x as f32, unit_init.y as f32, 0.),
        init_game_loop: game_loop,
        is_init: true,
        tag_index: unit_init.unit_tag_index,
        ..Default::default()
    };
    sc2_unit.set_unit_props();
    tracing::info!("Initializing unit: {:?}", sc2_unit);
    if let Some(ref mut unit) = sc2_state.units.get_mut(&unit_init.unit_tag_index) {
        // This happens for example when a unit is burrowed.
        unit.last_game_loop = game_loop;
        unit.pos = Vec3D::new(unit_init.x as f32, unit_init.y as f32, 0.);
        unit.is_init = true;
        unit.name.clone_from(&unit_init.unit_type_name);
    } else {
        sc2_state
            .units
            .insert(unit_init.unit_tag_index, sc2_unit.clone());
    }
    UnitChangeHint::Registered {
        unit: Box::new(sc2_unit),
        creator: None,
    }
}

/// Handles the state management for unit born messages
/// These are units with instant creation such as at the start of the game.
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_unit_born(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_born: &UnitBornEvent,
) -> UnitChangeHint {
    let creator: Option<SC2Unit> = if let Some(creator_unit_tag) = unit_born.creator_unit_tag_index
    {
        if let Some(ref mut unit) = sc2_state.units.get_mut(&creator_unit_tag) {
            unit.last_game_loop = game_loop;
            Some(unit.clone())
        } else {
            None
        }
    } else {
        None
    };
    if let Some(ref mut unit) = sc2_state.units.get_mut(&unit_born.unit_tag_index) {
        unit.creator_ability_name = unit_born.creator_ability_name.clone();
        unit.last_game_loop = game_loop;
        UnitChangeHint::Registered {
            unit: Box::new(unit.clone()),
            creator,
        }
    } else {
        let mut sc2_unit = SC2Unit {
            last_game_loop: game_loop,
            user_id: Some(unit_born.control_player_id),
            name: unit_born.unit_type_name.clone(),
            pos: Vec3D::new(unit_born.x as f32, unit_born.y as f32, 0.),
            init_game_loop: game_loop,
            tag_index: unit_born.unit_tag_index,
            ..Default::default()
        };
        sc2_unit.set_unit_props();
        sc2_state
            .units
            .insert(unit_born.unit_tag_index, sc2_unit.clone());
        UnitChangeHint::Registered {
            unit: Box::new(sc2_unit),
            creator,
        }
    }
}

/// Handles the state management for unit change messages
/// These are units that transition states such as zerglin to banelingcocoon to baneling
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_unit_type_change(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_change: &UnitTypeChangeEvent,
) -> UnitChangeHint {
    if let Some(ref mut unit) = sc2_state.units.get_mut(&unit_change.unit_tag_index) {
        let old_unit_state = unit.clone();
        unit.name.clone_from(&unit_change.unit_type_name);
        unit.last_game_loop = game_loop;
        unit.set_unit_props();
        UnitChangeHint::Registered {
            unit: Box::new(unit.clone()),
            creator: Some(old_unit_state),
        }
    } else {
        let mut sc2_unit = SC2Unit {
            last_game_loop: game_loop,
            user_id: None,
            name: unit_change.unit_type_name.clone(),
            init_game_loop: game_loop,
            tag_index: unit_change.unit_tag_index,
            ..Default::default()
        };
        sc2_unit.set_unit_props();
        sc2_state
            .units
            .insert(unit_change.unit_tag_index, sc2_unit.clone());
        UnitChangeHint::None
    }
}

/// Handles the state management for unit born messages
/// These are units with instant creation such as at the start of the game.
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_unit_done(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_done: &UnitDoneEvent,
) -> UnitChangeHint {
    if let Some(ref mut unit) = sc2_state.units.get_mut(&unit_done.unit_tag_index) {
        unit.last_game_loop = game_loop;
        unit.is_init = false;
        unit.set_unit_props();
        UnitChangeHint::Registered {
            unit: Box::new(unit.clone()),
            creator: None,
        }
    } else {
        tracing::warn!(
            "Unit {} done but not init before.",
            unit_done.unit_tag_index
        );
        UnitChangeHint::None
    }
}

#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_unit_position(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_pos: &UnitPositionsEvent,
) -> UnitChangeHint {
    let mut updated_unit_ids = vec![];
    let unit_positions = unit_pos.clone().to_unit_positions_vec();
    for unit_pos_item in &unit_positions {
        if let Some(ref mut sc2_unit) = sc2_state.units.get_mut(&unit_pos_item.tag) {
            updated_unit_ids.push(unit_pos_item.tag);
            sc2_unit.pos = Vec3D::new(
                unit_pos_item.x as f32 / UNIT_POSITION_RATIO,
                unit_pos_item.y as f32 / UNIT_POSITION_RATIO,
                0.,
            );
            sc2_unit.last_game_loop = game_loop;
        } else {
            tracing::debug!(
                "Unit {} did not exist but position registered.",
                unit_pos_item.tag
            );
        }
    }
    let updated_units = updated_unit_ids
        .iter()
        .filter_map(|id| sc2_state.units.get(id))
        .cloned()
        .collect();
    UnitChangeHint::Positions(updated_units)
}
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_unit_died(
    sc2_state: &mut SC2ReplayState,
    game_loop: i64,
    unit_dead: &UnitDiedEvent,
) -> UnitChangeHint {
    // Clean up the unit from previous groups where it was selected.
    for (_idx, state) in sc2_state.user_state.iter_mut() {
        for group_idx in 0..10 {
            state.control_groups[group_idx].retain(|&x| x != unit_dead.unit_tag_index);
        }
    }
    match sc2_state.units.remove(&unit_dead.unit_tag_index) {
        None => {
            // This may happen when a larva is transformed to a unit in zerg. so this is normal.
            tracing::warn!(
                "Unit {} reported dead but was not registered before.",
                unit_dead.unit_tag_index
            );
            UnitChangeHint::None
        }
        Some(val) => {
            tracing::debug!("Unit died: {:?}", val);
            let killer_unit: Option<SC2Unit> = match unit_dead.killer_unit_tag_index {
                Some(killer_unit_tag_index) => sc2_state.units.get(&killer_unit_tag_index).cloned(),
                None => None,
            };
            UnitChangeHint::Unregistered {
                killer: killer_unit,
                killed: Box::new(val),
            }
        }
    }
}
/// Handles a tracker event as it steps through the SC2 State
#[tracing::instrument(level = "debug", skip(sc2_state))]
pub fn handle_tracker_event(
    sc2_state: &mut SC2ReplayState,
    tracker_loop: i64,
    evt: &ReplayTrackerEvent,
) -> UnitChangeHint {
    match &evt {
        ReplayTrackerEvent::UnitBorn(unit_born) => {
            handle_unit_born(sc2_state, tracker_loop, unit_born)
        }
        ReplayTrackerEvent::UnitTypeChange(unit_change) => {
            handle_unit_type_change(sc2_state, tracker_loop, unit_change)
        }
        ReplayTrackerEvent::UnitInit(unit_init) => {
            handle_unit_init(sc2_state, tracker_loop, unit_init)
        }
        ReplayTrackerEvent::UnitDone(unit_done) => {
            handle_unit_done(sc2_state, tracker_loop, unit_done)
        }
        ReplayTrackerEvent::UnitDied(unit_died) => {
            handle_unit_died(sc2_state, tracker_loop, unit_died)
        }
        ReplayTrackerEvent::UnitPosition(unit_pos) => {
            handle_unit_position(sc2_state, tracker_loop, unit_pos)
        }
        ReplayTrackerEvent::PlayerStats(_player_stats) => {
            // For now the player stats are not recorded here.
            UnitChangeHint::None
        }
        ReplayTrackerEvent::PlayerSetup(player_setup) => {
            if let Some(user_id) = player_setup.user_id {
                sc2_state
                    .user_state
                    .insert(user_id as i64, SC2UserState::new());
                // TODO: Should we return the user_id to the consumer?
            }
            UnitChangeHint::None
        }
        _ => {
            tracing::debug!("Skipping event: {:?}", evt);
            UnitChangeHint::None
        }
    }
}
