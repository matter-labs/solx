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
use crate::evmla::assembly::Assembly;
use crate::process::input::Input as EVMProcessInput;
use crate::process::output::Output as EVMOutput;

use self::contract::ir::evmla::EVMLA as ContractEVMLA;
use self::contract::ir::llvm_ir::LLVMIR as ContractLLVMIR;
use self::contract::ir::yul::Yul as ContractYul;
use self::contract::ir::IR as ContractIR;
use self::contract::Contract;

///
/// The project representation.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Project {
    /// The project language.
    pub language: solx_solc::StandardJsonInputLanguage,
    /// The `solc` compiler version.
    pub solc_version: solx_solc::Version,
    /// The project build results.
    pub contracts: BTreeMap<String, Contract>,
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
        language: solx_solc::StandardJsonInputLanguage,
        contracts: BTreeMap<String, Contract>,
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
        solc_output: &mut solx_solc::StandardJsonOutput,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        if !via_ir {
            Assembly::preprocess_dependencies(&mut solc_output.contracts)?;
        }

        let mut input_contracts = Vec::with_capacity(solc_output.contracts.len());
        for (path, file) in solc_output.contracts.iter() {
            for (name, contract) in file.iter() {
                let name = era_compiler_common::ContractName::new(
                    (*path).to_owned(),
                    Some((*name).to_owned()),
                );
                input_contracts.push((name, contract));
            }
        }

        let results = input_contracts
            .into_par_iter()
            .filter_map(|(name, contract)| {
                let result = if via_ir {
                    ContractYul::try_from_source(
                        name.full_path.as_str(),
                        contract.ir_optimized.as_str(),
                        debug_config,
                    )
                    .map(|yul| yul.map(ContractIR::from))
                } else {
                    Ok(ContractEVMLA::try_from_contract(contract).map(ContractIR::from))
                };
                let ir = match result {
                    Ok(ir) => ir?,
                    Err(error) => return Some((name.full_path, Err(error))),
                };
                let contract = Contract::new(name.clone(), ir, contract.metadata.clone());
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
            solx_solc::StandardJsonInputLanguage::Solidity,
            contracts,
            libraries,
        ))
    }

    ///
    /// Reads the Yul source code `paths` and returns a Yul project.
    ///
    pub fn try_from_yul_paths(
        paths: &[PathBuf],
        libraries: era_compiler_common::Libraries,
        output_selection: solx_solc::StandardJsonInputSelection,
        solc_output: Option<&mut solx_solc::StandardJsonOutput>,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source = solx_solc::StandardJsonInputSource::from(path.as_path());
                (path.to_string_lossy().to_string(), source)
            })
            .collect::<BTreeMap<String, solx_solc::StandardJsonInputSource>>();
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
        sources: BTreeMap<String, solx_solc::StandardJsonInputSource>,
        libraries: era_compiler_common::Libraries,
        output_selection: solx_solc::StandardJsonInputSelection,
        mut solc_output: Option<&mut solx_solc::StandardJsonOutput>,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        let results = sources
            .into_par_iter()
            .filter_map(|(path, mut source)| {
                let source_code = match source.try_resolve() {
                    Ok(()) => source.take_content().expect("Always exists"),
                    Err(error) => return Some((path, Err(error))),
                };
                let ir = match ContractYul::try_from_source(
                    path.as_str(),
                    source_code.as_str(),
                    debug_config,
                ) {
                    Ok(ir) => ir?,
                    Err(error) => return Some((path, Err(error))),
                };

                let metadata = if output_selection.check_selection(
                    path.as_str(),
                    None,
                    solx_solc::StandardJsonInputSelector::Metadata,
                ) {
                    let source_hash = era_compiler_common::Hash::keccak256(source_code.as_bytes());
                    let metadata_json = serde_json::json!({
                        "source_hash": source_hash.to_string(),
                        "solc_version": solx_solc::Compiler::default().version,
                    });
                    Some(serde_json::to_string(&metadata_json).expect("Always valid"))
                } else {
                    None
                };

                let name = era_compiler_common::ContractName::new(
                    path.clone(),
                    Some(ir.object.0.identifier.clone()),
                );
                let full_path = name.full_path.clone();
                let contract = Contract::new(name, ir.into(), metadata);
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
            solx_solc::StandardJsonInputLanguage::Yul,
            contracts,
            libraries,
        ))
    }

    ///
    /// Reads the LLVM IR source code `paths` and returns an LLVM IR project.
    ///
    pub fn try_from_llvm_ir_paths(
        paths: &[PathBuf],
        libraries: era_compiler_common::Libraries,
        output_selection: solx_solc::StandardJsonInputSelection,
        solc_output: Option<&mut solx_solc::StandardJsonOutput>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source = solx_solc::StandardJsonInputSource::from(path.as_path());
                (path.to_string_lossy().to_string(), source)
            })
            .collect::<BTreeMap<String, solx_solc::StandardJsonInputSource>>();
        Self::try_from_llvm_ir_sources(sources, libraries, output_selection, solc_output)
    }

    ///
    /// Parses the LLVM IR `sources` and returns an LLVM IR project.
    ///
    pub fn try_from_llvm_ir_sources(
        sources: BTreeMap<String, solx_solc::StandardJsonInputSource>,
        libraries: era_compiler_common::Libraries,
        output_selection: solx_solc::StandardJsonInputSelection,
        mut solc_output: Option<&mut solx_solc::StandardJsonOutput>,
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
                    solx_solc::StandardJsonInputSelector::Metadata,
                ) {
                    let source_hash = era_compiler_common::Hash::keccak256(source_code.as_bytes());
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
                    ContractLLVMIR::new(path.clone(), source_code).into(),
                    metadata,
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
            solx_solc::StandardJsonInputLanguage::LLVMIR,
            contracts,
            libraries,
        ))
    }

    ///
    /// Compiles all contracts to EVM, returning their build artifacts.
    ///
    pub fn compile_to_evm(
        self,
        messages: &mut Vec<solx_solc::StandardJsonOutputError>,
        output_bytecode: bool,
        metadata_hash_type: era_compiler_common::HashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EVMBuild> {
        let deployed_libraries = self.libraries.as_paths();
        let results = self.contracts.into_par_iter().map(|(path, contract)| {
            let input = EVMProcessInput::new(
                contract,
                self.identifier_paths.clone(),
                output_bytecode,
                deployed_libraries.clone(),
                metadata_hash_type,
                optimizer_settings.clone(),
                llvm_options.clone(),
                debug_config.clone(),
            );
            let result: crate::Result<EVMOutput> =
                crate::process::call(path.as_str(), input);
            let result = result.map(|output| output.build);
            (path, result)
        }).collect::<BTreeMap<String, Result<EVMContractBuild, solx_solc::StandardJsonOutputError>>>();

        Ok(EVMBuild::new(results, messages))
    }
}
