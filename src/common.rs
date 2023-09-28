//! A collection of transformed enum variants to u8s.
//! This is done for space efficiency in the Arrow IPC file.

/// The user role as they observe
pub const OBSERVE_NONE: u8 = 0;
pub const OBSERVE_SPECTATOR: u8 = 1;
pub const OBSERVE_REFEREE: u8 = 2;

/// The speed of the game
pub const GAME_SPEED_SLOWER: u8 = 0;
pub const GAME_SPEED_SLOW: u8 = 1;
pub const GAME_SPEED_NORMAL: u8 = 2;
pub const GAME_SPEED_FAST: u8 = 3;
pub const GAME_SPEED_FASTER: u8 = 4;

/// The result of the game
pub const GAME_RESULT_UNDECIDED: u8 = 0;
pub const GAME_RESULT_WIN: u8 = 1;
pub const GAME_RESULT_DEFEAT: u8 = 2;
pub const GAME_RESULT_TIE: u8 = 3;
