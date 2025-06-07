//!
//! The `solc --standard-json` output contract EVM data.
//!

pub mod bytecode;

use std::collections::BTreeMap;

use self::bytecode::Bytecode;

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_assembly: Option<solx_evm_assembly::Assembly>,
    /// The contract function signatures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method_identifiers: Option<BTreeMap<String, String>>,

    /// The extra EVM legacy assembly metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_metadata: Option<solx_evm_assembly::ExtraMetadata>,
}

impl EVM {
    ///
    /// Extends the object with data from the other object.
    ///
    pub fn extend(&mut self, other: Self) {
        if let Some(bytecode) = other.bytecode {
            if let Some(existing_bytecode) = &mut self.bytecode {
                existing_bytecode.extend(bytecode);
            } else {
                self.bytecode = Some(bytecode);
            }
        }
        if let Some(deployed_bytecode) = other.deployed_bytecode {
            if let Some(existing_deployed_bytecode) = &mut self.deployed_bytecode {
                existing_deployed_bytecode.extend(deployed_bytecode);
            } else {
                self.deployed_bytecode = Some(deployed_bytecode);
            }
        }
        self.legacy_assembly = self.legacy_assembly.take().or(other.legacy_assembly);
        self.method_identifiers = self.method_identifiers.take().or(other.method_identifiers);
        self.extra_metadata = self.extra_metadata.take().or(other.extra_metadata);
    }

    ///
    /// Checks if all fields are `None`.
    ///
    pub fn is_empty(&self) -> bool {
        self.bytecode
            .as_ref()
            .map(|bytecode| bytecode.is_empty())
            .unwrap_or(true)
            && self
                .deployed_bytecode
                .as_ref()
                .map(|bytecode| bytecode.is_empty())
                .unwrap_or(true)
            && self.legacy_assembly.is_none()
            && self.method_identifiers.is_none()
    }
}
