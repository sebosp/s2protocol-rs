//! Why a unit of a UnitDied event left the game, see [`UnitDiedCause`].
//!
//! The tracker events have no event for a unit that becomes another one or whose time runs out:
//! the unit dies, without a killer or killed by itself or its creator. What happened is read from
//! the events of the same game loop and of the one before, which come before the death:
//! - a larva (whose egg is a larva again) dies right after the UnitBorn of the unit it hatched,
//!   whose creator it is; so does a zergling with its baneling cocoon;
//! - a drone dies at the loop of the UnitDone of its building, or the next one, at its place; two
//!   templars die with the UnitDone of their archon;
//! - the drone of a building killed while it is built, the templars of an archon killed while they
//!   merge and the units in a transport die without a killer, at the loop of the death of their
//!   container or the next one, at its place.

use super::{EXPIRING_UNIT_TYPES, UnitBornEvent, UnitDiedCause, UnitDiedEvent};
use crate::common::Vec3D;
use crate::game_events::VersionedBalanceUnits;
use crate::state::SC2Unit;
use std::collections::HashMap;

/// How far, in map cells, the unit that dies is from the unit that replaces or contains it. A drone
/// dies 2.2 cells from the center of its extractor, 0 to 1 from the other buildings.
const SAME_PLACE_DISTANCE: f32 = 3.0;

/// The units born, done or killed by another player in the last game loops, and the creators of
/// the units, kept by [`crate::state::SC2ReplayState`] to classify the deaths.
#[derive(Debug, Default, Clone)]
pub(crate) struct UnitDeathContext {
    /// The creator (tag index and recycle) of a unit, by its tag index, with its tag recycle.
    creators: HashMap<u32, (u32, (u32, u32))>,
    /// The changes of the current game loop and of the one before.
    recent: Vec<RecentChange>,
}

#[derive(Debug, Clone)]
struct RecentChange {
    game_loop: i64,
    player: Option<u8>,
    kind: RecentKind,
    /// The places of the unit: where its event happened and where the state has it.
    places: [(f32, f32); 2],
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RecentKind {
    /// A unit born from the unit of this tag index and recycle.
    BornFrom(u32, u32),
    /// A unit whose construction or morph is done.
    Done,
    /// A unit killed by another player.
    Killed,
}

fn place(pos: &Vec3D) -> (f32, f32) {
    (pos.x(), pos.y())
}

impl UnitDeathContext {
    /// Forgets the changes older than the game loop before this one.
    fn prune(&mut self, game_loop: i64) {
        self.recent
            .retain(|change| change.game_loop + 1 >= game_loop);
    }

    fn push(&mut self, change: RecentChange) {
        self.prune(change.game_loop);
        self.recent.push(change);
    }

    /// A unit was born, from its creator if it has one.
    pub(crate) fn born(&mut self, game_loop: i64, event: &UnitBornEvent) {
        let (Some(creator), Some(creator_recycle)) =
            (event.creator_unit_tag_index, event.creator_unit_tag_recycle)
        else {
            return;
        };
        self.creators.insert(
            event.unit_tag_index,
            (event.unit_tag_recycle, (creator, creator_recycle)),
        );
        let at = (event.x as f32, event.y as f32);
        self.push(RecentChange {
            game_loop,
            player: Some(event.control_player_id),
            kind: RecentKind::BornFrom(creator, creator_recycle),
            places: [at, at],
        });
    }

    /// The construction or morph of a unit is done.
    pub(crate) fn done(&mut self, game_loop: i64, unit: &SC2Unit) {
        let at = place(&unit.pos);
        self.push(RecentChange {
            game_loop,
            player: unit.user_id,
            kind: RecentKind::Done,
            places: [at, at],
        });
    }

    /// The creator of a unit, by tag index and recycle.
    fn creator(&self, unit: &UnitDiedEvent) -> Option<(u32, u32)> {
        self.creators
            .get(&unit.unit_tag_index)
            .filter(|(recycle, _)| *recycle == unit.unit_tag_recycle)
            .map(|(_, creator)| *creator)
    }

    /// Whether a change of this kind happened to a unit of the player at this game loop or the
    /// one before, at the place of the unit that dies.
    fn near(&self, game_loop: i64, player: Option<u8>, at: (f32, f32), kind: RecentKind) -> bool {
        self.recent.iter().any(|change| {
            change.kind == kind
                && change.player == player
                && change.game_loop + 1 >= game_loop
                && change
                    .places
                    .iter()
                    .any(|(x, y)| (x - at.0).hypot(y - at.1) <= SAME_PLACE_DISTANCE)
        })
    }

    /// Why the unit died, see [`UnitDiedCause`]. The unit is the one the state had for the tag of
    /// the event, before it was removed.
    pub(crate) fn classify(
        &mut self,
        game_loop: i64,
        event: &UnitDiedEvent,
        killed: &SC2Unit,
        balance_units: &VersionedBalanceUnits,
    ) -> UnitDiedCause {
        self.prune(game_loop);
        let creator = self.creator(event);
        self.creators.remove(&event.unit_tag_index);
        let tag = (event.unit_tag_index, event.unit_tag_recycle);
        let killer_unit = event
            .killer_unit_tag_index
            .zip(event.killer_unit_tag_recycle);
        let at = (event.x as f32, event.y as f32);
        let expiring = EXPIRING_UNIT_TYPES.contains(&killed.name.as_str());
        let cause = if killed
            .creator_ability_name
            .as_deref()
            .is_some_and(|ability| ability.starts_with("Hallucination"))
        {
            UnitDiedCause::Hallucination
        } else if let Some(killer) = event.killer_player_id {
            if Some(killer) != killed.user_id {
                UnitDiedCause::Killed
            } else if killer_unit == Some(tag)
                && killed.is_init
                && is_structure(killed, balance_units)
            {
                UnitDiedCause::Cancelled
            } else if expiring || (killer_unit.is_some() && killer_unit == creator) {
                UnitDiedCause::Expired
            } else {
                UnitDiedCause::OwnPlayer
            }
        } else if self.recent.iter().any(|change| {
            change.game_loop == game_loop && change.kind == RecentKind::BornFrom(tag.0, tag.1)
        }) || self.near(game_loop, killed.user_id, at, RecentKind::Done)
        {
            UnitDiedCause::Morphed
        } else if self.near(game_loop, killed.user_id, at, RecentKind::Killed) {
            UnitDiedCause::Killed
        } else if expiring {
            UnitDiedCause::Expired
        } else {
            UnitDiedCause::Removed
        };
        if cause == UnitDiedCause::Killed && event.killer_player_id.is_some() {
            self.push(RecentChange {
                game_loop,
                player: killed.user_id,
                kind: RecentKind::Killed,
                places: [at, place(&killed.pos)],
            });
        }
        cause
    }
}

/// Whether the balance data has the Structure attribute for the unit, true for a unit it does not
/// have.
fn is_structure(unit: &SC2Unit, balance_units: &VersionedBalanceUnits) -> bool {
    balance_units.get(&unit.name).is_none_or(|balance| {
        balance
            .attributes
            .inner
            .iter()
            .any(|attribute| attribute.kind.as_deref() == Some("Structure"))
    })
}
