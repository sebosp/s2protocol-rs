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
        Self::from_bytes(path.to_str().unwrap_or_default(), ext_fs_id, &file_contents)
    }

    /// Reads the InitData and Details from the contents of a replay, i.e. a file dropped in a
    /// browser, where there is no file system. The file name is kept as metadata.
    #[tracing::instrument(level = "debug", skip(file_contents))]
    pub fn from_bytes(
        file_name: &str,
        ext_fs_id: u64,
        file_contents: &[u8],
    ) -> Result<Self, S2ProtocolError> {
        let (_input, mpq) = crate::parser::parse(file_contents)?;
        let init_data = InitData::new(file_name, ext_fs_id, &mpq, file_contents)?;
        let details = Details::new(file_name, ext_fs_id, &mpq, file_contents)?;
        Ok(Self { init_data, details })
    }
}
