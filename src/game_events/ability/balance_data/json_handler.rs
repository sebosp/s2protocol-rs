//! Writes a snapshot of the BalanceData read from XML into JSON format for posterity.

use include_dir::{Dir, include_dir};
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use tracing;

use crate::S2ProtocolError;
use crate::game_events::{MultiVersionedBalanceUnits, VersionedBalanceUnit, VersionedBalanceUnits};

/// Writes the BalanceData to a JSON file at the specified path.
pub fn write_balance_data_to_json<P: AsRef<Path>>(
    root_path: P,
    balance_data: MultiVersionedBalanceUnits,
) -> Result<(), Box<dyn Error>> {
    // The balance_data contains a version and a unit name.
    // For each version we must ensure the path exists within the root_path

    tracing::info!(
        "Writing balance data to JSON files in {}",
        root_path.as_ref().display()
    );
    for (version, single_proto_balance_data) in &balance_data {
        for (unit_name, versioned_balance_unit) in single_proto_balance_data {
            let unit_path = root_path
                .as_ref()
                .join(format!("{}/{}.json", version, unit_name));
            // Ensure the directory exists
            if let Some(parent) = unit_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let json_data = json!(versioned_balance_unit);

            // Open the file for writing
            let file = File::create(unit_path)?;

            // Write the JSON data to the file
            serde_json::to_writer_pretty(file, &json_data)?;
        }
    }
    Ok(())
}

/// Reads the BalanceData from a JSON file at the specified path.
pub fn read_balance_data_from_json_dir<P: AsRef<Path>>(
    root_path: P,
) -> Result<MultiVersionedBalanceUnits, Box<dyn Error>> {
    // Each subdirectory in the root_path corresponds to a version.
    // Each file in the subdirectory corresponds to a unit name.

    tracing::info!(
        "Reading balance data from JSON files in {}",
        root_path.as_ref().display()
    );
    let mut balance_data: MultiVersionedBalanceUnits = HashMap::new();

    for sub_dir in std::fs::read_dir(root_path)? {
        let sub_dir = sub_dir?;
        if !sub_dir.file_type()?.is_dir() {
            continue;
        }
        let version: u32 = match sub_dir.file_name().to_string_lossy().to_string().parse() {
            Ok(v) => v,
            Err(_) => {
                tracing::warn!(
                    "Skipping non-numeric directory: {}",
                    sub_dir.file_name().to_string_lossy()
                );
                continue;
            }
        };
        // Read all files in the subdirectory
        for entry in std::fs::read_dir(sub_dir.path())? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let file_path = entry.path();
                let versioned_balance_unit: VersionedBalanceUnit =
                    serde_json::from_reader(File::open(file_path)?)?;
                let unit_name = entry
                    .file_name()
                    .to_string_lossy()
                    .trim_end_matches(".json")
                    .to_string();
                if let Some(v) = balance_data.get_mut(&version) {
                    v.insert(unit_name, versioned_balance_unit);
                } else {
                    let mut single_proto: VersionedBalanceUnits = VersionedBalanceUnits::new();
                    single_proto.insert(unit_name, versioned_balance_unit);
                    balance_data.insert(version, single_proto);
                }
            }
        }
    }
    tracing::info!(
        "Read {} versioned balance units from JSON files",
        balance_data.len()
    );

    Ok(balance_data)
}

static ARCHIVE_DIR: Dir = include_dir!("assets/BalanceData");
pub fn read_balance_data_from_included_assets()
-> Result<MultiVersionedBalanceUnits, S2ProtocolError> {
    tracing::info!("Reading balance data from included assets");
    let mut balance_data: MultiVersionedBalanceUnits = HashMap::new();

    let glob = "**/*.json";
    for entry in ARCHIVE_DIR.find(glob).unwrap() {
        let fname = entry.path();
        let parts: Vec<&str> = if let Some(fname_os_str) = fname.as_os_str().to_str() {
            fname_os_str.split('/').collect()
        } else {
            continue;
        };
        if parts.len() != 2 {
            tracing::warn!("Skipping invalid asset file: {:?}", fname);
            continue;
        }
        let contents = if let Some(file) = entry.as_file()
            && let Some(file_contents) = file.contents_utf8()
        {
            file_contents
        } else {
            continue;
        };
        let version: u32 = match parts[0].parse() {
            Ok(v) => v,
            Err(_) => {
                tracing::warn!(
                    "Skipping non-numeric version in asset file: {}",
                    fname.display()
                );
                continue;
            }
        };
        let unit_name = parts[1].trim_end_matches(".json").to_string();
        let versioned_balance_unit: VersionedBalanceUnit = serde_json::from_str(contents)?;
        if let Some(v) = balance_data.get_mut(&version) {
            v.insert(unit_name, versioned_balance_unit);
        } else {
            let mut single_proto: VersionedBalanceUnits = VersionedBalanceUnits::new();
            single_proto.insert(unit_name, versioned_balance_unit);
            balance_data.insert(version, single_proto);
        }
    }
    tracing::info!(
        "Read {} versioned balance units from included assets",
        balance_data.len()
    );
    Ok(balance_data)
}
