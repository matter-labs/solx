//!
//! The debug IR type.
//!

///
/// The debug IR type.
///
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IRType {
    /// Whether to dump the Yul code.
    Yul,
    /// Whether to dump the EVM legacy assembly code.
    EVMLA,
    /// Whether to dump the Ethereal IR code.
    EthIR,
    /// Whether to dump the LLVM IR code.
    LLVM,
    /// Whether to dump the EVM assembly code.
    EVMAssembly,
}

impl IRType {
    ///
    /// Returns the file extension for the specified IR.
    ///
    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::Yul => solx_utils::EXTENSION_YUL,
            Self::EthIR => solx_utils::EXTENSION_ETHIR,
            Self::EVMLA => solx_utils::EXTENSION_EVMLA,
            Self::LLVM => solx_utils::EXTENSION_LLVM_SOURCE,
            Self::EVMAssembly => solx_utils::EXTENSION_EVM_ASSEMBLY,
        }
    }
}
