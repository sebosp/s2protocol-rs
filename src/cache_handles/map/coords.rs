//! The different map coordinates systems
//!
//!

// Reference doocs from https://github.com/RFEphemeration/sc2-map-analyzer/blob/master/coordinates.hpp
//           Some object (see OBJ in playable area),
//           is aligned to cells, and its map coordinates
//           always end in .5
//
//                           |                       |
//                  #     #  |  #     #     #     #  |  #     #     # <--- mt(width+1, height+1)
//                           |                       |
//                     @     @     @     @     @     @     @     @ <--- mc(width, height)
//                           |                       |
//                  #     #  |  #     #     #     #  |  #     #     #
//                           |                       |
//    boundaries ------@-----@-----@-----@-----@-----@-----@-----@-----
//    top and                |                       |                      "%" marks a cell in the
//    bottom are    #     #  |  #     #     #     #  |  #     #     #       playable area and
//    integers               |      playable area    |                      "@" marks a cell in the
//    in the           @     %    OBJ    %     %     @     @     @          unplayable area
//    cell frame             |                       |
//                  #     #  |  #     #     #     #  |  #     #     #       Note that cells along the
//                           |                       |                      bottom and left boundaries
//               ------@-----%-----%-----%-----%-----@-----@-----@-----     are playable, but cells
//                           |                       |                      along top and right are not
//                  #     #  |  #     #     #     #  |  #     #     #
//                           |                       |
// mc(0, 0) is at ---> @     @     @     @     @     @     @     @ <--.
// m(0.5, 0.5)               |                       |                 \
//             .--> #     #  |  #     #     #     #  |  #     #     #    Every cell is bordered by
//            /              |                       |                   terrain points, so the MapInfo
// This is both              |                       |                   width x height of the cell area
// the origin               boundaries left and right are                is "surrounded" by a terrain
// m(0.0, 0.0) and          integers in the cell frame                   area (width+1) x (height+1)
// mt(0, 0)
//

use std::fmt::{Display, Formatter};

use super::super::map_info::MapInfo;

/// MapCoordinates
/// Origin is bottom left.
/// Unit short (m)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapCoord {
    pub x: f32,
    pub y: f32,
}

impl MapCoord {
    /// Create a new MapCoord
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Display for MapCoord {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MapCoord{{x: {:.2}m, y: {:.2}m}}", self.x, self.y)
    }
}

/// MapTerrainCoordinates
/// Expresses the location of cliffs.
/// These coordinates should be located inside the `MapCoord` boundaries.
/// Unit short (mt)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapTerrainCoord {
    pub x: i32,
    pub y: i32,
}

impl MapTerrainCoord {
    /// Create a new MapTerrainCoord
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// Aka m2mt
impl From<MapCoord> for MapTerrainCoord {
    fn from(map_coord: MapCoord) -> Self {
        Self {
            x: (map_coord.x + 0.5) as i32,
            y: (map_coord.y + 0.5) as i32,
        }
    }
}

// Aka mt2t
impl From<MapTerrainCoord> for MapCoord {
    fn from(terrain_coord: MapTerrainCoord) -> Self {
        Self {
            x: terrain_coord.x as f32,
            y: terrain_coord.y as f32,
        }
    }
}

impl Display for MapTerrainCoord {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MapTerrainCoord{{x: {}mt, y: {}mt}}", self.x, self.y)
    }
}

/// Map Cell Coordinates
/// These are the location of buildable cells.
/// When placing a building the overlay shows the cells.
/// The origin is `MapCoord{x: 0.5, y: 0.5}`
/// Unit short (mc)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapCellCoord {
    pub x: i32,
    pub y: i32,
}

impl MapCellCoord {
    /// Create a new MapCellCoord
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl Display for MapCellCoord {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MapCellCoord{{x: {}mc, y: {}mc}}", self.x, self.y)
    }
}

// Aka m2mc
impl From<MapCoord> for MapCellCoord {
    fn from(map_coord: MapCoord) -> Self {
        Self {
            x: map_coord.x as i32,
            y: map_coord.y as i32,
        }
    }
}

// Aka mc2m
impl From<MapCellCoord> for MapCoord {
    fn from(cell_coord: MapCellCoord) -> Self {
        Self {
            x: cell_coord.x as f32 + 0.5,
            y: cell_coord.y as f32 + 0.5,
        }
    }
}

// TODOO: Unsure this makes sense, maybe we can delete these two structs

/// Playable Terrain Coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayableTerrainCoord {
    pub x: i32,
    pub y: i32,
}

impl PlayableTerrainCoord {
    /// Create a new PlayableTerrainCoord
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_map_terrain(map_info: &MapInfo, map_terrain_coord: MapTerrainCoord) -> Self {
        Self {
            x: map_terrain_coord.x - map_info.cell_left,
            y: map_terrain_coord.y - map_info.cell_bottom,
        }
    }
}

impl Display for PlayableTerrainCoord {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "PlayableTerrainCoord{{x: {}pt, y: {}pt}}",
            self.x, self.y
        )
    }
}

/// Playable Cell Coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayableCellCoord {
    pub x: i32,
    pub y: i32,
}

impl PlayableCellCoord {
    /// Create a new PlayableCellCoord
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Display for PlayableCellCoord {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "PlayableCellCoord{{x: {}pc, y: {}pc}}", self.x, self.y)
    }
}
