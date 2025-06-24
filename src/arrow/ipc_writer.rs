//! A set of utilities for interacting with IPC writing to filesystem
//! Currently it's using Mutex, maybe in the future we can try mpsc channels.
//! But not sure this would be advantageous.

#[cfg(feature = "arrow")]
use arrow::{array::Array, chunk::Chunk};
#[cfg(feature = "arrow")]
use arrow_convert::serialize::FlattenChunk;

use std::path::PathBuf;

/// Converts the data into an Arrow IPC file, this is useful for small batches of data,
/// for example if we are reading all the details from all the files, they should fit in memory
/// (famous last words)
#[cfg(feature = "arrow")]
pub fn write_batches(
    path: PathBuf,
    schema: arrow::datatypes::Schema,
    chunks: &[Chunk<Box<dyn Array>>],
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(path)?;

    let options = arrow::io::ipc::write::WriteOptions { compression: None };
    let mut writer = arrow::io::ipc::write::FileWriter::new(file, schema, None, options);

    writer.start()?;
    for chunk in chunks {
        writer.write(chunk, None)?
    }
    writer.finish()?;
    Ok(())
}

/// Opens a mutex protected Arrow IPC file writer
#[cfg(feature = "arrow")]
pub fn open_arrow_mutex_writer(
    output: PathBuf,
    schema: arrow::datatypes::Schema,
) -> Result<
    std::sync::Mutex<arrow::io::ipc::write::FileWriter<std::fs::File>>,
    Box<dyn std::error::Error>,
> {
    let file = std::fs::File::create(output)?;

    let options = arrow::io::ipc::write::WriteOptions { compression: None };
    let mut writer = arrow::io::ipc::write::FileWriter::new(file, schema, None, options);
    writer.start()?;
    Ok(std::sync::Mutex::new(writer))
}

/// Writes the batch to the Arrow IPC file held over a mutex, to be called from within a parallel iterator
#[cfg(feature = "arrow")]
pub fn write_to_arrow_mutex_writer(
    writer: &std::sync::Mutex<arrow::io::ipc::write::FileWriter<std::fs::File>>,
    res: Box<dyn Array>,
    batch_length: usize,
) -> Option<usize> {
    if batch_length == 0 {
        return None;
    }
    let mut file_lock = match writer.lock() {
        Ok(lock) => lock,
        Err(err) => {
            tracing::error!("Error locking file: {:?}", err);
            return None;
        }
    };
    let chunk = match Chunk::new([res].to_vec()).flatten() {
        Ok(chunk) => chunk,
        Err(err) => {
            tracing::error!("Error converting to arrow: {:?}", err);
            return None;
        }
    };
    match file_lock.write(&chunk, None) {
        Ok(_) => Some(batch_length),
        Err(err) => {
            // At this point maybe we should fail because the lock write
            // should fail for any other parallel process.
            tracing::error!("Error writing chunk: {:?}", err);
            None
        }
    }
}

/// Closes the Arrow IPC file held over a mutex
#[cfg(feature = "arrow")]
pub fn close_arrow_mutex_writer(
    writer: std::sync::Mutex<arrow::io::ipc::write::FileWriter<std::fs::File>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = match writer.lock() {
        Ok(writer) => writer,
        Err(err) => {
            tracing::error!("Error locking file: {:?}", err);
            return Err("Lock error".into());
        }
    };
    writer.finish()?;
    Ok(())
}
