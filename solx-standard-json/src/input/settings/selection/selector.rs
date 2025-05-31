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
    DeveloperDocumentation,
    /// The user documentation.
    #[serde(rename = "userdoc")]
    UserDocumentation,
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
    EVMLegacyAssembly,
    /// The Yul IR.
    #[serde(rename = "irOptimized")]
    Yul,

    /// All EVM data.
    #[serde(rename = "evm")]
    EVM,
    /// The deploy bytecode.
    #[serde(rename = "evm.bytecode")]
    Bytecode,
    /// The deploy bytecode object.
    #[serde(rename = "evm.bytecode.object")]
    BytecodeObject,
    /// The deploy LLVM assembly.
    #[serde(rename = "evm.bytecode.llvmAssembly")]
    BytecodeLLVMAssembly,
    /// The deploy bytecode opcodes.
    #[serde(rename = "evm.bytecode.opcodes")]
    BytecodeOpcodes,
    /// The deploy bytecode link references.
    #[serde(rename = "evm.bytecode.linkReferences")]
    BytecodeLinkReferences,
    /// The deploy bytecode source maps.
    #[serde(rename = "evm.bytecode.sourceMap")]
    BytecodeSourceMap,
    /// The deploy bytecode function debug data.
    #[serde(rename = "evm.bytecode.functionDebugData")]
    BytecodeFunctionDebugData,
    /// The deploy bytecode generated sources
    #[serde(rename = "evm.bytecode.generatedSources")]
    BytecodeGeneratedSources,
    /// The runtime bytecode.
    #[serde(rename = "evm.deployedBytecode")]
    RuntimeBytecode,
    /// The runtime bytecode object.
    #[serde(rename = "evm.deployedBytecode.object")]
    RuntimeBytecodeObject,
    /// The runtime LLVM assembly.
    #[serde(rename = "evm.deployedBytecode.llvmAssembly")]
    RuntimeBytecodeLLVMAssembly,
    /// The runtime bytecode opcodes.
    #[serde(rename = "evm.deployedBytecode.opcodes")]
    RuntimeBytecodeOpcodes,
    /// The runtime bytecode link references.
    #[serde(rename = "evm.deployedBytecode.linkReferences")]
    RuntimeBytecodeLinkReferences,
    /// The runtime bytecode immutable references.
    #[serde(rename = "evm.deployedBytecode.immutableReferences")]
    RuntimeBytecodeImmutableReferences,
    /// The runtime bytecode source maps.
    #[serde(rename = "evm.deployedBytecode.sourceMap")]
    RuntimeBytecodeSourceMap,
    /// The runtime bytecode function debug data.
    #[serde(rename = "evm.deployedBytecode.functionDebugData")]
    RuntimeBytecodeFunctionDebugData,
    /// The runtime bytecode generated sources
    #[serde(rename = "evm.deployedBytecode.generatedSources")]
    RuntimeBytecodeGeneratedSources,

    /// The wildcard variant that selects everything.
    #[serde(rename = "*")]
    Any,
}

impl Selector {
    ///
    /// Whether the data source is `solc`.
    ///
    pub fn is_received_from_solc(&self) -> bool {
        !matches!(
            self,
            Self::EVM
                | Self::Bytecode
                | Self::BytecodeObject
                | Self::BytecodeLLVMAssembly
                | Self::BytecodeLinkReferences
                | Self::BytecodeSourceMap
                | Self::BytecodeGeneratedSources
                | Self::BytecodeFunctionDebugData
                | Self::RuntimeBytecode
                | Self::RuntimeBytecodeObject
                | Self::RuntimeBytecodeLLVMAssembly
                | Self::RuntimeBytecodeSourceMap
                | Self::RuntimeBytecodeGeneratedSources
                | Self::RuntimeBytecodeFunctionDebugData
                | Self::RuntimeBytecodeLinkReferences
                | Self::RuntimeBytecodeImmutableReferences
        )
    }

    ///
    /// Converts a multi-item selector into a group of single-item selectors.
    ///
    pub fn into_single_selectors(self) -> Vec<Self> {
        match self {
            Self::EVM => vec![
                Self::Bytecode,
                Self::BytecodeObject,
                Self::BytecodeLLVMAssembly,
                Self::BytecodeOpcodes,
                Self::BytecodeLinkReferences,
                Self::BytecodeSourceMap,
                Self::BytecodeFunctionDebugData,
                Self::BytecodeGeneratedSources,
                Self::RuntimeBytecode,
                Self::RuntimeBytecodeObject,
                Self::RuntimeBytecodeLLVMAssembly,
                Self::RuntimeBytecodeOpcodes,
                Self::RuntimeBytecodeLinkReferences,
                Self::RuntimeBytecodeImmutableReferences,
                Self::RuntimeBytecodeSourceMap,
                Self::RuntimeBytecodeFunctionDebugData,
                Self::RuntimeBytecodeGeneratedSources,
            ],
            Self::Bytecode => vec![
                Self::BytecodeObject,
                Self::BytecodeLLVMAssembly,
                Self::BytecodeOpcodes,
                Self::BytecodeLinkReferences,
                Self::BytecodeSourceMap,
                Self::BytecodeFunctionDebugData,
                Self::BytecodeGeneratedSources,
            ],
            Self::RuntimeBytecode => vec![
                Self::RuntimeBytecodeObject,
                Self::RuntimeBytecodeLLVMAssembly,
                Self::RuntimeBytecodeOpcodes,
                Self::RuntimeBytecodeLinkReferences,
                Self::RuntimeBytecodeImmutableReferences,
                Self::RuntimeBytecodeSourceMap,
                Self::RuntimeBytecodeFunctionDebugData,
                Self::RuntimeBytecodeGeneratedSources,
            ],
            Self::Any => vec![
                Self::AST,
                Self::ABI,
                Self::Metadata,
                Self::DeveloperDocumentation,
                Self::UserDocumentation,
                Self::StorageLayout,
                Self::TransientStorageLayout,
                Self::MethodIdentifiers,
                Self::EVMLegacyAssembly,
                Self::Yul,
                Self::EVM,
                Self::Bytecode,
                Self::BytecodeObject,
                Self::BytecodeLLVMAssembly,
                Self::BytecodeOpcodes,
                Self::BytecodeLinkReferences,
                Self::BytecodeSourceMap,
                Self::BytecodeFunctionDebugData,
                Self::BytecodeGeneratedSources,
                Self::RuntimeBytecode,
                Self::RuntimeBytecodeObject,
                Self::RuntimeBytecodeLLVMAssembly,
                Self::RuntimeBytecodeOpcodes,
                Self::RuntimeBytecodeLinkReferences,
                Self::RuntimeBytecodeImmutableReferences,
                Self::RuntimeBytecodeSourceMap,
                Self::RuntimeBytecodeFunctionDebugData,
                Self::RuntimeBytecodeGeneratedSources,
            ],
            selector => vec![selector],
        }
    }
}

impl From<bool> for Selector {
    fn from(via_ir: bool) -> Self {
        if via_ir {
            Self::Yul
        } else {
            Self::EVMLegacyAssembly
        }
    }
}
