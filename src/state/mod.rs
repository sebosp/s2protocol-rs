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
use crate::common::*;
use crate::filters::SC2ReplayFilters;
use crate::game_events::GameEventIteratorState;
use crate::tracker_events::TrackertEventIteratorState;
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
    pub fn set_unit_props(&mut self) {
        let (radius, color) = get_unit_sized_color(&self.name, self.user_id.unwrap_or(0) as i64);
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
    /// An ability has been used, the unit cmd abilities should be inspected.
    Abilities(Vec<SC2Unit>, GameSCmdEvent),
    /// A set of units has been selected and are in the active control group.
    Selection(Vec<SC2Unit>),
    /// No units have changed, for example, PlayerStats are generated, so nothing to inspect
    None,
}

/// The user state as it's collected through time.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SC2UserState {
    /// An array of registered control groups per user, the control group indexed as 10th is the
    /// currently selected units.
    pub control_groups: Vec<Vec<u32>>,
}

impl SC2UserState {
    pub fn new() -> Self {
        let mut control_groups = vec![];
        // populate as empty control groups.
        for _ in 0..11 {
            control_groups.push(vec![]);
        }
        Self { control_groups }
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
    next_tracker_event: Option<(SC2EventType, UnitChangeHint)>,
    /// The next event coming from the game iterator.
    next_game_event: Option<(SC2EventType, UnitChangeHint)>,
    /// The iterator filter helpers
    filters: Option<SC2ReplayFilters>,
}

impl SC2EventIterator {
    /// Creates a new SC2EventIterator from a PathBuf
    #[tracing::instrument(level = "debug")]
    pub fn new(source: &PathBuf) -> Result<Self, S2ProtocolError> {
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
        Ok(Self {
            protocol_version: proto_header.m_version.m_base_build,
            sc2_state,
            tracker_iterator_state: tracker_events.into(),
            game_iterator_state: game_events.into(),
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
        match &self.next_tracker_event {
            Some((SC2EventType::Tracker { tracker_loop, .. }, _)) => Some(*tracker_loop),
            _ => None,
        }
    }

    /// Returns the game loop inside the next_game_event collected.
    fn get_game_loop(&self) -> Option<i64> {
        match &self.next_game_event {
            Some((SC2EventType::Game { game_loop, .. }, _)) => Some(*game_loop),
            _ => None,
        }
    }
}

impl Iterator for SC2EventIterator {
    /// The item is a tuple of the SC2EventType with the accumulated (adjusted) game loop, and a
    /// hint of what has changed. An adjusted game loop is the `event_loop` adjusted to be in the same units as the game loops.
    /// Events may be of Game or Tracker type.
    /// They are produced in absolute order between them.
    type Item = (SC2EventType, UnitChangeHint);

    fn next(&mut self) -> Option<Self::Item> {
        // Fill the next_tracker_event if they are empty.
        if self.next_tracker_event.is_none() {
            self.next_tracker_event = self
                .tracker_iterator_state
                .transist_to_next_supported_event(
                    self.protocol_version,
                    &mut self.sc2_state,
                    &mut self.filters,
                );
        }
        // Likewise, fill the next game event if it's empty.
        if self.next_game_event.is_none() {
            self.next_game_event = self.game_iterator_state.transist_to_next_supported_event(
                self.protocol_version,
                &mut self.sc2_state,
                &mut self.filters,
            );
        }
        // Now compare the adjusted game loops and return the event with the lowest one, be it game or tracker.
        let next_tracker_event_loop = self.get_tracker_loop();
        let next_game_event_loop = self.get_game_loop();
        if let Some(next_tracker_event_loop) = next_tracker_event_loop {
            if let Some(next_game_event_loop) = next_game_event_loop {
                // Both events are populated, compare the loop and return the lowest one
                if next_tracker_event_loop <= next_game_event_loop {
                    self.next_tracker_event.take()
                } else {
                    self.next_game_event.take()
                }
            } else {
                // The game event is not populated, return the tracker event.
                self.next_tracker_event.take()
            }
        } else if next_game_event_loop.is_some() {
            // The tracker event is not populated, return the game event.
            self.next_game_event.take()
        } else {
            None
        }
    }
}
