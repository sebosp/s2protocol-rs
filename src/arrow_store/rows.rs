//! The flat rows of the snapshot files are converted to Arrow with their serde implementations.

use arrow::array::{ArrayRef, StructArray};
use arrow::datatypes::{FieldRef, Fields, Schema};
use arrow::error::ArrowError;
use serde::{Deserialize, Serialize};
use serde_arrow::schema::{SchemaLike, TracingOptions};
use std::sync::Arc;

/// A row of a snapshot file, its fields are the columns.
pub trait ArrowRow: Serialize + for<'de> Deserialize<'de> + Sized {
    /// The columns of the row, cached by each implementation.
    fn fields() -> &'static Fields;

    /// The columns traced from the serde implementation. The strings are Utf8, the `timestamps`
    /// fields are serialized as nanoseconds and stored as timestamps.
    fn traced_fields(timestamps: &[&str]) -> Fields {
        let mut options = TracingOptions::default()
            .strings_as_large_utf8(false)
            .allow_null_fields(true);
        // The dates can not be traced, their type is given.
        for name in timestamps {
            let field = serde_json::json!({
                "name": name,
                "data_type": "Timestamp(Nanosecond, None)",
            });
            options = options
                .overwrite(*name, &field)
                .expect("a timestamp field overwrites the traced one");
        }
        Vec::<FieldRef>::from_type::<Self>(options)
            .expect("the flat rows can be traced from their serde implementation")
            .into()
    }

    fn schema() -> Schema {
        Schema::new(Self::fields().clone())
    }
}

/// Converts the rows to a struct array, its fields are the columns of the snapshot file.
pub trait TryIntoArrow {
    fn try_into_arrow(&self) -> Result<ArrayRef, ArrowError>;
}

impl<T: ArrowRow> TryIntoArrow for [T] {
    fn try_into_arrow(&self) -> Result<ArrayRef, ArrowError> {
        let fields = T::fields();
        let arrays = serde_arrow::to_arrow(fields, self)
            .map_err(|err| ArrowError::ExternalError(Box::new(err)))?;
        Ok(Arc::new(StructArray::try_new(
            fields.clone(),
            arrays,
            None,
        )?))
    }
}

impl<T: ArrowRow> TryIntoArrow for Vec<T> {
    fn try_into_arrow(&self) -> Result<ArrayRef, ArrowError> {
        self.as_slice().try_into_arrow()
    }
}

/// Implements [`ArrowRow`] for a flat row, the fields are traced once.
#[macro_export]
macro_rules! impl_arrow_row {
    ($row:ty) => {
        $crate::impl_arrow_row!($row, []);
    };
    ($row:ty, [$($timestamp:literal),*]) => {
        impl $crate::arrow_store::rows::ArrowRow for $row {
            fn fields() -> &'static arrow::datatypes::Fields {
                static FIELDS: std::sync::OnceLock<arrow::datatypes::Fields> =
                    std::sync::OnceLock::new();
                FIELDS.get_or_init(|| Self::traced_fields(&[$($timestamp),*]))
            }
        }
    };
}
