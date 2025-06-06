//!
//! Process for compiling a single compilation unit.
//!
//! The EVM input data.
//!

use std::collections::BTreeMap;

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
    /// Output selection for the compilation.
    pub output_selection: solx_standard_json::InputSelection,
    /// The metadata hash type.
    pub metadata_hash_type: era_compiler_common::EVMMetadataHashType,
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
        output_selection: solx_standard_json::InputSelection,
        metadata_hash_type: era_compiler_common::EVMMetadataHashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            contract,
            identifier_paths,
            output_selection,
            metadata_hash_type,
            optimizer_settings,
            llvm_options,
            debug_config,
        }
    }
}
