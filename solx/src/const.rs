//!
//! Solidity compiler constants.
//!

/// The default executable name.
pub static DEFAULT_EXECUTABLE_NAME: &str = "solx";

/// The `solc` compiler production name.
pub static SOLC_PRODUCTION_NAME: &str = "solc";

/// The `solc` LLVM revision metadata tag.
pub static SOLC_LLVM_REVISION_METADATA_TAG: &str = "llvm";

/// The worker thread stack size.
pub const WORKER_THREAD_STACK_SIZE: usize = 64 * 1024 * 1024;

///
/// The compiler version default function.
///
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}
