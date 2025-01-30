//!
//! The contract LLVM IR source code.
//!

///
/// The contract LLVM IR source code.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LLVMIR {
    /// The LLVM IR file path.
    pub path: String,
    /// The LLVM IR source code.
    pub source: String,
}

impl LLVMIR {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(path: String, source: String) -> Self {
        Self { path, source }
    }
}
