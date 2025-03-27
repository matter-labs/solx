//!
//! The `solc --standard-json` expected output selector.
//!

///
/// The `solc --standard-json` expected output selector.
///
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Selector {
    /// The ABI JSON.
    #[serde(rename = "abi")]
    ABI,
    /// The metadata.
    #[serde(rename = "metadata")]
    Metadata,
    /// The developer documentation.
    #[serde(rename = "devdoc")]
    Devdoc,
    /// The user documentation.
    #[serde(rename = "userdoc")]
    Userdoc,
    /// The storage layout.
    #[serde(rename = "storageLayout")]
    StorageLayout,
    /// The AST JSON.
    #[serde(rename = "ast")]
    AST,
    /// The Yul IR.
    #[serde(rename = "irOptimized")]
    Yul,
    /// The EVM bytecode.
    #[serde(rename = "evm")]
    EVM,
    /// The EVM legacy assembly JSON.
    #[serde(rename = "evm.legacyAssembly")]
    EVMLA,
    /// The function signature hashes JSON.
    #[serde(rename = "evm.methodIdentifiers")]
    MethodIdentifiers,

    /// The deploy bytecode.
    #[serde(rename = "evm.bytecode.object")]
    BytecodeObject,
    /// The runtime bytecode.
    #[serde(rename = "evm.deployedBytecode.object")]
    RuntimeBytecodeObject,

    /// The catch-all variant.
    #[serde(other)]
    Other,
}

impl Selector {
    ///
    /// Whether the data source is `solc`.
    ///
    pub fn is_received_from_solc(&self) -> bool {
        !matches!(
            self,
            Self::BytecodeObject | Self::RuntimeBytecodeObject | Self::Other
        )
    }
}

impl From<bool> for Selector {
    fn from(via_ir: bool) -> Self {
        if via_ir {
            Self::Yul
        } else {
            Self::EVMLA
        }
    }
}
