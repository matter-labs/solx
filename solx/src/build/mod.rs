//!
//! The Solidity project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use solx_standard_json::CollectableError;

use self::contract::object::Object as ContractObject;
use self::contract::Contract;

///
/// The Solidity project build.
///
#[derive(Debug, Default)]
pub struct Build {
    /// The contract data,
    pub results: BTreeMap<String, Result<Contract, solx_standard_json::OutputError>>,
    /// The additional message to output.
    pub messages: Vec<solx_standard_json::OutputError>,
}

impl Build {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        results: BTreeMap<String, Result<Contract, solx_standard_json::OutputError>>,
        messages: &mut Vec<solx_standard_json::OutputError>,
    ) -> Self {
        Self {
            results,
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
        let mut contracts: BTreeMap<String, Contract> = self
            .results
            .into_iter()
            .map(|(path, result)| (path, result.expect("Cannot link a project with errors")))
            .collect();

        loop {
            let assembled_objects_data = {
                let all_objects = contracts
                    .iter()
                    .filter_map(|(_path, contract)| {
                        Some(vec![
                            contract.deploy_object.as_ref()?,
                            contract.runtime_object.as_ref()?,
                        ])
                    })
                    .flatten()
                    .collect::<Vec<&ContractObject>>();
                let assembleable_objects = all_objects
                    .iter()
                    .filter(|object| {
                        object.requires_assembling()
                            && object.dependencies.inner.iter().all(|dependency| {
                                all_objects
                                    .iter()
                                    .find(|object| {
                                        object.identifier.as_str() == dependency.as_str()
                                    })
                                    .map(|object| !object.requires_assembling())
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
                    let memory_buffer =
                        inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                            object.bytecode.as_slice(),
                            object.identifier.as_str(),
                            false,
                        );
                    let mut memory_buffers =
                        Vec::with_capacity(1 + object.dependencies.inner.len());
                    memory_buffers.push((object.identifier.to_owned(), memory_buffer));

                    memory_buffers.extend(object.dependencies.inner.iter().map(|dependency| {
                        let original_dependency_identifier = dependency.to_owned();
                        let dependency = all_objects
                            .iter()
                            .find(|object| object.identifier.as_str() == dependency.as_str())
                            .expect("Dependency not found");
                        let memory_buffer =
                            inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                                dependency.bytecode.as_slice(),
                                dependency.identifier.as_str(),
                                false,
                            );
                        (original_dependency_identifier, memory_buffer)
                    }));

                    let bytecode_buffers = memory_buffers
                        .iter()
                        .map(|(_identifier, memory_buffer)| memory_buffer)
                        .collect::<Vec<&inkwell::memory_buffer::MemoryBuffer>>();
                    let bytecode_ids = memory_buffers
                        .iter()
                        .map(|(identifier, _memory_buffer)| identifier.as_str())
                        .collect::<Vec<&str>>();
                    let assembled_object = match era_compiler_llvm_context::evm_assemble(
                        bytecode_buffers.as_slice(),
                        bytecode_ids.as_slice(),
                        object.code_segment,
                    ) {
                        Ok(assembled_object) => assembled_object,
                        Err(error) => {
                            self.messages
                                .push(solx_standard_json::OutputError::new_error(
                                    None, error, None, None,
                                ));
                            continue;
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
                let contract = contracts
                    .get_mut(full_path.as_str())
                    .expect("Always exists");
                let object = match code_segment {
                    era_compiler_common::CodeSegment::Deploy => &mut contract.deploy_object,
                    era_compiler_common::CodeSegment::Runtime => &mut contract.runtime_object,
                };
                if let Some(object) = object {
                    object.bytecode = assembled_object.as_slice().to_owned();
                    object.is_assembled = true;
                }
            }
        }

        for contract in contracts.values_mut() {
            for object in [&mut contract.deploy_object, &mut contract.runtime_object].into_iter() {
                let object = match object {
                    Some(object) => object,
                    None => continue,
                };
                match object.link(&linker_symbols) {
                    Ok(_) => {}
                    Err(error) => {
                        self.messages
                            .push(solx_standard_json::OutputError::new_error(
                                None, error, None, None,
                            ));
                        continue;
                    }
                }
            }
        }

        Self::new(
            contracts
                .into_iter()
                .map(|(path, contract)| (path, Ok(contract)))
                .collect(),
            &mut self.messages,
        )
    }

    ///
    /// Writes all contracts to the terminal.
    ///
    pub fn write_to_terminal(mut self, output_metadata: bool) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.exit_on_error();

        for (path, build) in self.results.into_iter() {
            build
                .expect("Always valid")
                .write_to_terminal(path, output_metadata)?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts to the specified directory.
    ///
    pub fn write_to_directory(
        mut self,
        output_directory: &Path,
        overwrite: bool,
        output_metadata: bool,
    ) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.exit_on_error();

        std::fs::create_dir_all(output_directory)?;

        for build in self.results.into_values() {
            build.expect("Always valid").write_to_directory(
                output_directory,
                overwrite,
                output_metadata,
            )?;
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
        self,
        standard_json: &mut solx_standard_json::Output,
    ) -> anyhow::Result<()> {
        let mut errors = Vec::with_capacity(self.results.len());
        for result in self.results.into_values() {
            let build = match result {
                Ok(contract) => {
                    errors.extend(
                        contract
                            .deploy_object
                            .as_ref()
                            .map(|object| {
                                object
                                    .warnings
                                    .iter()
                                    .map(|error| {
                                        solx_standard_json::OutputError::new_warning(
                                            error.code(),
                                            error.to_string(),
                                            Some(
                                                solx_standard_json::OutputErrorSourceLocation::new(
                                                    contract.name.full_path.clone(),
                                                ),
                                            ),
                                            None,
                                        )
                                    })
                                    .collect::<Vec<solx_standard_json::OutputError>>()
                            })
                            .unwrap_or_default(),
                    );
                    errors.extend(
                        contract
                            .runtime_object
                            .as_ref()
                            .map(|object| {
                                object
                                    .warnings
                                    .iter()
                                    .map(|error| {
                                        solx_standard_json::OutputError::new_warning(
                                            error.code(),
                                            error.to_string(),
                                            Some(
                                                solx_standard_json::OutputErrorSourceLocation::new(
                                                    contract.name.full_path.clone(),
                                                ),
                                            ),
                                            None,
                                        )
                                    })
                                    .collect::<Vec<solx_standard_json::OutputError>>()
                            })
                            .unwrap_or_default(),
                    );
                    contract
                }
                Err(error) => {
                    errors.push(error);
                    continue;
                }
            };
            let name = build.name.clone();

            match standard_json
                .contracts
                .get_mut(name.path.as_str())
                .and_then(|contracts| {
                    contracts.get_mut(name.name.as_deref().unwrap_or(name.path.as_str()))
                }) {
                Some(contract) => {
                    build.write_to_standard_json(contract)?;
                }
                None => {
                    let contracts = standard_json
                        .contracts
                        .entry(name.path.clone())
                        .or_default();
                    let mut contract = solx_standard_json::OutputContract::default();
                    build.write_to_standard_json(&mut contract)?;
                    contracts.insert(name.name.unwrap_or(name.path), contract);
                }
            }
        }

        standard_json.errors.extend(errors);
        Ok(())
    }
}

impl solx_standard_json::CollectableError for Build {
    fn errors(&self) -> Vec<&solx_standard_json::OutputError> {
        let mut errors: Vec<&solx_standard_json::OutputError> = self
            .results
            .values()
            .filter_map(|build| build.as_ref().err())
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
        for contract in self.results.values_mut().flatten() {
            warnings.extend(
                contract
                    .deploy_object
                    .as_ref()
                    .map(|object| {
                        object
                            .warnings
                            .iter()
                            .map(|error| {
                                solx_standard_json::OutputError::new_warning(
                                    error.code(),
                                    error.to_string(),
                                    Some(solx_standard_json::OutputErrorSourceLocation::new(
                                        contract.name.full_path.clone(),
                                    )),
                                    None,
                                )
                            })
                            .collect::<Vec<solx_standard_json::OutputError>>()
                    })
                    .unwrap_or_default(),
            );
            warnings.extend(
                contract
                    .runtime_object
                    .as_ref()
                    .map(|object| {
                        object
                            .warnings
                            .iter()
                            .map(|error| {
                                solx_standard_json::OutputError::new_warning(
                                    error.code(),
                                    error.to_string(),
                                    Some(solx_standard_json::OutputErrorSourceLocation::new(
                                        contract.name.full_path.clone(),
                                    )),
                                    None,
                                )
                            })
                            .collect::<Vec<solx_standard_json::OutputError>>()
                    })
                    .unwrap_or_default(),
            );
        }
        self.messages
            .retain(|message| message.severity != "warning");
        warnings
    }
}
