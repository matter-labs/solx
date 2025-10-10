//!
//! The `solc --standard-json` input settings optimizer.
//!

use serde::Serialize;

///
/// The `solc --standard-json` input settings optimizer.
///
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Optimizer {
    /// Whether the optimizer is enabled.
    pub enabled: bool,
    /// The MLIR optimizer mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<char>,
}

impl Optimizer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(enabled: bool, mode: Option<char>) -> Self {
        Self { enabled, mode }
    }
}
