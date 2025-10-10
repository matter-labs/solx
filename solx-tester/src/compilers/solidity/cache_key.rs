//!
//! The Solidity compiler cache key.
//!

use crate::compilers::solidity::codegen::Codegen;

///
/// The Solidity compiler cache key.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    /// The test path.
    pub test_path: String,
    /// The Solidity compiler version.
    pub version: semver::Version,
    /// The Solidity compiler output type.
    pub codegen: Option<Codegen>,
    /// Whether to enable the EVMLA codegen via Yul IR.
    pub via_ir: bool,
    /// Whether to enable the MLIR codegen.
    pub via_mlir: bool,
    /// Whether to run the Solidity compiler optimizer.
    pub optimize: bool,
}

impl CacheKey {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        test_path: String,
        version: semver::Version,
        codegen: Option<Codegen>,
        via_ir: bool,
        via_mlir: bool,
        optimize: bool,
    ) -> Self {
        Self {
            test_path,
            version,
            codegen,
            via_ir,
            via_mlir,
            optimize,
        }
    }
}
