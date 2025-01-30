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
    #[serde(
        alias = "bytecodeHash",
        default = "Metadata::default_hash_type",
        skip_serializing
    )]
    pub hash_type: era_compiler_common::HashType,
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new(false, Self::default_hash_type())
    }
}

impl Metadata {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(use_literal_content: bool, hash_type: era_compiler_common::HashType) -> Self {
        Self {
            hash_type,
            use_literal_content,
        }
    }

    ///
    /// The default metadata hash type.
    ///
    fn default_hash_type() -> era_compiler_common::HashType {
        era_compiler_common::HashType::Keccak256
    }
}
