//! ABANDON ALL HOPE, YE WHO ENTER HERE
//!
//! Protocol Code Parsing Generator.
//! This code reads the Protocol definition from the JSON Definition.
//! It generates structures for parsing progressively the Tracker Events for now.
//! It is meant to generate once the structures and method impl and then a human
//! can properly:
//! - RustFmt
//! - Document it.
//! Thus, [`syn`] and [`quote`] crates were not involved.
//! This is really ugly and probably only works for a handful of versions.

pub mod decoder_type;
pub mod proto_morphist;
pub use decoder_type::DecoderType;
pub use proto_morphist::ProtoMorphist;

/// A mapping between proto types defined in the json and the rust equivalent
#[derive(Debug)]
pub struct ProtoTypeConversion {
    /// The destination type in Rust types
    pub rust_ty: String,
    /// Whether a TryInto should be attempted
    pub do_try_from: bool,
    /// A value parser
    pub parser: String,
    /// Whether the type is optional
    pub is_optional: bool,
    /// Whether the type is a vec, NOTE: No `Vec<Option<T>>` have been observed. But there are
    /// `Option<Vec<T>>`.
    pub is_vec: bool,
    /// The type may be an int made of up several bits
    pub is_sized_int: bool,
}

impl Default for ProtoTypeConversion {
    fn default() -> Self {
        Self {
            rust_ty: "unknown_type".to_string(),
            do_try_from: false,
            parser: "unknown_parser".to_string(),
            is_optional: false,
            is_vec: false,
            is_sized_int: false,
        }
    }
}
