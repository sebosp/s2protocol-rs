//! Handling of cache_handles on replay files.
//! These are <some_id>.s2ma files that can be downloaded from blizzard,
//! containing map resources, mod information, visual resources such
//! as images used in overlays for tournament, organizers, etc.

use crate::{MAX_ERROR_CONTEXT_CHARS, S2ProtocolError, basic_replay_data::SC2ReplayBasicData};

pub mod cache_objects;
pub mod document_header;
pub mod map;
pub mod map_info;
pub mod t3_height_map;
pub mod t3_terrain;

use cache_objects::PlacedObjects;
use document_header::DocumentHeader;
use futures::future::join_all;
use map_info::MapInfo;
use rayon::prelude::*;
use std::{collections::HashMap, path::Path};
use t3_height_map::T3HeightMap;
use t3_terrain::T3Terrain;

pub use map::*;
use tracing::{self, instrument};

pub const CACHE_MPQ_ARCHIVE_EXTENSION: &str = "s2ma";
pub const MAP_INFO_FILE_NAME: &str = "MapInfo";
pub const DOCUMENT_HEADER_FILE_NAME: &str = "DocumentHeader";
pub const T3_HEIGHT_MAP_FILE_NAME: &str = "t3HeightMap";
pub const T3_TERRAIN_MAP_FILE_NAME: &str = "t3Terrain.xml";
pub const PLACED_OBJECTS_FILE_NAME: &str = "Objects";

pub type CacheIdWithMapInfoSha = HashMap<String, Option<String>>;

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

fn is_file_known(fname: &str) -> bool {
    match fname.as_ref() {
        MAP_INFO_FILE_NAME
        | DOCUMENT_HEADER_FILE_NAME
        | T3_HEIGHT_MAP_FILE_NAME
        | T3_TERRAIN_MAP_FILE_NAME
        | PLACED_OBJECTS_FILE_NAME => true,
        _ => false,
    }
}

fn try_read_embedded_sectors_with_contents(
    cache_handle_fname: &str,
) -> Result<Vec<MPQNamedSector>, S2ProtocolError> {
    let (_, cache_contents) = crate::read_mpq(cache_handle_fname)?;
    let (_contents, mpq) = nom_mpq::parser::parse(&cache_contents)?;
    let mut embedded_files: Vec<MPQNamedSector> = vec![];
    if let Ok(mpq_embedded_files) = mpq.get_files(&cache_contents) {
        for (mpq_embedded_file_name, _file_size) in mpq_embedded_files {
            if !is_file_known(&mpq_embedded_file_name) {
                continue;
            }
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
    #[instrument(level = "debug", skip(self, cache_ids))]
    pub fn add_cache_ids(&mut self, cache_ids: &[String]) {
        let cache_ids = cache_ids.to_owned();
        let cache_id_with_named_sectors = cache_ids
            .into_iter()
            .filter(|k| !self.cache_fs.contains_key(k))
            .collect::<Vec<String>>()
            .par_iter()
            .map(|cache_id| {
                let cache_handle_fname = format!(
                    "{}/{}.{}",
                    self.cache_path, cache_id, CACHE_MPQ_ARCHIVE_EXTENSION
                );
                (
                    cache_id.to_string(),
                    match try_read_embedded_sectors_with_contents(&cache_handle_fname) {
                        Ok(val) => val,
                        Err(err) => {
                            tracing::error!(
                                "failed to read mpq contents on {1} {2:.0$}",
                                MAX_ERROR_CONTEXT_CHARS,
                                cache_handle_fname,
                                format!("{:?}", err)
                            );
                            vec![]
                        }
                    },
                )
            })
            .collect::<Vec<(String, Vec<MPQNamedSector>)>>();
        for (cache_id, named_sector) in cache_id_with_named_sectors {
            self.cache_fs.insert(cache_id, named_sector);
        }
    }

    /// Iterates over the [`cache_ids`] stored at [`cache_path`] and returns the byte content if any.
    #[instrument(level = "debug", skip(self))]
    pub fn try_get_sector_content_from_collection(
        &self,
        cache_ids: &[&str],
        target_file_name: &str,
    ) -> Result<(String, Vec<u8>), S2ProtocolError> {
        for cache_id in cache_ids {
            if let Some(cache_contents) = self.cache_fs.get(*cache_id) {
                for mpq_file_with_digest in cache_contents {
                    if mpq_file_with_digest.name == target_file_name {
                        return Ok((cache_id.to_string(), mpq_file_with_digest.content.clone()));
                    }
                }
            }
        }
        Err(S2ProtocolError::CacheResource {
            name: target_file_name.to_string(),
            cache_handles: cache_ids
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        })
    }

    #[instrument(level = "debug", skip(self))]
    pub fn build_map_cache(&self, cache_ids: &[String]) -> Result<MapCache, S2ProtocolError> {
        let cache_ids = cache_ids.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
        // Each MPQ seems to potentially contain a DocumentHeader. Maybe the MapInfo/T3HeightMap/DocumentHeader
        // are must be in the same MPQ.
        let (map_cache_handle_id, cache_contents) =
            self.try_get_sector_content_from_collection(&cache_ids, MAP_INFO_FILE_NAME)?;
        let (_, map_info) = MapInfo::parse(map_cache_handle_id.clone(), &cache_contents)?;

        let map_info_only_cache_id = vec![map_cache_handle_id.as_str()];
        let (_, cache_contents) = self.try_get_sector_content_from_collection(
            &map_info_only_cache_id,
            DOCUMENT_HEADER_FILE_NAME,
        )?;
        let (_, document_header) =
            DocumentHeader::parse(map_cache_handle_id.clone(), &cache_contents)?;

        let (_, cache_contents) = self.try_get_sector_content_from_collection(
            &map_info_only_cache_id,
            T3_HEIGHT_MAP_FILE_NAME,
        )?;
        let (_, t3_height_map) =
            T3HeightMap::parse(map_cache_handle_id.clone(), &cache_contents, &map_info)?;

        let (_, cache_contents) = self.try_get_sector_content_from_collection(
            &map_info_only_cache_id,
            T3_TERRAIN_MAP_FILE_NAME,
        )?;
        let t3_terrain = T3Terrain::parse(map_cache_handle_id.clone(), &cache_contents)?;

        let (_, cache_contents) = self.try_get_sector_content_from_collection(
            &map_info_only_cache_id,
            PLACED_OBJECTS_FILE_NAME,
        )?;
        let placed_objects = PlacedObjects::parse(map_cache_handle_id, &cache_contents)?;
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
) -> CacheIdWithMapInfoSha {
    let init_time = std::time::Instant::now();
    let mut cache_handle_ids: CacheIdWithMapInfoSha = HashMap::new();
    let mut unique_cache_handles = sources
        .iter()
        .flat_map(|source| {
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
        .collect::<Vec<_>>();
    unique_cache_handles.sort_unstable();
    unique_cache_handles.dedup();
    let downloaded_cache_count: usize = join_all(unique_cache_handles.iter().map(
        |(cache_handle_region, cache_handle_extension, cache_handle_str)| {
            download_cache(
                cache_handle_str,
                cache_handle_region,
                cache_handle_extension,
                &destination,
            )
        },
    ))
    .await
    .iter()
    .map(|status| match status {
        Ok(()) => 1,
        Err(err) => {
            println!("{}", err);
            0
        }
    })
    .collect::<Vec<usize>>()
    .iter()
    .sum();
    let mut cache_builder = CacheCollection::new(destination.clone());
    for source in sources.iter() {
        cache_builder.add_cache_ids(
            &source
                .init_data
                .sync_lobby_state
                .game_description
                .cache_handles,
        );
    }
    tracing::info!(
        "- init_1 After {} downloads/checks: {:?}",
        downloaded_cache_count,
        init_time.elapsed(),
    );

    for source in sources.iter() {
        let cache_handles = &source
            .init_data
            .sync_lobby_state
            .game_description
            .cache_handles;
        let has_map_info: bool = cache_handles
            .iter()
            .any(|cache_id| cache_handle_ids.contains_key(cache_id));
        if has_map_info {
            continue;
        }
        match cache_builder.build_map_cache(cache_handles) {
            Ok(map_cache) => {
                let _ = cache_handle_ids.insert(
                    map_cache.map_info.cache_handle_id,
                    Some(map_cache.map_info.sector_sha256_sum),
                );
            }
            Err(err) => println!("Error building map cache: {:?}", err),
        }
    }
    println!(
        "populate_map_info_digest_from_caches: Total time: {:?}",
        init_time.elapsed()
    );
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
    println!("Downloading cache with handle: {}", handle);
    let cache_download_target =
        destination.join(format!("{}.{}", handle, CACHE_MPQ_ARCHIVE_EXTENSION));
    println!(
        "Downloading cache to destination: {:?}",
        cache_download_target
    );
    if cache_download_target.exists() {
        tracing::info!(
            "Cache {} already exists, skipping download.",
            cache_download_target.display()
        );
        return Ok(());
    }
    let mut region = region.to_string();
    if region.to_lowercase() == "cn" {
        // I can't resolve  cn-s2-depot.classic.blizzard.com
        region = String::from("eu");
    }

    let url = format!(
        "https://{}-s2-depot.classic.blizzard.com/{}.{}",
        region, handle, ext
    );
    let response = reqwest::get(&url).await?;
    if !response.status().is_success() {
        return Err(S2ProtocolError::CacheHandleDownload(format!(
            "Failed to download cache {}, status code: {}",
            handle,
            response.status()
        )));
    }
    let response_bytes = response.bytes().await?;
    std::fs::write(&cache_download_target, response_bytes)?;
    Ok(())
}
