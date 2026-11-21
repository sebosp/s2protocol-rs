//! A protocol agnostic Unit Died

use crate::filters::SC2ReplayFilters;
use serde::{Deserialize, Serialize};

/// A Unit died event, it does not contain the unit owner player id.
/// Or the unit name.  For a process of enriching this data see UnitBornEventFlatRow.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitDiedEvent {
    pub unit_tag_index: u32,
    pub unit_tag_recycle: u32,
    pub killer_player_id: Option<u8>,
    pub x: u8,
    pub y: u8,
    pub killer_unit_tag_index: Option<u32>,
    pub killer_unit_tag_recycle: Option<u32>,
    /// Why the unit left the game, from the state of the replay: the tracker events report a
    /// UnitDied for a drone that becomes a building or a MULE whose time ran out too.
    /// [`UnitDiedCause::Unknown`] for an event read without the state (the event readers),
    /// [`crate::state::SC2EventIterator`] and [`crate::tracker_events::handle_tracker_event`]
    /// classify it.
    #[serde(default)]
    pub cause: UnitDiedCause,
}

/// Why a unit of a UnitDied event left the game. The tracker events report the same UnitDied for
/// a unit that is killed and for a unit that becomes another one, expires or is cancelled; they
/// are told apart with the killer and the events of the same game loop, see
/// [`crate::tracker_events::handle_unit_died`].
///
/// On the 22,037 games of two players of SpawningTool (2026-11-21): 15.1M UnitDied, 6.5M killed,
/// 4.8M expired (3.8M are the InvisibleTargetDummy of the lurkers), 3.5M morphed (3.0M larvae,
/// 270k drones for the 270k Zerg buildings done, 195k zerglings, 60k templars), 235k by their own
/// player, 39k cancelled (35.6k `BuildInProgress.Cancel` commands), 15k hallucinations, 27k
/// removed (most in arcade maps and players that left).
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitDiedCause {
    /// Not classified: the event was read without the state of the replay.
    #[default]
    Unknown,
    /// Killed by another player (an enemy, or the neutral player): the killer of the event, or,
    /// without a killer, a unit inside a transport or a construction that was killed at the same
    /// time (the drone of a spore crawler killed while it is built, the templars of an archon
    /// killed while it merges, the units in a dead medivac or overlord).
    Killed,
    /// Killed by its own player or by itself: friendly fire, a baneling that exploded, a Terran
    /// building that burned down, a salvaged bunker, a warp-in that lost its power. A loss for the
    /// player, not a kill of the other player.
    OwnPlayer,
    /// Became another unit, nothing was lost: a larva (after its egg) that hatched, a zergling that
    /// became a baneling cocoon, a drone that became a building (it dies when the building is
    /// done), two templars that merged into an archon. No killer, the new unit is born (from this
    /// unit) or done at the same loop or the one before, at the same place.
    Morphed,
    /// A unit with a timed life, or that its creator removes, ended: MULE, AutoTurret,
    /// AdeptPhaseShift, Broodling, Locust, Changeling, KD8Charge, the Purification Nova of a
    /// disruptor (DisruptorPhased), ForceField, the larvae of a dead hatchery, see
    /// [`EXPIRING_UNIT_TYPES`]. Killed by another player, these are [`UnitDiedCause::Killed`].
    Expired,
    /// A structure whose construction was cancelled by its owner (it kills itself while it is
    /// built), most of its cost is refunded.
    Cancelled,
    /// A hallucination of a sentry (born from a `Hallucination*` ability) that ended or was
    /// killed: not a unit of the player.
    Hallucination,
    /// No killer and nothing in the events explains it (a mineral field mined out, rocks, the
    /// units of a player that left).
    Removed,
}

impl UnitDiedCause {
    /// The name of the cause, as written in the `cause` column of the snapshot.
    pub fn as_str(&self) -> &'static str {
        match self {
            UnitDiedCause::Unknown => "unknown",
            UnitDiedCause::Killed => "killed",
            UnitDiedCause::OwnPlayer => "own_player",
            UnitDiedCause::Morphed => "morphed",
            UnitDiedCause::Expired => "expired",
            UnitDiedCause::Cancelled => "cancelled",
            UnitDiedCause::Hallucination => "hallucination",
            UnitDiedCause::Removed => "removed",
        }
    }

    /// Whether the player lost the unit: killed by another player or by its own player. A morph,
    /// an expired or cancelled unit, a hallucination are not losses.
    pub fn is_loss(&self) -> bool {
        matches!(self, UnitDiedCause::Killed | UnitDiedCause::OwnPlayer)
    }

    /// Whether the unit was killed by another player: the killer of the event, if any, gets the
    /// kill.
    pub fn is_kill(&self) -> bool {
        matches!(self, UnitDiedCause::Killed)
    }
}

impl std::fmt::Display for UnitDiedCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The unit types whose death without another player's killer is the end of their timed life or
/// of the ability that made them, [`UnitDiedCause::Expired`]. The balance data has no timed life,
/// and its cost cannot tell them apart (SupplyDepotLowered, SiegeTankSieged and the Archon have no
/// cost either). Their killer is themselves, their creator (an OrbitalCommand for a MULE, the
/// Adept for its shade) or their owner without a unit (a broodling, the MULE of a dead orbital).
pub const EXPIRING_UNIT_TYPES: &[&str] = &[
    "AdeptPhaseShift",
    "AutoTurret",
    "Broodling",
    "BroodlingEscort",
    "Changeling",
    "ChangelingMarine",
    "ChangelingMarineShield",
    "ChangelingZealot",
    "ChangelingZergling",
    "ChangelingZerglingWings",
    "DisruptorPhased",
    "ForceField",
    "InvisibleTargetDummy",
    "KD8Charge",
    "Larva",
    "Locust",
    "LocustMP",
    "LocustMPFlying",
    "LocustMPPrecursor",
    "MULE",
    "OracleStasisTrap",
    "ParasiticBombDummy",
    "ParasiticBombRelayDummy",
    "PhotonBarrier",
    "PointDefenseDrone",
    "RavenRepairDrone",
];

impl UnitDiedEvent {
    pub fn should_skip(&self, filters: &SC2ReplayFilters) -> bool {
        // TODO: Should we add the unit_died_player_id/name?
        if let Some(player_id) = filters.player_id
            && self.killer_player_id != Some(player_id)
        {
            return true;
        }
        false
    }
}
