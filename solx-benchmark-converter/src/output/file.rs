//!
//! Represents a single benchmark output file in a set of many.
//!

use std::path::PathBuf;

///
/// Represents a single benchmark output file in a set of many.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct File {
    /// Path to this file relative to user-provided root.
    pub path: PathBuf,
    /// File content.
    pub content: String,
}

impl File {
    ///
    /// Create a new file instance with an object serialized to JSON.
    ///
    pub fn new<S, V>(path: S, object: V) -> Self
    where
        S: std::fmt::Display,
        V: Sized + serde::Serialize,
    {
        let path = format!("{path}.{}", solx_utils::EXTENSION_JSON).into();
        let content = serde_json::to_string_pretty(&object).expect("Always valid");
        Self { path, content }
    }
}
