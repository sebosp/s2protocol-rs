//! Provides common filter operations.
//! This should be use in combination with the iterators and filter()

use serde::{Deserialize, Serialize};

use crate::cli::Cli;
use crate::details::Details;

/// A set of known filters for users of the library
/// TODO: Add include_ai, player_name, etc
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SC2ReplayFilters {
    /// Filters a specific player id.
    pub player_id: Option<u8>,

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

    /// Whether or not the PlayerStats event should be shown. To be replaced by a proper filter
    pub include_stats: bool,
}

impl SC2ReplayFilters {
    /// Locates the user id inside the replays for the given username
    pub fn set_player_id_from_user_name(&mut self, username: &str, details: &Details) {
        for details_player in &details.player_list {
            if details_player.name == username {
                self.player_id = details_player.working_set_slot_id;
            }
        }
    }

    /// Decreases the amount of processed event counters.
    pub fn decrease_allowed_event_counter(&mut self) {
        if let Some(max_events) = self.max_events
            && max_events > 0
        {
            self.max_events = Some(max_events - 1);
        }
    }

    pub fn is_max_event_reached(&self) -> bool {
        if let Some(max_events) = self.max_events
            && max_events == 0
        {
            return true;
        }
        false
    }
}

impl From<Cli> for SC2ReplayFilters {
    fn from(cli: Cli) -> Self {
        SC2ReplayFilters {
            player_id: cli.player_id,
            min_loop: cli.min_loop,
            max_loop: cli.max_loop,
            event_type: cli.event_type,
            unit_name: cli.unit_name,
            max_events: cli.max_events,
            include_stats: cli.include_stats,
            ..Default::default()
        }
    }
}
