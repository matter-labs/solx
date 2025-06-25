//!
//! Process for compiling a single compilation unit.
//!
//! The EVM input data.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::project::contract::ir::IR as ContractIR;

///
/// The EVM input data.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Input {
    /// The input contract name.
    pub contract_name: era_compiler_common::ContractName,
    /// The input contract IR.
    pub contract_ir: ContractIR,
    /// The code segment.
    pub code_segment: era_compiler_common::CodeSegment,
    /// The mapping of auxiliary identifiers, e.g. Yul object names, to full contract paths.
    pub identifier_paths: BTreeMap<String, String>,
    /// Output selection for the compilation.
    pub output_selection: solx_standard_json::InputSelection,
    /// Immutables produced by the runtime code run.
    pub immutables: Option<BTreeMap<String, BTreeSet<u64>>>,
    /// The metadata bytes.
    pub metadata_bytes: Option<Vec<u8>>,
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
        contract_name: era_compiler_common::ContractName,
        contract_ir: ContractIR,
        code_segment: era_compiler_common::CodeSegment,
        identifier_paths: BTreeMap<String, String>,
        output_selection: solx_standard_json::InputSelection,
        immutables: Option<BTreeMap<String, BTreeSet<u64>>>,
        metadata_bytes: Option<Vec<u8>>,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            contract_name,
            contract_ir,
            code_segment,
            identifier_paths,
            output_selection,
            immutables,
            metadata_bytes,
            optimizer_settings,
            llvm_options,
            debug_config,
        }
    }
}
