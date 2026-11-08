//! The geometry and diffuse textures of the M3 (`MD34`) models stored in the map caches.
//!
//! An M3 file ends with an index of tagged blocks, a reference is the count and the index of a
//! block. The layout of the structures changes with their version, so the references are found by
//! their tags and only the fields that are stable between versions are read at fixed offsets. The
//! vertices are the bind pose of the model, there are no animations here.

const MD34_TAG: &[u8; 4] = b"43DM";
/// The bytes of the block index entry: the tag, the offset, the count and the version.
const INDEX_ENTRY_SIZE: usize = 16;
/// The texture coordinates are fixed point numbers.
const UV_SCALE: f32 = 2048.;
/// The bytes of a BAT_ batch, the same in the versions seen on the map caches.
const BATCH_SIZE: usize = 14;
const BATCH_REGION_OFFSET: usize = 4;
const BATCH_MATERIAL_REF_OFFSET: usize = 10;
/// The bytes of a MATM material reference: its type and index.
const MATERIAL_REF_SIZE: usize = 8;
/// The type of a MATM reference to a standard material, MAT_.
const STANDARD_MATERIAL_TYPE: u32 = 1;
/// The vertex flags of the second and later sets of texture coordinates and the color.
const VERTEX_COLOR_FLAG: u32 = 0x200;
const VERTEX_UV_SETS_SHIFT: u32 = 17;
const VERTEX_UV_SETS_MASK: u32 = 0xF;
const VERTEX_HEADER_SIZE: usize = 20;

/// One vertex of a model, in the coordinates of the model: Z is up.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct M3Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

/// The triangles of a model that share a material.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct M3Region {
    /// Triples of indices into [`M3Model::vertices`].
    pub indices: Vec<u32>,
    /// The diffuse texture as named by the model, None when it has no image.
    pub diffuse: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct M3Model {
    pub vertices: Vec<M3Vertex>,
    pub regions: Vec<M3Region>,
}

#[derive(Debug, Clone, Copy)]
struct IndexEntry {
    tag: [u8; 4],
    offset: usize,
    count: usize,
    version: u32,
}

/// The block index of an M3 file.
struct M3Index<'a> {
    data: &'a [u8],
    entries: Vec<IndexEntry>,
}

fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
    let bytes = data.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes(bytes.try_into().ok()?))
}

fn read_u16(data: &[u8], offset: usize) -> Option<u16> {
    let bytes = data.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_le_bytes(bytes.try_into().ok()?))
}

fn read_i16(data: &[u8], offset: usize) -> Option<i16> {
    read_u16(data, offset).map(|value| value as i16)
}

fn read_f32(data: &[u8], offset: usize) -> Option<f32> {
    read_u32(data, offset).map(f32::from_bits)
}

impl<'a> M3Index<'a> {
    fn parse(data: &'a [u8]) -> Result<Self, String> {
        if data.get(..4) != Some(MD34_TAG.as_slice()) {
            return Err("not an MD34 model".to_string());
        }
        let offset = read_u32(data, 4).ok_or("no index offset")? as usize;
        let count = read_u32(data, 8).ok_or("no index count")? as usize;
        let mut entries = Vec::with_capacity(count.min(data.len() / INDEX_ENTRY_SIZE));
        for ith in 0..count {
            let at = offset + ith * INDEX_ENTRY_SIZE;
            let tag = data.get(at..at + 4).ok_or("short index")?;
            entries.push(IndexEntry {
                tag: [tag[3], tag[2], tag[1], tag[0]],
                offset: read_u32(data, at + 4).ok_or("short index")? as usize,
                count: read_u32(data, at + 8).ok_or("short index")? as usize,
                version: read_u32(data, at + 12).ok_or("short index")?,
            });
        }
        Ok(Self { data, entries })
    }

    /// The bytes of a block, up to the start of the next one so with its padding.
    fn block(&self, index: usize) -> Option<&'a [u8]> {
        let start = self.entries.get(index)?.offset;
        let end = self
            .entries
            .iter()
            .map(|entry| entry.offset)
            .filter(|offset| *offset > start)
            .min()
            .unwrap_or(self.data.len());
        self.data.get(start..end)
    }

    /// The indices of the blocks with this tag referenced by a structure, in order. A reference is
    /// a count and an index that match an entry of the block index.
    fn references(&self, structure: &[u8], tag: &[u8; 4]) -> Vec<(usize, usize)> {
        let mut found = vec![];
        let mut at = 0;
        while at + 8 <= structure.len() {
            if let (Some(count), Some(index)) =
                (read_u32(structure, at), read_u32(structure, at + 4))
            {
                let (count, index) = (count as usize, index as usize);
                if count > 0
                    && self
                        .entries
                        .get(index)
                        .is_some_and(|entry| entry.tag == *tag && entry.count == count)
                {
                    found.push((at, index));
                }
            }
            at += 4;
        }
        found
    }

    /// The first reference of a tag, with its offset in the structure.
    fn reference(&self, structure: &[u8], tag: &[u8; 4]) -> Result<(usize, usize), String> {
        self.references(structure, tag)
            .first()
            .copied()
            .ok_or_else(|| format!("no {} reference", String::from_utf8_lossy(tag)))
    }

    /// The text of a CHAR block, None when it is empty.
    fn text(&self, index: usize) -> Option<String> {
        let entry = self.entries.get(index)?;
        let bytes = self.data.get(entry.offset..entry.offset + entry.count)?;
        let text = String::from_utf8_lossy(bytes)
            .trim_end_matches('\0')
            .to_string();
        (!text.is_empty()).then_some(text)
    }
}

/// The bytes of a region, they change with its version.
fn region_size(version: u32) -> Result<usize, String> {
    match version {
        3 => Ok(36),
        4 => Ok(40),
        5 => Ok(48),
        other => Err(format!("unknown REGN version {other}")),
    }
}

/// The bytes of a vertex, when `vertex_bytes` divides evenly by what the flags say.
fn vertex_layout(flags: u32) -> (usize, usize) {
    let has_color = flags & VERTEX_COLOR_FLAG != 0;
    let uv_sets = ((flags >> VERTEX_UV_SETS_SHIFT) & VERTEX_UV_SETS_MASK).count_ones() as usize;
    let normal_offset = VERTEX_HEADER_SIZE + if has_color { 4 } else { 0 };
    // The position, weights, bones, color, normal and sets of coordinates, then the tangent.
    let stride = normal_offset + 4 + uv_sets.max(1) * 4 + 4;
    (normal_offset, stride)
}

fn decode_normal(byte: u8) -> f32 {
    f32::from(byte) / 255. * 2. - 1.
}

/// The diffuse texture of each material reference of a model. A standard material lists its layers
/// in a fixed order and the diffuse one is the first.
fn material_textures(index: &M3Index, model: &[u8]) -> Result<Vec<Option<String>>, String> {
    let Ok((_, matm)) = index.reference(model, b"MATM") else {
        return Ok(vec![]);
    };
    let Some(matm_block) = index.block(matm) else {
        return Ok(vec![]);
    };
    let material_refs = index.entries[matm].count;
    let mut diffuse_of_standard = vec![];
    if let Ok((_, materials)) = index.reference(model, b"MAT_") {
        let count = index.entries[materials].count;
        let layer_refs = index.references(index.block(materials).unwrap_or_default(), b"LAYR");
        let per_material = layer_refs.len().checked_div(count).unwrap_or(0).max(1);
        for layers in layer_refs.chunks(per_material) {
            // The diffuse layer, otherwise the first one with an image.
            let images = layers.iter().filter_map(|(_, layer)| {
                let block = index.block(*layer)?;
                let (_, char_index) = index.reference(block, b"CHAR").ok()?;
                index.text(char_index)
            });
            diffuse_of_standard.push(images.into_iter().next());
        }
    }
    Ok((0..material_refs)
        .map(|ith| {
            let at = ith * MATERIAL_REF_SIZE;
            let kind = read_u32(matm_block, at)?;
            let material = read_u32(matm_block, at + 4)? as usize;
            (kind == STANDARD_MATERIAL_TYPE)
                .then(|| diffuse_of_standard.get(material).cloned().flatten())
                .flatten()
        })
        .collect())
}

/// Reads the vertices and triangles of an M3 file.
pub fn parse_m3(data: &[u8]) -> Result<M3Model, String> {
    let index = M3Index::parse(data)?;
    let model_index = index
        .entries
        .iter()
        .position(|entry| entry.tag == *b"MODL")
        .ok_or("no MODL block")?;
    let model = index.block(model_index).ok_or("short MODL block")?;
    let (vertices_at, vertices_index) = index.reference(model, b"U8__")?;
    // The vertex flags are right before the vertices.
    let flags = vertices_at
        .checked_sub(4)
        .and_then(|at| read_u32(model, at))
        .ok_or("no vertex flags")?;
    let vertices_entry = index.entries[vertices_index];
    let vertex_bytes = data
        .get(vertices_entry.offset..vertices_entry.offset + vertices_entry.count)
        .ok_or("short vertices")?;
    let (normal_offset, stride) = vertex_layout(flags);
    if !vertex_bytes.len().is_multiple_of(stride) {
        return Err(format!(
            "{} vertex bytes do not divide by {stride} of flags {flags:#x}",
            vertex_bytes.len()
        ));
    }
    let uv_offset = normal_offset + 4;
    let vertices = vertex_bytes
        .chunks_exact(stride)
        .map(|vertex| {
            Some(M3Vertex {
                position: [
                    read_f32(vertex, 0)?,
                    read_f32(vertex, 4)?,
                    read_f32(vertex, 8)?,
                ],
                normal: [
                    decode_normal(vertex[normal_offset]),
                    decode_normal(vertex[normal_offset + 1]),
                    decode_normal(vertex[normal_offset + 2]),
                ],
                uv: [
                    f32::from(read_i16(vertex, uv_offset)?) / UV_SCALE,
                    f32::from(read_i16(vertex, uv_offset + 2)?) / UV_SCALE,
                ],
            })
        })
        .collect::<Option<Vec<M3Vertex>>>()
        .ok_or("short vertex")?;

    let (_, division_index) = index.reference(model, b"DIV_")?;
    let division = index.block(division_index).ok_or("short DIV_ block")?;
    let (_, faces_index) = index.reference(division, b"U16_")?;
    let faces_entry = index.entries[faces_index];
    let faces = data
        .get(faces_entry.offset..faces_entry.offset + faces_entry.count * 2)
        .ok_or("short faces")?;
    let (_, regions_index) = index.reference(division, b"REGN")?;
    let regions_entry = index.entries[regions_index];
    let region_bytes = region_size(regions_entry.version)?;
    let regions_block = index.block(regions_index).ok_or("short REGN block")?;
    let textures = material_textures(&index, model)?;

    // The material of each region is told by the batches.
    let mut material_of_region: Vec<Option<usize>> = vec![None; regions_entry.count];
    if let Ok((_, batches_index)) = index.reference(division, b"BAT_") {
        let batches = index.block(batches_index).ok_or("short BAT_ block")?;
        for ith in 0..index.entries[batches_index].count {
            let at = ith * BATCH_SIZE;
            let region = read_u16(batches, at + BATCH_REGION_OFFSET).ok_or("short batch")?;
            let material =
                read_u16(batches, at + BATCH_MATERIAL_REF_OFFSET).ok_or("short batch")?;
            if let Some(slot) = material_of_region.get_mut(region as usize) {
                slot.get_or_insert(material as usize);
            }
        }
    }

    let mut regions = Vec::with_capacity(regions_entry.count);
    for ith in 0..regions_entry.count {
        let at = ith * region_bytes;
        let first_vertex = read_u32(regions_block, at + 8).ok_or("short region")?;
        let vertex_count = read_u32(regions_block, at + 12).ok_or("short region")?;
        let first_face = read_u32(regions_block, at + 16).ok_or("short region")? as usize;
        let face_count = read_u32(regions_block, at + 20).ok_or("short region")? as usize;
        let mut indices = Vec::with_capacity(face_count);
        for face in first_face..first_face + face_count {
            let vertex = u32::from(read_u16(faces, face * 2).ok_or("face out of range")?);
            if vertex >= vertex_count || (first_vertex + vertex) as usize >= vertices.len() {
                return Err(format!("face vertex {vertex} out of region {ith}"));
            }
            indices.push(first_vertex + vertex);
        }
        regions.push(M3Region {
            indices,
            diffuse: material_of_region[ith]
                .and_then(|material| textures.get(material).cloned().flatten()),
        });
    }
    Ok(M3Model { vertices, regions })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds an M3 file with one triangle of a model, the blocks after the header.
    fn triangle_m3() -> Vec<u8> {
        let mut blocks: Vec<([u8; 4], u32, u32, Vec<u8>)> = vec![];
        let mut add = |tag: &[u8; 4], count: u32, version: u32, bytes: Vec<u8>| {
            blocks.push((*tag, count, version, bytes));
            blocks.len() as u32 - 1
        };
        let mut vertices = vec![];
        for (x, u) in [(1f32, 0i16), (2., 2048), (3., 1024)] {
            vertices.extend(x.to_le_bytes());
            vertices.extend(0f32.to_le_bytes());
            vertices.extend(5f32.to_le_bytes());
            vertices.extend([0u8; 8]); // The bone weights and indices.
            vertices.extend([255u8, 0, 128, 0]); // The normal.
            vertices.extend(u.to_le_bytes());
            vertices.extend(0i16.to_le_bytes());
            vertices.extend([0u8; 4]); // The tangent.
        }
        let vertices_len = vertices.len() as u32;
        let image = b"Assets/Textures/a.dds\0".to_vec();
        let image_len = image.len() as u32;
        let image_index = add(b"CHAR", image_len, 0, image);
        let mut layer = vec![0u8; 4];
        layer.extend(image_len.to_le_bytes());
        layer.extend(image_index.to_le_bytes());
        layer.extend([0u8; 4]);
        let layer_index = add(b"LAYR", 1, 22, layer);
        let mut material = vec![0u8; 16];
        material.extend(1u32.to_le_bytes());
        material.extend(layer_index.to_le_bytes());
        material.extend([0u8; 4]);
        let material_index = add(b"MAT_", 1, 15, material);
        let mut material_ref = 1u32.to_le_bytes().to_vec();
        material_ref.extend(0u32.to_le_bytes());
        let material_ref_index = add(b"MATM", 1, 0, material_ref);
        let vertices_index = add(b"U8__", vertices_len, 0, vertices);
        let mut faces = vec![];
        for face in [0u16, 1, 2] {
            faces.extend(face.to_le_bytes());
        }
        let faces_index = add(b"U16_", 3, 0, faces);
        let mut region = vec![0u8; 8];
        for value in [0u32, 3, 0, 3] {
            region.extend(value.to_le_bytes());
        }
        region.resize(36, 0);
        let region_index = add(b"REGN", 1, 3, region);
        let mut batch = vec![0u8; BATCH_SIZE];
        batch[BATCH_MATERIAL_REF_OFFSET] = 0;
        let batch_index = add(b"BAT_", 1, 1, batch);
        let mut division = vec![];
        for (count, index) in [(3, faces_index), (1, region_index), (1, batch_index)] {
            division.extend((count as u32).to_le_bytes());
            division.extend(index.to_le_bytes());
            division.extend([0u8; 4]);
        }
        let division_index = add(b"DIV_", 1, 2, division);
        let mut model = vec![0u8; 16];
        // One set of coordinates.
        model.extend(0x20000u32.to_le_bytes());
        model.extend(vertices_len.to_le_bytes());
        model.extend(vertices_index.to_le_bytes());
        model.extend([0u8; 4]);
        for (count, index) in [
            (1, division_index),
            (1, material_ref_index),
            (1, material_index),
        ] {
            model.extend((count as u32).to_le_bytes());
            model.extend(index.to_le_bytes());
            model.extend([0u8; 4]);
        }
        add(b"MODL", 1, 23, model);

        let mut file = vec![0u8; 32];
        file[..4].copy_from_slice(MD34_TAG);
        let mut index = vec![];
        for (tag, count, version, bytes) in blocks.iter() {
            index.extend([tag[3], tag[2], tag[1], tag[0]]);
            index.extend((file.len() as u32).to_le_bytes());
            index.extend(count.to_le_bytes());
            index.extend(version.to_le_bytes());
            file.extend(bytes);
            file.resize(file.len().next_multiple_of(16), 0);
        }
        let index_offset = file.len() as u32;
        file[4..8].copy_from_slice(&index_offset.to_le_bytes());
        file[8..12].copy_from_slice(&(blocks.len() as u32).to_le_bytes());
        file.extend(index);
        file
    }

    #[test]
    fn it_reads_the_vertices_faces_and_diffuse_texture() {
        let model = parse_m3(&triangle_m3()).unwrap();
        assert_eq!(model.vertices.len(), 3);
        assert_eq!(model.vertices[1].position, [2., 0., 5.]);
        assert_eq!(model.vertices[1].uv, [1., 0.]);
        assert_eq!(model.vertices[0].normal[0], 1.);
        assert_eq!(model.regions.len(), 1);
        assert_eq!(model.regions[0].indices, vec![0, 1, 2]);
        assert_eq!(
            model.regions[0].diffuse.as_deref(),
            Some("Assets/Textures/a.dds")
        );
    }

    #[test]
    fn it_refuses_what_is_not_a_model() {
        assert!(parse_m3(b"Standard Data: Liberty.SC2Mod").is_err());
        let mut truncated = triangle_m3();
        truncated.truncate(40);
        assert!(parse_m3(&truncated).is_err());
    }

    /// The models of real map caches, with their directory in `S2PROTOCOL_MAP_CACHES`.
    #[test]
    #[ignore]
    fn it_reads_the_models_of_the_map_caches() {
        let dir = std::env::var("S2PROTOCOL_MAP_CACHES").unwrap();
        let (mut read, mut failed) = (0, 0);
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let Ok(contents) = std::fs::read(entry.path()) else {
                continue;
            };
            let Ok((_, mpq)) = nom_mpq::parser::parse(&contents) else {
                continue;
            };
            let Ok(files) = mpq.get_files(&contents) else {
                continue;
            };
            for (name, _) in files.iter().filter(|(name, _)| name.ends_with(".m3")) {
                let Ok((_, m3)) = mpq.read_mpq_file_sector(name, false, &contents) else {
                    continue;
                };
                match parse_m3(&m3) {
                    Ok(_) => read += 1,
                    // Particle effects have no vertices of their own.
                    Err(err) if err == "no U8__ reference" => {}
                    Err(err) => {
                        failed += 1;
                        println!("{name}: {err}");
                    }
                }
            }
        }
        println!("{read} models read, {failed} failed");
        assert_eq!(failed, 0, "{read} models read, {failed} failed");
    }
}
