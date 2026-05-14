//! Handling of cache_handles on replay files.
//! These are <some_id>.s2ma files that can be downloaded from blizzar,
//! containing map resources, mod information, visual resources such
//! as images used in overlays for tournament, organizers, etc.

pub mod document_header;
pub mod map;
pub mod map_info;
pub mod t3_height_map;
pub use map::*;
