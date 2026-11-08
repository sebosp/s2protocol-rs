//! The graphics a map carries in its caches and where they go on the map: an object placed in the
//! `Objects` file names a model id, `ModelData.xml` gives the `.m3` file of the id and the model
//! names its `.dds` textures. Most objects use the models of the game itself, which are not in the
//! caches, those have no graphic here.

use super::cache_objects::PlacedObjects;
use super::m3::{M3Model, parse_m3};
use super::{CACHE_MPQ_ARCHIVE_EXTENSION, PLACED_OBJECTS_FILE_NAME};
use crate::S2ProtocolError;
use nom_mpq::MPQ;
use std::collections::HashMap;

const MODEL_DATA_FILE_NAME: &str = "Base.SC2Data\\GameData\\ModelData.xml";
const MODEL_EXTENSION: &str = ".m3";

/// An object of the map drawn with a model of the caches.
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedGraphic {
    /// The key of the model in [`MapGraphics::models`].
    pub model: String,
    /// The type of the object in the `Objects` file.
    pub kind: String,
    pub id: String,
    /// The position on the map: x and y, and the height in the map's own units.
    pub position: [f32; 3],
    /// Around the up axis, in radians.
    pub rotation: f32,
    pub scale: [f32; 3],
}

/// The number of graphics files found, and how many objects of the map use a model of the caches.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GraphicsCounts {
    pub textures: usize,
    pub models: usize,
    pub objects: usize,
    pub objects_with_model: usize,
}

#[derive(Debug, Default)]
pub struct MapGraphics {
    pub models: HashMap<String, M3Model>,
    /// The `.dds` files used by [`Self::models`], by [`normalized_path`].
    pub textures: HashMap<String, Vec<u8>>,
    pub placements: Vec<PlacedGraphic>,
    pub counts: GraphicsCounts,
}

/// The form of a path to compare it: the models write them with `/`, the archives with `\`, and
/// their case differs.
pub fn normalized_path(path: &str) -> String {
    path.replace('/', "\\").to_lowercase()
}

/// The `Model` value of each `CModel` id of a `ModelData.xml`.
pub fn parse_model_data(xml: &str) -> HashMap<String, String> {
    let mut models = HashMap::new();
    for section in xml.split("<CModel").skip(1) {
        let section = section.split("</CModel>").next().unwrap_or_default();
        let id = attribute(section, "id=\"");
        let model = section
            .split_once("<Model ")
            .and_then(|(_, model)| attribute(model, "value=\""));
        if let (Some(id), Some(model)) = (id, model) {
            models.insert(id.to_string(), model.to_string());
        }
    }
    models
}

fn attribute<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let (_, rest) = text.split_once(name)?;
    rest.split('"').next()
}

/// The files a model id may be stored as, a model with variations has one file per variation.
pub fn model_candidates(id: &str, declared: Option<&str>, variation: &str) -> Vec<String> {
    let path = declared
        .map(str::to_string)
        .unwrap_or_else(|| format!("Assets\\Doodads\\{id}\\{id}{MODEL_EXTENSION}"));
    let path = normalized_path(&path);
    let stem = path.strip_suffix(MODEL_EXTENSION).unwrap_or(&path);
    let variation: u32 = variation.parse().unwrap_or(0);
    vec![
        format!("{stem}_{variation:02}{MODEL_EXTENSION}"),
        path.clone(),
        format!("{stem}_00{MODEL_EXTENSION}"),
    ]
}

fn parse_floats<const N: usize>(text: &str) -> Option<[f32; N]> {
    let values: Vec<f32> = text
        .split(',')
        .filter_map(|value| value.trim().parse().ok())
        .collect();
    values.try_into().ok()
}

/// A cache archive with the names of its files.
struct CacheArchive {
    contents: Vec<u8>,
    mpq: MPQ,
    files: HashMap<String, String>,
}

impl CacheArchive {
    /// None for the caches that are not archives or have no list of files.
    fn read(path: &str) -> Option<Self> {
        let contents = std::fs::read(path).ok()?;
        let (_, mpq) = nom_mpq::parser::parse(&contents)
            .inspect_err(|err| tracing::debug!("{path} is not read: {err}"))
            .ok()?;
        let files = mpq
            .get_files(&contents)
            .ok()?
            .into_iter()
            .map(|(name, _)| (normalized_path(&name), name))
            .collect();
        Some(Self {
            contents,
            mpq,
            files,
        })
    }

    fn read_file(&self, normalized: &str) -> Option<Vec<u8>> {
        let name = self.files.get(normalized)?;
        self.mpq
            .read_mpq_file_sector(name, false, &self.contents)
            .ok()
            .map(|(_, content)| content)
    }
}

/// Reads the graphics of a map from its caches, the same list of ids of a replay.
pub fn read_map_graphics(
    cache_path: &str,
    cache_ids: &[String],
) -> Result<MapGraphics, S2ProtocolError> {
    let archives: Vec<CacheArchive> = cache_ids
        .iter()
        .filter_map(|id| {
            CacheArchive::read(&format!("{cache_path}/{id}.{CACHE_MPQ_ARCHIVE_EXTENSION}"))
        })
        .collect();
    let read_file = |normalized: &str| {
        archives
            .iter()
            .find_map(|archive| archive.read_file(normalized))
    };
    let mut model_data = HashMap::new();
    let model_data_name = normalized_path(MODEL_DATA_FILE_NAME);
    // Each archive may add models, the first one to name an id wins as with the other files.
    for archive in archives.iter().rev() {
        if let Some(xml) = archive.read_file(&model_data_name) {
            model_data.extend(parse_model_data(&String::from_utf8_lossy(&xml)));
        }
    }
    let objects = read_file(&normalized_path(PLACED_OBJECTS_FILE_NAME)).ok_or_else(|| {
        S2ProtocolError::CacheResource {
            name: PLACED_OBJECTS_FILE_NAME.to_string(),
            cache_handles: cache_ids.to_vec(),
        }
    })?;
    let placed_objects = PlacedObjects::parse(cache_ids.join(","), &objects)?;

    let mut graphics = MapGraphics::default();
    for file in archives.iter().flat_map(|archive| archive.files.keys()) {
        if file.ends_with(".dds") || file.ends_with(".tga") {
            graphics.counts.textures += 1;
        } else if file.ends_with(MODEL_EXTENSION) {
            graphics.counts.models += 1;
        }
    }
    graphics.counts.objects = placed_objects.doodas.len();
    // None for the models that could not be read, so that they are read once.
    let mut read_models: HashMap<String, Option<M3Model>> = HashMap::new();
    for doodad in placed_objects.doodas {
        let declared = model_data.get(&doodad.kind).map(String::as_str);
        let Some(model_key) = model_candidates(&doodad.kind, declared, &doodad.variation)
            .into_iter()
            .find(|candidate| archives.iter().any(|a| a.files.contains_key(candidate)))
        else {
            continue;
        };
        let Some(position) = doodad.position.as_deref().and_then(parse_floats::<3>) else {
            continue;
        };
        let model = read_models.entry(model_key.clone()).or_insert_with(|| {
            read_file(&model_key).and_then(|m3| {
                parse_m3(&m3)
                    .inspect_err(|err| tracing::warn!("Unable to read {model_key}: {err}"))
                    .ok()
            })
        });
        if model.is_none() {
            continue;
        }
        graphics.placements.push(PlacedGraphic {
            model: model_key,
            kind: doodad.kind,
            id: doodad.id,
            position,
            rotation: doodad.rotation.parse().unwrap_or(0.),
            scale: parse_floats::<3>(&doodad.scale).unwrap_or([1., 1., 1.]),
        });
    }
    graphics.counts.objects_with_model = graphics.placements.len();
    graphics.models = read_models
        .into_iter()
        .filter_map(|(key, model)| Some((key, model?)))
        .collect();
    let textures: Vec<String> = graphics
        .models
        .values()
        .flat_map(|model| model.regions.iter())
        .filter_map(|region| region.diffuse.as_deref().map(normalized_path))
        .collect();
    for texture in textures {
        if !graphics.textures.contains_key(&texture)
            && let Some(dds) = read_file(&texture)
        {
            graphics.textures.insert(texture, dds);
        }
    }
    Ok(graphics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_model_of_each_id() {
        let xml = r#"<Catalog>
    <CModel id="Bricks">
        <Model value="Assets\Models\Bricks\Bricks.m3"/>
        <VariationCount value="1"/>
    </CModel>
    <CModel id="OnlySpeed">
        <AnimSpeed value="0.3"/>
    </CModel>
</Catalog>"#;
        let models = parse_model_data(xml);
        assert_eq!(models.len(), 1);
        assert_eq!(models["Bricks"], "Assets\\Models\\Bricks\\Bricks.m3");
    }

    #[test]
    fn it_tries_the_variation_of_the_model_first() {
        assert_eq!(
            model_candidates("Bricks", Some("Assets/Models/Bricks.m3"), "2"),
            vec![
                "assets\\models\\bricks_02.m3",
                "assets\\models\\bricks.m3",
                "assets\\models\\bricks_00.m3"
            ]
        );
        assert_eq!(
            model_candidates("Sign", None, ""),
            vec![
                "assets\\doodads\\sign\\sign_00.m3",
                "assets\\doodads\\sign\\sign.m3",
                "assets\\doodads\\sign\\sign_00.m3"
            ]
        );
    }

    #[test]
    fn it_reads_the_numbers_of_an_object() {
        assert_eq!(parse_floats::<3>("1,2.5,3"), Some([1., 2.5, 3.]));
        assert_eq!(parse_floats::<3>("1,2"), None);
    }

    #[test]
    fn it_compares_paths_of_both_forms() {
        assert_eq!(
            normalized_path("Assets/Textures/A.DDS"),
            normalized_path("assets\\textures\\a.dds")
        );
    }

    /// The graphics of every map of real caches, with their directory in `S2PROTOCOL_MAP_CACHES`.
    #[test]
    #[ignore]
    fn it_places_the_graphics_of_the_map_caches() {
        let dir = std::env::var("S2PROTOCOL_MAP_CACHES").unwrap();
        let mut totals = GraphicsCounts::default();
        let (mut maps, mut models) = (0, 0);
        let mut textures = 0;
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            let name = entry.path();
            let id = name.file_stem().unwrap().to_string_lossy().to_string();
            let Ok(graphics) = read_map_graphics(&dir, &[id]) else {
                continue;
            };
            maps += 1;
            totals.objects += graphics.counts.objects;
            totals.objects_with_model += graphics.counts.objects_with_model;
            models += graphics.models.len();
            textures += graphics.textures.len();
        }
        println!("{maps} maps, {totals:?}, {models} models used, {textures} textures found");
        assert!(totals.objects_with_model > 0);
    }
}
