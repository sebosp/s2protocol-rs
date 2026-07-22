//! Handling of cache_handles on replay files.
//! These are <some_id>.s2ma files that can be downloaded from blizzard,
//! containing map resources, mod information, visual resources such
//! as images used in overlays for tournament, organizers, etc.

use crate::{InitData, S2ProtocolError};

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
use nom_mpq::MPQ;
use tracing::{self, instrument};

pub const CACHE_MPQ_ARCHIVE_EXTENSION: &'static str = "s2ma";
pub const MAP_INFO_FILE_NAME: &'static str = "MapInfo";
pub const DOCUMENT_HEADER_FILE_NAME: &'static str = "DocumentHeader";
pub const T3_HEIGHT_MAP_FILE_NAME: &'static str = "t3HeightMap";
pub const T3_TERRAIN_MAP_FILE_NAME: &'static str = "t3Terrain.xml";
pub const PLACED_OBJECTS_FILE_NAME: &'static str = "Objects";

/// The SC2Replay has a list of cache files, these caches must be downloaded from the blizzard depots.
/// This struct helps loading caches from this list.
/// For now. the first entry found from the list is returned.
/// Not sure there is some logic overwritting files.
#[derive(Debug)]
pub struct CacheCollection {
    pub cache_path: String,
    pub cache_ids: Vec<String>,
    pub map_info: MapInfo,
    pub document_header: DocumentHeader,
    pub t3_height_map: T3HeightMap,
    pub t3_terrain: T3Terrain,
    pub placed_objects: PlacedObjects,
}

// A helper build to attempt to locate the different resources from the CacheCollection.
#[derive(Debug)]
pub struct CacheCollectionBuilder {
    pub cache_path: String,
    pub cache_ids: Vec<String>,
}

impl CacheCollectionBuilder {
    pub fn new(cache_path: String, cache_ids: Vec<String>) -> Self {
        Self {
            cache_path,
            cache_ids,
        }
    }

    /// Tries to get a file from an MPQ archive
    /// name.
    #[instrument(level = "debug", skip(self))]
    pub fn try_get_target_file_from_mpq(
        &self,
        cache_handle_fname: &str,
        target_file_name: &str,
    ) -> Result<Option<(MPQ, Vec<u8>)>, S2ProtocolError> {
        tracing::info!("Checking cache_handle_fname: {}", cache_handle_fname);
        let (_, cache_contents) = crate::read_mpq(&cache_handle_fname)?;
        let (_contents, mpq) = nom_mpq::parser::parse(&cache_contents)?;
        for (embedded_file_name, _file_size) in mpq.get_files(&cache_contents)? {
            if embedded_file_name == target_file_name {
                return Ok(Some((mpq, cache_contents)));
            }
        }
        Ok(None)
    }

    /// Iterates over the [`cache_ids`] stored at [`cache_path`] and tries to locate the MPQ by
    /// name.
    #[instrument(level = "debug", skip(self))]
    pub fn try_get_file_from_mpq_list(
        &self,
        target_file_name: &str,
    ) -> Result<(String, MPQ, Vec<u8>), S2ProtocolError> {
        for cache_handle_id in &self.cache_ids {
            if cache_handle_id.is_empty() {
                continue;
            }
            let cache_handle_fname = format!(
                "{}/{}.{}",
                self.cache_path, cache_handle_id, CACHE_MPQ_ARCHIVE_EXTENSION
            );
            let Ok(Some((mpq, cache_contents))) =
                self.try_get_target_file_from_mpq(&cache_handle_fname, target_file_name)
            else {
                continue;
            };
            return Ok((cache_handle_id.to_owned(), mpq, cache_contents));
        }
        Err(S2ProtocolError::CacheResource(format!(
            "Unable to locate {} in path {} with cache_ids {:?}",
            target_file_name, self.cache_path, self.cache_ids
        )))
    }

    #[instrument(level = "debug", skip(self))]
    pub fn build(self) -> Result<CacheCollection, S2ProtocolError> {
        let (cache_handle_id, mpq, cache_contents) =
            self.try_get_file_from_mpq_list(MAP_INFO_FILE_NAME)?;
        let map_info = MapInfo::from_mpq(cache_handle_id, &mpq, &cache_contents)?;
        let (cache_handle_id, mpq, cache_contents) =
            self.try_get_file_from_mpq_list(DOCUMENT_HEADER_FILE_NAME)?;
        let document_header = DocumentHeader::from_mpq(cache_handle_id, &mpq, &cache_contents)?;
        let (cache_handle_id, mpq, cache_contents) =
            self.try_get_file_from_mpq_list(T3_HEIGHT_MAP_FILE_NAME)?;
        let t3_height_map =
            T3HeightMap::from_mpq(cache_handle_id, &mpq, &cache_contents, &map_info)?;
        let (cache_handle_id, mpq, cache_contents) =
            self.try_get_file_from_mpq_list(T3_TERRAIN_MAP_FILE_NAME)?;
        let t3_terrain = T3Terrain::from_mpq(cache_handle_id, &mpq, &cache_contents)?;
        let (cache_handle_id, mpq, cache_contents) =
            self.try_get_file_from_mpq_list(PLACED_OBJECTS_FILE_NAME)?;
        let placed_objects = PlacedObjects::from_mpq(cache_handle_id, &mpq, &cache_contents)?;
        Ok(CacheCollection {
            cache_path: self.cache_path,
            cache_ids: self.cache_ids,
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
    sources: &[InitData],
    destination: String,
) -> HashMap<String, String> {
    let mut cache_handle_ids: HashMap<String, String> = HashMap::new();
    for source in sources.iter() {
        for cache_handle_str in &source.sync_lobby_state.game_description.cache_handles {
            if let Some(_) = cache_handle_ids.get(cache_handle_str) {
                continue;
            }
            match download_cache(
                cache_handle_str,
                &source.sync_lobby_state.game_description.cache_handle_region,
                &source
                    .sync_lobby_state
                    .game_description
                    .cache_handle_extension,
                &destination,
            )
            .await
            {
                Ok(handle) => handle,
                Err(err) => {
                    tracing::error!("Unable to download cache: {:?}, skipping.", err);
                    cache_handle_ids.insert(cache_handle_str.to_owned(), String::from(""));
                    continue;
                }
            };

            cache_handle_ids.insert(cache_handle_str.to_owned(), String::from(""));
        }
        let cache_builder = CacheCollectionBuilder::new(
            destination.clone(),
            source
                .sync_lobby_state
                .game_description
                .cache_handles
                .clone(),
        );
        if let Ok(cache_collecion) = cache_builder.build() {
            cache_handle_ids.insert(
                cache_collecion.map_info.cache_handle_id,
                cache_collecion.map_info.sector_sha256_sum,
            );
        } else {
            continue;
        }
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
    panic!("download_cache: URL: {}", url);
    let response = reqwest::get(url).await?;
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
