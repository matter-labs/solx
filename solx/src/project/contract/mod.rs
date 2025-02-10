//!
//! The contract data.
//!

pub mod ir;
pub mod metadata;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use era_compiler_llvm_context::IContext;

use crate::build_evm::contract::Contract as EVMContractBuild;
use crate::yul::parser::wrapper::Wrap;

use self::ir::IR;
use self::metadata::Metadata;

///
/// The contract data.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The contract name.
    pub name: era_compiler_common::ContractName,
    /// The IR source code data.
    pub ir: IR,
    /// The metadata JSON.
    pub source_metadata: serde_json::Value,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: era_compiler_common::ContractName,
        ir: IR,
        source_metadata: serde_json::Value,
    ) -> Self {
        Self {
            name,
            ir,
            source_metadata,
        }
    }

    ///
    /// Returns the contract identifier, which is:
    /// - the Yul object identifier for Yul
    /// - the full contract path for EVM legacy assembly
    /// - the module name for LLVM IR
    ///
    pub fn identifier(&self) -> &str {
        match self.ir {
            IR::Yul(ref yul) => yul.object.0.identifier.as_str(),
            IR::EVMLA(ref evm) => evm.assembly.full_path(),
            IR::LLVMIR(ref llvm_ir) => llvm_ir.path.as_str(),
        }
    }

    ///
    /// Compiles the specified contract to EVM, returning its build artifacts.
    ///
    pub fn compile_to_evm(
        self,
        identifier_paths: BTreeMap<String, String>,
        missing_libraries: BTreeSet<String>,
        metadata_hash_type: era_compiler_common::HashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EVMContractBuild> {
        use era_compiler_llvm_context::EVMWriteLLVM;

        let solc_version = solx_solc::Compiler::default().version;

        let identifier = self.identifier().to_owned();

        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings);

        let metadata = Metadata::new(
            self.source_metadata,
            solc_version.default.to_owned(),
            solc_version.l2_revision.to_owned(),
            optimizer.settings().to_owned(),
            llvm_options.as_slice(),
        );
        let metadata_json = serde_json::to_value(&metadata).expect("Always valid");
        let metadata_bytes = serde_json::to_vec(&metadata_json).expect("Always valid");
        let metadata_hash = match metadata_hash_type {
            era_compiler_common::HashType::None => None,
            era_compiler_common::HashType::Keccak256 => Some(era_compiler_common::Hash::keccak256(
                metadata_bytes.as_slice(),
            )),
            era_compiler_common::HashType::Ipfs => {
                Some(era_compiler_common::Hash::ipfs(metadata_bytes.as_slice()))
            }
        };

        match self.ir {
            IR::Yul(mut deploy_code) => {
                let mut runtime_code = deploy_code
                    .take_runtime_code()
                    .ok_or_else(|| anyhow::anyhow!("Contract `{identifier}` has no runtime code"))?
                    .wrap();

                let deploy_code_identifier = deploy_code.object.0.identifier.clone();
                let runtime_code_identifier = runtime_code.0.identifier.clone();

                let runtime_code_segment = era_compiler_common::CodeSegment::Runtime;
                let runtime_llvm = inkwell::context::Context::create();
                let runtime_module = runtime_llvm.create_module(
                    format!("{}.{runtime_code_segment}", self.name.full_path).as_str(),
                );
                let mut runtime_context = era_compiler_llvm_context::EVMContext::new(
                    &runtime_llvm,
                    runtime_module,
                    llvm_options.clone(),
                    runtime_code_segment,
                    optimizer.clone(),
                    debug_config.clone(),
                );
                runtime_context.set_yul_data(era_compiler_llvm_context::EVMContextYulData::new(
                    identifier_paths.clone(),
                ));
                runtime_code.declare(&mut runtime_context)?;
                runtime_code
                    .into_llvm(&mut runtime_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{runtime_code_segment} code LLVM IR generator: {error}")
                    })?;
                let runtime_buffer = runtime_context.build(self.name.path.as_str())?;

                let immutables_map = runtime_buffer.get_immutables_evm();

                let deploy_code_segment = era_compiler_common::CodeSegment::Deploy;
                let deploy_llvm = inkwell::context::Context::create();
                let deploy_module = deploy_llvm.create_module(
                    format!("{}.{deploy_code_segment}", self.name.full_path).as_str(),
                );
                let mut deploy_context = era_compiler_llvm_context::EVMContext::new(
                    &deploy_llvm,
                    deploy_module,
                    llvm_options.clone(),
                    deploy_code_segment,
                    optimizer.clone(),
                    debug_config.clone(),
                );
                deploy_context.set_solidity_data(
                    era_compiler_llvm_context::EVMContextSolidityData::new(immutables_map),
                );
                deploy_context.set_yul_data(era_compiler_llvm_context::EVMContextYulData::new(
                    identifier_paths,
                ));
                deploy_code.declare(&mut deploy_context)?;
                deploy_code
                    .into_llvm(&mut deploy_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{deploy_code_segment} code LLVM IR generator: {error}")
                    })?;
                let deploy_buffer = deploy_context.build(self.name.path.as_str())?;

                Ok(EVMContractBuild::new(
                    self.name,
                    deploy_code_identifier,
                    deploy_buffer.as_slice().to_owned(),
                    runtime_code_identifier,
                    runtime_buffer.as_slice().to_owned(),
                    metadata_hash,
                    metadata_json,
                    missing_libraries,
                    era_compiler_common::ObjectFormat::ELF,
                ))
            }
            IR::EVMLA(mut deploy_code) => {
                let mut runtime_code_assembly = deploy_code.assembly.runtime_code()?.to_owned();
                runtime_code_assembly.set_full_path(deploy_code.assembly.full_path().to_owned());

                let evmla_data =
                    era_compiler_llvm_context::EVMContextEVMLAData::new(solc_version.default);

                let deploy_code_segment = era_compiler_common::CodeSegment::Deploy;
                let runtime_code_segment = era_compiler_common::CodeSegment::Runtime;

                let deploy_code_identifier =
                    format!("{}.{deploy_code_segment}", self.name.full_path);
                let runtime_code_identifier =
                    format!("{}.{runtime_code_segment}", self.name.full_path);

                let runtime_llvm = inkwell::context::Context::create();
                let runtime_module = runtime_llvm.create_module(
                    format!("{}.{runtime_code_segment}", self.name.full_path).as_str(),
                );
                let mut runtime_context = era_compiler_llvm_context::EVMContext::new(
                    &runtime_llvm,
                    runtime_module,
                    llvm_options.clone(),
                    runtime_code_segment,
                    optimizer.clone(),
                    debug_config.clone(),
                );
                runtime_context.set_evmla_data(evmla_data.clone());
                runtime_code_assembly.declare(&mut runtime_context)?;
                runtime_code_assembly
                    .into_llvm(&mut runtime_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{runtime_code_segment} code LLVM IR generator: {error}")
                    })?;
                let runtime_buffer = runtime_context.build(self.name.path.as_str())?;

                let immutables_map = runtime_buffer.get_immutables_evm();

                let deploy_llvm = inkwell::context::Context::create();
                let deploy_module = deploy_llvm.create_module(
                    format!("{}.{deploy_code_segment}", self.name.full_path).as_str(),
                );
                let mut deploy_context = era_compiler_llvm_context::EVMContext::new(
                    &deploy_llvm,
                    deploy_module,
                    llvm_options.clone(),
                    deploy_code_segment,
                    optimizer.clone(),
                    debug_config.clone(),
                );
                deploy_context.set_solidity_data(
                    era_compiler_llvm_context::EVMContextSolidityData::new(immutables_map),
                );
                deploy_context.set_evmla_data(evmla_data);
                deploy_code.declare(&mut deploy_context)?;
                deploy_code
                    .into_llvm(&mut deploy_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{deploy_code_segment} code LLVM IR generator: {error}")
                    })?;
                let deploy_buffer = deploy_context.build(self.name.path.as_str())?;

                Ok(EVMContractBuild::new(
                    self.name,
                    deploy_code_identifier,
                    deploy_buffer.as_slice().to_owned(),
                    runtime_code_identifier,
                    runtime_buffer.as_slice().to_owned(),
                    metadata_hash,
                    metadata_json,
                    missing_libraries,
                    era_compiler_common::ObjectFormat::ELF,
                ))
            }
            IR::LLVMIR(mut llvm_ir) => {
                let llvm = inkwell::context::Context::create();

                llvm_ir.source.push(char::from(0));
                let input = llvm_ir.source.as_bytes();
                let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                    &input[..input.len() - 1],
                    self.name.full_path.as_str(),
                    true,
                );

                let module = llvm
                    .create_module_from_ir(memory_buffer)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
                let context = era_compiler_llvm_context::EVMContext::new(
                    &llvm,
                    module,
                    llvm_options,
                    era_compiler_common::CodeSegment::Runtime,
                    optimizer,
                    debug_config,
                );
                let runtime_build = context.build(self.name.path.as_str())?;

                Ok(EVMContractBuild::new(
                    self.name,
                    identifier.clone(),
                    vec![],
                    identifier,
                    runtime_build.as_slice().to_owned(),
                    metadata_hash,
                    metadata_json,
                    missing_libraries,
                    era_compiler_common::ObjectFormat::ELF,
                ))
            }
        }
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self, deployed_libraries: &BTreeSet<String>) -> BTreeSet<String> {
        self.ir
            .get_missing_libraries()
            .into_iter()
            .filter(|library| !deployed_libraries.contains(library))
            .collect::<BTreeSet<String>>()
    }
}
