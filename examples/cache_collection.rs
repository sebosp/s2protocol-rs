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
        "b1c834f48b618b17caae9d1d174625bab89b84da581d94ef6ce7f5a6e8344802".to_string(),
        "421c8aa0f3619b652d23a2735dfee812ab644228235e7a797edecfe8b67da30e".to_string(),
        "29198eca59d0f326f06c90c106348469415c08f9bd76da8413a7f9cd3bde8694".to_string(),
        "66093832128453efffbb787c80b7d3eec1ad81bde55c83c930dea79c4e505a04".to_string(),
        "37e14076c10fbbe322a2bb2c1d2fd62d1c230e94fd6e37520b968d14bc72dc7e".to_string(),
        "d92dfc48c484c59154270b924ad7d57484f2ab9a47621c7ab16431bf66c53b40".to_string(),
        "18faf973a073da237632c56e45cefd7b1ea5b793020310bdaf557415bff9010b".to_string(),
        "139e69c209051badff7c8c9cb47d19eb6f87e9889dd08239a79e2410436722ad".to_string(),
        "85bf767f9ffd66cb7b004cf4733e8f2d52e2e3d7152a05efa730ed1dcb04be01".to_string(),
        "5c37e98ae7c5666e011e23eea0ee3035755b64554d2a180c22068b6fa8cb3599".to_string(),
        "c08f7c4b9392e8e2f5fc4bcf3860977a03ede545df321086f602732baffbabb8".to_string(),
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
