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
        vec!["2c7d0cb6d1d30f7e2731c1f24c948edbf363f3756de34873d727bfb1be2b837f".to_string()];
    cache_builder.add_cache_ids(&cache_ids_2);
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
}
