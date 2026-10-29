//! Handling of cache_handles on replay files.
//! These are <some_id>.s2ma files that can be downloaded from blizzard,
//! containing map resources, mod information, visual resources such
//! as images used in overlays for tournament, organizers, etc.

use crate::{MAX_ERROR_CONTEXT_CHARS, S2ProtocolError, basic_replay_data::SC2ReplayBasicData};

pub mod cache_objects;
pub mod document_header;
pub mod map;
pub mod map_info;
pub mod minimap;
pub mod t3_height_map;
pub mod t3_terrain;

use cache_objects::PlacedObjects;
use document_header::DocumentHeader;
use map_info::MapInfo;
use minimap::Minimap;
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
pub const MINIMAP_FILE_NAME: &str = "Minimap.tga";

pub type CacheIdWithMapInfoSha = HashMap<String, Option<String>>;

/// The MapCaches are specific to one SC2Replay File
#[derive(Debug)]
pub struct MapCache {
    pub map_info: MapInfo,
    pub document_header: DocumentHeader,
    pub t3_height_map: T3HeightMap,
    pub t3_terrain: T3Terrain,
    pub placed_objects: PlacedObjects,
    /// The minimap image, None if the map cache does not have it or it cannot be decoded.
    pub minimap: Option<Minimap>,
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
        | PLACED_OBJECTS_FILE_NAME
        | MINIMAP_FILE_NAME => true,
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
        let placed_objects = PlacedObjects::parse(map_cache_handle_id.clone(), &cache_contents)?;

        let minimap = self
            .try_get_sector_content_from_collection(&map_info_only_cache_id, MINIMAP_FILE_NAME)
            .and_then(|(_, cache_contents)| Minimap::parse(map_cache_handle_id, &cache_contents))
            .inspect_err(|err| tracing::warn!("No minimap: {err:?}"))
            .ok();
        Ok(MapCache {
            map_info,
            document_header,
            t3_height_map,
            t3_terrain,
            placed_objects,
            minimap,
        })
    }
}

/// Attempts to download the replay cache from theblizzard depots.
/// The caches are needed to identify a unique map version.
/// There's a version to a map, but I haven't identified it yet from the decoded data.
// The sources are counted, their debug output is every replay of the snapshot, repeated in every
// event of the span.
#[instrument(skip(sources), fields(sources = sources.len()))]
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
    let unique_cache_handles: Vec<CacheHandle> = unique_cache_handles
        .into_iter()
        .map(|(region, extension, handle)| CacheHandle::new(region, extension, handle))
        .collect();
    let downloads = download_caches(&unique_cache_handles, &destination).await;
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
    tracing::info!("Map caches after {:?}: {downloads}", init_time.elapsed(),);

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
            Err(err) => tracing::warn!("Error building map cache: {:?}", err),
        }
    }
    tracing::info!(
        "populate_map_info_digest_from_caches: Total time: {:?}",
        init_time.elapsed()
    );
    cache_handle_ids
}

/// The downloads at a time from the depot servers.
pub const PARALLEL_CACHE_DOWNLOADS: usize = 4;

/// The errors kept in [`CacheDownloads::errors`], all of them are logged.
const MAX_CACHE_DOWNLOAD_ERRORS: usize = 10;

/// A map cache of a replay: the depot region, the extension and the handle, the sha256 of the
/// file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CacheHandle {
    pub region: String,
    pub extension: String,
    pub handle: String,
}

impl CacheHandle {
    pub fn new(region: &str, extension: &str, handle: &str) -> Self {
        Self {
            region: region.to_string(),
            extension: extension.to_string(),
            handle: handle.to_string(),
        }
    }

    /// The file of the cache in `destination`.
    pub fn path(&self, destination: &Path) -> std::path::PathBuf {
        destination.join(format!("{}.{}", self.handle, CACHE_MPQ_ARCHIVE_EXTENSION))
    }

    /// The URL of the cache in Blizzard's depot servers.
    pub fn url(&self) -> String {
        // cn-s2-depot.classic.blizzard.com does not resolve, the caches are the same in every
        // region.
        let region = if self.region.eq_ignore_ascii_case("cn") {
            "eu"
        } else {
            self.region.as_str()
        };
        format!(
            "https://{}-s2-depot.classic.blizzard.com/{}.{}",
            region, self.handle, self.extension
        )
    }
}

/// What [`download_caches`] did, the errors are returned instead of only logged so that a client
/// without a console, i.e. a desktop app, can show them.
#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CacheDownloads {
    /// The caches asked for.
    pub handles: usize,
    /// The caches that were already in the destination.
    pub present: usize,
    pub downloaded: usize,
    pub failed: usize,
    /// The first errors, i.e. the destination is not writable or the depot is not reachable.
    pub errors: Vec<String>,
}

impl std::fmt::Display for CacheDownloads {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} caches, {} present, {} downloaded, {} failed",
            self.handles, self.present, self.downloaded, self.failed
        )?;
        if let Some(error) = self.errors.first() {
            write!(f, ", i.e. {error}")?;
        }
        Ok(())
    }
}

/// Downloads the caches that are not in `destination` yet, [`PARALLEL_CACHE_DOWNLOADS`] at a
/// time. The destination is created when it does not exist.
#[instrument(skip(handles), fields(handles = handles.len()))]
pub async fn download_caches(handles: &[CacheHandle], destination: &str) -> CacheDownloads {
    use futures::stream::StreamExt;
    let mut downloads = CacheDownloads {
        handles: handles.len(),
        ..Default::default()
    };
    let destination = Path::new(destination);
    let missing: Vec<&CacheHandle> = handles
        .iter()
        .filter(|handle| !handle.path(destination).exists())
        .collect();
    downloads.present = handles.len() - missing.len();
    if missing.is_empty() {
        return downloads;
    }
    if let Err(err) = std::fs::create_dir_all(destination) {
        let error = format!(
            "Unable to create the caches directory {}: {err}",
            destination.display()
        );
        tracing::error!("{error}");
        downloads.failed = missing.len();
        downloads.errors.push(error);
        return downloads;
    }
    let client = CacheClient::default();
    let mut results = futures::stream::iter(missing)
        .map(|handle| {
            let client = &client;
            async move {
                client
                    .download(handle, destination)
                    .await
                    .map_err(|err| format!("{}: {err}", handle.url()))
            }
        })
        .buffer_unordered(PARALLEL_CACHE_DOWNLOADS);
    while let Some(result) = results.next().await {
        match result {
            Ok(()) => downloads.downloaded += 1,
            Err(err) => {
                tracing::warn!("Unable to download the map cache {err}");
                downloads.failed += 1;
                if downloads.errors.len() < MAX_CACHE_DOWNLOAD_ERRORS {
                    downloads.errors.push(err);
                }
            }
        }
    }
    downloads
}

/// The HTTP client of the downloads, shared so that the connections to the depot are reused.
#[derive(Default)]
struct CacheClient {
    #[cfg(feature = "dep_reqwest")]
    client: reqwest::Client,
}

impl CacheClient {
    /// Downloads the cache into `destination`, written to a temporary file first so that an
    /// interrupted download does not leave a broken cache behind.
    async fn download(
        &self,
        handle: &CacheHandle,
        destination: &Path,
    ) -> Result<(), S2ProtocolError> {
        let url = handle.url();
        #[cfg(not(feature = "dep_reqwest"))]
        {
            let _ = destination;
            return Err(S2ProtocolError::CacheHandleDownload(format!(
                "Will not download {url} because feature dep_reqwest of s2protocol-rs is not enabled."
            )));
        }
        #[cfg(feature = "dep_reqwest")]
        {
            let response = self.client.get(&url).send().await?;
            if !response.status().is_success() {
                return Err(S2ProtocolError::CacheHandleDownload(format!(
                    "Failed to download cache {}, status code: {}",
                    handle.handle,
                    response.status()
                )));
            }
            let response_bytes = response.bytes().await?;
            let target = handle.path(destination);
            let mut tmp_name = target.as_os_str().to_owned();
            tmp_name.push(".part");
            let tmp = std::path::PathBuf::from(tmp_name);
            std::fs::write(&tmp, response_bytes)?;
            std::fs::rename(&tmp, &target)?;
            Ok(())
        }
    }
}

/// Downloads one cache into `destination` unless it is there already, see [`download_caches`]
/// for many.
#[instrument]
pub async fn download_cache(
    handle: &str,
    region: &str,
    ext: &str,
    destination: &str,
) -> Result<(), S2ProtocolError> {
    let downloads = download_caches(&[CacheHandle::new(region, ext, handle)], destination).await;
    match downloads.errors.into_iter().next() {
        Some(error) => Err(S2ProtocolError::CacheHandleDownload(error)),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_the_depot_urls() {
        let handle = CacheHandle::new("CN", "s2ma", "abc");
        assert_eq!(
            handle.url(),
            "https://eu-s2-depot.classic.blizzard.com/abc.s2ma"
        );
        let handle = CacheHandle::new("us", "s2ma", "abc");
        assert_eq!(
            handle.url(),
            "https://us-s2-depot.classic.blizzard.com/abc.s2ma"
        );
        assert_eq!(
            handle.path(Path::new("caches")),
            Path::new("caches/abc.s2ma")
        );
    }

    #[test]
    fn it_reports_an_unwritable_destination() {
        let file =
            std::env::temp_dir().join(format!("s2protocol-not-a-dir-{}", std::process::id()));
        std::fs::write(&file, b"").unwrap();
        let destination = file.join("caches");
        let downloads = futures::executor::block_on(download_caches(
            &[CacheHandle::new("eu", "s2ma", "abc")],
            destination.to_str().unwrap(),
        ));
        std::fs::remove_file(&file).unwrap();
        assert_eq!(downloads.failed, 1);
        assert_eq!(downloads.downloaded, 0);
        assert!(downloads.errors[0].starts_with("Unable to create the caches directory"));
    }
}
