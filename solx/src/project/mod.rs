//!
//! The project representation.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::path::PathBuf;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::build::contract::Contract as EVMContractBuild;
use crate::build::Build as EVMBuild;
use crate::error::Error;
use crate::process::input::Input as EVMProcessInput;
use crate::process::output::Output as EVMProcessOutput;

use self::contract::ir::evmla::EVMLegacyAssembly as ContractEVMLegacyAssembly;
use self::contract::ir::llvm_ir::LLVMIR as ContractLLVMIR;
use self::contract::ir::yul::Yul as ContractYul;
use self::contract::ir::IR as ContractIR;
use self::contract::metadata::Metadata as ContractMetadata;
use self::contract::Contract;

///
/// The project representation.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Project {
    /// The project language.
    pub language: solx_standard_json::InputLanguage,
    /// The `solc` compiler version.
    pub solc_version: solx_standard_json::Version,
    /// The project build results.
    pub contracts: BTreeMap<String, Contract>,
    /// The Solidity AST JSONs of the source files.
    pub ast_jsons: Option<BTreeMap<String, Option<serde_json::Value>>>,
    /// The mapping of auxiliary identifiers, e.g. Yul object names, to full contract paths.
    pub identifier_paths: BTreeMap<String, String>,
    /// The library addresses.
    pub libraries: era_compiler_common::Libraries,
}

impl Project {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        language: solx_standard_json::InputLanguage,
        contracts: BTreeMap<String, Contract>,
        ast_jsons: Option<BTreeMap<String, Option<serde_json::Value>>>,
        libraries: era_compiler_common::Libraries,
    ) -> Self {
        let mut identifier_paths = BTreeMap::new();
        for (path, contract) in contracts.iter() {
            identifier_paths.insert(contract.identifier().to_owned(), path.to_owned());
        }

        Self {
            language,
            solc_version: solx_solc::Compiler::default().version,
            contracts,
            ast_jsons,
            identifier_paths,
            libraries,
        }
    }

    ///
    /// Parses the Solidity `sources` and returns a Solidity project.
    ///
    pub fn try_from_solc_output(
        libraries: era_compiler_common::Libraries,
        via_ir: bool,
        solc_output: &mut solx_standard_json::Output,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        if !via_ir {
            let legacy_assemblies: BTreeMap<
                String,
                BTreeMap<String, &mut solx_evm_assembly::Assembly>,
            > = solc_output
                .contracts
                .iter_mut()
                .map(|(path, file)| {
                    let legacy_assemblies: BTreeMap<String, &mut solx_evm_assembly::Assembly> =
                        file.iter_mut()
                            .filter_map(|(name, contract)| {
                                Some((
                                    name.to_owned(),
                                    contract
                                        .evm
                                        .as_mut()
                                        .and_then(|evm| evm.legacy_assembly.as_mut())?,
                                ))
                            })
                            .collect();
                    (path.to_owned(), legacy_assemblies)
                })
                .collect();
            solx_evm_assembly::Assembly::preprocess_dependencies(legacy_assemblies)?;
        }

        let ast_jsons = solc_output
            .sources
            .iter_mut()
            .map(|(path, source)| (path.to_owned(), source.ast.take()))
            .collect::<BTreeMap<String, Option<serde_json::Value>>>();

        let mut input_contracts = Vec::with_capacity(solc_output.contracts.len());
        for path in solc_output
            .contracts
            .keys()
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
        {
            let file = solc_output
                .contracts
                .remove(path.as_str())
                .expect("Always exists");
            for (name, contract) in file.into_iter() {
                let name = era_compiler_common::ContractName::new(path.clone(), Some(name));
                input_contracts.push((name, contract));
            }
        }

        let results = input_contracts
            .into_par_iter()
            .filter_map(|(name, mut contract)| {
                let method_identifiers = contract
                    .evm
                    .as_mut()
                    .and_then(|evm| evm.method_identifiers.take());
                let legacy_assembly = contract
                    .evm
                    .as_mut()
                    .and_then(|evm| evm.legacy_assembly.take());
                let extra_metadata = contract
                    .evm
                    .as_mut()
                    .and_then(|evm| evm.extra_metadata.take());

                let result = if via_ir {
                    ContractYul::try_from_source(
                        name.full_path.as_str(),
                        contract.ir_optimized.as_deref()?,
                        debug_config,
                    )
                    .map(|yul| yul.map(ContractIR::from))
                } else {
                    Ok(ContractEVMLegacyAssembly::try_from_contract(
                        legacy_assembly.clone()?,
                        extra_metadata,
                    )
                    .map(ContractIR::from))
                };
                let ir = match result {
                    Ok(ir) => ir?,
                    Err(error) => return Some((name.full_path, Err(error))),
                };
                let contract = Contract::new(
                    name.clone(),
                    ir,
                    contract.metadata,
                    contract.abi,
                    method_identifiers,
                    contract.userdoc,
                    contract.devdoc,
                    contract.storage_layout,
                    contract.transient_storage_layout,
                    legacy_assembly,
                    contract.ir_optimized,
                );
                Some((name.full_path, Ok(contract)))
            })
            .collect::<BTreeMap<String, anyhow::Result<Contract>>>();

        let mut contracts = BTreeMap::new();
        for (path, result) in results.into_iter() {
            match result {
                Ok(contract) => {
                    contracts.insert(path, contract);
                }
                Err(error) => solc_output.push_error(Some(path), error),
            }
        }
        Ok(Project::new(
            solx_standard_json::InputLanguage::Solidity,
            contracts,
            Some(ast_jsons),
            libraries,
        ))
    }

    ///
    /// Reads the Yul source code `paths` and returns a Yul project.
    ///
    pub fn try_from_yul_paths(
        paths: &[PathBuf],
        libraries: era_compiler_common::Libraries,
        output_selection: &solx_standard_json::InputSelection,
        solc_output: Option<&mut solx_standard_json::Output>,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source = solx_standard_json::InputSource::try_from_path(path.as_path())?;
                let path = if path.to_string_lossy()
                    == solx_standard_json::InputSource::STDIN_INPUT_IDENTIFIER
                {
                    solx_standard_json::InputSource::STDIN_OUTPUT_IDENTIFIER.to_owned()
                } else {
                    path.to_string_lossy().to_string()
                };
                Ok((path, source))
            })
            .collect::<anyhow::Result<BTreeMap<String, solx_standard_json::InputSource>>>()?;

        Self::try_from_yul_sources(
            sources,
            libraries,
            output_selection,
            solc_output,
            debug_config,
        )
    }

    ///
    /// Parses the Yul `sources` and returns a Yul project.
    ///
    pub fn try_from_yul_sources(
        sources: BTreeMap<String, solx_standard_json::InputSource>,
        libraries: era_compiler_common::Libraries,
        output_selection: &solx_standard_json::InputSelection,
        mut solc_output: Option<&mut solx_standard_json::Output>,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        let results = sources
            .into_par_iter()
            .filter_map(|(path, mut source)| {
                let source_code = match source.try_resolve() {
                    Ok(()) => source.take_content().expect("Always exists"),
                    Err(error) => return Some((path, Err(error))),
                };

                let metadata = if output_selection.check_selection(
                    path.as_str(),
                    None,
                    solx_standard_json::InputSelector::Metadata,
                ) {
                    let source_hash =
                        era_compiler_common::Keccak256Hash::from_slice(source_code.as_bytes());
                    let metadata_json = serde_json::json!({
                        "source_hash": source_hash.to_string(),
                        "solc_version": solx_solc::Compiler::default().version,
                    });
                    Some(serde_json::to_string(&metadata_json).expect("Always valid"))
                } else {
                    None
                };

                let ir = match ContractYul::try_from_source(
                    path.as_str(),
                    source_code.as_str(),
                    debug_config,
                ) {
                    Ok(ir) => ir?,
                    Err(error) => return Some((path, Err(error))),
                };

                let name = era_compiler_common::ContractName::new(
                    path.clone(),
                    Some(ir.object.0.identifier.clone()),
                );
                let full_path = name.full_path.clone();
                let contract = Contract::new(
                    name,
                    ir.into(),
                    metadata,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                );
                Some((full_path, Ok(contract)))
            })
            .collect::<BTreeMap<String, anyhow::Result<Contract>>>();

        let mut contracts = BTreeMap::new();
        for (path, result) in results.into_iter() {
            match result {
                Ok(contract) => {
                    contracts.insert(path, contract);
                }
                Err(error) => match solc_output {
                    Some(ref mut solc_output) => solc_output.push_error(Some(path), error),
                    None => anyhow::bail!(error),
                },
            }
        }
        Ok(Self::new(
            solx_standard_json::InputLanguage::Yul,
            contracts,
            None,
            libraries,
        ))
    }

    ///
    /// Reads the LLVM IR source code `paths` and returns an LLVM IR project.
    ///
    pub fn try_from_llvm_ir_paths(
        paths: &[PathBuf],
        libraries: era_compiler_common::Libraries,
        output_selection: &solx_standard_json::InputSelection,
        solc_output: Option<&mut solx_standard_json::Output>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source = solx_standard_json::InputSource::try_from_path(path.as_path())?;
                let path = if path.to_string_lossy()
                    == solx_standard_json::InputSource::STDIN_INPUT_IDENTIFIER
                {
                    solx_standard_json::InputSource::STDIN_OUTPUT_IDENTIFIER.to_owned()
                } else {
                    path.to_string_lossy().to_string()
                };
                Ok((path, source))
            })
            .collect::<anyhow::Result<BTreeMap<String, solx_standard_json::InputSource>>>()?;

        Self::try_from_llvm_ir_sources(sources, libraries, output_selection, solc_output)
    }

    ///
    /// Parses the LLVM IR `sources` and returns an LLVM IR project.
    ///
    pub fn try_from_llvm_ir_sources(
        sources: BTreeMap<String, solx_standard_json::InputSource>,
        libraries: era_compiler_common::Libraries,
        output_selection: &solx_standard_json::InputSelection,
        mut solc_output: Option<&mut solx_standard_json::Output>,
    ) -> anyhow::Result<Self> {
        let results = sources
            .into_par_iter()
            .map(|(path, mut source)| {
                let source_code = match source.try_resolve() {
                    Ok(()) => source.take_content().expect("Always exists"),
                    Err(error) => return (path, Err(error)),
                };

                let metadata = if output_selection.check_selection(
                    path.as_str(),
                    None,
                    solx_standard_json::InputSelector::Metadata,
                ) {
                    let source_hash =
                        era_compiler_common::Keccak256Hash::from_slice(source_code.as_bytes());
                    let metadata_json = serde_json::json!({
                        "source_hash": source_hash.to_string(),
                        "llvm_version": era_compiler_llvm_context::LLVM_VERSION,
                    });
                    Some(serde_json::to_string(&metadata_json).expect("Always valid"))
                } else {
                    None
                };

                let contract = Contract::new(
                    era_compiler_common::ContractName::new(path.clone(), None),
                    ContractLLVMIR::new(
                        path.clone(),
                        era_compiler_common::CodeSegment::Runtime,
                        source_code,
                    )
                    .into(),
                    metadata,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                );

                (path, Ok(contract))
            })
            .collect::<BTreeMap<String, anyhow::Result<Contract>>>();

        let mut contracts = BTreeMap::new();
        for (path, result) in results.into_iter() {
            match result {
                Ok(contract) => {
                    contracts.insert(path, contract);
                }
                Err(error) => match solc_output {
                    Some(ref mut solc_output) => solc_output.push_error(Some(path), error),
                    None => anyhow::bail!(error),
                },
            }
        }
        Ok(Self::new(
            solx_standard_json::InputLanguage::LLVMIR,
            contracts,
            None,
            libraries,
        ))
    }

    ///
    /// Compiles all contracts to EVM, returning their build artifacts.
    ///
    pub fn compile_to_evm(
        self,
        messages: &mut Vec<solx_standard_json::OutputError>,
        output_selection: &solx_standard_json::InputSelection,
        metadata_hash_type: era_compiler_common::EVMMetadataHashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        spill_area_size: Option<BTreeMap<String, solx_standard_json::InputOptimizerSpillAreaSize>>,
        llvm_options: Vec<String>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EVMBuild> {
        let results = self
            .contracts
            .into_par_iter()
            .map(|(path, mut contract)| {
                let contract_name = contract.name.clone();

                let metadata = contract.metadata.take();
                let abi = contract.abi.take();
                let method_identifiers = contract.method_identifiers.take();
                let userdoc = contract.userdoc.take();
                let devdoc = contract.devdoc.take();
                let storage_layout = contract.storage_layout.take();
                let transient_storage_layout = contract.transient_storage_layout.take();
                let legacy_assembly = contract.legacy_assembly.take();
                let ir_optimized = contract.ir_optimized.take();

                let (deploy_code_ir, runtime_code_ir): (ContractIR, ContractIR) = match contract.ir
                {
                    ContractIR::Yul(mut deploy_code) => {
                        let runtime_code: ContractYul =
                            *deploy_code.runtime_code.take().expect("Always exists");
                        (deploy_code.into(), runtime_code.into())
                    }
                    ContractIR::EVMLegacyAssembly(mut deploy_code) => {
                        let runtime_code: ContractEVMLegacyAssembly =
                            *deploy_code.runtime_code.take().expect("Always exists");
                        (deploy_code.into(), runtime_code.into())
                    }
                    ContractIR::LLVMIR(runtime_code) => {
                        let deploy_code_identifier = contract.name.full_path.to_owned();
                        let runtime_code_identifier = format!(
                            "{deploy_code_identifier}.{}",
                            era_compiler_common::CodeSegment::Runtime
                        );

                        let deploy_code = ContractLLVMIR::new(
                            deploy_code_identifier.clone(),
                            era_compiler_common::CodeSegment::Deploy,
                            era_compiler_llvm_context::evm_minimal_deploy_code(
                                deploy_code_identifier.as_str(),
                                runtime_code_identifier.as_str(),
                            ),
                        );
                        (deploy_code.into(), runtime_code.into())
                    }
                };

                let (runtime_object_result, metadata) = {
                    let metadata = metadata.map(|metadata| {
                        ContractMetadata::new(optimizer_settings.clone(), llvm_options.as_slice())
                            .insert_into(metadata.as_str())
                    });
                    let metadata_bytes =
                        metadata
                            .as_ref()
                            .and_then(|metadata| match metadata_hash_type {
                                era_compiler_common::EVMMetadataHashType::None => None,
                                era_compiler_common::EVMMetadataHashType::IPFS => Some(
                                    era_compiler_common::IPFSHash::from_slice(metadata.as_bytes())
                                        .to_vec(),
                                ),
                            });

                    let spill_area_size = spill_area_size
                        .as_ref()
                        .and_then(|sizes| sizes.get(contract_name.full_path.as_str()));
                    let mut optimizer_settings = optimizer_settings.clone();
                    if let Some(spill_area_size) = spill_area_size {
                        optimizer_settings.set_spill_area_size(spill_area_size.runtime);
                    }
                    let input = EVMProcessInput::new(
                        contract_name.clone(),
                        runtime_code_ir,
                        era_compiler_common::CodeSegment::Runtime,
                        self.identifier_paths.clone(),
                        output_selection.to_owned(),
                        None,
                        metadata_bytes,
                        optimizer_settings,
                        llvm_options.clone(),
                        debug_config.clone(),
                    );
                    let mut result: crate::Result<EVMProcessOutput> =
                        crate::process::call(path.as_str(), input);
                    if let Err(Error::StackTooDeep(ref mut stack_too_deep)) = result {
                        stack_too_deep.contract_name = Some(contract_name.clone());
                        stack_too_deep.code_segment =
                            Some(era_compiler_common::CodeSegment::Runtime);
                    }
                    (result, metadata)
                };

                let immutables = runtime_object_result
                    .as_ref()
                    .ok()
                    .and_then(|output| output.object.immutables.to_owned());
                let deploy_object_result = {
                    let spill_area_size = spill_area_size
                        .as_ref()
                        .and_then(|sizes| sizes.get(contract_name.full_path.as_str()));
                    let mut optimizer_settings = optimizer_settings.clone();
                    if let Some(spill_area_size) = spill_area_size {
                        optimizer_settings.set_spill_area_size(spill_area_size.creation);
                    }

                    let input = EVMProcessInput::new(
                        contract_name.clone(),
                        deploy_code_ir,
                        era_compiler_common::CodeSegment::Deploy,
                        self.identifier_paths.clone(),
                        output_selection.to_owned(),
                        immutables,
                        None,
                        optimizer_settings,
                        llvm_options.clone(),
                        debug_config.clone(),
                    );
                    let mut result: crate::Result<EVMProcessOutput> =
                        crate::process::call(path.as_str(), input);
                    if let Err(Error::StackTooDeep(ref mut stack_too_deep)) = result {
                        stack_too_deep.contract_name = Some(contract_name.clone());
                        stack_too_deep.code_segment =
                            Some(era_compiler_common::CodeSegment::Deploy);
                    }
                    result
                };

                let build = EVMContractBuild::new(
                    contract_name,
                    deploy_object_result.map(|deploy_code_output| deploy_code_output.object),
                    runtime_object_result.map(|runtime_code_output| runtime_code_output.object),
                    metadata,
                    abi,
                    method_identifiers,
                    userdoc,
                    devdoc,
                    storage_layout,
                    transient_storage_layout,
                    legacy_assembly,
                    ir_optimized,
                );
                (path, build)
            })
            .collect::<BTreeMap<String, EVMContractBuild>>();

        Ok(EVMBuild::new(results, self.ast_jsons, messages))
    }
}
