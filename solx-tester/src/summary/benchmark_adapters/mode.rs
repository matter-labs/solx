//!
//! Representation of compiler mode stored in the benchmark.
//!

use crate::compilers::mode::imode::IMode;

const DEFAULT_CODEGEN: &str = "NoCodegen";
const DEFAULT_OPTIMIZATIONS: &str = "NoOptimizations";
const DEFAULT_VERSION: &str = "NoVersion";

///
/// Representation of compiler mode stored in the benchmark.
///
pub struct ModeInfo {
    /// Codegen type if applicable, or a default value [`DEFAULT_CODEGEN`].
    pub codegen: String,
    /// Optimization level if applicable, or a default value [`DEFAULT_OPTIMIZATIONS`].
    pub optimizations: String,
    /// Language version if applicable, or a default value [`DEFAULT_VERSION`].
    pub version: String,
}

impl<T> From<T> for ModeInfo
where
    T: IMode,
{
    fn from(value: T) -> ModeInfo {
        ModeInfo {
            codegen: value.codegen().unwrap_or(DEFAULT_CODEGEN.into()),
            optimizations: value
                .optimizations()
                .unwrap_or(DEFAULT_OPTIMIZATIONS.into()),
            version: value.version().unwrap_or(DEFAULT_VERSION.into()),
        }
    }
}
