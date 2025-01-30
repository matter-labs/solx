//!
//! The `solc --standard-json` output contract EVM bytecode.
//!

///
/// The `solc --standard-json` output contract EVM bytecode.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bytecode {
    /// The bytecode object.
    pub object: String,
}

impl Bytecode {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(object: String) -> Self {
        Self { object }
    }
}
