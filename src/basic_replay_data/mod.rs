//! Contains the regions of the SC2Replay that are light and used before
//! These are used to avoid loading all the MPQ in memory or to have to read the same MPQ file
//! mulitple times.
//!

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{InitData, S2ProtocolError, details::Details};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SC2ReplayBasicData {
    pub init_data: InitData,
    pub details: Details,
}

impl SC2ReplayBasicData {
    /// Reads the file from the path and calls the per-protocol parser for the InitData.
    /// Sets the metadata if successful.
    /// Returns an error if the file cannot be read or the parser fails.
    #[tracing::instrument(level = "debug")]
    pub fn new(path: &PathBuf, ext_fs_id: u64) -> Result<Self, S2ProtocolError> {
        let file_contents = crate::read_file(&path)?;
        let (_input, mpq) = crate::parser::parse(&file_contents)?;
        let path_str = path.to_str().unwrap_or_default();
        let init_data = InitData::new(path_str, ext_fs_id, &mpq, &file_contents)?;
        let details = Details::new(path_str, ext_fs_id, &mpq, &file_contents)?;
        Ok(Self { init_data, details })
    }
}
