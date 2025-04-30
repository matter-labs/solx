//!
//! The `solc --standard-json` output contract EVM data.
//!

pub mod bytecode;
pub mod extra_metadata;

use std::collections::BTreeMap;

use self::bytecode::Bytecode;
use self::extra_metadata::ExtraMetadata;

///
/// The `solc --standard-json` output contract EVM data.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EVM {
    /// The contract deploy bytecode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytecode: Option<Bytecode>,
    /// The contract runtime bytecode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployed_bytecode: Option<Bytecode>,
    /// The contract EVM legacy assembly code.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub legacy_assembly: serde_json::Value,
    /// The contract function signatures.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub method_identifiers: BTreeMap<String, String>,

    /// The extra EVMLA metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_metadata: Option<ExtraMetadata>,
}

impl EVM {
    ///
    /// Checks if all fields are `None`.
    ///
    pub fn is_empty(&self) -> bool {
        self.bytecode
            .as_ref()
            .map(|bytecode| bytecode.is_empty())
            .unwrap_or_default()
            && self
                .deployed_bytecode
                .as_ref()
                .map(|bytecode| bytecode.is_empty())
                .unwrap_or_default()
            && self.legacy_assembly.is_null()
            && self.method_identifiers.is_empty()
            && self.extra_metadata.is_none()
    }
}
