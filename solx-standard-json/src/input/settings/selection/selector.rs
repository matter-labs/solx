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
    /// The AST JSON.
    #[serde(rename = "ast")]
    AST,
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
    /// The transient storage layout.
    #[serde(rename = "transientStorageLayout")]
    TransientStorageLayout,
    /// The function signature hashes JSON.
    #[serde(rename = "evm.methodIdentifiers")]
    MethodIdentifiers,
    /// The EVM legacy assembly JSON.
    #[serde(rename = "evm.legacyAssembly")]
    EVMLA,
    /// The Yul IR.
    #[serde(rename = "irOptimized")]
    Yul,

    /// The deploy bytecode.
    #[serde(rename = "evm.bytecode", alias = "evm.bytecode.object")]
    Bytecode,
    /// The deploy LLVM assembly.
    #[serde(rename = "evm.bytecode.llvmAssembly")]
    DeployLLVMAssembly,
    /// The runtime bytecode.
    #[serde(rename = "evm.deployedBytecode", alias = "evm.deployedBytecode.object")]
    RuntimeBytecode,
    /// The runtime LLVM assembly.
    #[serde(rename = "evm.deployedBytecode.llvmAssembly")]
    RuntimeLLVMAssembly,

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
            Self::Bytecode
                | Self::RuntimeBytecode
                | Self::DeployLLVMAssembly
                | Self::RuntimeLLVMAssembly
                | Self::Other
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
