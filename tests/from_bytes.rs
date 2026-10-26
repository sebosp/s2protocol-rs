//! The replays can be read from their contents, without a file system, i.e. in a browser.

use s2protocol::SC2EventIterator;
use s2protocol::basic_replay_data::SC2ReplayBasicData;
use s2protocol::game_events::read_balance_data_from_included_assets;

#[test]
fn it_reads_the_events_from_the_replay_contents() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/FieldsDeath202507.SC2Replay"
    );
    let contents = std::fs::read(path).unwrap();
    let basic = SC2ReplayBasicData::from_bytes("dropped.SC2Replay", 0, &contents).unwrap();
    assert_eq!(basic.init_data.ext_fs_file_name, "dropped.SC2Replay");
    let balance = read_balance_data_from_included_assets().unwrap();
    let from_bytes = SC2EventIterator::from_bytes(&basic, &contents, &balance)
        .unwrap()
        .count();
    let from_path =
        SC2EventIterator::new(&SC2ReplayBasicData::new(&path.into(), 0).unwrap(), &balance)
            .unwrap()
            .count();
    assert!(from_bytes > 0);
    assert_eq!(from_bytes, from_path);
}
