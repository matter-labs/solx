//!
//! Contract data.
//!

pub mod ir;
pub mod metadata;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use era_compiler_llvm_context::IContext;

use crate::build::contract::object::Object as EVMContractObject;
use crate::error::Error;

use self::ir::IR;

///
/// Contract data.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// Contract name.
    pub name: era_compiler_common::ContractName,
    /// IR source code data.
    pub ir: IR,
    /// solc metadata.
    pub metadata: Option<String>,
    /// solc ABI.
    pub abi: Option<serde_json::Value>,
    /// solc method identifiers.
    pub method_identifiers: Option<BTreeMap<String, String>>,
    /// solc user documentation.
    pub userdoc: Option<serde_json::Value>,
    /// solc developer documentation.
    pub devdoc: Option<serde_json::Value>,
    /// solc storage layout.
    pub storage_layout: Option<serde_json::Value>,
    /// solc transient storage layout.
    pub transient_storage_layout: Option<serde_json::Value>,
    /// solc EVM legacy assembly.
    pub legacy_assembly: Option<solx_evm_assembly::Assembly>,
    /// solc Yul IR.
    pub yul: Option<String>,
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
        yul: Option<String>,
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
            yul,
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
        contract_name: era_compiler_common::ContractName,
        contract_ir: IR,
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
        let mut profiler = era_compiler_llvm_context::EVMProfiler::default();

        let solc_version = solx_solc::Compiler::default().version;
        let solidity_data = era_compiler_llvm_context::EVMContextSolidityData::new(immutables);
        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings.clone());
        let output_bytecode = output_selection.is_bytecode_set_for_any();

        match (contract_ir, code_segment) {
            (IR::Yul(mut yul), era_compiler_common::CodeSegment::Deploy) => {
                let deploy_code_identifier = yul.object.0.identifier.clone();

                let deploy_llvm = inkwell::context::Context::create();
                let deploy_module = deploy_llvm.create_module(contract_name.full_path.as_str());
                let mut deploy_context = era_compiler_llvm_context::EVMContext::new(
                    &deploy_llvm,
                    deploy_module,
                    llvm_options.clone(),
                    code_segment,
                    optimizer,
                    debug_config.clone(),
                );
                inkwell::context::Context::install_stack_error_handler(
                    crate::process::evm_stack_error_handler,
                );
                deploy_context.set_solidity_data(solidity_data);
                deploy_context.set_yul_data(era_compiler_llvm_context::EVMContextYulData::new(
                    identifier_paths,
                ));
                let run_yul_lowering = profiler.start_evm_translation_unit(
                    contract_name.full_path.as_str(),
                    code_segment,
                    "YulToLLVMIR",
                    &optimizer_settings,
                );
                yul.object.declare(&mut deploy_context)?;
                yul.object.into_llvm(&mut deploy_context).map_err(|error| {
                    anyhow::anyhow!("{code_segment} code LLVM IR generator: {error}")
                })?;
                run_yul_lowering.borrow_mut().finish();
                let deploy_build = deploy_context.build(
                    output_selection.check_selection(
                        contract_name.path.as_str(),
                        contract_name.name.as_deref(),
                        solx_standard_json::InputSelector::BytecodeLLVMAssembly,
                    ),
                    output_bytecode,
                    false,
                    &mut profiler,
                )?;
                let deploy_object = EVMContractObject::new(
                    deploy_code_identifier,
                    contract_name.clone(),
                    deploy_build.assembly,
                    deploy_build.bytecode,
                    true,
                    code_segment,
                    None,
                    None,
                    yul.dependencies,
                    deploy_build.is_size_fallback,
                    deploy_build.warnings,
                    profiler.to_vec(),
                );
                Ok(deploy_object)
            }
            (IR::Yul(mut yul), era_compiler_common::CodeSegment::Runtime) => {
                let runtime_code_identifier = yul.object.0.identifier.clone();

                let runtime_llvm = inkwell::context::Context::create();
                let runtime_module = runtime_llvm
                    .create_module(format!("{}.{code_segment}", contract_name.full_path).as_str());
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
                runtime_context.set_solidity_data(solidity_data);
                runtime_context.set_yul_data(era_compiler_llvm_context::EVMContextYulData::new(
                    identifier_paths.clone(),
                ));
                let run_yul_lowering = profiler.start_evm_translation_unit(
                    contract_name.full_path.as_str(),
                    code_segment,
                    "YulToLLVMIR",
                    &optimizer_settings,
                );
                yul.object.declare(&mut runtime_context)?;
                yul.object
                    .into_llvm(&mut runtime_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{code_segment} code LLVM IR generator: {error}")
                    })?;
                run_yul_lowering.borrow_mut().finish();
                let runtime_build = runtime_context.build(
                    output_selection.check_selection(
                        contract_name.path.as_str(),
                        contract_name.name.as_deref(),
                        solx_standard_json::InputSelector::RuntimeBytecodeLLVMAssembly,
                    ),
                    output_bytecode,
                    false,
                    &mut profiler,
                )?;
                let immutables = runtime_build.immutables.unwrap_or_default();
                let runtime_object = EVMContractObject::new(
                    runtime_code_identifier,
                    contract_name.clone(),
                    runtime_build.assembly,
                    runtime_build.bytecode,
                    true,
                    code_segment,
                    Some(immutables),
                    metadata_bytes,
                    yul.dependencies,
                    runtime_build.is_size_fallback,
                    runtime_build.warnings,
                    profiler.to_vec(),
                );
                Ok(runtime_object)
            }
            (IR::EVMLegacyAssembly(mut deploy_code), era_compiler_common::CodeSegment::Deploy) => {
                let evmla_data =
                    era_compiler_llvm_context::EVMContextEVMLAData::new(solc_version.default);
                let deploy_code_identifier = contract_name.full_path.to_owned();
                let mut deploy_code_dependencies =
                    solx_yul::Dependencies::new(deploy_code_identifier.as_str());
                deploy_code
                    .assembly
                    .accumulate_evm_dependencies(&mut deploy_code_dependencies);

                let deploy_llvm = inkwell::context::Context::create();
                let deploy_module = deploy_llvm.create_module(deploy_code_identifier.as_str());
                let mut deploy_context = era_compiler_llvm_context::EVMContext::new(
                    &deploy_llvm,
                    deploy_module,
                    llvm_options.clone(),
                    code_segment,
                    optimizer,
                    debug_config.clone(),
                );
                inkwell::context::Context::install_stack_error_handler(
                    crate::process::evm_stack_error_handler,
                );
                deploy_context.set_solidity_data(solidity_data);
                deploy_context.set_evmla_data(evmla_data);
                let run_evm_assembly_lowering = profiler.start_evm_translation_unit(
                    contract_name.full_path.as_str(),
                    code_segment,
                    "EVMAssemblyToLLVMIR",
                    &optimizer_settings,
                );
                deploy_code.assembly.declare(&mut deploy_context)?;
                deploy_code
                    .assembly
                    .into_llvm(&mut deploy_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{code_segment} code LLVM IR generator: {error}")
                    })?;
                run_evm_assembly_lowering.borrow_mut().finish();
                let deploy_build = deploy_context.build(
                    output_selection.check_selection(
                        contract_name.path.as_str(),
                        contract_name.name.as_deref(),
                        solx_standard_json::InputSelector::BytecodeLLVMAssembly,
                    ),
                    output_bytecode,
                    false,
                    &mut profiler,
                )?;
                let deploy_object = EVMContractObject::new(
                    deploy_code_identifier,
                    contract_name.clone(),
                    deploy_build.assembly,
                    deploy_build.bytecode,
                    false,
                    code_segment,
                    None,
                    None,
                    deploy_code_dependencies,
                    deploy_build.is_size_fallback,
                    deploy_build.warnings,
                    profiler.to_vec(),
                );
                Ok(deploy_object)
            }
            (
                IR::EVMLegacyAssembly(mut runtime_code),
                era_compiler_common::CodeSegment::Runtime,
            ) => {
                let runtime_code_identifier = format!("{}.{code_segment}", contract_name.full_path);
                let evmla_data =
                    era_compiler_llvm_context::EVMContextEVMLAData::new(solc_version.default);

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
                runtime_context.set_solidity_data(solidity_data);
                runtime_context.set_evmla_data(evmla_data.clone());
                let run_evm_assembly_lowering = profiler.start_evm_translation_unit(
                    contract_name.full_path.as_str(),
                    code_segment,
                    "EVMAssemblyToLLVMIR",
                    &optimizer_settings,
                );
                runtime_code.assembly.declare(&mut runtime_context)?;
                runtime_code
                    .assembly
                    .into_llvm(&mut runtime_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{code_segment} code LLVM IR generator: {error}")
                    })?;
                run_evm_assembly_lowering.borrow_mut().finish();
                let runtime_build = runtime_context.build(
                    output_selection.check_selection(
                        contract_name.path.as_str(),
                        contract_name.name.as_deref(),
                        solx_standard_json::InputSelector::RuntimeBytecodeLLVMAssembly,
                    ),
                    output_bytecode,
                    false,
                    &mut profiler,
                )?;
                let immutables = runtime_build.immutables.unwrap_or_default();
                let runtime_object = EVMContractObject::new(
                    runtime_code_identifier,
                    contract_name.clone(),
                    runtime_build.assembly,
                    runtime_build.bytecode,
                    false,
                    code_segment,
                    Some(immutables),
                    metadata_bytes,
                    runtime_code.dependencies,
                    runtime_build.is_size_fallback,
                    runtime_build.warnings,
                    profiler.to_vec(),
                );
                Ok(runtime_object)
            }
            (IR::LLVMIR(deploy_llvm_ir), era_compiler_common::CodeSegment::Deploy) => {
                let deploy_code_identifier = contract_name.full_path.to_owned();
                let deploy_memory_buffer =
                    inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                        &deploy_llvm_ir.source.as_bytes()[..deploy_llvm_ir.source.len() - 1],
                        deploy_code_identifier.as_str(),
                        true,
                    );

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
                deploy_context.set_solidity_data(solidity_data);
                let deploy_build = deploy_context.build(
                    output_selection.check_selection(
                        contract_name.path.as_str(),
                        contract_name.name.as_deref(),
                        solx_standard_json::InputSelector::BytecodeLLVMAssembly,
                    ),
                    output_bytecode,
                    false,
                    &mut profiler,
                )?;
                let deploy_object = EVMContractObject::new(
                    deploy_code_identifier,
                    contract_name.clone(),
                    deploy_build.assembly,
                    deploy_build.bytecode,
                    false,
                    code_segment,
                    None,
                    None,
                    deploy_llvm_ir.dependencies,
                    deploy_build.is_size_fallback,
                    deploy_build.warnings,
                    profiler.to_vec(),
                );
                Ok(deploy_object)
            }
            (IR::LLVMIR(runtime_llvm_ir), era_compiler_common::CodeSegment::Runtime) => {
                let runtime_code_identifier = format!("{}.{code_segment}", contract_name.full_path);
                let runtime_memory_buffer =
                    inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                        &runtime_llvm_ir.source.as_bytes()[..runtime_llvm_ir.source.len() - 1],
                        runtime_code_identifier.as_str(),
                        true,
                    );

                let runtime_llvm = inkwell::context::Context::create();
                let runtime_module = runtime_llvm
                    .create_module_from_ir(runtime_memory_buffer)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
                let mut runtime_context = era_compiler_llvm_context::EVMContext::new(
                    &runtime_llvm,
                    runtime_module,
                    llvm_options.clone(),
                    code_segment,
                    optimizer,
                    debug_config.clone(),
                );
                inkwell::context::Context::install_stack_error_handler(
                    crate::process::evm_stack_error_handler,
                );
                runtime_context.set_solidity_data(solidity_data);
                let runtime_build = runtime_context.build(
                    output_selection.check_selection(
                        contract_name.path.as_str(),
                        contract_name.name.as_deref(),
                        solx_standard_json::InputSelector::RuntimeBytecodeLLVMAssembly,
                    ),
                    output_bytecode,
                    false,
                    &mut profiler,
                )?;
                let runtime_object = EVMContractObject::new(
                    runtime_code_identifier,
                    contract_name.clone(),
                    runtime_build.assembly,
                    runtime_build.bytecode,
                    false,
                    code_segment,
                    Some(BTreeMap::new()),
                    metadata_bytes,
                    runtime_llvm_ir.dependencies,
                    runtime_build.is_size_fallback,
                    runtime_build.warnings,
                    profiler.to_vec(),
                );
                Ok(runtime_object)
            }
        }
    }
}
