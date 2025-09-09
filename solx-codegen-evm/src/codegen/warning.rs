//!
//! EVM target warning.
//!

///
/// EVM target warning.
///
#[derive(Debug, thiserror::Error, Clone, serde::Serialize, serde::Deserialize)]
pub enum Warning {
    /// Deploy code size warning.
    #[error(
        "{0} bytecode size is {found}B that exceeds the EVM limit of {1}B",
        era_compiler_common::CodeSegment::Deploy,
        crate::r#const::DEPLOY_CODE_SIZE_LIMIT
    )]
    DeployCodeSize {
        /// Bytecode size.
        found: usize,
    },

    /// Runtime code size warning.
    #[error(
        "{0} bytecode size is {found}B that exceeds the EVM limit of {1}B",
        era_compiler_common::CodeSegment::Runtime,
        crate::r#const::RUNTIME_CODE_SIZE_LIMIT
    )]
    RuntimeCodeSize {
        /// Bytecode size.
        found: usize,
    },
}

impl Warning {
    ///
    /// Warning code.
    ///
    /// Mimic `solc` warning codes where possible for compatibility.
    ///
    pub fn code(&self) -> Option<isize> {
        match self {
            Self::DeployCodeSize { .. } => Some(3860),
            Self::RuntimeCodeSize { .. } => Some(5574),
        }
    }
}
