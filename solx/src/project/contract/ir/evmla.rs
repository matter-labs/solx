//!
//! The contract EVM legacy assembly source code.
//!

use std::collections::BTreeSet;

use crate::evmla::assembly::Assembly;

///
/// The contract EVM legacy assembly source code.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EVMLegacyAssembly {
    /// The EVM legacy assembly source code.
    pub assembly: Assembly,
}

impl EVMLegacyAssembly {
    ///
    /// Transforms the `solc` standard JSON output contract into an EVM legacy assembly object.
    ///
    pub fn try_from_contract(
        legacy_assembly: serde_json::Value,
        extra_metadata: Option<solx_standard_json::OutputContractEVMExtraMetadata>,
    ) -> Option<Self> {
        let mut assembly: Assembly = serde_json::from_value(legacy_assembly).ok()?;
        assembly.extra_metadata = extra_metadata.clone();
        if let Ok(runtime_code) = assembly.runtime_code_mut() {
            runtime_code.extra_metadata = extra_metadata;
        }

        Some(Self { assembly })
    }

    ///
    /// Get the list of unlinked deployable libraries.
    ///
    pub fn get_unlinked_libraries(&self) -> BTreeSet<String> {
        self.assembly.get_unlinked_libraries()
    }

    ///
    /// Get the list of EVM dependencies.
    ///
    pub fn accumulate_evm_dependencies(&self, dependencies: &mut solx_yul::Dependencies) {
        self.assembly.accumulate_evm_dependencies(dependencies);
    }
}

impl era_compiler_llvm_context::EVMWriteLLVM for EVMLegacyAssembly {
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EVMContext,
    ) -> anyhow::Result<()> {
        self.assembly.declare(context)
    }

    fn into_llvm(self, context: &mut era_compiler_llvm_context::EVMContext) -> anyhow::Result<()> {
        self.assembly.into_llvm(context)
    }
}
