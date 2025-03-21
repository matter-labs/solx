//!
//! The `solc --standard-json` input settings metadata.
//!

///
/// The `solc --standard-json` input settings metadata.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Whether to use literal content.
    #[serde(default)]
    pub use_literal_content: bool,
    /// The metadata hash type.
    #[serde(default = "Metadata::default_bytecode_hash", skip_serializing)]
    pub bytecode_hash: era_compiler_common::HashType,
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new(false, Self::default_bytecode_hash())
    }
}

impl Metadata {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(use_literal_content: bool, hash_type: era_compiler_common::HashType) -> Self {
        Self {
            bytecode_hash: hash_type,
            use_literal_content,
        }
    }

    ///
    /// The default metadata hash type.
    ///
    fn default_bytecode_hash() -> era_compiler_common::HashType {
        era_compiler_common::HashType::Ipfs
    }
}
