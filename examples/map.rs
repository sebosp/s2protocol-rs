use s2protocol::cache_handles::document_header::*;
use s2protocol::cache_handles::map_info::*;
use s2protocol::cache_handles::t3_height_map::*;
use s2protocol::error::S2ProtocolError;
use std::fs::File;
use std::io::Read;
use tracing::*;

/// Attempts to read the s2ma file.
pub fn read_mpq_file(path: &str) -> Result<Vec<u8>, S2ProtocolError> {
    tracing::info!("Opening file.");
    let mut f = File::open(path)?;
    tracing::info!("Reading into buffer.");
    let mut buffer: Vec<u8> = vec![];
    // read the whole file
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn try_get_t3_height_map_from_mpq(
    cache_handle_fname: &str,
) -> Result<(MapInfo, T3HeightMap, Option<DocumentHeader>), S2ProtocolError> {
    let mut document_header: Option<DocumentHeader> = None;
    let cache_contents = read_mpq_file(cache_handle_fname)?;
    // based on sc2-map-analyzer/analyser/read.cpp
    let (_input, mpq) = nom_mpq::parser::parse(&cache_contents)?;
    for (file, _file_size) in mpq.get_files(&cache_contents)? {
        if file == "DocumentHeader"
            && let Ok(docu_header) = DocumentHeader::from_mpq(&mpq, &cache_contents)
        {
            document_header = Some(docu_header);
        }
    }
    let map_info = MapInfo::from_mpq(&mpq, &cache_contents)?;
    let t3_height_map = T3HeightMap::from_mpq(&mpq, &cache_contents, &map_info)?;
    Ok((map_info, t3_height_map, document_header))
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_ansi(true)
        .init();
    let path = "/home/seb/SC2Replays/swarmy/a76deb95741e1d3d24527f0a303914824455bc9d68411fa143d23cc4edee9c27.s2ma";
    if let Ok((map, _height, _docu_header)) = try_get_t3_height_map_from_mpq(&path) {
        tracing::info!("Map Info: {map:?}");
    }
}
