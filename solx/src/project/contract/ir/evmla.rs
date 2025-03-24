//!
//! The contract EVM legacy assembly source code.
//!

use std::collections::BTreeSet;

use crate::evmla::assembly::Assembly;

///
/// The contract EVM legacy assembly source code.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EVMLA {
    /// The EVM legacy assembly source code.
    pub assembly: Assembly,
}

impl EVMLA {
    ///
    /// Transforms the `solc` standard JSON output contract into an EVM legacy assembly object.
    ///
    pub fn try_from_contract(contract: &solx_solc::StandardJsonOutputContract) -> Option<Self> {
        let evm = contract.evm.as_ref()?;

        let mut assembly: Assembly = serde_json::from_value(evm.legacy_assembly.to_owned()).ok()?;
        assembly.extra_metadata = evm.extra_metadata.to_owned();
        if let Ok(runtime_code) = assembly.runtime_code_mut() {
            runtime_code.extra_metadata = evm.extra_metadata.to_owned();
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

impl era_compiler_llvm_context::EVMWriteLLVM for EVMLA {
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
