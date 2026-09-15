use s2protocol::cache_handles::CacheCollection;
use tracing::*;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_ansi(true)
        .init();
    let path = "/home/seb/SCReplaysOnNVMe/swarmy".to_string();

    let mut cache_builder = CacheCollection::new(path);
    let cache_ids = vec![
        "6de41503baccd05656360b6f027db88169fa1989bb6357b1b215a2547939f5fb".to_string(),
        "421c8aa0f3619b652d23a2735dfee812ab644228235e7a797edecfe8b67da30e".to_string(),
        "66093832128453efffbb787c80b7d3eec1ad81bde55c83c930dea79c4e505a04".to_string(),
        "d92dfc48c484c59154270b924ad7d57484f2ab9a47621c7ab16431bf66c53b40".to_string(),
        "9e4eb0b5828dd241e0303e1c8bb11a1dac116e44e731d57e44d4406ae968b7c6".to_string(),
        "6c0552724744d9826059fa49fc1ed226f5113b6e3b6c2c8063e00dbcbc637e1f".to_string(),
        "045488e38c9f850143e0bb5703c08efcc26c8f7997438365a248dfcfb4d1a165".to_string(),
        "7f41411aa597f4b46440d42a563348bf53822d2a68112f0104f9b891f6f05ae1".to_string(),
        "c2bcc137d9ce43fbf9a5ddd04f5167892c779c29616de980bf7e92f31cdc1742".to_string(),
    ];
    cache_builder.add_cache_ids(&cache_ids);
    for (k, vs) in &cache_builder.cache_fs {
        tracing::info!("k: {k}");
        for v in vs {
            tracing::info!("v.name: {}", v.name);
        }
    }
    tracing::info!("{:?}", cache_builder.cache_fs.keys());
    match cache_builder.build_map_cache(&cache_ids) {
        Ok(map_cache) => tracing::info!("{:?}", map_cache.map_info),
        Err(err) => tracing::error!("{:?}", err),
    };
}
