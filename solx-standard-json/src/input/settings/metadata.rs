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

    /// Whether to append CBOR metadata.
    #[serde(
        rename = "appendCBOR",
        default = "Metadata::default_append_cbor",
        skip_serializing
    )]
    pub append_cbor: bool,

    /// The metadata hash type.
    #[serde(default = "Metadata::default_bytecode_hash", skip_serializing)]
    pub bytecode_hash: era_compiler_common::EVMMetadataHashType,
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new(false, true, Self::default_bytecode_hash())
    }
}

impl Metadata {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        use_literal_content: bool,
        append_cbor: bool,
        hash_type: era_compiler_common::EVMMetadataHashType,
    ) -> Self {
        Self {
            bytecode_hash: hash_type,
            append_cbor,
            use_literal_content,
        }
    }

    ///
    /// The default metadata hash type.
    ///
    fn default_bytecode_hash() -> era_compiler_common::EVMMetadataHashType {
        era_compiler_common::EVMMetadataHashType::IPFS
    }

    ///
    /// The default append CBOR flag.
    ///
    fn default_append_cbor() -> bool {
        true
    }
}
