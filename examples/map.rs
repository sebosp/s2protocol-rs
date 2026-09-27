use std::path::PathBuf;

use s2protocol::cache_handles::document_header::*;
use s2protocol::cache_handles::map_info::*;
use s2protocol::cache_handles::t3_height_map::*;
use s2protocol::error::S2ProtocolError;
use tracing::*;

fn try_get_t3_height_map_from_mpq(
    cache_handle_fname: &str,
) -> Result<(MapInfo, T3HeightMap, Option<DocumentHeader>), S2ProtocolError> {
    let cache_id: String = format!(
        "{}",
        PathBuf::from(cache_handle_fname)
            .file_prefix()
            .unwrap_or_default()
            .display()
    );
    let mut document_header: Option<DocumentHeader> = None;
    let (_, cache_contents) = s2protocol::read_mpq(cache_handle_fname)?;
    // based on sc2-map-analyzer/analyser/read.cpp
    let (_input, mpq) = nom_mpq::parser::parse(&cache_contents)?;
    for (file, _file_size) in mpq.get_files(&cache_contents)? {
        if file == "DocumentHeader"
            && let Ok(docu_header) =
                DocumentHeader::from_mpq(cache_id.clone(), &mpq, &cache_contents)
        {
            document_header = Some(docu_header);
        }
    }
    let map_info = MapInfo::from_mpq(cache_id.clone(), &mpq, &cache_contents)?;
    let t3_height_map = T3HeightMap::from_mpq(cache_id.clone(), &mpq, &cache_contents, &map_info)?;
    Ok((map_info, t3_height_map, document_header))
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_ansi(true)
        .init();
    let path = "/home/seb/SC2Replays/swarmy/e854e11f57f133675385ea8ff4183724faf320c8ad1dc9a179b3f4bc959ea360.s2ma";
    match try_get_t3_height_map_from_mpq(&path) {
        Ok((map, _height, _docu_header)) => tracing::info!("Map Info: {map:?}"),
        Err(err) => tracing::error!("Error: {:?}", err),
    }
    let path = "/home/seb/SC2Replays/swarmy/a76deb95741e1d3d24527f0a303914824455bc9d68411fa143d23cc4edee9c27.s2ma";
    match try_get_t3_height_map_from_mpq(&path) {
        Ok((map, _height, _docu_header)) => tracing::info!("Map Info: {map:?}"),
        Err(err) => tracing::error!("Error: {:?}", err),
    }
    let path = "/home/seb/SC2Replays/swarmy/957f63b479ac02e0e6a22523553c94aea6883211700adc1d14453079b5cf3c18.s2ma";
    match try_get_t3_height_map_from_mpq(&path) {
        Ok((map, _height, _docu_header)) => tracing::info!("Map Info: {map:?}"),
        Err(err) => tracing::error!("Error: {:?}", err),
    }
}
