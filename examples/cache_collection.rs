use s2protocol::cache_handles::CacheCollection;
use tracing::*;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_ansi(true)
        .init();
    let path = "/home/seb/SC2Replays/swarmy/".to_string();

    let mut cache_builder = CacheCollection::new(path);
    let cache_ids = vec![
        "6de41503baccd05656360b6f027db88169fa1989bb6357b1b215a2547939f5fb".to_string(),
        "421c8aa0f3619b652d23a2735dfee812ab644228235e7a797edecfe8b67da30e".to_string(),
        "66093832128453efffbb787c80b7d3eec1ad81bde55c83c930dea79c4e505a04".to_string(),
        "d92dfc48c484c59154270b924ad7d57484f2ab9a47621c7ab16431bf66c53b40".to_string(),
        "fe419789c1acfa7ccdb5bbe2855fef5ac63cff5eab8ac29a822f680fe593d94e".to_string(),
        "6c0552724744d9826059fa49fc1ed226f5113b6e3b6c2c8063e00dbcbc637e1f".to_string(),
        "905ddf879d4bfc5d90de103aee9be5ebb1753d77397d8fd58f8faa03fec0fa43".to_string(),
        "7f41411aa597f4b46440d42a563348bf53822d2a68112f0104f9b891f6f05ae1".to_string(),
        "957f63b479ac02e0e6a22523553c94aea6883211700adc1d14453079b5cf3c18".to_string(),
    ];
    cache_builder.add_cache_ids(&cache_ids);
    for (k, vs) in &cache_builder.cache_fs {
        tracing::info!("k: {k}");
        for v in vs {
            tracing::info!("v.name: {}", v.name);
        }
    }
    tracing::info!("{:?}", cache_builder.cache_fs.keys());
    let res = cache_builder.build_map_cache(&cache_ids);
    match cache_builder.build_map_cache(&cache_ids) {
        Ok(map_cache) => tracing::info!("{:?}", map_cache.map_info),
        Err(err) => tracing::error!("{:?}", err),
    };
}
