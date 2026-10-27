//! Writes a snapshot of a directory of replays, to compare the snapshots of two versions:
//! SNAPSHOT_SOURCE=replays SNAPSHOT_OUTPUT=ipcs SNAPSHOT_CACHES=caches \
//!   cargo test --features dep_arrow --test write_snapshot -- --ignored
#![cfg(feature = "dep_arrow")]

use s2protocol::game_events::read_balance_data_from_included_assets;
use s2protocol::{ArrowIpcTypes, WriteArrowIpcProps};
use std::path::PathBuf;

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn it_writes_a_snapshot() {
    let source = PathBuf::from(std::env::var("SNAPSHOT_SOURCE").unwrap());
    let output = PathBuf::from(std::env::var("SNAPSHOT_OUTPUT").unwrap());
    let caches = std::env::var("SNAPSHOT_CACHES").unwrap();
    std::fs::create_dir_all(&output).unwrap();
    let props = WriteArrowIpcProps {
        scan_max_files: 1000,
        process_max_files: 1000,
        traverse_max_depth: 2,
        min_version: None,
        max_version: None,
        recreate: true,
    };
    let balance = read_balance_data_from_included_assets().unwrap();
    ArrowIpcTypes::handle_arrow_ipc_cmd_with_progress(
        source, output, &props, &balance, false, caches, None,
    )
    .await
    .unwrap();
}

/// Compares the record batches of each file of two snapshots:
/// SNAPSHOT_OUTPUT=ipcs SNAPSHOT_EXPECTED=other_ipcs cargo test ... -- --ignored
#[test]
#[ignore]
fn it_matches_another_snapshot() {
    use arrow::ipc::reader::FileReader;
    let output = PathBuf::from(std::env::var("SNAPSHOT_OUTPUT").unwrap());
    let expected = PathBuf::from(std::env::var("SNAPSHOT_EXPECTED").unwrap());
    let read = |path: &std::path::Path| {
        let reader = FileReader::try_new(std::fs::File::open(path).unwrap(), None).unwrap();
        let schema = reader.schema();
        let batches: Vec<_> = reader.map(Result::unwrap).collect();
        arrow::compute::concat_batches(&schema, &batches).unwrap()
    };
    fn arrow_files(dir: &std::path::Path, files: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                arrow_files(&path, files);
            } else if path.extension().is_some_and(|ext| ext == "arrow") {
                files.push(path);
            }
        }
    }
    let mut files = vec![];
    arrow_files(&expected, &mut files);
    let mut compared = 0;
    for expected_file in files {
        let relative = expected_file.strip_prefix(&expected).unwrap();
        let expected_batch = read(&expected_file);
        let actual_batch = read(&output.join(relative));
        assert_eq!(
            expected_batch.schema(),
            actual_batch.schema(),
            "{relative:?}"
        );
        // The replays are read in parallel, the rows are compared in any order.
        let sorted_rows = |batch: &arrow::record_batch::RecordBatch| {
            let converter = arrow::row::RowConverter::new(
                batch
                    .schema()
                    .fields()
                    .iter()
                    .map(|field| arrow::row::SortField::new(field.data_type().clone()))
                    .collect(),
            )
            .unwrap();
            let rows = converter.convert_columns(batch.columns()).unwrap();
            let mut rows: Vec<Vec<u8>> = rows.iter().map(|row| row.as_ref().to_vec()).collect();
            rows.sort();
            rows
        };
        assert_eq!(
            expected_batch.num_rows(),
            actual_batch.num_rows(),
            "{relative:?}"
        );
        assert!(
            sorted_rows(&expected_batch) == sorted_rows(&actual_batch),
            "{relative:?}"
        );
        compared += 1;
    }
    println!("{compared} files are the same");
    assert!(compared > 0);
}
