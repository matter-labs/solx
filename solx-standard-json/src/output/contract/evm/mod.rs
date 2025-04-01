//!
//! The `solc --standard-json` output contract EVM data.
//!

pub mod bytecode;
pub mod extra_metadata;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

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
    /// Sets the EVM and deploy and runtime bytecode.
    ///
    pub fn modify(
        &mut self,
        deploy_bytecode: String,
        deploy_object_format: era_compiler_common::ObjectFormat,
        deploy_unlinked_libraries: BTreeSet<String>,
        runtime_bytecode: String,
        runtime_object_format: era_compiler_common::ObjectFormat,
        runtime_unlinked_libraries: BTreeSet<String>,
    ) {
        self.bytecode = Some(Bytecode::new(
            deploy_bytecode,
            deploy_unlinked_libraries,
            deploy_object_format,
        ));
        self.deployed_bytecode = Some(Bytecode::new(
            runtime_bytecode,
            runtime_unlinked_libraries,
            runtime_object_format,
        ));
    }

    ///
    /// Checks if all fields are `None`.
    ///
    pub fn is_empty(&self) -> bool {
        self.bytecode.is_none()
            && self.deployed_bytecode.is_none()
            && self.legacy_assembly.is_null()
            && self.method_identifiers.is_empty()
            && self.extra_metadata.is_none()
    }
}
