use s2protocol::cache_handles::CacheCollection;
use tracing::*;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_ansi(true)
        .with_ansi_sanitization(false)
        .init();
    let path = "/home/seb/SCReplaysOnNVMe/swarmy".to_string();

    let mut cache_builder = CacheCollection::new(path);
    let cache_ids =
        vec!["6ef1d8a13dfaf718f117ff859bd430202d24e09eb5bcee1cb9716a3959f8aa66".to_string()];
    cache_builder.add_cache_ids(&cache_ids);
    let cache_ids_2 =
        vec!["d2dc0fa6e0bc390050e511a2f11f184e6c8c936871b0579231660283f30ee9e1".to_string()];
    cache_builder.add_cache_ids(&cache_ids_2);
    let cache_ids_3 =
        vec!["2c7d0cb6d1d30f7e2731c1f24c948edbf363f3756de34873d727bfb1be2b837f".to_string()];
    cache_builder.add_cache_ids(&cache_ids_3);
    let cache_ids_4 =
        vec!["9486c9499dc71914fbfd9dea497d0f51d6e4015e527cf6cfec3b17fb326459eb".to_string()];
    cache_builder.add_cache_ids(&cache_ids_4);
    let cache_ids_5 =
        vec!["300d0946f3f5bcd955b533e7acac0dd22445339b38a837efcca7ebe2d93badca".to_string()];
    cache_builder.add_cache_ids(&cache_ids_5);
    let cache_ids_6 =
        vec!["e854e11f57f133675385ea8ff4183724faf320c8ad1dc9a179b3f4bc959ea360".to_string()];
    cache_builder.add_cache_ids(&cache_ids_6);
    /*for (k, vs) in &cache_builder.cache_fs {
        tracing::info!("k: {k}");
        for v in vs {
            tracing::info!("v.name: {}", v.name);
        }
    }*/
    match cache_builder.build_map_cache(&cache_ids) {
        Ok(map_cache) => tracing::debug!("{:#?}", map_cache.document_header.name),
        Err(err) => tracing::error!("{:?}", err),
    };
    match cache_builder.build_map_cache(&cache_ids_2) {
        Ok(map_cache) => tracing::debug!("{:#?}", map_cache.document_header.name),
        Err(err) => tracing::error!("{:?}", err),
    };
    match cache_builder.build_map_cache(&cache_ids_3) {
        Ok(map_cache) => tracing::debug!("{:#?}", map_cache.document_header.name),
        Err(err) => tracing::error!("{:?}", err),
    };
    match cache_builder.build_map_cache(&cache_ids_4) {
        Ok(map_cache) => tracing::debug!("{:#?}", map_cache.document_header.name),
        Err(err) => tracing::error!("{:?}", err),
    };
    match cache_builder.build_map_cache(&cache_ids_5) {
        Ok(map_cache) => tracing::debug!("{:#?}", map_cache.document_header.name),
        Err(err) => tracing::error!("{:?}", err),
    };
    match cache_builder.build_map_cache(&cache_ids_6) {
        Ok(map_cache) => tracing::debug!("{:#?}", map_cache.document_header.name),
        Err(err) => tracing::error!("{:?}", err),
    };
}
