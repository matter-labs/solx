//!
//! The `solc --standard-json` output contract EVM bytecode.
//!

use serde::Deserialize;

///
/// The `solc --standard-json` output contract EVM bytecode.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bytecode {
    /// The bytecode object.
    pub object: String,
}
