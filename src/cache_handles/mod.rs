//! Handling of cache_handles on replay files.
//! These are <some_id>.s2ma files that can be downloaded from blizzard,
//! containing map resources, mod information, visual resources such
//! as images used in overlays for tournament, organizers, etc.

use crate::{S2ProtocolError, basic_replay_data::SC2ReplayBasicData};

pub mod cache_objects;
pub mod document_header;
pub mod map;
pub mod map_info;
pub mod t3_height_map;
pub mod t3_terrain;

use cache_objects::PlacedObjects;
use document_header::DocumentHeader;
use map_info::MapInfo;
use std::{collections::HashMap, path::Path};
use t3_height_map::T3HeightMap;
use t3_terrain::T3Terrain;

pub use map::*;
use tracing::{self, instrument};

pub const CACHE_MPQ_ARCHIVE_EXTENSION: &'static str = "s2ma";
pub const MAP_INFO_FILE_NAME: &'static str = "MapInfo";
pub const DOCUMENT_HEADER_FILE_NAME: &'static str = "DocumentHeader";
pub const T3_HEIGHT_MAP_FILE_NAME: &'static str = "t3HeightMap";
pub const T3_TERRAIN_MAP_FILE_NAME: &'static str = "t3Terrain.xml";
pub const PLACED_OBJECTS_FILE_NAME: &'static str = "Objects";

/// The MapCaches are specific to one SC2Replay File
#[derive(Debug)]
pub struct MapCache {
    pub map_info: MapInfo,
    pub document_header: DocumentHeader,
    pub t3_height_map: T3HeightMap,
    pub t3_terrain: T3Terrain,
    pub placed_objects: PlacedObjects,
}

#[derive(Debug, PartialEq)]
pub struct MPQNamedSector {
    pub name: String,
    pub content: Vec<u8>,
}

/// The SC2Replay has a list of cache files, these caches must be downloaded from the blizzard depots.
/// This struct helps loading caches from this list.
/// For now. the first entry found from the list is returned.
/// Not sure there is some logic overwritting files.
#[derive(Debug)]
pub struct CacheCollection {
    pub cache_path: String,
    // A cache_fs of sorts, one cache_id contains the MPQ files names.
    pub cache_fs: HashMap<String, Vec<MPQNamedSector>>,
}

fn try_read_embedded_sectors_with_contents(
    cache_handle_fname: &str,
) -> Result<Vec<MPQNamedSector>, S2ProtocolError> {
    let (_, cache_contents) = crate::read_mpq(cache_handle_fname)?;
    let (_contents, mpq) = nom_mpq::parser::parse(&cache_contents)?;
    let mut embedded_files: Vec<MPQNamedSector> = vec![];
    if let Ok(mpq_embedded_files) = mpq.get_files(&cache_contents) {
        for (mpq_embedded_file_name, _file_size) in mpq_embedded_files {
            let (_, sector_content) =
                mpq.read_mpq_file_sector(&mpq_embedded_file_name, false, &cache_contents)?;
            embedded_files.push(MPQNamedSector {
                name: mpq_embedded_file_name,
                content: sector_content,
            });
        }
    }
    Ok(embedded_files)
}
impl CacheCollection {
    pub fn new(cache_path: String) -> Self {
        Self {
            cache_path,
            cache_fs: HashMap::new(),
        }
    }

    /// Adds a list of caches to the collection.
    pub fn add_cache_ids(&mut self, cache_ids: &[String]) {
        for cache_id in cache_ids {
            if self.cache_fs.contains_key(cache_id) {
                continue;
            }
            let cache_handle_fname = format!(
                "{}/{}.{}",
                self.cache_path, cache_id, CACHE_MPQ_ARCHIVE_EXTENSION
            );
            if let Ok(named_sector) = try_read_embedded_sectors_with_contents(&cache_handle_fname) {
                self.cache_fs.insert(cache_id.to_string(), named_sector);
            } else {
                self.cache_fs.insert(cache_id.to_string(), vec![]);
            }
        }
    }

    /// Iterates over the [`cache_ids`] stored at [`cache_path`] and returns the byte content if any.
    #[instrument(level = "debug", skip(self))]
    pub fn try_get_sector_content_from_collection(
        &self,
        cache_ids: &[String],
        target_file_name: &str,
    ) -> Result<(String, Vec<u8>), S2ProtocolError> {
        for cache_id in cache_ids {
            if let Some(cache_contents) = self.cache_fs.get(cache_id) {
                for mpq_file_with_digest in cache_contents {
                    if mpq_file_with_digest.name == target_file_name {
                        return Ok((cache_id.clone(), mpq_file_with_digest.content.clone()));
                    }
                }
            }
        }
        Err(S2ProtocolError::CacheResource(format!(
            "{} Not found in CacheCollection",
            target_file_name
        )))
    }

    #[instrument(level = "debug", skip(self))]
    pub fn build_map_cache(&self, cache_ids: &[String]) -> Result<MapCache, S2ProtocolError> {
        let init_time_4 = std::time::Instant::now();

        let (cache_handle_id, cache_contents) =
            self.try_get_sector_content_from_collection(cache_ids, MAP_INFO_FILE_NAME)?;
        let (_, map_info) = MapInfo::parse(cache_handle_id, &cache_contents)?;

        let (cache_handle_id, cache_contents) =
            self.try_get_sector_content_from_collection(cache_ids, DOCUMENT_HEADER_FILE_NAME)?;
        let (_, document_header) = DocumentHeader::parse(cache_handle_id, &cache_contents)?;

        let (cache_handle_id, cache_contents) =
            self.try_get_sector_content_from_collection(cache_ids, T3_HEIGHT_MAP_FILE_NAME)?;
        let (_, t3_height_map) = T3HeightMap::parse(cache_handle_id, &cache_contents, &map_info)?;

        let (cache_handle_id, cache_contents) =
            self.try_get_sector_content_from_collection(cache_ids, T3_TERRAIN_MAP_FILE_NAME)?;
        let t3_terrain = T3Terrain::parse(cache_handle_id, &cache_contents)?;

        let (cache_handle_id, cache_contents) =
            self.try_get_sector_content_from_collection(cache_ids, PLACED_OBJECTS_FILE_NAME)?;
        let placed_objects = PlacedObjects::parse(cache_handle_id, &cache_contents)?;
        println!(
            "---- init_4: CacheCollectionBuilder::build : {:?}",
            init_time_4.elapsed(),
        );
        Ok(MapCache {
            map_info,
            document_header,
            t3_height_map,
            t3_terrain,
            placed_objects,
        })
    }
}

/// Attempts to download the replay cache from theblizzard depots.
/// The caches are needed to identify a unique map version.
/// There's a version to a map, but I haven't identified it yet from the decoded data.
#[instrument]
pub async fn populate_map_info_digest_from_caches(
    sources: &[SC2ReplayBasicData],
    destination: String,
) -> HashMap<String, Option<String>> {
    let init_time = std::time::Instant::now();
    let mut cache_handle_ids: HashMap<String, Option<String>> = HashMap::new();
    let downloaded_cache_count: usize = sources
        .iter()
        .map(|source| {
            source
                .init_data
                .sync_lobby_state
                .game_description
                .cache_handles
                .iter()
                .map(|x| {
                    (
                        source
                            .init_data
                            .sync_lobby_state
                            .game_description
                            .cache_handle_region
                            .as_str(),
                        source
                            .init_data
                            .sync_lobby_state
                            .game_description
                            .cache_handle_extension
                            .as_str(),
                        x.as_str(),
                    )
                })
        })
        .flatten()
        .map(
            async |(cache_handle_region, cache_handle_extension, cache_handle_str)| {
                match download_cache(
                    cache_handle_str,
                    cache_handle_region,
                    cache_handle_extension,
                    &destination,
                )
                .await
                {
                    Ok(()) => 1,
                    Err(err) => {
                        tracing::error!("Unable to download cache: {:?}, skipping.", err);
                        0
                    }
                }
            },
        )
        .count();
    println!(
        "- init_1 After {} downloads/checks: {:?}",
        downloaded_cache_count,
        init_time.elapsed(),
    );
    let mut cache_builder = CacheCollection::new(destination.clone());
    for source in sources.iter() {
        cache_builder.add_cache_ids(&source.details.cache_handles)
    }

    for source in sources.iter() {
        let mut has_known_map_sha_digest = false;
        for cache_handle_str in source
            .init_data
            .sync_lobby_state
            .game_description
            .cache_handles
            .iter()
        {
            // Try to find, from the cache_handle list, if any is a recognized MapInfo.
            if cache_handle_ids.contains_key(cache_handle_str) {
                has_known_map_sha_digest = true;
            } else {
                cache_handle_ids.insert(cache_handle_str.to_string(), None);
            }
        }
        if !has_known_map_sha_digest {
            // We need to traverse the cache_handles on the current replay and locate the Mapinfo
            // from them:
            if let Ok(map_cache) = cache_builder.build_map_cache(&source.details.cache_handles) {
                let _ = cache_handle_ids.insert(
                    map_cache.map_info.cache_handle_id,
                    Some(map_cache.map_info.sector_sha256_sum),
                );
            }
        }
    }
    println!(
        "populate_map_info_digest_from_caches: Total time: {:?}",
        init_time.elapsed()
    );
    let owned_cache_handle_ids: Vec<(String, Option<String>)> = cache_handle_ids
        .into_iter()
        .map(|(key, val)| (key.to_owned(), val.map(|x| x.to_owned())))
        .collect();

    let mut cache_handle_ids: HashMap<String, Option<String>> = HashMap::new();
    for (handle_key, handle_val) in owned_cache_handle_ids {
        cache_handle_ids.insert(handle_key, handle_val);
    }
    cache_handle_ids
}

/// Attempts to download the replay cache from the
#[instrument]
pub async fn download_cache(
    handle: &str,
    region: &str,
    ext: &str,
    destination: &str,
) -> Result<(), S2ProtocolError> {
    let destination = Path::new(destination);
    tracing::info!("Downloading cache with handle: {}", handle);
    let cache_download_target =
        destination.join(format!("{}.{}", handle, CACHE_MPQ_ARCHIVE_EXTENSION));
    if cache_download_target.exists() {
        tracing::info!(
            "Cache {} already exists, skipping download.",
            cache_download_target.display()
        );
        return Ok(());
    }

    let url = format!(
        "https://{}-s2-depot.classic.blizzard.com/{}.{}",
        region, handle, ext
    );
    let response = reqwest::get(&url).await?;
    if !response.status().is_success() {
        return Err(S2ProtocolError::CacheResource(format!(
            "Failed to download cache {}, status code: {}",
            handle,
            response.status()
        )));
    }
    let response_bytes = response.bytes().await?;
    std::fs::write(&cache_download_target, response_bytes)?;
    Ok(())
}
