//!
//! The `solc --standard-json` output error source location.
//!

use std::collections::BTreeMap;

///
/// The `solc --standard-json` output error source location.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceLocation {
    /// The source file path.
    pub file: String,
    /// The start location.
    pub start: isize,
    /// The end location.
    pub end: isize,
}

impl SourceLocation {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(file: String) -> Self {
        Self {
            file,
            start: -1,
            end: -1,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Please note that `start` and `end` are not line and column,
    /// but absolute char offsets in the source code file.
    ///
    pub fn new_with_offsets(file: String, start: isize, end: isize) -> Self {
        Self { file, start, end }
    }

    ///
    /// A shortcut constructor from a `solc` AST node.
    ///
    pub fn try_from_ast(source: &str, id_paths: &BTreeMap<usize, &String>) -> Option<Self> {
        let mut parts = source.split(':');
        let start = parts
            .next()
            .map(|string| string.parse::<isize>())
            .and_then(Result::ok)
            .unwrap_or_default();
        let length = parts
            .next()
            .map(|string| string.parse::<isize>())
            .and_then(Result::ok)
            .unwrap_or_default();
        let path = parts
            .next()
            .and_then(|string| string.parse::<usize>().ok())
            .and_then(|file_id| id_paths.get(&file_id))?;

        Some(Self::new_with_offsets(
            (*path).to_owned(),
            start,
            start + length,
        ))
    }
}
