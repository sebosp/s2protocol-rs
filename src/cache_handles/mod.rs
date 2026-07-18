//! Handling of cache_handles on replay files.
//! These are <some_id>.s2ma files that can be downloaded from blizzard,
//! containing map resources, mod information, visual resources such
//! as images used in overlays for tournament, organizers, etc.

use crate::S2ProtocolError;

pub mod cache_objects;
pub mod document_header;
pub mod map;
pub mod map_info;
pub mod t3_height_map;
pub mod t3_terrain;

use cache_objects::PlacedObjects;
use document_header::DocumentHeader;
use map_info::MapInfo;
use std::path::Path;
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
pub const OBJECTS_FILE_NAME: &'static str = "Objects";

/// The SC2Replay has a list of cache files, these caches must be downloaded from the blizzard depots.
/// This struct helps loading caches from this list.
/// For now. the first entry found from the list is returned.
/// Not sure there is some logic overwritting files.
#[derive(Debug)]
pub struct CacheCollection {
    pub cache_path: String,
    pub cache_ids: String,
}

impl CacheCollection {
    pub fn new(cache_path: String, cache_ids: String) -> Self {
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
    ) -> Result<Option<(MPQ, Vec<u8>)>, S2ProtocolError> {
        for cache_handle_id in self.cache_ids.split(",") {
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
            return Ok(Some((mpq, cache_contents)));
        }
        Ok(None)
    }

    #[instrument(level = "debug", skip(self))]
    pub fn load_map_info(&self) -> Result<MapInfo, S2ProtocolError> {
        if let Ok(Some((mpq, cache_contents))) = self.try_get_file_from_mpq_list(MAP_INFO_FILE_NAME)
        {
            MapInfo::from_mpq(&mpq, &cache_contents)
        } else {
            Err(S2ProtocolError::CacheResource(format!(
                "Unable to locate {} in path {} with cache_ids {}",
                MAP_INFO_FILE_NAME, self.cache_path, self.cache_ids
            )))
        }
    }

    #[instrument(level = "debug", skip(self))]
    pub fn load_document_header(&self) -> Result<DocumentHeader, S2ProtocolError> {
        if let Ok(Some((mpq, cache_contents))) =
            self.try_get_file_from_mpq_list(DOCUMENT_HEADER_FILE_NAME)
        {
            DocumentHeader::from_mpq(&mpq, &cache_contents)
        } else {
            Err(S2ProtocolError::CacheResource(format!(
                "Unable to locate {} in path {} with cache_ids {}",
                DOCUMENT_HEADER_FILE_NAME, self.cache_path, self.cache_ids
            )))
        }
    }

    /// Tries to  locate the [`T3_HEIGHT_MAP_FILE_NAME`] from the configured caches.
    /// Calculating the boundaries of the T3HeightMap requires getting the map information as well.
    #[instrument(level = "debug", skip(self))]
    pub fn load_t3_height_map(&self) -> Result<T3HeightMap, S2ProtocolError> {
        let map_info = self.load_map_info()?;
        if let Ok(Some((mpq, cache_contents))) =
            self.try_get_file_from_mpq_list(T3_HEIGHT_MAP_FILE_NAME)
        {
            T3HeightMap::from_mpq(&mpq, &cache_contents, &map_info)
        } else {
            Err(S2ProtocolError::CacheResource(format!(
                "Unable to locate {} in path {} with cache_ids {}",
                T3_HEIGHT_MAP_FILE_NAME, self.cache_path, self.cache_ids
            )))
        }
    }

    pub fn load_t3_terrain(&self) -> Result<T3Terrain, S2ProtocolError> {
        if let Ok(Some((mpq, cache_contents))) =
            self.try_get_file_from_mpq_list(T3_TERRAIN_MAP_FILE_NAME)
        {
            T3Terrain::from_mpq(&mpq, &cache_contents)
        } else {
            Err(S2ProtocolError::CacheResource(format!(
                "Unable to locate {} in path {} with cache_ids {}",
                T3_TERRAIN_MAP_FILE_NAME, self.cache_path, self.cache_ids
            )))
        }
    }

    pub fn load_objects(&self) -> Result<PlacedObjects, S2ProtocolError> {
        if let Ok(Some((mpq, cache_contents))) = self.try_get_file_from_mpq_list(OBJECTS_FILE_NAME)
        {
            PlacedObjects::from_mpq(&mpq, &cache_contents)
        } else {
            Err(S2ProtocolError::CacheResource(format!(
                "Unable to locate {} in path {} with cache_ids {}",
                OBJECTS_FILE_NAME, self.cache_path, self.cache_ids
            )))
        }
    }
}

/// Attempts to download the replay cache from the
#[instrument]
pub async fn download_cache(handle: &str, destination: &Path) -> Result<(), S2ProtocolError> {
    tracing::info!("Downloading cache with handle: {}", handle);
    let cache_download_target = destination.join(format!("{}.s2ma", handle));
    if cache_download_target.exists() {
        tracing::info!(
            "Cache {} already exists, skipping download.",
            cache_download_target.display()
        );
        return Ok(());
    }

    let response = reqwest::get(format!(
        "https://eu-s2-depot.classic.blizzard.com/{}.s2ma",
        handle
    ))
    .await?;
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
