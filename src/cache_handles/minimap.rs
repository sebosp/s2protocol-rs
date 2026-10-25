//! The minimap of the map, the `Minimap.tga` file embedded in the map caches.
//!
//! The image is a window of 256x256 cells centered on the playable area of the map, see
//! [`super::map_info::MapInfo`], at `max(width, height) / 256` pixels per cell, with the first row
//! at the top (the highest y), the cells outside the playable area are black.
//! [`Minimap::crop_to_playable`] keeps only the playable area.
//! Only the true color TGA images are supported, raw or run length encoded, with 24 or 32 bits per
//! pixel, which is what the map caches contain.

use super::map_info::MapInfo;
use crate::S2ProtocolError;
use serde::{Deserialize, Serialize};

/// The size of the TGA header.
const HEADER_SIZE: usize = 18;

/// The cells covered by the larger side of the image.
const MINIMAP_CELLS: f32 = 256.;

/// An RGBA8 image of the minimap.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Minimap {
    /// The cache_handle_id where the minimap was located.
    pub cache_handle_id: String,
    pub width: u32,
    pub height: u32,
    /// The RGBA8 pixels, row by row from the top left corner.
    pub pixels: Vec<u8>,
}

fn error(message: impl Into<String>) -> S2ProtocolError {
    S2ProtocolError::Minimap(message.into())
}

impl Minimap {
    /// Decodes the TGA file.
    pub fn parse(cache_handle_id: String, tga: &[u8]) -> Result<Self, S2ProtocolError> {
        let header = tga
            .get(..HEADER_SIZE)
            .ok_or_else(|| error("The file is smaller than the TGA header"))?;
        let id_length = header[0] as usize;
        let color_map_type = header[1];
        let image_type = header[2];
        let width = u16::from_le_bytes([header[12], header[13]]) as usize;
        let height = u16::from_le_bytes([header[14], header[15]]) as usize;
        let bits_per_pixel = header[16];
        let descriptor = header[17];
        if color_map_type != 0 {
            return Err(error("Color mapped TGA images are not supported"));
        }
        let bytes_per_pixel = match bits_per_pixel {
            24 => 3,
            32 => 4,
            bits => return Err(error(format!("{bits} bits per pixel are not supported"))),
        };
        let data = tga
            .get(HEADER_SIZE + id_length..)
            .ok_or_else(|| error("The file is smaller than the TGA image id"))?;
        let pixel_count = width * height;
        // The pixels in the file order, BGR(A).
        let file_pixels = match image_type {
            2 => data
                .get(..pixel_count * bytes_per_pixel)
                .ok_or_else(|| error("The image data is truncated"))?
                .to_vec(),
            10 => decode_rle(data, pixel_count, bytes_per_pixel)?,
            image_type => {
                return Err(error(format!(
                    "TGA image type {image_type} is not supported"
                )));
            }
        };
        // Bit 5 is set when the first row is the top one, bit 4 when the first column is the
        // right one.
        let top_to_bottom = descriptor & 0x20 != 0;
        let right_to_left = descriptor & 0x10 != 0;
        let mut pixels = vec![0u8; pixel_count * 4];
        for (idx, pixel) in file_pixels.chunks_exact(bytes_per_pixel).enumerate() {
            let (file_x, file_y) = (idx % width, idx / width);
            let x = if right_to_left {
                width - 1 - file_x
            } else {
                file_x
            };
            let y = if top_to_bottom {
                file_y
            } else {
                height - 1 - file_y
            };
            let alpha = if bytes_per_pixel == 4 { pixel[3] } else { 255 };
            let offset = (y * width + x) * 4;
            pixels[offset..offset + 4].copy_from_slice(&[pixel[2], pixel[1], pixel[0], alpha]);
        }
        Ok(Self {
            cache_handle_id,
            width: width as u32,
            height: height as u32,
            pixels,
        })
    }
}

impl Minimap {
    /// The pixels of each map cell.
    pub fn pixels_per_cell(&self) -> f32 {
        self.width.max(self.height) as f32 / MINIMAP_CELLS
    }

    /// Keeps the playable area of the map, from `cell_left` to `cell_right` and from `cell_top`
    /// to `cell_bottom`, the image then covers exactly the playable area.
    pub fn crop_to_playable(&self, map_info: &MapInfo) -> Self {
        let scale = self.pixels_per_cell();
        let playable_width = map_info.cell_right.saturating_sub(map_info.cell_left) as f32;
        let playable_height = map_info.cell_top.saturating_sub(map_info.cell_bottom) as f32;
        let clamp = |value: f32, max: u32| (value.round().max(0.) as u32).min(max);
        // The playable area is centered in the image.
        let left = clamp(
            (self.width as f32 - playable_width * scale) / 2.,
            self.width,
        );
        let top = clamp(
            (self.height as f32 - playable_height * scale) / 2.,
            self.height,
        );
        let width = clamp(playable_width * scale, self.width - left);
        let height = clamp(playable_height * scale, self.height - top);
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);
        for row in top..top + height {
            let start = ((row * self.width + left) * 4) as usize;
            pixels.extend_from_slice(&self.pixels[start..start + (width * 4) as usize]);
        }
        Self {
            cache_handle_id: self.cache_handle_id.clone(),
            width,
            height,
            pixels,
        }
    }
}

/// Decodes the run length encoded pixels, each packet starts with a byte, the high bit set for a
/// pixel repeated `count` times, otherwise `count` raw pixels follow.
fn decode_rle(
    data: &[u8],
    pixel_count: usize,
    bytes_per_pixel: usize,
) -> Result<Vec<u8>, S2ProtocolError> {
    let mut pixels = Vec::with_capacity(pixel_count * bytes_per_pixel);
    let mut pos = 0;
    let truncated = || error("The run length encoded data is truncated");
    while pixels.len() < pixel_count * bytes_per_pixel {
        let packet = *data.get(pos).ok_or_else(truncated)?;
        pos += 1;
        let count = (packet & 0x7f) as usize + 1;
        if packet & 0x80 != 0 {
            let pixel = data.get(pos..pos + bytes_per_pixel).ok_or_else(truncated)?;
            pos += bytes_per_pixel;
            for _ in 0..count {
                pixels.extend_from_slice(pixel);
            }
        } else {
            let raw = data
                .get(pos..pos + count * bytes_per_pixel)
                .ok_or_else(truncated)?;
            pos += count * bytes_per_pixel;
            pixels.extend_from_slice(raw);
        }
    }
    pixels.truncate(pixel_count * bytes_per_pixel);
    Ok(pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header(image_type: u8, width: u16, height: u16, bits: u8, descriptor: u8) -> Vec<u8> {
        let mut header = vec![0u8; HEADER_SIZE];
        header[2] = image_type;
        header[12..14].copy_from_slice(&width.to_le_bytes());
        header[14..16].copy_from_slice(&height.to_le_bytes());
        header[16] = bits;
        header[17] = descriptor;
        header
    }

    #[test]
    fn it_decodes_a_bottom_up_bgr_image() {
        let mut tga = header(2, 2, 2, 24, 0);
        // The bottom row first: blue, green, then the top row: red, white.
        tga.extend([255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255]);
        let minimap = Minimap::parse("id".to_string(), &tga).unwrap();
        assert_eq!(
            minimap.pixels,
            vec![
                255, 0, 0, 255, 255, 255, 255, 255, // red, white
                0, 0, 255, 255, 0, 255, 0, 255, // blue, green
            ]
        );
    }

    #[test]
    fn it_decodes_a_run_length_encoded_top_down_image() {
        let mut tga = header(10, 3, 1, 32, 0x20);
        // A run of 2 red pixels, then 1 raw transparent blue pixel.
        tga.extend([0x81, 0, 0, 255, 255, 0x00, 255, 0, 0, 0]);
        let minimap = Minimap::parse("id".to_string(), &tga).unwrap();
        assert_eq!(
            minimap.pixels,
            vec![255, 0, 0, 255, 255, 0, 0, 255, 0, 0, 255, 0]
        );
    }

    #[test]
    fn it_crops_the_playable_area() {
        // A 512x512 image is 2 pixels per cell, the 100x50 cells playable area is centered.
        let minimap = Minimap {
            cache_handle_id: "id".to_string(),
            width: 512,
            height: 512,
            pixels: vec![0; 512 * 512 * 4],
        };
        let map_info = MapInfo {
            cell_left: 10,
            cell_right: 110,
            cell_bottom: 20,
            cell_top: 70,
            ..Default::default()
        };
        let playable = minimap.crop_to_playable(&map_info);
        assert_eq!((playable.width, playable.height), (200, 100));
        assert_eq!(playable.pixels.len(), 200 * 100 * 4);
    }

    #[test]
    fn it_rejects_truncated_images() {
        let mut tga = header(2, 4, 4, 24, 0);
        tga.extend([0; 10]);
        assert!(Minimap::parse("id".to_string(), &tga).is_err());
    }
}
