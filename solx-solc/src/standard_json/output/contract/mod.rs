//!
//! The `solc --standard-json` output contract.
//!

pub mod evm;

use self::evm::EVM;

///
/// The `solc --standard-json` output contract.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    /// The contract ABI.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub abi: serde_json::Value,
    /// The contract storage layout.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub storage_layout: serde_json::Value,
    /// The contract storage layout.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub transient_storage_layout: serde_json::Value,
    /// The contract metadata.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub metadata: String,
    /// The contract developer documentation.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub devdoc: serde_json::Value,
    /// The contract user documentation.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub userdoc: serde_json::Value,
    /// The contract optimized IR code.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ir_optimized: String,
    /// The EVM data of the contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evm: Option<EVM>,
}

impl Contract {
    ///
    /// Checks if all fields are unset or empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.abi.is_null()
            && self.storage_layout.is_null()
            && self.transient_storage_layout.is_null()
            && self.metadata.is_empty()
            && self.devdoc.is_null()
            && self.userdoc.is_null()
            && self.ir_optimized.is_empty()
            && self.evm.is_none()
    }
}
