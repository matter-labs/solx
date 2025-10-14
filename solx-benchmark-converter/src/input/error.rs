//!
//! Benchmark input format.
//!

use std::path::PathBuf;

///
/// Input report reading error.
///
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error reading the input file.
    #[error("Reading input file {path:?}: {error}")]
    Reading {
        /// The underlying IO error.
        error: std::io::Error,
        /// The path to the input file.
        path: PathBuf,
    },
    /// Error parsing the input file.
    #[error("Parsing input file {path:?}: {error}")]
    Parsing {
        /// The underlying JSON parsing error.
        error: serde_json::Error,
        /// The path to the input file.
        path: PathBuf,
    },
    /// Empty file error.
    #[error("Input file {path:?} is empty")]
    EmptyFile {
        /// The path to the input file.
        path: PathBuf,
    },
}
