//! What the players did each minute that is too frequent for a row per event: the camera moves,
//! the control groups and the selections. A pro game has about 10,000 camera moves, 3,000 control
//! group recalls and 5,000 selection changes; the commands have a row each in the cmd files.

#[cfg(feature = "dep_arrow")]
use arrow::datatypes::Schema;

use crate::UnitChangeHint;
use crate::game_events::{GameEControlGroupUpdate, ReplayGameEvent};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The actions of a player in a minute of a game, a row of
/// [`crate::arrow_store::GAME_ACTIONS_ARROW_FNAME`].
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GameActionsFlatRow {
    pub ext_fs_id: u64,
    pub user_id: i64,
    pub player_name: Option<String>,
    /// The minute of the game, from 0.
    pub minute: u32,
    /// The commands of the minute, with or without a target, also the ones of the bots that the
    /// cmd files leave out.
    pub commands: u32,
    /// The moves of the camera to a point of the map: the edge scrolls, the minimap clicks, the
    /// camera hotkeys.
    pub camera_moves: u32,
    /// The control groups set or appended to, with or without stealing the units of other groups.
    pub control_group_assigns: u32,
    pub control_group_recalls: u32,
    /// The control groups assigned in the minute, a bit per group (bit 0 is group 0).
    pub control_groups_assigned: u32,
    /// The changes of the selection: clicks, box selections and control group recalls.
    pub selection_changes: u32,
}

/// The selections of a unit type by a player in a minute of a game, a row of
/// [`crate::arrow_store::GAME_SELECTIONS_ARROW_FNAME`]: what the player looked at and controlled.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GameSelectionsFlatRow {
    pub ext_fs_id: u64,
    pub user_id: i64,
    pub player_name: Option<String>,
    pub minute: u32,
    /// A unit type in the selection, i.e. `Marine`.
    pub unit_type_name: String,
    /// The selection changes with units of this type.
    pub selections: u32,
    /// The units of this type in these selections, added up.
    pub units: u32,
    /// The second of the game of the first and the last of these selections.
    pub first_seconds: u32,
    pub last_seconds: u32,
}

/// Counts the actions of the players of a game as its events are read.
#[derive(Debug, Default)]
pub struct GameActionsAccumulator {
    actions: BTreeMap<(i64, u32), GameActionsFlatRow>,
    selections: BTreeMap<(i64, u32, String), GameSelectionsFlatRow>,
}

impl GameActionsAccumulator {
    /// Adds a game event of the player `user_id`, the change hint is the one of the event
    /// iterator (the units of the new selection).
    pub fn add(
        &mut self,
        user_id: i64,
        player_name: &Option<String>,
        game_loop: i64,
        event: &ReplayGameEvent,
        change_hint: &UnitChangeHint,
    ) {
        let seconds = crate::convert_game_loop_to_seconds(game_loop);
        let minute = seconds / 60;
        let actions = self
            .actions
            .entry((user_id, minute))
            .or_insert_with(|| GameActionsFlatRow {
                user_id,
                player_name: player_name.clone(),
                minute,
                ..Default::default()
            });
        match event {
            ReplayGameEvent::Cmd(_) => actions.commands += 1,
            ReplayGameEvent::CameraUpdate(camera) if camera.m_target.is_some() => {
                actions.camera_moves += 1
            }
            ReplayGameEvent::ControlGroupUpdate(update) => match update.m_control_group_update {
                GameEControlGroupUpdate::ERecall => actions.control_group_recalls += 1,
                GameEControlGroupUpdate::EClear => {}
                GameEControlGroupUpdate::ESet
                | GameEControlGroupUpdate::EAppend
                | GameEControlGroupUpdate::ESetAndSteal
                | GameEControlGroupUpdate::EAppendAndSteal => {
                    actions.control_group_assigns += 1;
                    if update.m_control_group_index < 32 {
                        actions.control_groups_assigned |= 1 << update.m_control_group_index;
                    }
                }
            },
            _ => {}
        }
        let UnitChangeHint::Selection(units) = change_hint else {
            return;
        };
        actions.selection_changes += 1;
        let mut types: BTreeMap<&str, u32> = BTreeMap::new();
        for unit in units {
            *types.entry(unit.name.as_str()).or_default() += 1;
        }
        for (unit_type_name, count) in types {
            let row = self
                .selections
                .entry((user_id, minute, unit_type_name.to_string()))
                .or_insert_with(|| GameSelectionsFlatRow {
                    user_id,
                    player_name: player_name.clone(),
                    minute,
                    unit_type_name: unit_type_name.to_string(),
                    first_seconds: seconds,
                    ..Default::default()
                });
            row.selections += 1;
            row.units += count;
            row.last_seconds = seconds;
        }
    }

    /// The rows of the game, by player and minute.
    pub fn rows(self, ext_fs_id: u64) -> (Vec<GameActionsFlatRow>, Vec<GameSelectionsFlatRow>) {
        let actions = self
            .actions
            .into_values()
            // The observers and the events before the game have no player.
            .filter(|row| row.player_name.is_some())
            .map(|row| GameActionsFlatRow { ext_fs_id, ..row })
            .collect();
        let selections = self
            .selections
            .into_values()
            .filter(|row| row.player_name.is_some())
            .map(|row| GameSelectionsFlatRow { ext_fs_id, ..row })
            .collect();
        (actions, selections)
    }
}

impl GameActionsFlatRow {
    #[cfg(feature = "dep_arrow")]
    pub fn schema() -> Schema {
        <Self as crate::arrow_store::rows::ArrowRow>::schema()
    }
}

impl GameSelectionsFlatRow {
    #[cfg(feature = "dep_arrow")]
    pub fn schema() -> Schema {
        <Self as crate::arrow_store::rows::ArrowRow>::schema()
    }
}

#[cfg(feature = "dep_arrow")]
crate::impl_arrow_row!(GameActionsFlatRow);

#[cfg(feature = "dep_arrow")]
crate::impl_arrow_row!(GameSelectionsFlatRow);
