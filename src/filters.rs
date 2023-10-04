//! Provides way to provide common filter operations.
//! This should be use in combination with the iterators and filter(), but helps a lot of the code
//! required to match and dive into types.

use serde::{Deserialize, Serialize};

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
}
