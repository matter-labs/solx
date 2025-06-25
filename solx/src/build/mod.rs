//!
//! The Solidity project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use normpath::PathExt;

use solx_standard_json::CollectableError;

use crate::error::stack_too_deep::StackTooDeep as StackTooDeepError;
use crate::error::Error;

use self::contract::object::Object as ContractObject;
use self::contract::Contract;

///
/// The Solidity project build.
///
#[derive(Debug, Default)]
pub struct Build {
    /// The contract builds,
    pub contracts: BTreeMap<String, Contract>,
    /// The Solidity AST JSONs of the source files.
    pub ast_jsons: Option<BTreeMap<String, Option<serde_json::Value>>>,
    /// The additional message to output.
    pub messages: Vec<solx_standard_json::OutputError>,
}

impl Build {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        contracts: BTreeMap<String, Contract>,
        ast_jsons: Option<BTreeMap<String, Option<serde_json::Value>>>,
        messages: &mut Vec<solx_standard_json::OutputError>,
    ) -> Self {
        Self {
            contracts,
            ast_jsons,
            messages: std::mem::take(messages),
        }
    }

    ///
    /// Links the EVM build.
    ///
    pub fn link(
        mut self,
        linker_symbols: BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>,
        cbor_data: Option<Vec<(String, semver::Version)>>,
    ) -> Self {
        let ast_jsons = self.ast_jsons.take();

        loop {
            let assembled_objects_data = {
                let all_objects = self
                    .contracts
                    .values()
                    .flat_map(|contract| {
                        vec![
                            contract
                                .deploy_object_result
                                .as_ref()
                                .expect("Always exists"),
                            contract
                                .runtime_object_result
                                .as_ref()
                                .expect("Always exists"),
                        ]
                    })
                    .collect::<Vec<&ContractObject>>();

                let assembleable_objects = all_objects
                    .iter()
                    .filter(|object| {
                        !object.is_assembled
                            && object.dependencies.inner.iter().all(|dependency| {
                                all_objects
                                    .iter()
                                    .find(|object| {
                                        object.identifier.as_str() == dependency.as_str()
                                    })
                                    .map(|object| object.is_assembled)
                                    .unwrap_or_default()
                            })
                    })
                    .copied()
                    .collect::<Vec<_>>();
                if assembleable_objects.is_empty() {
                    break;
                }

                let mut assembled_objects_data = Vec::with_capacity(assembleable_objects.len());
                for object in assembleable_objects.into_iter() {
                    let assembled_object =
                        match object.assemble(all_objects.as_slice(), cbor_data.clone()) {
                            Ok(assembled_object) => assembled_object,
                            Err(error) => {
                                self.messages
                                    .push(solx_standard_json::OutputError::new_error(
                                        None, &error, None, None,
                                    ));
                                return Self::new(BTreeMap::new(), ast_jsons, &mut self.messages);
                            }
                        };
                    assembled_objects_data.push((
                        object.contract_name.full_path.to_owned(),
                        object.code_segment,
                        assembled_object,
                    ));
                }
                assembled_objects_data
            };

            for (full_path, code_segment, assembled_object) in assembled_objects_data.into_iter() {
                let contract = self
                    .contracts
                    .get_mut(full_path.as_str())
                    .expect("Always exists");
                let object = match code_segment {
                    era_compiler_common::CodeSegment::Deploy => contract
                        .deploy_object_result
                        .as_mut()
                        .expect("Always exists"),
                    era_compiler_common::CodeSegment::Runtime => contract
                        .runtime_object_result
                        .as_mut()
                        .expect("Always exists"),
                };
                object.bytecode = Some(assembled_object.as_slice().to_owned());
                for undefined_reference in assembled_object
                    .get_undefined_references_evm()
                    .into_iter()
                    .filter(|reference| !linker_symbols.contains_key(reference))
                {
                    let symbol_offsets =
                        assembled_object.get_symbol_offsets_evm(undefined_reference.as_str());
                    object
                        .unlinked_symbols
                        .insert(undefined_reference, symbol_offsets);
                }
                object.is_assembled = true;
            }
        }

        for contract in self.contracts.values_mut() {
            for object in [
                contract
                    .deploy_object_result
                    .as_mut()
                    .expect("Always exists"),
                contract
                    .runtime_object_result
                    .as_mut()
                    .expect("Always exists"),
            ]
            .into_iter()
            {
                if let Err(error) = object.link(&linker_symbols) {
                    self.messages
                        .push(solx_standard_json::OutputError::new_error(
                            None, &error, None, None,
                        ));
                    return Self::new(BTreeMap::new(), ast_jsons, &mut self.messages);
                }
            }
        }

        Self::new(self.contracts, ast_jsons, &mut self.messages)
    }

    ///
    /// Writes all contracts to the terminal.
    ///
    pub fn write_to_terminal(
        mut self,
        output_selection: &solx_standard_json::InputSelection,
    ) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.exit_on_error();

        for (path, ast) in self.ast_jsons.unwrap_or_default().into_iter() {
            if output_selection.check_selection(
                path.as_str(),
                None,
                solx_standard_json::InputSelector::AST,
            ) {
                writeln!(std::io::stdout(), "\n======= {path} =======",)?;
                writeln!(
                    std::io::stdout(),
                    "JSON AST:\n{}",
                    ast.expect("Always exists")
                )?;
            }
        }

        for contract in self.contracts.into_values() {
            contract.write_to_terminal(output_selection)?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts to the specified directory.
    ///
    pub fn write_to_directory(
        mut self,
        output_directory: &Path,
        output_selection: &solx_standard_json::InputSelection,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.exit_on_error();

        std::fs::create_dir_all(output_directory)?;

        for (path, ast_json) in self.ast_jsons.into_iter().flatten() {
            if output_selection.check_selection(
                path.as_str(),
                None,
                solx_standard_json::InputSelector::AST,
            ) {
                let path = PathBuf::from(path).normalize()?;
                let path = if path.starts_with(std::env::current_dir()?) {
                    path.as_path().strip_prefix(std::env::current_dir()?)?
                } else {
                    path.as_path()
                }
                .to_string_lossy()
                .replace(['\\', '/'], "_");

                let output_name = format!("{path}_{}.ast", era_compiler_common::EXTENSION_JSON);
                let mut output_path = output_directory.to_owned();
                output_path.push(output_name.as_str());

                let ast_json = ast_json.expect("Always exists").to_string();
                Contract::write_to_file(output_path.as_path(), ast_json, overwrite)?;
            }
        }

        for contract in self.contracts.into_values() {
            contract.write_to_directory(output_directory, output_selection, overwrite)?;
        }

        writeln!(
            std::io::stderr(),
            "Compiler run successful. Artifact(s) can be found in directory {output_directory:?}."
        )?;
        Ok(())
    }

    ///
    /// Writes all contracts assembly and bytecode to the standard JSON.
    ///
    pub fn write_to_standard_json(
        &mut self,
        standard_json: &mut solx_standard_json::Output,
        output_selection: &solx_standard_json::InputSelection,
        is_bytecode_linked: bool,
    ) -> anyhow::Result<()> {
        for (path, ast_json) in self.ast_jsons.iter_mut().flatten() {
            if let Some(source) = standard_json.sources.get_mut(path.as_str()) {
                if let Some(ast_json) = ast_json.take().filter(|_| {
                    output_selection.check_selection(
                        path.as_str(),
                        None,
                        solx_standard_json::InputSelector::AST,
                    )
                }) {
                    source.ast = Some(ast_json);
                }
            }
        }

        let mut errors = Vec::with_capacity(self.contracts.len());
        for contract in self.contracts.values_mut() {
            errors.extend(
                contract
                    .deploy_object_result
                    .as_ref()
                    .map(|object| object.warnings_standard_json(contract.name.full_path.as_str()))
                    .unwrap_or_default(),
            );
            if let Err(ref error) = contract.deploy_object_result {
                errors.push(error.to_owned().unwrap_standard_json());
            }
            errors.extend(
                contract
                    .runtime_object_result
                    .as_ref()
                    .map(|object| object.warnings_standard_json(contract.name.full_path.as_str()))
                    .unwrap_or_default(),
            );
            if let Err(ref error) = contract.runtime_object_result {
                errors.push(error.to_owned().unwrap_standard_json());
            }
            if contract.deploy_object_result.is_err() || contract.runtime_object_result.is_err() {
                continue;
            }

            let name = contract.name.clone();

            match standard_json
                .contracts
                .get_mut(name.path.as_str())
                .and_then(|contracts| {
                    contracts.get_mut(name.name.as_deref().unwrap_or(name.path.as_str()))
                }) {
                Some(standard_json_contract) => {
                    contract.write_to_standard_json(
                        standard_json_contract,
                        output_selection,
                        is_bytecode_linked,
                    );
                }
                None => {
                    let contracts = standard_json
                        .contracts
                        .entry(name.path.clone())
                        .or_default();
                    let mut standard_json_contract = solx_standard_json::OutputContract::default();
                    contract.write_to_standard_json(
                        &mut standard_json_contract,
                        output_selection,
                        is_bytecode_linked,
                    );
                    contracts.insert(name.name.unwrap_or(name.path), standard_json_contract);
                }
            }
        }
        standard_json.errors.extend(errors);
        if standard_json.has_errors() {
            standard_json.contracts.clear();
        }
        Ok(())
    }

    ///
    /// Extracts stack-too-deep errors from the build.
    ///
    pub fn take_stack_too_deep_errors(&mut self) -> Vec<StackTooDeepError> {
        let mut stack_too_deep_errors = Vec::new();
        for contract in self.contracts.values() {
            if let Err(Error::StackTooDeep(stack_too_deep_error)) =
                contract.deploy_object_result.as_ref()
            {
                let mut error = stack_too_deep_error.to_owned();
                error.contract_name = Some(contract.name.to_owned());
                error.code_segment = Some(era_compiler_common::CodeSegment::Deploy);
                stack_too_deep_errors.push(error);
            }
            if let Err(Error::StackTooDeep(stack_too_deep_error)) =
                contract.runtime_object_result.as_ref()
            {
                let mut error = stack_too_deep_error.to_owned();
                error.contract_name = Some(contract.name.to_owned());
                error.code_segment = Some(era_compiler_common::CodeSegment::Runtime);
                stack_too_deep_errors.push(error);
            }
        }
        self.contracts.retain(|_, contract| {
            !matches!(contract.deploy_object_result, Err(Error::StackTooDeep(_)))
                && !matches!(contract.runtime_object_result, Err(Error::StackTooDeep(_)))
        }); // TODO: replace with `extract_if` when stabilized
        stack_too_deep_errors
    }
}

impl solx_standard_json::CollectableError for Build {
    fn errors(&self) -> Vec<&solx_standard_json::OutputError> {
        let mut errors: Vec<&solx_standard_json::OutputError> = self
            .contracts
            .values()
            .flat_map(|contract| {
                vec![
                    contract.deploy_object_result.as_ref().err(),
                    contract.runtime_object_result.as_ref().err(),
                ]
            })
            .flatten()
            .map(|error| error.unwrap_standard_json_ref())
            .collect();
        errors.extend(
            self.messages
                .iter()
                .filter(|message| message.severity == "error"),
        );
        errors
    }

    fn take_warnings(&mut self) -> Vec<solx_standard_json::OutputError> {
        let mut warnings: Vec<solx_standard_json::OutputError> = self
            .messages
            .iter()
            .filter(|message| message.severity == "warning")
            .cloned()
            .collect();
        for contract in self.contracts.values_mut() {
            warnings.extend(
                contract
                    .deploy_object_result
                    .as_ref()
                    .map(|object| object.warnings_standard_json(contract.name.full_path.as_str()))
                    .unwrap_or_default(),
            );
            warnings.extend(
                contract
                    .runtime_object_result
                    .as_ref()
                    .map(|object| object.warnings_standard_json(contract.name.full_path.as_str()))
                    .unwrap_or_default(),
            );
        }
        self.messages
            .retain(|message| message.severity != "warning");
        warnings
    }
}
