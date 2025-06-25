//!
//! The contract EVM legacy assembly source code.
//!

use std::collections::BTreeSet;

///
/// The contract EVM legacy assembly source code.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EVMLegacyAssembly {
    /// The EVM legacy assembly source code.
    pub assembly: solx_evm_assembly::Assembly,
    /// Dependencies of the EVM assembly object.
    pub dependencies: solx_yul::Dependencies,
    /// Runtime code object that is only set in deploy code.
    pub runtime_code: Option<Box<Self>>,
}

impl EVMLegacyAssembly {
    ///
    /// Transforms the `solc` standard JSON output contract into an EVM legacy assembly object.
    ///
    pub fn try_from_contract(
        mut assembly: solx_evm_assembly::Assembly,
        extra_metadata: Option<solx_evm_assembly::ExtraMetadata>,
    ) -> Option<Self> {
        assembly.extra_metadata = extra_metadata.clone();
        if let Ok(runtime_code) = assembly.runtime_code_mut() {
            runtime_code.extra_metadata = extra_metadata;
        }
        let full_path = assembly.full_path().to_owned();

        let mut runtime_code_assembly = assembly.runtime_code().expect("Always exists").to_owned();
        runtime_code_assembly.set_full_path(full_path.clone());
        let runtime_code_identifier =
            format!("{full_path}.{}", era_compiler_common::CodeSegment::Runtime);
        let mut runtime_code_dependencies =
            solx_yul::Dependencies::new(runtime_code_identifier.as_str());
        runtime_code_assembly.accumulate_evm_dependencies(&mut runtime_code_dependencies);
        let runtime_code = Some(Box::new(Self {
            assembly: runtime_code_assembly,
            dependencies: runtime_code_dependencies,
            runtime_code: None,
        }));

        let mut deploy_code_dependencies = solx_yul::Dependencies::new(full_path.as_str());
        assembly.accumulate_evm_dependencies(&mut deploy_code_dependencies);

        Some(Self {
            assembly,
            dependencies: deploy_code_dependencies,
            runtime_code,
        })
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
