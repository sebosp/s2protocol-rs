//! A set of utilities for interacting with IPC writing to filesystem
//! Currently it's using Mutex, maybe in the future we can try mpsc channels.
//! But not sure this would be advantageous.

#[cfg(feature = "dep_arrow")]
use arrow::{
    array::ArrayRef,
    datatypes::Schema,
    ipc::{
        CompressionType,
        writer::{FileWriter, IpcWriteOptions},
    },
    record_batch::RecordBatch,
};
use std::path::PathBuf;

/// The options of the snapshot files, the batches are compressed with zstd, the unit and ability
/// names repeat a lot and the files are about ten times smaller. Polars, pyarrow and DuckDB read
/// them as any other IPC file.
#[cfg(feature = "dep_arrow")]
fn write_options() -> Result<IpcWriteOptions, arrow::error::ArrowError> {
    IpcWriteOptions::default().try_with_compression(Some(CompressionType::ZSTD))
}

/// Opens an Arrow IPC file writer with the [`write_options`] of the snapshot files.
#[cfg(feature = "dep_arrow")]
fn open_file_writer(
    path: PathBuf,
    schema: &Schema,
) -> Result<FileWriter<std::fs::File>, Box<dyn std::error::Error>> {
    let file = std::fs::File::create(path)?;
    Ok(FileWriter::try_new_with_options(
        file,
        schema,
        write_options()?,
    )?)
}

/// Converts the data into an Arrow IPC file, this is useful for small batches of data,
/// for example if we are reading all the details from all the files, they should fit in memory
/// (famous last words)
#[cfg(feature = "dep_arrow")]
pub fn write_batches(
    path: PathBuf,
    schema: Schema,
    chunk: RecordBatch,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = open_file_writer(path, &schema)?;

    writer.write(&chunk)?;
    writer.finish()?;
    Ok(())
}

/// An Arrow IPC file writer shared by parallel iterators.
#[cfg(feature = "dep_arrow")]
pub type ArrowMutexWriter = std::sync::Mutex<FileWriter<std::fs::File>>;

/// Opens a mutex protected Arrow IPC file writer
#[cfg(feature = "dep_arrow")]
pub fn open_arrow_mutex_writer(
    output: PathBuf,
    schema: Schema,
) -> Result<ArrowMutexWriter, Box<dyn std::error::Error>> {
    Ok(std::sync::Mutex::new(open_file_writer(output, &schema)?))
}

/// Writes the batch to the Arrow IPC file held over a mutex, to be called from within a parallel iterator
#[cfg(feature = "dep_arrow")]
pub fn write_to_arrow_mutex_writer(
    writer: &std::sync::Mutex<FileWriter<std::fs::File>>,
    res: ArrayRef,
) -> Option<usize> {
    let mut file_lock = match writer.lock() {
        Ok(lock) => lock,
        Err(err) => {
            tracing::error!("Error locking file: {:?}", err);
            return None;
        }
    };
    let batch_length = res.len();
    let chunk: RecordBatch = res
        .as_any()
        .downcast_ref::<arrow::array::StructArray>()
        .unwrap()
        .into();
    match file_lock.write(&chunk) {
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
#[cfg(feature = "dep_arrow")]
pub fn close_arrow_mutex_writer(
    writer: std::sync::Mutex<FileWriter<std::fs::File>>,
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
