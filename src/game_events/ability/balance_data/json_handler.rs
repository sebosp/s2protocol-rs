//! Writes a snapshot of the BalanceData read from XML into JSON format for posterity.

use include_assets::{NamedArchive, include_dir};
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use tracing;

use crate::game_events::VersionedBalanceUnit;

/// Writes the BalanceData to a JSON file at the specified path.
pub fn write_balance_data_to_json<P: AsRef<Path>>(
    root_path: P,
    balance_data: HashMap<(u32, String), VersionedBalanceUnit>,
) -> Result<(), Box<dyn Error>> {
    // The balance_data contains a version and a unit name.
    // For each version we must ensure the path exists within the root_path

    tracing::info!(
        "Writing balance data to JSON files in {}",
        root_path.as_ref().display()
    );
    for ((version, unit_name), versioned_balance_unit) in &balance_data {
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
    Ok(())
}

/// Reads the BalanceData from a JSON file at the specified path.
pub fn read_balance_data_from_json_dir<P: AsRef<Path>>(
    root_path: P,
) -> Result<HashMap<(u32, String), VersionedBalanceUnit>, Box<dyn Error>> {
    // Each subdirectory in the root_path corresponds to a version.
    // Each file in the subdirectory corresponds to a unit name.

    tracing::info!(
        "Reading balance data from JSON files in {}",
        root_path.as_ref().display()
    );
    let mut balance_data: HashMap<(u32, String), VersionedBalanceUnit> = HashMap::new();

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
                balance_data.insert((version, unit_name), versioned_balance_unit);
            }
        }
    }
    tracing::info!(
        "Read {} versioned balance units from JSON files",
        balance_data.len()
    );

    Ok(balance_data)
}

pub fn read_balance_data_from_included_assets()
-> Result<HashMap<(u32, String), VersionedBalanceUnit>, Box<dyn Error>> {
    tracing::info!("Reading balance data from included assets");
    let mut balance_data: HashMap<(u32, String), VersionedBalanceUnit> = HashMap::new();

    let archive = NamedArchive::load(include_dir!("assets/BalanceData"));

    for (fname, contents) in archive.assets() {
        let parts: Vec<&str> = fname.split('/').collect();
        if parts.len() != 2 {
            tracing::warn!("Skipping invalid asset file: {}", fname);
            continue;
        }
        let version: u32 = match parts[0].parse() {
            Ok(v) => v,
            Err(_) => {
                tracing::warn!("Skipping non-numeric version in asset file: {}", fname);
                continue;
            }
        };
        let unit_name = parts[1].trim_end_matches(".json").to_string();
        let versioned_balance_unit: VersionedBalanceUnit = serde_json::from_slice(contents)?;
        balance_data.insert((version, unit_name), versioned_balance_unit);
    }
    tracing::info!(
        "Read {} versioned balance units from included assets",
        balance_data.len()
    );
    Ok(balance_data)
}
