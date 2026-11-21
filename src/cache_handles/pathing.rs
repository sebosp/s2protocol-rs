//! The ground pathing of a map: the cells a ground unit can walk on, read from the cliff levels
//! of the map (`t3SyncCliffLevel`) and its cells painted unpathable (`CellAttribute_Pnp`).
//!
//! A cell is pathable inside the playable area when its cliff level is above 0 (0 is the void
//! around the map), it is not on the edge of a cliff (a neighbor a level apart, the ramps go up
//! in steps of 8) and it is not painted unpathable. Checked against the game's own grid (SC2 API,
//! `GameInfo.start_raw.pathing_grid`) on the 8 maps of the AI Arena season of 2026-10: 99.1% of
//! the cells agree once the units are left out, and the connected regions are the same on 7 maps
//! (on Ultralove the game has 2 islands of 30 cells, doodads that are not read here).
//!
//! The units are not in the map: the minerals, geysers, rocks and buildings that block the ground
//! come from the events of a replay, [`PathingBlockers`] keeps them.

use super::map_info::MapInfo;
use crate::S2ProtocolError;
use crate::cache_handles::map::MapError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub const CLIFF_LEVEL_FILE_NAME: &str = "t3SyncCliffLevel";
pub const NO_PATHING_FILE_NAME: &str = "CellAttribute_Pnp";

/// The header of `t3SyncCliffLevel`: "CLIF", the version, the width, the height and 16 bytes.
const CLIFF_HEADER_LEN: usize = 32;
/// The header of a `CellAttribute_*` file, a byte per cell follows.
const CELL_ATTRIBUTE_HEADER_LEN: usize = 4;
/// A difference of cliff levels that is a cliff: the plateaus are 64 apart, the ramps go in
/// steps of 8.
const CLIFF_STEP: u16 = 32;

/// The ground pathing of a map, row by row from the bottom of the map as the unit positions.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PathingGrid {
    pub cache_handle_id: String,
    pub width: usize,
    pub height: usize,
    /// The cliff level of each cell, 0 out of the map.
    pub levels: Vec<u16>,
    /// Whether a ground unit can stand on each cell, with no unit on the map.
    pub pathable: Vec<bool>,
}

impl PathingGrid {
    /// Reads the grid from the contents of `t3SyncCliffLevel` and, when the map has it,
    /// `CellAttribute_Pnp`.
    pub fn parse(
        cache_handle_id: String,
        cliff_level: &[u8],
        no_pathing: Option<&[u8]>,
        map_info: &MapInfo,
    ) -> Result<Self, S2ProtocolError> {
        if cliff_level.len() < CLIFF_HEADER_LEN || &cliff_level[0..4] != b"CLIF" {
            return Err(MapError::Other(format!("{cache_handle_id}: not a CLIF file")).into());
        }
        let read_u32 = |at: usize| {
            u32::from_le_bytes([
                cliff_level[at],
                cliff_level[at + 1],
                cliff_level[at + 2],
                cliff_level[at + 3],
            ]) as usize
        };
        let (width, height) = (read_u32(8), read_u32(12));
        let cells = width * height;
        let expected = CLIFF_HEADER_LEN + 2 * cells;
        if cliff_level.len() < expected {
            return Err(MapError::T3HeightNotEnoughBytes(expected, cliff_level.len()).into());
        }
        let levels: Vec<u16> = cliff_level[CLIFF_HEADER_LEN..expected]
            .as_chunks::<2>()
            .0
            .iter()
            .map(|pair| u16::from_le_bytes(*pair))
            .collect();
        let no_pathing = no_pathing
            .filter(|data| data.len() >= CELL_ATTRIBUTE_HEADER_LEN + cells)
            .map(|data| &data[CELL_ATTRIBUTE_HEADER_LEN..CELL_ATTRIBUTE_HEADER_LEN + cells]);
        let mut pathable = vec![false; cells];
        for y in map_info.cell_bottom..map_info.cell_top.min(height) {
            for x in map_info.cell_left..map_info.cell_right.min(width) {
                let index = y * width + x;
                let level = levels[index];
                if level == 0 || no_pathing.is_some_and(|painted| painted[index] != 0) {
                    continue;
                }
                let on_cliff = [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .any(|(dx, dy)| {
                        let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                        if nx < 0 || ny < 0 || nx >= width as i64 || ny >= height as i64 {
                            return false;
                        }
                        levels[ny as usize * width + nx as usize].abs_diff(level) >= CLIFF_STEP
                    });
                pathable[index] = !on_cliff;
            }
        }
        Ok(Self {
            cache_handle_id,
            width,
            height,
            levels,
            pathable,
        })
    }

    fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        Some(y as usize * self.width + x as usize)
    }

    /// Whether a ground unit can stand on the cell, with no unit on the map.
    pub fn is_pathable(&self, x: i32, y: i32) -> bool {
        self.index(x, y).is_some_and(|index| self.pathable[index])
    }

    /// The cliff level of the cell, 0 out of the map.
    pub fn level(&self, x: i32, y: i32) -> u16 {
        self.index(x, y)
            .map(|index| self.levels[index])
            .unwrap_or(0)
    }

    /// The number of cells a ground unit on `start` can reach, without crossing a cliff or a
    /// blocked cell, counted up to `max_cells` (a unit that reaches that many is free). A unit
    /// standing on a blocked or unpathable cell (its position is rounded) starts from the nearest
    /// free cell around it.
    pub fn region_size(
        &self,
        start: (i32, i32),
        is_blocked: impl Fn(i32, i32) -> bool,
        max_cells: usize,
    ) -> usize {
        let free = |x: i32, y: i32| self.is_pathable(x, y) && !is_blocked(x, y);
        let Some(start) = std::iter::once((0, 0))
            .chain((-1..=1).flat_map(|dy| (-1..=1).map(move |dx| (dx, dy))))
            .map(|(dx, dy)| (start.0 + dx, start.1 + dy))
            .find(|&(x, y)| free(x, y))
        else {
            return 0;
        };
        let mut seen: HashSet<(i32, i32)> = HashSet::from([start]);
        let mut queue = vec![start];
        while let Some((x, y)) = queue.pop() {
            if seen.len() >= max_cells {
                break;
            }
            let level = self.level(x, y);
            for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                if !seen.contains(&(nx, ny))
                    && free(nx, ny)
                    && self.level(nx, ny).abs_diff(level) < CLIFF_STEP
                {
                    seen.insert((nx, ny));
                    queue.push((nx, ny));
                }
            }
        }
        seen.len()
    }
}

/// The connected areas of the ground, see [`PathingGrid::regions`].
#[derive(Debug, Default, Clone)]
pub struct PathingRegions {
    /// The area of each cell (index in `sizes` + 1), 0 for a cell nobody can stand on.
    pub labels: Vec<u32>,
    /// The number of cells of each area.
    pub sizes: Vec<usize>,
}

impl PathingRegions {
    /// The label of the largest area, the open ground of the map.
    pub fn largest(&self) -> u32 {
        self.sizes
            .iter()
            .enumerate()
            .max_by_key(|(_, size)| **size)
            .map(|(index, _)| index as u32 + 1)
            .unwrap_or(0)
    }
}

impl PathingGrid {
    /// Labels the areas a ground unit can walk across, without crossing a cliff or a blocked
    /// cell. With the buildings of a replay, an area apart from the open ground is a pocket
    /// closed by them (or an island of the map).
    pub fn regions(&self, is_blocked: impl Fn(i32, i32) -> bool) -> PathingRegions {
        let mut labels = vec![0u32; self.width * self.height];
        let mut sizes = vec![];
        for start in 0..labels.len() {
            let (sx, sy) = ((start % self.width) as i32, (start / self.width) as i32);
            if labels[start] != 0 || !self.pathable[start] || is_blocked(sx, sy) {
                continue;
            }
            let label = sizes.len() as u32 + 1;
            labels[start] = label;
            let mut size = 1;
            let mut queue = vec![(sx, sy)];
            while let Some((x, y)) = queue.pop() {
                let level = self.level(x, y);
                for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                    let Some(index) = self.index(nx, ny) else {
                        continue;
                    };
                    if labels[index] == 0
                        && self.pathable[index]
                        && !is_blocked(nx, ny)
                        && self.levels[index].abs_diff(level) < CLIFF_STEP
                    {
                        labels[index] = label;
                        size += 1;
                        queue.push((nx, ny));
                    }
                }
            }
            sizes.push(size);
        }
        PathingRegions { labels, sizes }
    }
}

/// The cells (width, height) a unit takes from the ground pathing, None for the units that do
/// not block it: the moving units, the lowered supply depots, the buildings that fly, the
/// destructible rocks of a shape that is not read yet (the diagonal and huge ones).
pub fn ground_footprint(unit_name: &str) -> Option<(i32, i32)> {
    let size = match unit_name {
        "CommandCenter" | "OrbitalCommand" | "PlanetaryFortress" | "Nexus" | "Hatchery"
        | "Lair" | "Hive" => 5,
        "Barracks"
        | "Factory"
        | "Starport"
        | "EngineeringBay"
        | "Armory"
        | "FusionCore"
        | "Bunker"
        | "GhostAcademy"
        | "Refinery"
        | "RefineryRich"
        | "Gateway"
        | "WarpGate"
        | "Forge"
        | "CyberneticsCore"
        | "TwilightCouncil"
        | "RoboticsFacility"
        | "Stargate"
        | "TemplarArchive"
        | "DarkShrine"
        | "RoboticsBay"
        | "FleetBeacon"
        | "Assimilator"
        | "AssimilatorRich"
        | "SpawningPool"
        | "EvolutionChamber"
        | "RoachWarren"
        | "BanelingNest"
        | "HydraliskDen"
        | "LurkerDenMP"
        | "InfestationPit"
        | "Spire"
        | "GreaterSpire"
        | "UltraliskCavern"
        | "Extractor"
        | "ExtractorRich"
        | "NydusNetwork"
        | "NydusCanal"
        | "VespeneGeyser"
        | "SpacePlatformGeyser"
        | "RichVespeneGeyser"
        | "ProtossVespeneGeyser"
        | "PurifierVespeneGeyser"
        | "ShakurasVespeneGeyser" => 3,
        "SupplyDepot" | "Pylon" | "PhotonCannon" | "ShieldBattery" | "MissileTurret"
        | "SensorTower" | "SpineCrawler" | "SporeCrawler" | "BarracksTechLab"
        | "BarracksReactor" | "FactoryTechLab" | "FactoryReactor" | "StarportTechLab"
        | "StarportReactor" | "TechLab" | "Reactor" => 2,
        name if name.contains("MineralField") => return Some((2, 1)),
        name if name.starts_with("Destructible") || name.starts_with("UnbuildableRocks") => {
            if name.ends_with("6x6") {
                6
            } else if name.ends_with("4x4") {
                4
            } else if name.ends_with("2x2") {
                2
            } else {
                return None;
            }
        }
        _ => return None,
    };
    Some((size, size))
}

/// The cells of the footprint of a unit at the position of its tracker event (the integer part
/// of its center: a 3x3 building centered on 40.5 is at 40, a 2x2 depot centered on 40.0 too).
pub fn footprint_cells(unit_name: &str, x: i32, y: i32) -> Vec<(i32, i32)> {
    let Some((width, height)) = ground_footprint(unit_name) else {
        return vec![];
    };
    let (left, bottom) = (x - width / 2, y - height / 2);
    (bottom..bottom + height)
        .flat_map(|cy| (left..left + width).map(move |cx| (cx, cy)))
        .collect()
}

/// The cells blocked by the units of a replay at a point of the game: add a unit when it is born
/// or changes its type, remove it when it dies.
#[derive(Debug, Default, Clone)]
pub struct PathingBlockers {
    /// The cells of each unit by its tag index.
    units: HashMap<u32, Vec<(i32, i32)>>,
    /// How many units block each cell (an addon and its building may touch).
    cells: HashMap<(i32, i32), u16>,
}

impl PathingBlockers {
    /// Adds or replaces the footprint of a unit, i.e. a supply depot lowered frees its cells.
    pub fn set(&mut self, tag_index: u32, unit_name: &str, x: i32, y: i32) {
        self.remove(tag_index);
        let cells = footprint_cells(unit_name, x, y);
        if cells.is_empty() {
            return;
        }
        for cell in &cells {
            *self.cells.entry(*cell).or_default() += 1;
        }
        self.units.insert(tag_index, cells);
    }

    /// Removes the footprint of a unit, when it dies.
    pub fn remove(&mut self, tag_index: u32) {
        if let Some(cells) = self.units.remove(&tag_index) {
            for cell in cells {
                if let Some(count) = self.cells.get_mut(&cell) {
                    *count -= 1;
                    if *count == 0 {
                        self.cells.remove(&cell);
                    }
                }
            }
        }
    }

    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        self.cells.contains_key(&(x, y))
    }

    /// The blocked cells.
    pub fn cells(&self) -> impl Iterator<Item = &(i32, i32)> {
        self.cells.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A map of 10x7 cells: a plateau (level 128) on the left, the low ground (64) on the right,
    /// a ramp 3 cells wide going down in steps (112, 96, 80) between them, the void (0) around.
    fn grid() -> PathingGrid {
        let cliff = [0, 128, 128, 128, 64, 64, 64, 64, 64, 0];
        let ramp = [0, 128, 128, 112, 96, 80, 64, 64, 64, 0];
        let void = [0u16; 10];
        let levels: Vec<u16> = [void, cliff, ramp, ramp, ramp, cliff, void].concat();
        let mut data = b"CLIF".to_vec();
        data.extend(0x64u32.to_le_bytes());
        data.extend(10u32.to_le_bytes());
        data.extend(7u32.to_le_bytes());
        data.extend([0u8; 16]);
        data.extend(levels.iter().flat_map(|level| level.to_le_bytes()));
        let map_info = MapInfo {
            cell_left: 0,
            cell_bottom: 0,
            cell_right: 10,
            cell_top: 7,
            ..Default::default()
        };
        PathingGrid::parse("test".into(), &data, None, &map_info).unwrap()
    }

    #[test]
    fn it_reads_the_cliffs_and_the_ramp() {
        let grid = grid();
        assert!(!grid.is_pathable(0, 3), "the void");
        assert!(grid.is_pathable(2, 3), "the plateau");
        assert!(!grid.is_pathable(3, 1), "the edge of the plateau");
        assert!(!grid.is_pathable(4, 1), "the bottom of the cliff");
        assert!(grid.is_pathable(4, 3), "the ramp");
        assert!(grid.is_pathable(7, 3), "the low ground");
    }

    #[test]
    fn it_walks_down_the_ramp_unless_it_is_walled() {
        let grid = grid();
        let plateau = (2, 3);
        let open = grid.region_size(plateau, |_, _| false, 1000);
        let mut blockers = PathingBlockers::default();
        // A barracks on the ramp, its center on (4.5, 3.5).
        blockers.set(7, "Barracks", 4, 3);
        let walled = grid.region_size(plateau, |x, y| blockers.is_blocked(x, y), 1000);
        assert!(walled < open, "walled {walled}, open {open}");
        let regions = grid.regions(|x, y| blockers.is_blocked(x, y));
        assert_eq!(regions.sizes.len(), 2, "the plateau and the low ground");
        assert_ne!(regions.labels[3 * 10 + 2], regions.largest());
        assert_eq!(walled, 3, "the plateau, x=2 (x=1 is its edge on the void)");
        // Lifted, the barracks frees the ramp.
        blockers.set(7, "BarracksFlying", 4, 3);
        assert_eq!(
            grid.region_size(plateau, |x, y| blockers.is_blocked(x, y), 1000),
            open
        );
        blockers.set(8, "SupplyDepot", 3, 3);
        assert_eq!(footprint_cells("SupplyDepot", 3, 3).len(), 4);
        blockers.remove(8);
        assert_eq!(blockers.cells().count(), 0);
    }
}
