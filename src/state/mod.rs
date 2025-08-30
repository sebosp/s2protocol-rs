//! Handling of state of SC2 Replay as it steps through game loops
//!
//!
//! Events are ordered by their "priority", this is a guessed priority for now.
//! For example, if a TrackerEvent and a GameEvent, happen at the same game loop,
//! the tracker events take priority (See the const below). This may not be true but
//! seems to work so far.
//! In this version, the game_loop will be multiplied by 10 and added the priority.
//! This means 10 max events types are supported.

use super::*;
use crate::filters::SC2ReplayFilters;
use crate::game_events::{
    handle_game_event, GameEventIteratorState, VersionedBalanceUnit, VersionedBalanceUnits,
};
use crate::tracker_events::{handle_tracker_event, TrackertEventIteratorState};
use crate::{common::*, game_events::GameSPointMini};
use game_events::GameSCmdEvent;
use serde::{Deserialize, Serialize};
pub mod unit_cmd;
pub mod unit_props;
pub use unit_cmd::*;
pub use unit_props::*;

pub const TRACKER_PRIORITY: i64 = 1;
pub const GAME_PRIORITY: i64 = 2;

/// The game event loops and tracker event loops differ in their units.
/// The true ratio should be identified somehow.
/// There seems to be a ratio and the ratio based on initial calculations seems to be:
pub const TRACKER_SPEED_RATIO: f32 = 0.70996;

/// The currently selected units is stored as a group outside of the boundaries of the usable
/// groups.
pub const ACTIVE_UNITS_GROUP_IDX: usize = 10usize;

/// Unit Attributes, this changes through time as the state machine overwrites the values.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SC2Unit {
    /// The tag index of the unit.
    pub tag_index: u32,
    /// The last time the unit was updated
    pub last_game_loop: i64,
    /// The owner user_id
    pub user_id: Option<u8>,
    /// The name of the unit.
    pub name: String,
    /// The XYZ position.
    pub pos: Vec3D,
    /// The game loop in which the unit was created.
    pub init_game_loop: i64,
    /// The creator ability name.
    pub creator_ability_name: Option<String>,
    // Potentially a creator of the unit
    pub creator_tag_index: Option<String>,
    /// The radius of the unit, this is a parameter that may be stored
    /// by the client side better, since it's very specific to Swarmy.
    /// Maybe next version we can move it there.
    pub radius: f32,
    /// The color of the unit, this should be later on allowed to be overridden by the client.
    pub color: [u8; 4],
    /// Whether the unit is selected
    pub is_selected: bool,
    /// Whether the unit is in Initializing state, for example morphing.
    pub is_init: bool,
    /// The current unit command.
    pub cmd: SC2UnitCmd,
}

impl Ord for SC2Unit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tag_index.cmp(&other.tag_index)
    }
}

impl PartialEq for SC2Unit {
    fn eq(&self, other: &Self) -> bool {
        self.tag_index == other.tag_index
    }
}

impl PartialOrd for SC2Unit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SC2Unit {}

impl SC2Unit {
    /// Sets the unit properties based on the unit name.
    pub fn set_unit_props(&mut self, balance_units: &VersionedBalanceUnits) {
        let (radius, color) =
            get_unit_sized_color(&self.name, self.user_id.unwrap_or(0) as i64, balance_units);
        self.radius = radius;
        self.color = color;
    }
}

/// Supported event types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SC2EventType {
    Tracker {
        tracker_loop: i64,
        event: ReplayTrackerEvent,
    },
    Game {
        game_loop: i64,
        user_id: i64,
        event: ReplayGameEvent,
    },
}

impl SC2EventType {
    pub fn should_skip(&self, filters: &SC2ReplayFilters) -> bool {
        match self {
            SC2EventType::Tracker { event, .. } => event.should_skip(filters),
            SC2EventType::Game { event, user_id, .. } => {
                if let Some(user_id_filter) = filters.player_id {
                    if *user_id as u8 != user_id_filter {
                        return true;
                    }
                }
                event.should_skip(filters)
            }
        }
    }
}

/// When a unit changes in the state, certain information is provided back.
/// For example, if the unit dies, it is deleted from the state, but all its information is
/// returned back for reporting purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitChangeHint {
    /// A unit has been added, the full unit is returned in case the caller wants to inspect it.
    /// This covers UnitBorn, InitInit, UnitDone, and UnitTypeChange.
    Registered {
        unit: Box<SC2Unit>,
        creator: Option<SC2Unit>,
    },
    /// Unit positions are being reported, a vector of units changed is returned.
    Positions(Vec<SC2Unit>),
    /// Unit positions are being reported, a vector of units changed is returned.
    TargetPoints(Vec<SC2Unit>),
    /// Selected units in the first item of the tuple (.0) are targetting the unit on the second item of the tuple (.1)
    TargetUnits {
        units: Vec<SC2Unit>,
        target: Box<SC2Unit>,
    },
    /// A unit has been deleted from the state registry, the full killer unit information and the
    /// killed unit is returned. Killer is cloned and may be expensive.
    Unregistered {
        killer: Option<SC2Unit>,
        killed: Box<SC2Unit>,
    },
    /// An ability has been used, the unit cmd abilities should be inspected, it may potentially
    /// target another unit (last param.)
    Abilities {
        units: Vec<SC2Unit>,
        event: GameSCmdEvent,
        target: Option<SC2Unit>,
    },
    /// A set of units has been selected and are in the active control group.
    Selection(Vec<SC2Unit>),
    /// No units have changed, for example, PlayerStats are generated, so nothing to inspect
    None,
}

impl UnitChangeHint {
    /// Retuns the name of the variant, for short debugging.
    pub fn variant_name(&self) -> &'static str {
        match self {
            UnitChangeHint::Registered { .. } => "Registered",
            UnitChangeHint::Positions(_) => "Positions",
            UnitChangeHint::TargetPoints(_) => "TargetPoints",
            UnitChangeHint::TargetUnits { .. } => "TargetUnits",
            UnitChangeHint::Unregistered { .. } => "Unregistered",
            UnitChangeHint::Abilities { .. } => "Abilities",
            UnitChangeHint::Selection(_) => "Selection",
            UnitChangeHint::None => "None",
        }
    }
}

/// The user state as it's collected through time.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SC2UserState {
    /// An array of registered control groups per user, the control group indexed as 10th is the
    /// currently selected units.
    pub control_groups: Vec<Vec<u32>>,

    /// The camera position.
    pub camera_pos: GameSPointMini,
}

impl SC2UserState {
    pub fn new() -> Self {
        let mut control_groups = vec![];
        // populate as empty control groups.
        for _ in 0..11 {
            control_groups.push(vec![]);
        }
        Self {
            control_groups,
            camera_pos: GameSPointMini { x: 0, y: 0 },
        }
    }
}

/// The state of the replay as it's being processed, units are added to owners, control groups are
/// updated, unit position recorded, etc.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SC2ReplayState {
    /// The registered units state as they change through time.
    /// These are with unit index as reference
    pub units: HashMap<u32, SC2Unit>,

    /// The per-user state, the control groups, the supply, units, upgrades, as it progresses
    /// through time.
    pub user_state: HashMap<i64, SC2UserState>,

    /// The filename of the replay
    pub filename: String,

    /// The sha256 digest of the replay file.
    pub sha256: String,
}

impl SC2ReplayState {
    /// When an event is meant to be consumed (i.e. it's the next in order), then the state of the
    /// game needs to transition through the event, returning a Hint of what has changed.
    pub fn handle_transition_to_next_event(
        &mut self,
        balance_units: &VersionedBalanceUnits,
        event: SC2EventType,
    ) -> SC2EventIteratorItem {
        match event {
            SC2EventType::Tracker {
                tracker_loop,
                event,
            } => {
                let change_hint = handle_tracker_event(self, tracker_loop, &event, balance_units);
                tracing::info!(
                    "Trac [{:>08}]: Evt:{:?} Hint:{:?}",
                    tracker_loop,
                    event,
                    change_hint
                );
                SC2EventIteratorItem {
                    event_type: SC2EventType::Tracker {
                        tracker_loop,
                        event,
                    },
                    change_hint,
                }
            }
            SC2EventType::Game {
                game_loop,
                user_id,
                event,
            } => {
                let (enriched_event, change_hint) =
                    handle_game_event(self, game_loop, user_id, event, balance_units);
                tracing::info!(
                    "Game [{:>08}]: uid: {} Evt:{:?} Hint:{:?}",
                    game_loop,
                    user_id,
                    enriched_event,
                    change_hint
                );
                SC2EventIteratorItem {
                    event_type: SC2EventType::Game {
                        game_loop,
                        user_id,
                        event: enriched_event,
                    },
                    change_hint,
                }
            }
        }
    }
}

/// The iterator that returns the events as they happen.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SC2EventIterator {
    /// The protocol version
    protocol_version: u32,
    /// The replay state machine transducer
    pub sc2_state: SC2ReplayState,
    /// The tracker event iterator.
    tracker_iterator_state: TrackertEventIteratorState,
    /// The game event iterator.
    game_iterator_state: GameEventIteratorState,
    /// The next event coming from the tracker iterator.
    next_tracker_event: Option<SC2EventType>,
    /// The next event coming from the game iterator.
    next_game_event: Option<SC2EventType>,
    /// The iterator filter helpers
    filters: Option<SC2ReplayFilters>,
    /// The current protocol version abilities, containing a possible string translation.
    balance_units: VersionedBalanceUnits,
}

impl SC2EventIterator {
    /// Creates a new SC2EventIterator from a PathBuf
    #[tracing::instrument(level = "debug")]
    pub fn new(
        source: &PathBuf,
        multi_version_abilities: HashMap<(u32, String), VersionedBalanceUnit>,
    ) -> Result<Self, S2ProtocolError> {
        let total_initial_abilities = multi_version_abilities.len();
        // The sc2 replay state is not shared between the two iterators...
        tracing::debug!("Processing {:?}", source);
        let file_contents = crate::read_file(source)?;
        let source_filename = format!("{source:?}");
        let (_input, mpq) = crate::parser::parse(&file_contents)?;
        let (_tail, proto_header) = crate::read_protocol_header(&mpq)?;
        let (_event_tail, tracker_events) =
            mpq.read_mpq_file_sector("replay.tracker.events", false, &file_contents)?;
        let (_event_tail, game_events) =
            mpq.read_mpq_file_sector("replay.game.events", false, &file_contents)?;
        let sc2_state = SC2ReplayState {
            filename: source_filename,
            ..Default::default()
        };
        let balance_units: HashMap<String, VersionedBalanceUnit> = multi_version_abilities
            .into_iter()
            .filter_map(|((version, name), unit)| {
                if version == proto_header.m_version.m_base_build {
                    Some((name, unit))
                } else {
                    None
                }
            })
            .collect();
        tracing::info!(
            "Collected {} unit definitions for protocol version {} out of {} total definitions",
            balance_units.len(),
            proto_header.m_version.m_base_build,
            total_initial_abilities
        );
        Ok(Self {
            protocol_version: proto_header.m_version.m_base_build,
            sc2_state,
            tracker_iterator_state: tracker_events.into(),
            game_iterator_state: game_events.into(),
            balance_units,
            ..Default::default()
        })
    }

    /// Sets the filters for the iterator
    pub fn with_filters(mut self, filters: SC2ReplayFilters) -> Self {
        self.filters = Some(filters);
        self
    }

    /// Returns the tracker loop inside the next_tracker_event collected.
    fn get_tracker_loop(&self) -> Option<i64> {
        match self.next_tracker_event.as_ref()? {
            SC2EventType::Tracker { tracker_loop, .. } => Some(*tracker_loop),
            _ => None,
        }
    }

    /// Returns the game loop inside the next_game_event collected.
    fn get_game_loop(&self) -> Option<i64> {
        match self.next_game_event.as_ref()? {
            SC2EventType::Game { game_loop, .. } => Some(*game_loop),
            _ => None,
        }
    }

    /// Consumes the Iterator collecting only the CmdTargetPoint events into a vector of CmdEventFlatRow
    #[cfg(feature = "dep_arrow")]
    pub fn collect_into_game_cmd_target_points_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<game_events::CmdTargetPointEventFlatRow> {
        // We could return some Iterator and write in batches.
        // Right now everything is expanded into memory for a given replay.
        let res: Vec<game_events::CmdTargetPointEventFlatRow> = self
            .into_iter()
            .flat_map(|event_item| {
                if let SC2EventType::Game {
                    event: game_events::ReplayGameEvent::Cmd(event),
                    game_loop,
                    user_id,
                } = event_item.event_type
                {
                    if let game_events::GameSCmdData::TargetPoint(_) = event.m_data {
                        return game_events::CmdTargetPointEventFlatRow::new(
                            details,
                            event,
                            game_loop,
                            user_id,
                            event_item.change_hint,
                        );
                    }
                }
                vec![]
            })
            .collect();
        tracing::error!("Collected {} CmdTargetPointEventFlatRow rows", res.len());
        res
    }

    /// Consumes the Iterator collecting only the CmdTargetUnit events into a vector of CmdEventFlatRow
    #[cfg(feature = "dep_arrow")]
    pub fn collect_into_game_cmd_target_units_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<game_events::CmdTargetUnitEventFlatRow> {
        let res: Vec<game_events::CmdTargetUnitEventFlatRow> = self
            .into_iter()
            .flat_map(|event_item| {
                if let SC2EventType::Game {
                    event: game_events::ReplayGameEvent::Cmd(event),
                    game_loop,
                    user_id,
                } = event_item.event_type
                {
                    if let game_events::GameSCmdData::TargetUnit(_) = event.m_data {
                        return game_events::CmdTargetUnitEventFlatRow::new(
                            details,
                            event,
                            game_loop,
                            user_id,
                            event_item.change_hint,
                        );
                    }
                }
                vec![]
            })
            .collect();
        tracing::error!("Collected {} CmdTargetUnitEventFlatRow rows", res.len());
        res
    }

    /// Consumes the Iterator collecting only the PlayerStats events into a vector of PlayerStatsFlatRow
    #[cfg(feature = "dep_arrow")]
    pub fn collect_into_player_stats_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::PlayerStatsFlatRow> {
        self.into_iter()
            .filter_map(|event_item| match event_item.event_type {
                SC2EventType::Tracker {
                    tracker_loop,
                    event,
                } => {
                    if let tracker_events::ReplayTrackerEvent::PlayerStats(event) = event {
                        Some(tracker_events::PlayerStatsFlatRow::new(
                            event,
                            tracker_loop,
                            details.clone(),
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }

    /// Consumes the Iterator collecting only the Upgrade events into a vector of UpgradeEventFlatRow
    #[cfg(feature = "dep_arrow")]
    pub fn collect_into_upgrades_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::UpgradeEventFlatRow> {
        self.into_iter()
            .filter_map(|event_item| match event_item.event_type {
                SC2EventType::Tracker {
                    tracker_loop,
                    event,
                } => {
                    if let tracker_events::ReplayTrackerEvent::Upgrade(event) = event {
                        Some(tracker_events::UpgradeEventFlatRow::new(
                            event,
                            tracker_loop,
                            details.clone(),
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }

    /// Consumes the Iterator collecting only the UnitBorn events into a vector of UnitBornEventFlatRow
    #[cfg(feature = "dep_arrow")]
    pub fn collect_into_unit_born_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::UnitBornEventFlatRow> {
        self.into_iter()
            .filter_map(|event_item| match event_item.event_type {
                SC2EventType::Tracker {
                    tracker_loop,
                    event,
                } => match event {
                    tracker_events::ReplayTrackerEvent::UnitBorn(event) => {
                        tracker_events::UnitBornEventFlatRow::from_unit_born(
                            event,
                            tracker_loop,
                            details,
                            event_item.change_hint,
                        )
                    }
                    tracker_events::ReplayTrackerEvent::UnitDone(event) => {
                        tracker_events::UnitBornEventFlatRow::from_unit_done(
                            event,
                            tracker_loop,
                            details,
                            event_item.change_hint,
                        )
                    }
                    tracker_events::ReplayTrackerEvent::UnitTypeChange(event) => {
                        match event_item.change_hint {
                            UnitChangeHint::None => None,
                            change_hint => {
                                tracker_events::UnitBornEventFlatRow::from_unit_type_change(
                                    event,
                                    tracker_loop,
                                    details,
                                    change_hint,
                                )
                            }
                        }
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect()
    }

    /// Consumes the Iterator collecting only the UnitDied events into a vector of UnitBornEventFlatRow
    #[cfg(feature = "dep_arrow")]
    pub fn collect_into_unit_died_flat_rows(
        self,
        details: &crate::details::Details,
    ) -> Vec<tracker_events::UnitDiedEventFlatRow> {
        self.into_iter()
            .filter_map(|event_item| match event_item.event_type {
                SC2EventType::Tracker {
                    tracker_loop,
                    event,
                } => {
                    if let tracker_events::ReplayTrackerEvent::UnitDied(event) = event {
                        tracker_events::UnitDiedEventFlatRow::new(
                            details,
                            event,
                            tracker_loop,
                            event_item.change_hint,
                        )
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SC2EventIteratorItem {
    /// The event type, either Tracker or Game
    pub event_type: SC2EventType,
    /// The unit change hint, if any.
    pub change_hint: UnitChangeHint,
}

impl SC2EventIteratorItem {
    /// Creates a new SC2EventIteratorItem from the event type and change hint.
    pub fn new(event_type: SC2EventType, change_hint: UnitChangeHint) -> Self {
        Self {
            event_type,
            change_hint,
        }
    }

    /// Returns true if the event should be skipped based on the filters
    fn shoud_skip_event(&self, event: &SC2EventType, filters: &SC2ReplayFilters) -> bool {
        if let Some(min_loop) = filters.min_loop {
            if let SC2EventType::Tracker { tracker_loop, .. } = event {
                if *tracker_loop < min_loop {
                    return true;
                }
            }
        }
        if let Some(max_loop) = filters.max_loop {
            if let SC2EventType::Tracker { tracker_loop, .. } = event {
                if *tracker_loop > max_loop {
                    return true;
                }
            }
        }
        event.should_skip(filters)
    }

    /// Retuns true if the variant of event type is Game
    pub fn is_game_event(&self) -> bool {
        matches!(self.event_type, SC2EventType::Game { .. })
    }

    /// Retuns true if the variant of event type is Tracker
    pub fn is_tracker_event(&self) -> bool {
        matches!(self.event_type, SC2EventType::Tracker { .. })
    }
}

impl Iterator for SC2EventIterator {
    /// The item is a tuple of the SC2EventType with the accumulated (adjusted) game loop, and a
    /// hint of what has changed. An adjusted game loop is the `event_loop` adjusted to be in the same units as the game loops.
    /// Events may be of Game or Tracker type.
    /// They are produced in absolute order between them.
    type Item = SC2EventIteratorItem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Fill the next_tracker_event if they are empty.
            if self.next_tracker_event.is_none() {
                self.next_tracker_event = self
                    .tracker_iterator_state
                    .get_next_event(self.protocol_version);
            }
            // Likewise, fill the next game event if it's empty.
            if self.next_game_event.is_none() {
                self.next_game_event = self
                    .game_iterator_state
                    .get_next_event(self.protocol_version, &self.balance_units);
            }
            // Now compare the adjusted game loops and return the event with the lowest one, be it game or tracker.
            let next_tracker_event_loop = self.get_tracker_loop();
            let next_game_event_loop = self.get_game_loop();
            let event: SC2EventType = if let Some(next_tracker_event_loop) = next_tracker_event_loop
            {
                if let Some(next_game_event_loop) = next_game_event_loop {
                    // Both events are populated, compare the loop and return the lowest one
                    if next_tracker_event_loop <= next_game_event_loop {
                        self.next_tracker_event.take().unwrap()
                    } else {
                        self.next_game_event.take().unwrap()
                    }
                } else {
                    // The game event is not populated, return the tracker event.
                    self.next_tracker_event.take().unwrap()
                }
            } else if next_game_event_loop.is_some() {
                // The tracker event is not populated, return the game event.
                self.next_game_event.take().unwrap()
            } else {
                return None;
            };
            let iterator_item = self
                .sc2_state
                .handle_transition_to_next_event(&self.balance_units, event);
            if let Some(ref mut filters) = self.filters {
                if iterator_item.shoud_skip_event(&iterator_item.event_type, filters) {
                    continue;
                }
                filters.decrease_allowed_event_counter();
                if filters.is_max_event_reached() {
                    return None;
                }
            }
        }
    }
}
