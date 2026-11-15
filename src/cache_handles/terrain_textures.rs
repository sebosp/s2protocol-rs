//! The textures of the terrain: which texture covers each point of the map.
//!
//! - `t3Terrain.xml` names the 8 textures the terrain blends, see
//!   [`super::t3_terrain::T3Terrain::blend_texture_names`].
//! - `t3TextureMasks` has the weight of each of the 8 textures at each point, see
//!   [`T3TextureMasks`].
//! - `Base.SC2Data\GameData\TerrainData.xml`, when the map changes its terrain type, has the fog,
//!   the lighting and the cliffs of the terrain, see [`TerrainData`].
//! - The colors of the textures are not in the map (they are in the game's files): they are
//!   estimated from the minimap, see [`LayerColors::fit`].

use super::map_info::MapInfo;
use super::minimap::Minimap;
use super::t3_terrain::TERRAIN_BLEND_LAYERS;
use crate::S2ProtocolError;
use serde::{Deserialize, Serialize};

pub const T3_TEXTURE_MASKS_FILE_NAME: &str = "t3TextureMasks";
pub const TERRAIN_DATA_FILE_NAME: &str = "Base.SC2Data\\GameData\\TerrainData.xml";

/// The size of the header of `t3TextureMasks`.
const MASKS_HEADER_SIZE: usize = 64;

/// The weights are stored in blocks of 64x64 points.
const MASKS_BLOCK: usize = 64;

/// The largest weight, the weights of a point add up to it.
pub const MAX_LAYER_WEIGHT: u8 = 15;

fn error(message: impl Into<String>) -> S2ProtocolError {
    S2ProtocolError::TerrainTextures(message.into())
}

/// The weights of the 8 terrain textures at each point of the map, `t3TextureMasks`.
///
/// The file is the magic `MASK`, the version (102), 4 unknown bytes, the width and the height in
/// points (8 per map cell), 44 bytes of padding, then the 8 layers one after the other. A layer
/// has 4 bits per point, the high nibble first, in blocks of 64x64 points, the blocks row by row,
/// the rows of a block from the bottom of the map. Format from Talv's sc2-texture-scanner.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct T3TextureMasks {
    pub cache_handle_id: String,
    pub version: u32,
    /// The points across the map, 8 per cell.
    pub width: usize,
    /// The points along the map, 8 per cell.
    pub height: usize,
    /// The 8 weights of each point, row by row from the bottom left corner, 0 to
    /// [`MAX_LAYER_WEIGHT`]; the weights of a point add up to [`MAX_LAYER_WEIGHT`].
    pub weights: Vec<u8>,
}

impl T3TextureMasks {
    pub fn parse(cache_handle_id: String, data: &[u8]) -> Result<Self, S2ProtocolError> {
        if data.len() < MASKS_HEADER_SIZE || &data[0..4] != b"MASK" {
            return Err(error("t3TextureMasks does not start with MASK"));
        }
        let u32_at = |pos: usize| {
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]])
        };
        let version = u32_at(4);
        let width = u32_at(12) as usize;
        let height = u32_at(16) as usize;
        if width == 0
            || height == 0
            || !width.is_multiple_of(MASKS_BLOCK)
            || !height.is_multiple_of(MASKS_BLOCK)
        {
            return Err(error(format!("t3TextureMasks of {width}x{height} points")));
        }
        let layer_bytes = width * height / 2;
        let body = &data[MASKS_HEADER_SIZE..];
        if body.len() < layer_bytes * TERRAIN_BLEND_LAYERS {
            return Err(error(format!(
                "t3TextureMasks of {width}x{height} points has {} bytes, not {}",
                body.len(),
                layer_bytes * TERRAIN_BLEND_LAYERS
            )));
        }
        let blocks_x = width / MASKS_BLOCK;
        let mut weights = vec![0u8; width * height * TERRAIN_BLEND_LAYERS];
        for layer in 0..TERRAIN_BLEND_LAYERS {
            let raw = &body[layer * layer_bytes..(layer + 1) * layer_bytes];
            for (byte_idx, byte) in raw.iter().enumerate() {
                for (nibble, value) in [(0, byte >> 4), (1, byte & 0x0F)] {
                    let point = byte_idx * 2 + nibble;
                    let block = point / (MASKS_BLOCK * MASKS_BLOCK);
                    let in_block = point % (MASKS_BLOCK * MASKS_BLOCK);
                    let x = (block % blocks_x) * MASKS_BLOCK + in_block % MASKS_BLOCK;
                    let y = (block / blocks_x) * MASKS_BLOCK + in_block / MASKS_BLOCK;
                    weights[(y * width + x) * TERRAIN_BLEND_LAYERS + layer] = value;
                }
            }
        }
        Ok(Self {
            cache_handle_id,
            version,
            width,
            height,
            weights,
        })
    }

    /// The 8 weights of the point, None outside the map.
    pub fn weights_at(&self, x: usize, y: usize) -> Option<&[u8]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let start = (y * self.width + x) * TERRAIN_BLEND_LAYERS;
        self.weights.get(start..start + TERRAIN_BLEND_LAYERS)
    }

    /// The points of each map cell.
    pub fn points_per_cell(&self, map_info: &MapInfo) -> usize {
        (self.width / map_info.cell_width.max(1)).max(1)
    }

    /// The weights as two RGBA8 images for a GPU, the layers 0 to 3 and 4 to 7, each weight from
    /// 0 to 255, averaged over squares of `step` points: `(width, height, [first, second])`, row
    /// by row from the bottom of the map.
    pub fn to_rgba_images(&self, step: usize) -> (usize, usize, [Vec<u8>; 2]) {
        let step = step.max(1);
        let (width, height) = (self.width / step, self.height / step);
        let mut images = [vec![0u8; width * height * 4], vec![0u8; width * height * 4]];
        let samples = (step * step) as u32;
        for y in 0..height {
            for x in 0..width {
                let mut sums = [0u32; TERRAIN_BLEND_LAYERS];
                for dy in 0..step {
                    let row = (y * step + dy) * self.width;
                    for dx in 0..step {
                        let start = (row + x * step + dx) * TERRAIN_BLEND_LAYERS;
                        for (layer, sum) in sums.iter_mut().enumerate() {
                            *sum += self.weights[start + layer] as u32;
                        }
                    }
                }
                for (layer, sum) in sums.iter().enumerate() {
                    let value = (sum * 255 / (samples * MAX_LAYER_WEIGHT as u32)).min(255) as u8;
                    images[layer / 4][(y * width + x) * 4 + layer % 4] = value;
                }
            }
        }
        (width, height, images)
    }
}

/// The terrain type of the map when the map changes it, `TerrainData.xml`: the textures, the
/// cliffs, the fog and the lighting. The maps that use a terrain type of the game as it is do not
/// have the file.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TerrainData {
    /// The `CTerrain` id, the `tileSet` of `t3Terrain.xml`.
    pub tile_set: String,
    /// The textures the terrain type offers, by index.
    pub blend_textures: Vec<String>,
    /// The cliff sets, by index.
    pub cliff_sets: Vec<String>,
    /// The fog color, RGBA.
    pub fog_color: Option<[u8; 4]>,
    pub lighting: Option<String>,
    pub skybox: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct CatalogTag {
    #[serde(rename = "CTerrain", default)]
    terrains: Vec<CTerrainTag>,
}

#[derive(Debug, Deserialize, Default)]
struct CTerrainTag {
    #[serde(rename = "@id", default)]
    id: String,
    #[serde(rename = "BlendTextures", default)]
    blend_textures: Vec<IndexValueTag>,
    #[serde(rename = "CliffSets", default)]
    cliff_sets: Vec<IndexValueTag>,
    #[serde(rename = "FogColor")]
    fog_color: Option<IndexValueTag>,
    #[serde(rename = "Lighting")]
    lighting: Option<IndexValueTag>,
    #[serde(rename = "FixedSkyboxModel")]
    skybox: Option<IndexValueTag>,
}

#[derive(Debug, Deserialize, Default)]
struct IndexValueTag {
    #[serde(rename = "@index", default)]
    index: String,
    #[serde(rename = "@value", default)]
    value: String,
    #[serde(rename = "@removed", default)]
    removed: String,
}

/// The values by index, the removed ones empty.
fn by_index(tags: &[IndexValueTag]) -> Vec<String> {
    let mut values: Vec<String> = vec![];
    for (position, tag) in tags.iter().enumerate() {
        let index = tag.index.parse::<usize>().unwrap_or(position);
        if values.len() <= index {
            values.resize(index + 1, String::new());
        }
        if tag.removed != "1" {
            values[index] = tag.value.clone();
        }
    }
    values
}

impl TerrainData {
    /// The terrain type `tile_set` of the file, or its first one.
    pub fn parse(file_contents: &[u8], tile_set: &str) -> Result<Self, S2ProtocolError> {
        let catalog: CatalogTag = serde_xml_rs::from_str(str::from_utf8(file_contents)?)?;
        let terrain = catalog
            .terrains
            .iter()
            .find(|terrain| terrain.id == tile_set)
            .or(catalog.terrains.first())
            .ok_or_else(|| error("TerrainData.xml has no CTerrain"))?;
        // "255,124,177,149": alpha, red, green, blue.
        let fog_color = terrain.fog_color.as_ref().and_then(|fog| {
            let parts: Vec<u8> = fog
                .value
                .split(',')
                .filter_map(|part| part.trim().parse().ok())
                .collect();
            (parts.len() == 4).then(|| [parts[1], parts[2], parts[3], parts[0]])
        });
        Ok(Self {
            tile_set: terrain.id.clone(),
            blend_textures: by_index(&terrain.blend_textures),
            cliff_sets: by_index(&terrain.cliff_sets),
            fog_color,
            lighting: terrain.lighting.as_ref().map(|tag| tag.value.clone()),
            skybox: terrain.skybox.as_ref().map(|tag| tag.value.clone()),
        })
    }
}

/// The weight from which a layer covers a minimap pixel alone, for its color.
const DOMINANT_WEIGHT: u8 = 12;

/// The pixels a layer must cover alone for its color to be their median.
const MIN_DOMINANT_PIXELS: usize = 20;

/// The share of the map a layer must cover to have a color at all.
const MIN_LAYER_SHARE: f32 = 0.005;

/// The colors of the 8 terrain layers, estimated from the minimap: the median color of the pixels
/// a layer covers alone, else the least squares fit of the minimap by the weights.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LayerColors {
    /// The sRGB color of each layer, None for the layers that cover too little of the map.
    pub colors: [Option<[u8; 3]>; TERRAIN_BLEND_LAYERS],
    /// The share of the playable area of each layer, 0 to 1.
    pub shares: [f32; TERRAIN_BLEND_LAYERS],
    /// The mean difference, 0 to 255 per channel, between the minimap and the weights painted
    /// with the colors. Above 20 the minimap is likely painted by hand, not rendered from the
    /// terrain, and the colors are not those of the textures.
    pub mean_abs_error: f32,
}

impl LayerColors {
    /// Fits the colors of the layers to the minimap: each pixel of the playable area is matched
    /// with the point of the masks at its center.
    pub fn fit(masks: &T3TextureMasks, minimap: &Minimap, map_info: &MapInfo) -> Self {
        let minimap = minimap.crop_to_playable(map_info);
        let playable_width = map_info
            .cell_right
            .saturating_sub(map_info.cell_left)
            .max(1);
        let pixels_per_cell = minimap.width as f32 / playable_width as f32;
        let points_per_cell = masks.points_per_cell(map_info) as f32;
        // The pixels with their weights, the black ones (outside the map) left out.
        let mut samples: Vec<([u8; TERRAIN_BLEND_LAYERS], [u8; 3])> = vec![];
        for py in 0..minimap.height as usize {
            for px in 0..minimap.width as usize {
                let start = (py * minimap.width as usize + px) * 4;
                let rgb = [
                    minimap.pixels[start],
                    minimap.pixels[start + 1],
                    minimap.pixels[start + 2],
                ];
                if rgb.iter().all(|channel| *channel < 4) {
                    continue;
                }
                let cell_x = map_info.cell_left as f32 + (px as f32 + 0.5) / pixels_per_cell;
                let cell_y = map_info.cell_top as f32 - (py as f32 + 0.5) / pixels_per_cell;
                let (x, y) = (
                    (cell_x * points_per_cell) as usize,
                    (cell_y * points_per_cell) as usize,
                );
                if let Some(weights) = masks.weights_at(x, y) {
                    let mut layer_weights = [0u8; TERRAIN_BLEND_LAYERS];
                    layer_weights.copy_from_slice(weights);
                    samples.push((layer_weights, rgb));
                }
            }
        }
        let mut result = Self::default();
        if samples.is_empty() {
            return result;
        }
        // The median of the pixels each layer covers alone.
        let mut histograms = vec![[[0u32; 256]; 3]; TERRAIN_BLEND_LAYERS];
        let mut dominant = [0usize; TERRAIN_BLEND_LAYERS];
        let mut sums = [0f32; TERRAIN_BLEND_LAYERS];
        for (weights, rgb) in &samples {
            for layer in 0..TERRAIN_BLEND_LAYERS {
                sums[layer] += weights[layer] as f32;
                if weights[layer] >= DOMINANT_WEIGHT {
                    dominant[layer] += 1;
                    for channel in 0..3 {
                        histograms[layer][channel][rgb[channel] as usize] += 1;
                    }
                }
            }
        }
        let least_squares = fit_least_squares(&samples);
        for layer in 0..TERRAIN_BLEND_LAYERS {
            let share = sums[layer] / (samples.len() as f32 * MAX_LAYER_WEIGHT as f32);
            result.shares[layer] = share;
            if share < MIN_LAYER_SHARE {
                continue;
            }
            result.colors[layer] = Some(if dominant[layer] >= MIN_DOMINANT_PIXELS {
                let median = |histogram: &[u32; 256]| {
                    let half = (dominant[layer] as u32).div_ceil(2);
                    let mut seen = 0;
                    histogram
                        .iter()
                        .position(|count| {
                            seen += count;
                            seen >= half
                        })
                        .unwrap_or(0) as u8
                };
                [
                    median(&histograms[layer][0]),
                    median(&histograms[layer][1]),
                    median(&histograms[layer][2]),
                ]
            } else {
                least_squares[layer].map(|channel| channel.clamp(0., 255.).round() as u8)
            });
        }
        // How well the colors paint the minimap.
        let mut error = 0f64;
        for (weights, rgb) in &samples {
            for channel in 0..3 {
                let painted: f32 = (0..TERRAIN_BLEND_LAYERS)
                    .map(|layer| {
                        weights[layer] as f32 / MAX_LAYER_WEIGHT as f32
                            * result.colors[layer].map_or(0., |color| color[channel] as f32)
                    })
                    .sum();
                error += (painted - rgb[channel] as f32).abs() as f64;
            }
        }
        result.mean_abs_error = (error / (samples.len() * 3) as f64) as f32;
        result
    }
}

/// The colors of the layers that paint the samples best, least squares with a small ridge.
fn fit_least_squares(
    samples: &[([u8; TERRAIN_BLEND_LAYERS], [u8; 3])],
) -> [[f32; 3]; TERRAIN_BLEND_LAYERS] {
    const N: usize = TERRAIN_BLEND_LAYERS;
    let mut ata = [[0f64; N]; N];
    let mut aty = [[0f64; 3]; N];
    for (weights, rgb) in samples {
        let w: Vec<f64> = weights
            .iter()
            .map(|w| *w as f64 / MAX_LAYER_WEIGHT as f64)
            .collect();
        for i in 0..N {
            for j in 0..N {
                ata[i][j] += w[i] * w[j];
            }
            for c in 0..3 {
                aty[i][c] += w[i] * rgb[c] as f64;
            }
        }
    }
    let ridge = 1e-3 * (0..N).map(|i| ata[i][i]).sum::<f64>().max(1e-9) / N as f64;
    for (i, row) in ata.iter_mut().enumerate() {
        row[i] += ridge;
    }
    // Gaussian elimination with partial pivoting, the 3 channels at once.
    let mut m = ata;
    let mut b = aty;
    for col in 0..N {
        let pivot = (col..N)
            .max_by(|a, b| m[*a][col].abs().total_cmp(&m[*b][col].abs()))
            .unwrap_or(col);
        m.swap(col, pivot);
        b.swap(col, pivot);
        let diag = m[col][col];
        if diag.abs() < 1e-12 {
            continue;
        }
        for row in 0..N {
            if row == col {
                continue;
            }
            let factor = m[row][col] / diag;
            if factor == 0. {
                continue;
            }
            let (pivot_row, pivot_b) = (m[col], b[col]);
            for (value, pivot) in m[row][col..].iter_mut().zip(&pivot_row[col..]) {
                *value -= factor * pivot;
            }
            for (value, pivot) in b[row].iter_mut().zip(&pivot_b) {
                *value -= factor * pivot;
            }
        }
    }
    let mut colors = [[0f32; 3]; N];
    for i in 0..N {
        if m[i][i].abs() > 1e-12 {
            for c in 0..3 {
                colors[i][c] = (b[i][c] / m[i][i]) as f32;
            }
        }
    }
    colors
}

/// What the terrain is painted with: the textures of its 8 layers, their weights at each point and
/// their colors estimated from the minimap.
#[derive(Debug, Clone, Default)]
pub struct TerrainTextures {
    pub cache_handle_id: String,
    /// The terrain type, the `tileSet` of `t3Terrain.xml`.
    pub tile_set: String,
    /// The texture of each layer of the masks, empty for an unused layer.
    pub layer_names: [String; TERRAIN_BLEND_LAYERS],
    pub masks: T3TextureMasks,
    /// The terrain type when the map changes it.
    pub terrain_data: Option<TerrainData>,
    /// None when the map has no minimap.
    pub layer_colors: Option<LayerColors>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `t3TextureMasks` of `width`x`height` points whose weight of each layer is `weight(layer,
    /// x, y)`.
    fn masks_file(
        width: usize,
        height: usize,
        weight: impl Fn(usize, usize, usize) -> u8,
    ) -> Vec<u8> {
        let mut data = b"MASK".to_vec();
        data.extend_from_slice(&102u32.to_le_bytes());
        data.extend_from_slice(&0u32.to_le_bytes());
        data.extend_from_slice(&(width as u32).to_le_bytes());
        data.extend_from_slice(&(height as u32).to_le_bytes());
        data.resize(MASKS_HEADER_SIZE, 0);
        let blocks_x = width / MASKS_BLOCK;
        for layer in 0..TERRAIN_BLEND_LAYERS {
            let mut nibbles = vec![];
            for block in 0..(width / MASKS_BLOCK) * (height / MASKS_BLOCK) {
                for row in 0..MASKS_BLOCK {
                    for col in 0..MASKS_BLOCK {
                        let x = (block % blocks_x) * MASKS_BLOCK + col;
                        let y = (block / blocks_x) * MASKS_BLOCK + row;
                        nibbles.push(weight(layer, x, y));
                    }
                }
            }
            data.extend(nibbles.chunks(2).map(|pair| pair[0] << 4 | pair[1]));
        }
        data
    }

    #[test]
    fn it_reads_the_weights_of_the_blocks() {
        // 128x64 points: 2 blocks. Layer 1 on the left half, layer 5 on the right half, the top
        // rows mix layers 0 and 7.
        let weight = |layer: usize, x: usize, y: usize| -> u8 {
            match (y >= 60, x < 64, layer) {
                (true, _, 0) => 10,
                (true, _, 7) => 5,
                (false, true, 1) | (false, false, 5) => 15,
                _ => 0,
            }
        };
        let masks = T3TextureMasks::parse("id".into(), &masks_file(128, 64, weight)).unwrap();
        assert_eq!((masks.width, masks.height, masks.version), (128, 64, 102));
        assert_eq!(masks.weights_at(3, 2).unwrap(), &[0, 15, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            masks.weights_at(100, 0).unwrap(),
            &[0, 0, 0, 0, 0, 15, 0, 0]
        );
        assert_eq!(
            masks.weights_at(70, 61).unwrap(),
            &[10, 0, 0, 0, 0, 0, 0, 5]
        );
        assert!(masks.weights_at(128, 0).is_none());
        let (width, height, images) = masks.to_rgba_images(2);
        assert_eq!((width, height), (64, 32));
        // The left half: layer 1 is the green of the first image.
        assert_eq!(&images[0][0..4], &[0, 255, 0, 0]);
        // The right half: layer 5 is the green of the second image.
        assert_eq!(&images[1][40 * 4..40 * 4 + 4], &[0, 255, 0, 0]);
    }

    #[test]
    fn it_refuses_a_truncated_file() {
        let mut data = masks_file(64, 64, |_, _, _| 0);
        data.truncate(1000);
        assert!(T3TextureMasks::parse("id".into(), &data).is_err());
        assert!(T3TextureMasks::parse("id".into(), b"MAS").is_err());
    }

    #[test]
    fn it_reads_the_terrain_data() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<Catalog>
    <CTerrain id="HybridLabXelNaga">
        <BlendTextures index="0" value="DesertWorld4"/>
        <BlendTextures index="1" value="CharDunes_5"/>
        <CliffSets index="0" value="DesertWorldCliff0"/>
        <CliffSets index="1" removed="1"/>
        <FogEnabled value="1"/>
        <FogColor value="255,124,177,149"/>
        <FixedSkyboxModel value="BelShirSkyBox"/>
        <Lighting value="HybridLabOuter"/>
    </CTerrain>
</Catalog>"#;
        let data = TerrainData::parse(xml.as_bytes(), "HybridLabXelNaga").unwrap();
        assert_eq!(data.blend_textures, vec!["DesertWorld4", "CharDunes_5"]);
        assert_eq!(
            data.cliff_sets,
            vec!["DesertWorldCliff0".to_string(), String::new()]
        );
        assert_eq!(data.fog_color, Some([124, 177, 149, 255]));
        assert_eq!(data.lighting.as_deref(), Some("HybridLabOuter"));
        assert_eq!(data.skybox.as_deref(), Some("BelShirSkyBox"));
    }

    #[test]
    fn it_fits_the_colors_of_the_layers_to_the_minimap() {
        // A 16x8 cell map, 8 points per cell: layer 2 on the left half, layer 6 on the right
        // half. The minimap paints them red and blue.
        let masks_bytes = masks_file(128, 64, |layer, x, _| match (x < 64, layer) {
            (true, 2) | (false, 6) => 15,
            _ => 0,
        });
        let masks = T3TextureMasks::parse("id".into(), &masks_bytes).unwrap();
        let map_info = MapInfo {
            cell_width: 16,
            cell_height: 8,
            cell_left: 0,
            cell_bottom: 0,
            cell_right: 16,
            cell_top: 8,
            ..Default::default()
        };
        // 256 cells across the larger side of the image, 2 pixels per cell.
        let (width, height) = (512u32, 512u32);
        let mut pixels = vec![0u8; (width * height * 4) as usize];
        let (left, top) = ((512 - 32) / 2, (512 - 16) / 2);
        for py in top..top + 16 {
            for px in left..left + 32 {
                let color = if px - left < 16 {
                    [200, 20, 30, 255]
                } else {
                    [10, 40, 220, 255]
                };
                let start = ((py * width + px) * 4) as usize;
                pixels[start..start + 4].copy_from_slice(&color);
            }
        }
        let minimap = Minimap {
            cache_handle_id: "id".into(),
            width,
            height,
            pixels,
        };
        let colors = LayerColors::fit(&masks, &minimap, &map_info);
        assert_eq!(colors.colors[2], Some([200, 20, 30]));
        assert_eq!(colors.colors[6], Some([10, 40, 220]));
        assert_eq!(colors.colors[0], None);
        assert!((colors.shares[2] - 0.5).abs() < 0.01);
        assert!(colors.mean_abs_error < 1.);
    }
}
