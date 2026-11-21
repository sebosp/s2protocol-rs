//! The pathing grid read from a map against the grid of the game (SC2 API), on the maps of the AI
//! Arena season of 2026-10. The grids of the game are in `tests/pathing/*.json`, the maps are not
//! in the repository:
//!
//! S2PROTOCOL_PATHING_MAPS=/data/seb/sc2-linux/aiarena-maps cargo test --test pathing -- --ignored --nocapture

use s2protocol::cache_handles::map_info::MapInfo;
use s2protocol::cache_handles::pathing::{
    CLIFF_LEVEL_FILE_NAME, NO_PATHING_FILE_NAME, PathingGrid,
};
use std::path::PathBuf;

#[derive(serde::Deserialize)]
struct GameGrid {
    map_file: String,
    width: usize,
    height: usize,
    playable: [usize; 4],
    pathing_bits_hex: String,
}

impl GameGrid {
    fn is_pathable(&self, x: usize, y: usize) -> bool {
        let index = y * self.width + x;
        let byte = u8::from_str_radix(
            &self.pathing_bits_hex[2 * (index / 8)..2 * (index / 8) + 2],
            16,
        )
        .unwrap();
        (byte >> (7 - index % 8)) & 1 == 1
    }
}

#[test]
#[ignore]
fn it_matches_the_pathing_grid_of_the_game() {
    let maps_dir = PathBuf::from(
        std::env::var("S2PROTOCOL_PATHING_MAPS").expect("S2PROTOCOL_PATHING_MAPS, the maps dir"),
    );
    let mut fixtures: Vec<PathBuf> = std::fs::read_dir("tests/pathing")
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    fixtures.sort();
    assert_eq!(fixtures.len(), 8);
    for fixture in fixtures {
        let game: GameGrid =
            serde_json::from_str(&std::fs::read_to_string(&fixture).unwrap()).unwrap();
        let contents = std::fs::read(maps_dir.join(&game.map_file)).unwrap();
        let (_, mpq) = s2protocol::parser::parse(&contents).unwrap();
        let map_info = MapInfo::from_mpq(game.map_file.clone(), &mpq, &contents).unwrap();
        // The game's playable area is at times a cell inside the bounds of MapInfo.
        let bounds = [
            map_info.cell_left,
            map_info.cell_bottom,
            map_info.cell_right,
            map_info.cell_top,
        ];
        assert!(
            bounds
                .iter()
                .zip(game.playable)
                .all(|(bound, playable)| bound.abs_diff(playable) <= 1),
            "{}: the playable area {bounds:?} {:?}",
            game.map_file,
            game.playable
        );
        let (_, cliff) = mpq
            .read_mpq_file_sector(CLIFF_LEVEL_FILE_NAME, false, &contents)
            .unwrap();
        let no_pathing = mpq
            .read_mpq_file_sector(NO_PATHING_FILE_NAME, false, &contents)
            .ok()
            .map(|(_, data)| data);
        let grid = PathingGrid::parse(
            game.map_file.clone(),
            &cliff,
            no_pathing.as_deref(),
            &map_info,
        )
        .unwrap();
        assert_eq!((grid.width, grid.height), (game.width, game.height));
        let [left, bottom, right, top] = game.playable;
        let (mut agree, mut cells) = (0, 0);
        for y in bottom..top {
            for x in left..right {
                cells += 1;
                if grid.is_pathable(x as i32, y as i32) == game.is_pathable(x, y) {
                    agree += 1;
                }
            }
        }
        // The game's grid also blocks the units seen at the start (the minerals and geysers of
        // a main, its town hall, the rocks), not in the map.
        let ratio = agree as f64 / cells as f64;
        println!(
            "{}: {:.2}% of the playable cells agree",
            game.map_file,
            100. * ratio
        );
        assert!(ratio > 0.95, "{}: {ratio}", game.map_file);
    }
}
