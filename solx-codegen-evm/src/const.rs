//!
//! EVM codegen constants.
//!

/// The LLVM framework version.
pub const LLVM_VERSION: semver::Version = semver::Version::new(19, 1, 0);

/// The entry function name.
pub const ENTRY_FUNCTION_NAME: &str = "__entry";

/// The deployed Yul object identifier suffix.
pub static YUL_OBJECT_DEPLOYED_SUFFIX: &str = "_deployed";

/// Library deploy address Yul identifier.
pub static LIBRARY_DEPLOY_ADDRESS_TAG: &str = "library_deploy_address";

/// The deploy bytecode size limit.
pub const DEPLOY_CODE_SIZE_LIMIT: usize = 49152;

/// The runtime bytecode size limit.
pub const RUNTIME_CODE_SIZE_LIMIT: usize = 24576;

/// The `solc` general memory offset.
pub const SOLC_GENERAL_MEMORY_OFFSET: u64 = 128;
