//! The schemas of the snapshot files, the snapshots written by previous versions must stay
//! readable with the same queries.
#![cfg(feature = "dep_arrow")]

use s2protocol::details::PlayerLobbyDetailsFlatRow;
use s2protocol::game_events::{CmdTargetPointEventFlatRow, CmdTargetUnitEventFlatRow};
use s2protocol::init_data::UserInitDataFlatRow;
use s2protocol::tracker_events::{
    PlayerStatsFlatRow, UnitBornEventFlatRow, UnitDiedEventFlatRow, UpgradeEventFlatRow,
};

fn describe(schema: arrow::datatypes::Schema) -> String {
    schema
        .fields()
        .iter()
        .map(|field| {
            format!(
                "{} {:?} {}",
                field.name(),
                field.data_type(),
                field.is_nullable()
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn it_keeps_the_snapshot_schemas() {
    let schemas = [
        ("details", describe(PlayerLobbyDetailsFlatRow::schema())),
        ("init_data", describe(UserInitDataFlatRow::schema())),
        ("stats", describe(PlayerStatsFlatRow::schema())),
        ("upgrades", describe(UpgradeEventFlatRow::schema())),
        ("unit_born", describe(UnitBornEventFlatRow::schema())),
        ("unit_died", describe(UnitDiedEventFlatRow::schema())),
        (
            "cmd_target_point",
            describe(CmdTargetPointEventFlatRow::schema()),
        ),
        (
            "cmd_target_unit",
            describe(CmdTargetUnitEventFlatRow::schema()),
        ),
    ];
    let actual = schemas
        .iter()
        .map(|(name, schema)| format!("## {name}\n{schema}\n"))
        .collect::<String>();
    let expected_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/arrow_schemas.txt");
    if std::env::var("UPDATE_SCHEMAS").is_ok() {
        std::fs::write(expected_path, &actual).unwrap();
    }
    let expected = std::fs::read_to_string(expected_path).unwrap();
    assert_eq!(expected, actual);
}
