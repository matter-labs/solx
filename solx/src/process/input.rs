//!
//! Process for compiling a single compilation unit.
//!
//! The EVM input data.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::project::contract::Contract;

///
/// The EVM input data.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Input {
    /// The input contract.
    pub contract: Contract,
    /// The mapping of auxiliary identifiers, e.g. Yul object names, to full contract paths.
    pub identifier_paths: BTreeMap<String, String>,
    /// Whether to output the bytecode, or only the metadata.
    pub output_bytecode: bool,
    /// Already deployed libraries.
    pub deployed_libraries: BTreeSet<String>,
    /// The metadata hash type.
    pub metadata_hash_type: era_compiler_common::HashType,
    /// The optimizer settings.
    pub optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    /// The extra LLVM arguments.
    pub llvm_options: Vec<String>,
    /// The debug output config.
    pub debug_config: Option<era_compiler_llvm_context::DebugConfig>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        contract: Contract,
        identifier_paths: BTreeMap<String, String>,
        output_bytecode: bool,
        deployed_libraries: BTreeSet<String>,
        metadata_hash_type: era_compiler_common::HashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            contract,
            identifier_paths,
            output_bytecode,
            deployed_libraries,
            metadata_hash_type,
            optimizer_settings,
            llvm_options,
            debug_config,
        }
    }
}
