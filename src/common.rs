//! A collection of transformed enum variants to u8s.
//! This is done for space efficiency in the Arrow IPC file.

use super::game_events::{GAME_EVENT_POS_RATIO, GameSMapCoord3D};
use serde::{Deserialize, Serialize};

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

/// Unit position  will be provided like this to match as much as possible the protocols
/// themselves.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Vec3D(pub [f32; 3]);
impl Vec3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    /// Returns the x coordinate from the vector.
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    /// Returns the y coordinate from the vector.
    pub fn y(&self) -> f32 {
        self.0[1]
    }

    /// Returns the z coordinate from the vector.
    pub fn z(&self) -> f32 {
        self.0[2]
    }
}

impl From<GameSMapCoord3D> for Vec3D {
    fn from(coord: GameSMapCoord3D) -> Self {
        Vec3D::new(
            coord.x as f32 / GAME_EVENT_POS_RATIO,
            -(coord.y as f32) / GAME_EVENT_POS_RATIO,
            coord.z as f32 / GAME_EVENT_POS_RATIO,
        )
    }
}
