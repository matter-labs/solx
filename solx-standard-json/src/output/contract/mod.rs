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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abi: Option<serde_json::Value>,
    /// The contract storage layout.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_layout: Option<serde_json::Value>,
    /// The contract storage layout.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient_storage_layout: Option<serde_json::Value>,
    /// The contract metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// The contract developer documentation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devdoc: Option<serde_json::Value>,
    /// The contract user documentation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub userdoc: Option<serde_json::Value>,
    /// The contract optimized IR code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ir_optimized: Option<String>,
    /// The EVM data of the contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evm: Option<EVM>,
}

impl Contract {
    ///
    /// Extends the object with data from the other object.
    ///
    pub fn extend(&mut self, other: Self) {
        self.abi = self.abi.take().or(other.abi);
        self.storage_layout = self.storage_layout.take().or(other.storage_layout);
        self.transient_storage_layout = self
            .transient_storage_layout
            .take()
            .or(other.transient_storage_layout);
        self.metadata = self.metadata.take().or(other.metadata);
        self.devdoc = self.devdoc.take().or(other.devdoc);
        self.userdoc = self.userdoc.take().or(other.userdoc);
        self.ir_optimized = self.ir_optimized.take().or(other.ir_optimized);
        if let Some(evm) = other.evm {
            if let Some(existing_evm) = &mut self.evm {
                existing_evm.extend(evm);
            } else {
                self.evm = Some(evm);
            }
        }
    }

    ///
    /// Checks if all fields are unset or empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.abi.is_none()
            && self.storage_layout.is_none()
            && self.transient_storage_layout.is_none()
            && self.metadata.is_none()
            && self.devdoc.is_none()
            && self.userdoc.is_none()
            && self.ir_optimized.is_none()
            && self.evm.is_none()
    }
}
