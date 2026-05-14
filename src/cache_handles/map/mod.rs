//! Map Cache Handles related functionality.

pub mod coords;
use coords::*;

#[derive(thiserror::Error, Debug)]
pub enum MapError {
    /// Map Size is bigger than max supported in game (I guess...)
    #[error("Expected max 256 for map size, got {0}")]
    InvalidMapSize(i32),
    // /The map coordinates bounds are invalid
    #[error("Expected coordinate {0} to be less than {1}")]
    InvalidCoordinateBounds(String, i32, String, i32),
    /// The MapInfo and t3HeightMay dimensions do not match
    #[error("T3 Height Map Terrain Dimensions {0:?} do not match Map Info Map Dimensions {1:?}")]
    T3HeightDimDoNotMatchMapInfoDim(MapTerrainCoord, MapTerrainCoord),
    /// Expected at least n bytes but got x bytes
    #[error("Expected at least {0} bytes, got {1} bytes")]
    T3HeightNotEnoughBytes(usize, usize),
    /// The height unit is out of bounds.
    #[error("Height unit out of bounds should be between 1 and 4, but got: {0}")]
    T3HeightUnitOutOfBounds(i32),
    /// Other error
    #[error("Other Error: {0}")]
    Other(String),
}
