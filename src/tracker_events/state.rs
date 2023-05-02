//! Handles the state change of units for TrackerEvents

use super::*;
use crate::*;

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
