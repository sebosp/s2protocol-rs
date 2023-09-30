//! Handling of state of SC2 Replay as it steps trhough time.
//!
//!
//! Events are ordered by their "priority", this is a guessed priority for now.
//! For example, if a TrackerEvent and a GameEvent, happen at the same game loop,
//! the game events take priority (See the const below). This may not be true but
//! seems to work so far.
//! In this version, the game_loop will be multiplied by 10 and added the priority.
//! This means 10 max events types are supported.

pub const TRACKER_PRIORITY: i64 = 1;
pub const GAME_PRIORITY: i64 = 2;

// The game event loops and tracker event loops differ in their units.
// The true ratio should be identified somehow.
// There seems to be a ratio and the ratio based on initial calculations seems to be:
pub const TRACKER_SPEED_RATIO: f32 = 0.70996;

// These many event types (replays, game, attributes, etc) are supported.
// This should be the real number, but for it's just 10 to help debugging.
pub const MAX_EVENT_TYPES: i64 = 10;

/// The currently selected units is stored as a group outside of the boundaries of the usable
/// groups.
pub const ACTIVE_UNITS_GROUP_IDX: usize = 10usize;

use super::*;

/// Unit position  will be provided like this to match as much as possible the protocols
/// themselves.
#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3D(pub [f32; 3]);
impl Vec3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }
}

/// Unit Attributes.
#[derive(Debug, Default, Clone)]
pub struct SC2Unit {
    /// The last time the unit was updated
    pub last_game_loop: i64,
    /// The owner user_id
    pub user_id: Option<u8>,
    /// The name of the unit.
    pub name: String,
    /// The XYZ position.
    pub pos: Vec3D,
    /// The target of this unit.
    pub target: Option<Vec3D>,
    /// The game loop in which the unit was created.
    pub init_game_loop: i64,
    /// The creator ability name.
    pub creator_ability_name: Option<String>,
    /// The radius of the unit, this is a parameter that may be stored
    /// by the client side better, since it's very specific to Swarmy.
    /// Maybe next version we can move it there.
    pub radius: f32,
    /// Whether the unit is selected
    pub is_selected: bool,
}

/// Supported event types.
#[derive(Debug, Clone)]
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

/// When a unit changes in the state, certain information is provided back.
/// For example, if the unit dies, it is deleted from the state, but all its information is
/// returned back for reporting purposes.
#[derive(Debug, Clone)]
pub enum UnitChangeHint {
    /// A unit has been added, the index in the state registry is returned.
    /// This would return both for UnitBorn and InitInit
    Registered(u32),
    /// Unit positions are being reported, the indexes in the unit registry are returned.
    Batch(Vec<u32>),
    /// Selected units in the first item of the tuple (.0) are targetting the unit on the second item of the tuple (.1)
    BatchWithTarget(Vec<u32>, u32),
    /// A unit has been deleted from the state registry, the full killer unit information and the
    /// killed unit is returned. Killer is cloned and may be expensive.
    Unregistered {
        killer: Option<SC2Unit>,
        killed: SC2Unit,
    },
    /// No units have changed, for example, PlayerStats are generated, so nothing to inspect
    None,
}

/// The user state as it's collected through time.
#[derive(Debug, Default, Clone)]
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

/// A set of filters to apply to the rerun session.
#[derive(Debug, Default, Clone)]
pub struct SC2ReplayFilters {
    /// Filters a specific user id.
    pub user_id: Option<i64>,

    /// Filters a specific unit tag.
    pub unit_tag: Option<i64>,

    /// Allows setting up a min event loop, in game_event units
    pub min_loop: Option<i64>,

    /// Allows setting up a max event loop
    pub max_loop: Option<i64>,

    /// Only show game of specific types
    pub event_type: Option<String>,

    /// Only show game of specific types
    pub unit_name: Option<String>,

    /// Allows setting up a max number of events of each type
    pub max_events: Option<usize>,
}

#[derive(Debug, Default, Clone)]
pub struct SC2ReplayState {
    /// The registered units state as they change through time.
    /// These are with unit index as reference
    pub units: HashMap<u32, SC2Unit>,

    /// The filters to be applied to the collection.
    pub filters: SC2ReplayFilters,

    /// Whether or not the PlayerStats event should be shown. To be replaced by a proper filter
    pub include_stats: bool,

    /// The per-user state, the control groups, the supply, units, upgrades, as it progresses
    /// through time.
    pub user_state: HashMap<i64, SC2UserState>,

    /// The events loaded from the MPQ file.
    pub events: HashMap<i64, Vec<SC2EventType>>,

    /// The current index in the events being iterated over
    pub current_loop_idx: usize,

    /// This should be a reference to the `events`, for now it's a usize pointing the events being
    /// iterated over
    pub loop_items: Vec<(i64, usize)>,
}

impl SC2ReplayState {
    /// Reads the MPQ at `file_path` and returns the state handler.
    /// The state handler can be used to construct a SC2EventIterator
    pub fn new(
        file_path: &str,
        filters: SC2ReplayFilters,
        include_stats: bool,
    ) -> Result<Self, S2ProtocolError> {
        let (mpq, file_contents) = read_mpq(file_path)?;
        Self::from_mpq(mpq, file_contents, filters, include_stats)
    }

    /// Constructs an SC2ReplayState from an MPQ file and its contents.
    pub fn from_mpq(
        mpq: MPQ,
        file_contents: Vec<u8>,
        filters: SC2ReplayFilters,
        include_stats: bool,
    ) -> Result<Self, S2ProtocolError> {
        let mut res = Self {
            units: HashMap::new(),
            filters,
            include_stats,
            user_state: HashMap::new(),
            events: HashMap::new(),
            current_loop_idx: 0usize,
            loop_items: vec![],
        };
        let filter_event_type = &res.filters.event_type.clone();
        let tracker_events = if let Some(event_type) = filter_event_type {
            if event_type.clone().to_lowercase().contains("tracker") {
                read_tracker_events(&mpq, &file_contents)?
            } else {
                vec![]
            }
        } else {
            read_tracker_events(&mpq, &file_contents)?
        };
        let mut sc2_events: HashMap<i64, Vec<SC2EventType>> = HashMap::new();
        let mut tracker_loop = 0i64;
        for game_step in tracker_events {
            tracker_loop += game_step.delta as i64;
            let adjusted_loop = (tracker_loop as f32 / TRACKER_SPEED_RATIO) as i64
                * MAX_EVENT_TYPES
                + TRACKER_PRIORITY;
            if let Some(step_evt) = sc2_events.get_mut(&adjusted_loop) {
                step_evt.push(SC2EventType::Tracker {
                    tracker_loop: (tracker_loop as f32 / TRACKER_SPEED_RATIO) as i64,
                    event: game_step.event,
                });
            } else {
                sc2_events.insert(
                    adjusted_loop,
                    vec![SC2EventType::Tracker {
                        tracker_loop: (tracker_loop as f32 / TRACKER_SPEED_RATIO) as i64,
                        event: game_step.event,
                    }],
                );
            }
        }
        let game_events = if let Some(event_type) = filter_event_type {
            if event_type.clone().to_lowercase().contains("game") {
                read_game_events(&mpq, &file_contents)?
            } else {
                vec![]
            }
        } else {
            read_game_events(&mpq, &file_contents)?
        };
        let mut game_loop = 0i64;
        for game_step in game_events {
            game_loop += game_step.delta;
            let adjusted_loop = game_loop * MAX_EVENT_TYPES + GAME_PRIORITY;
            if let Some(step_evt) = sc2_events.get_mut(&adjusted_loop) {
                step_evt.push(SC2EventType::Game {
                    game_loop,
                    user_id: game_step.user_id,
                    event: game_step.event,
                });
            } else {
                sc2_events.insert(
                    adjusted_loop,
                    vec![SC2EventType::Game {
                        game_loop,
                        user_id: game_step.user_id,
                        event: game_step.event,
                    }],
                );
            }
        }
        let mut ordered_event_loops: Vec<i64> = sc2_events.keys().cloned().collect();
        ordered_event_loops.sort_unstable();
        let mut items = vec![];
        for step in ordered_event_loops {
            for looped_event_idx in 0..sc2_events.get(&step).unwrap().len() {
                items.push((step, looped_event_idx));
            }
        }
        res.events = sc2_events;
        res.loop_items = items;
        Ok(res)
    }

    /// Transduces the state machine moving through the `loop_items`
    pub fn transduce(&mut self) -> Option<(SC2EventType, UnitChangeHint)> {
        // TODO: These could become .filter, .take, etc.
        // But still, some of these refer to the internal loop, and since we have a generated
        // game_loop based on event priorities, maybe it's not that easy.
        let min_filter = self.filters.min_loop;
        let max_filter = self.filters.max_loop;
        let user_id_filter = self.filters.user_id;
        let max_events = self.filters.max_events;
        loop {
            if self.current_loop_idx >= self.loop_items.len() {
                return None;
            };
            let (evt_loop, evt_idx) = self.loop_items[self.current_loop_idx];
            if let Some(max) = max_filter {
                // Skip the events greater than the requested filter.
                if evt_loop > max {
                    return None;
                }
            }
            if let Some(max) = max_events {
                // Cosue these max total events of any type.
                if self.current_loop_idx > max {
                    return None;
                }
            }
            let evt_type = self.events.get(&evt_loop).unwrap()[evt_idx].clone();
            let updated_hint = match &evt_type {
                SC2EventType::Tracker {
                    tracker_loop,
                    event,
                } => {
                    tracing::info!("Trac [{:>08}]: {:?}", tracker_loop, event);
                    crate::tracker_events::handle_tracker_event(self, *tracker_loop, event)
                }
                SC2EventType::Game {
                    user_id,
                    game_loop,
                    event,
                } => {
                    let updated_hint =
                        crate::game_events::handle_game_event(self, *game_loop, *user_id, event);
                    if let Some(target_user_id) = user_id_filter {
                        // Skip the events that are not for the requested user.
                        if target_user_id != *user_id {
                            continue;
                        }
                    }
                    tracing::info!("Game [{:>08}]: uid: {} {:?}", game_loop, *user_id, event);
                    updated_hint
                }
            };
            // Skip events only after stepping through them through the state. Otherwise the state
            // would be corrupted.
            if let Some(min) = min_filter {
                // Skip the events less than the requested filter.
                if evt_loop / MAX_EVENT_TYPES < min {
                    continue;
                }
            }
            self.current_loop_idx += 1;
            return Some((evt_type, updated_hint));
        }
    }
}
