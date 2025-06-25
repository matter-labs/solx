//!
//! The contract data.
//!

pub mod ir;
pub mod metadata;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use era_compiler_llvm_context::IContext;

use crate::build::contract::object::Object as EVMContractObject;
use crate::error::Error;
use crate::yul::parser::wrapper::Wrap;

use self::ir::llvm_ir::LLVMIR;
use self::ir::IR;

///
/// The contract data.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The contract name.
    pub name: era_compiler_common::ContractName,
    /// The IR source code data.
    pub ir: IR,
    /// The solc metadata.
    pub metadata: Option<String>,
    /// The solc ABI.
    pub abi: Option<serde_json::Value>,
    /// The solc method identifiers.
    pub method_identifiers: Option<BTreeMap<String, String>>,
    /// The solc user documentation.
    pub userdoc: Option<serde_json::Value>,
    /// The solc developer documentation.
    pub devdoc: Option<serde_json::Value>,
    /// The solc storage layout.
    pub storage_layout: Option<serde_json::Value>,
    /// The solc transient storage layout.
    pub transient_storage_layout: Option<serde_json::Value>,
    /// the solc EVM legacy assembly.
    pub legacy_assembly: Option<solx_evm_assembly::Assembly>,
    /// the solc optimized Yul IR assembly.
    pub ir_optimized: Option<String>,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: era_compiler_common::ContractName,
        ir: IR,
        metadata: Option<String>,
        abi: Option<serde_json::Value>,
        method_identifiers: Option<BTreeMap<String, String>>,
        userdoc: Option<serde_json::Value>,
        devdoc: Option<serde_json::Value>,
        storage_layout: Option<serde_json::Value>,
        transient_storage_layout: Option<serde_json::Value>,
        legacy_assembly: Option<solx_evm_assembly::Assembly>,
        ir_optimized: Option<String>,
    ) -> Self {
        Self {
            name,
            ir,
            metadata,
            abi,
            method_identifiers,
            userdoc,
            devdoc,
            storage_layout,
            transient_storage_layout,
            legacy_assembly,
            ir_optimized,
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
            IR::EVMLegacyAssembly(ref evm) => evm.assembly.full_path(),
            IR::LLVMIR(ref llvm_ir) => llvm_ir.path.as_str(),
        }
    }

    ///
    /// Compiles the specified contract to EVM, returning its build artifacts.
    ///
    pub fn compile_to_evm(
        self,
        code_segment: era_compiler_common::CodeSegment,
        identifier_paths: BTreeMap<String, String>,
        output_selection: solx_standard_json::InputSelection,
        immutables: Option<BTreeMap<String, BTreeSet<u64>>>,
        metadata_bytes: Option<Vec<u8>>,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Result<EVMContractObject, Error> {
        use era_compiler_llvm_context::EVMWriteLLVM;

        let solc_version = solx_solc::Compiler::default().version;
        let identifier = self.identifier().to_owned();
        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings);
        let output_bytecode = output_selection.is_bytecode_set_for_any();

        match (self.ir, code_segment) {
            (IR::Yul(mut deploy_code), era_compiler_common::CodeSegment::Deploy) => {
                let runtime_code = deploy_code.take_runtime_code().ok_or_else(|| {
                    anyhow::anyhow!("Contract `{identifier}` has no runtime code")
                })?;
                let deploy_code_dependencies =
                    deploy_code.get_evm_dependencies(Some(&runtime_code));
                let deploy_code_identifier = deploy_code.object.0.identifier.clone();

                let deploy_llvm = inkwell::context::Context::create();
                let deploy_module = deploy_llvm.create_module(self.name.full_path.as_str());
                let mut deploy_context = era_compiler_llvm_context::EVMContext::new(
                    &deploy_llvm,
                    deploy_module,
                    llvm_options.clone(),
                    code_segment,
                    optimizer.clone(),
                    debug_config.clone(),
                );
                inkwell::context::Context::install_stack_error_handler(
                    crate::process::evm_stack_error_handler,
                );
                deploy_context.set_solidity_data(
                    era_compiler_llvm_context::EVMContextSolidityData::new(
                        immutables.expect("Always exists"),
                    ),
                );
                deploy_context.set_yul_data(era_compiler_llvm_context::EVMContextYulData::new(
                    identifier_paths,
                ));
                deploy_code.declare(&mut deploy_context)?;
                deploy_code
                    .into_llvm(&mut deploy_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{code_segment} code LLVM IR generator: {error}")
                    })?;
                let deploy_build = deploy_context.build(
                    output_selection.check_selection(
                        self.name.path.as_str(),
                        self.name.name.as_deref(),
                        solx_standard_json::InputSelector::BytecodeLLVMAssembly,
                    ),
                    output_bytecode,
                    false,
                )?;
                let deploy_object = EVMContractObject::new(
                    deploy_code_identifier,
                    self.name.clone(),
                    deploy_build.assembly,
                    deploy_build.bytecode,
                    true,
                    code_segment,
                    None,
                    None,
                    deploy_code_dependencies,
                    deploy_build.warnings,
                );
                Ok(deploy_object)
            }
            (IR::Yul(mut deploy_code), era_compiler_common::CodeSegment::Runtime) => {
                let runtime_code = deploy_code.take_runtime_code().ok_or_else(|| {
                    anyhow::anyhow!("Contract `{identifier}` has no runtime code")
                })?;
                let runtime_code_dependencies = runtime_code.get_evm_dependencies(None);
                let mut runtime_code = runtime_code.wrap();
                let runtime_code_identifier = runtime_code.0.identifier.clone();

                let runtime_llvm = inkwell::context::Context::create();
                let runtime_module = runtime_llvm
                    .create_module(format!("{}.{code_segment}", self.name.full_path).as_str());
                let mut runtime_context = era_compiler_llvm_context::EVMContext::new(
                    &runtime_llvm,
                    runtime_module,
                    llvm_options.clone(),
                    code_segment,
                    optimizer.clone(),
                    debug_config.clone(),
                );
                inkwell::context::Context::install_stack_error_handler(
                    crate::process::evm_stack_error_handler,
                );
                runtime_context.set_yul_data(era_compiler_llvm_context::EVMContextYulData::new(
                    identifier_paths.clone(),
                ));
                runtime_code.declare(&mut runtime_context)?;
                runtime_code
                    .into_llvm(&mut runtime_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{code_segment} code LLVM IR generator: {error}")
                    })?;
                let runtime_build = runtime_context.build(
                    output_selection.check_selection(
                        self.name.path.as_str(),
                        self.name.name.as_deref(),
                        solx_standard_json::InputSelector::RuntimeBytecodeLLVMAssembly,
                    ),
                    output_bytecode,
                    false,
                )?;
                let immutables = runtime_build.immutables.unwrap_or_default();
                let runtime_object = EVMContractObject::new(
                    runtime_code_identifier,
                    self.name.clone(),
                    runtime_build.assembly,
                    runtime_build.bytecode,
                    true,
                    code_segment,
                    Some(immutables),
                    metadata_bytes,
                    runtime_code_dependencies,
                    runtime_build.warnings,
                );
                Ok(runtime_object)
            }
            (IR::EVMLegacyAssembly(mut deploy_code), era_compiler_common::CodeSegment::Deploy) => {
                let evmla_data =
                    era_compiler_llvm_context::EVMContextEVMLAData::new(solc_version.default);
                let deploy_code_identifier = self.name.full_path.to_owned();
                let mut deploy_code_dependencies =
                    solx_yul::Dependencies::new(deploy_code_identifier.as_str());
                deploy_code.accumulate_evm_dependencies(&mut deploy_code_dependencies);

                let deploy_llvm = inkwell::context::Context::create();
                let deploy_module = deploy_llvm.create_module(deploy_code_identifier.as_str());
                let mut deploy_context = era_compiler_llvm_context::EVMContext::new(
                    &deploy_llvm,
                    deploy_module,
                    llvm_options.clone(),
                    code_segment,
                    optimizer.clone(),
                    debug_config.clone(),
                );
                inkwell::context::Context::install_stack_error_handler(
                    crate::process::evm_stack_error_handler,
                );
                deploy_context.set_solidity_data(
                    era_compiler_llvm_context::EVMContextSolidityData::new(
                        immutables.expect("Always exists"),
                    ),
                );
                deploy_context.set_evmla_data(evmla_data);
                deploy_code.declare(&mut deploy_context)?;
                deploy_code
                    .into_llvm(&mut deploy_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{code_segment} code LLVM IR generator: {error}")
                    })?;
                let deploy_build = deploy_context.build(
                    output_selection.check_selection(
                        self.name.path.as_str(),
                        self.name.name.as_deref(),
                        solx_standard_json::InputSelector::BytecodeLLVMAssembly,
                    ),
                    output_bytecode,
                    false,
                )?;
                let deploy_object = EVMContractObject::new(
                    deploy_code_identifier,
                    self.name.clone(),
                    deploy_build.assembly,
                    deploy_build.bytecode,
                    false,
                    code_segment,
                    None,
                    None,
                    deploy_code_dependencies,
                    deploy_build.warnings,
                );
                Ok(deploy_object)
            }
            (IR::EVMLegacyAssembly(deploy_code), era_compiler_common::CodeSegment::Runtime) => {
                let evmla_data =
                    era_compiler_llvm_context::EVMContextEVMLAData::new(solc_version.default);
                let mut runtime_code_assembly = deploy_code.assembly.runtime_code()?.to_owned();
                runtime_code_assembly.set_full_path(deploy_code.assembly.full_path().to_owned());
                let runtime_code_identifier = format!("{}.{code_segment}", self.name.full_path);
                let mut runtime_code_dependencies =
                    solx_yul::Dependencies::new(runtime_code_identifier.as_str());
                runtime_code_assembly.accumulate_evm_dependencies(&mut runtime_code_dependencies);

                let runtime_llvm = inkwell::context::Context::create();
                let runtime_module = runtime_llvm.create_module(runtime_code_identifier.as_str());
                let mut runtime_context = era_compiler_llvm_context::EVMContext::new(
                    &runtime_llvm,
                    runtime_module,
                    llvm_options.clone(),
                    code_segment,
                    optimizer.clone(),
                    debug_config.clone(),
                );
                inkwell::context::Context::install_stack_error_handler(
                    crate::process::evm_stack_error_handler,
                );
                runtime_context.set_evmla_data(evmla_data.clone());
                runtime_code_assembly.declare(&mut runtime_context)?;
                runtime_code_assembly
                    .into_llvm(&mut runtime_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{code_segment} code LLVM IR generator: {error}")
                    })?;
                let runtime_build = runtime_context.build(
                    output_selection.check_selection(
                        self.name.path.as_str(),
                        self.name.name.as_deref(),
                        solx_standard_json::InputSelector::RuntimeBytecodeLLVMAssembly,
                    ),
                    output_bytecode,
                    false,
                )?;
                let immutables = runtime_build.immutables.unwrap_or_default();
                let runtime_object = EVMContractObject::new(
                    runtime_code_identifier,
                    self.name.clone(),
                    runtime_build.assembly,
                    runtime_build.bytecode,
                    false,
                    code_segment,
                    Some(immutables),
                    metadata_bytes,
                    runtime_code_dependencies,
                    runtime_build.warnings,
                );
                Ok(runtime_object)
            }
            (IR::LLVMIR(_), era_compiler_common::CodeSegment::Deploy) => {
                let deploy_code_identifier = self.name.full_path.to_owned();
                let runtime_code_identifier = format!(
                    "{}.{}",
                    self.name.full_path,
                    era_compiler_common::CodeSegment::Runtime
                );
                let mut deploy_llvm_ir = LLVMIR::new(
                    deploy_code_identifier.clone(),
                    era_compiler_llvm_context::evm_minimal_deploy_code(
                        deploy_code_identifier.as_str(),
                        runtime_code_identifier.as_str(),
                    ),
                );
                deploy_llvm_ir.source.push(char::from(0));
                let deploy_memory_buffer =
                    inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                        &deploy_llvm_ir.source.as_bytes()[..deploy_llvm_ir.source.len() - 1],
                        deploy_code_identifier.as_str(),
                        true,
                    );
                let mut deploy_code_dependencies =
                    solx_yul::Dependencies::new(deploy_code_identifier.as_str());
                deploy_code_dependencies.push(runtime_code_identifier.to_owned(), true);

                let deploy_llvm = inkwell::context::Context::create();
                let deploy_module = deploy_llvm
                    .create_module_from_ir(deploy_memory_buffer)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
                let mut deploy_context = era_compiler_llvm_context::EVMContext::new(
                    &deploy_llvm,
                    deploy_module,
                    llvm_options,
                    code_segment,
                    optimizer,
                    debug_config,
                );
                inkwell::context::Context::install_stack_error_handler(
                    crate::process::evm_stack_error_handler,
                );
                let deploy_build = deploy_context.build(
                    output_selection.check_selection(
                        self.name.path.as_str(),
                        self.name.name.as_deref(),
                        solx_standard_json::InputSelector::BytecodeLLVMAssembly,
                    ),
                    output_bytecode,
                    false,
                )?;
                let deploy_object = EVMContractObject::new(
                    deploy_code_identifier,
                    self.name.clone(),
                    deploy_build.assembly,
                    deploy_build.bytecode,
                    false,
                    code_segment,
                    None,
                    None,
                    deploy_code_dependencies,
                    deploy_build.warnings,
                );
                Ok(deploy_object)
            }
            (IR::LLVMIR(mut runtime_llvm_ir), era_compiler_common::CodeSegment::Runtime) => {
                let runtime_code_identifier = format!("{}.{code_segment}", self.name.full_path);
                runtime_llvm_ir.source.push(char::from(0));
                let runtime_memory_buffer =
                    inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                        &runtime_llvm_ir.source.as_bytes()[..runtime_llvm_ir.source.len() - 1],
                        runtime_code_identifier.as_str(),
                        true,
                    );
                let runtime_code_dependencies =
                    solx_yul::Dependencies::new(runtime_code_identifier.as_str());

                let runtime_llvm = inkwell::context::Context::create();
                let runtime_module = runtime_llvm
                    .create_module_from_ir(runtime_memory_buffer)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
                let mut runtime_context = era_compiler_llvm_context::EVMContext::new(
                    &runtime_llvm,
                    runtime_module,
                    llvm_options.clone(),
                    code_segment,
                    optimizer.clone(),
                    debug_config.clone(),
                );
                inkwell::context::Context::install_stack_error_handler(
                    crate::process::evm_stack_error_handler,
                );
                let runtime_build = runtime_context.build(
                    output_selection.check_selection(
                        self.name.path.as_str(),
                        self.name.name.as_deref(),
                        solx_standard_json::InputSelector::RuntimeBytecodeLLVMAssembly,
                    ),
                    output_bytecode,
                    false,
                )?;
                let runtime_object = EVMContractObject::new(
                    runtime_code_identifier,
                    self.name.clone(),
                    runtime_build.assembly,
                    runtime_build.bytecode,
                    false,
                    code_segment,
                    Some(BTreeMap::new()),
                    metadata_bytes,
                    runtime_code_dependencies,
                    runtime_build.warnings,
                );
                Ok(runtime_object)
            }
        }
    }
}
