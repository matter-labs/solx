//!
//! The Solidity project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use solx_solc::CollectableError;

use self::contract::Contract;

///
/// The Solidity project build.
///
#[derive(Debug)]
pub struct Build {
    /// The contract data,
    pub results: BTreeMap<String, Result<Contract, solx_solc::StandardJsonOutputError>>,
    /// The additional message to output.
    pub messages: Vec<solx_solc::StandardJsonOutputError>,
}

impl Build {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        results: BTreeMap<String, Result<Contract, solx_solc::StandardJsonOutputError>>,
        messages: &mut Vec<solx_solc::StandardJsonOutputError>,
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
    ) -> Self {
        for contract in self.results.values_mut().filter_map(|result| {
            let contract = result.as_mut().expect("Cannot link a project with errors");
            match contract.object_format {
                era_compiler_common::ObjectFormat::ELF => Some(contract),
                _ => None,
            }
        }) {
            let deploy_memory_buffer =
                inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                    contract.deploy_build.as_slice(),
                    contract.deploy_identifier.as_str(),
                    false,
                );
            let runtime_memory_buffer =
                inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                    contract.runtime_build.as_slice(),
                    contract.runtime_identifier.as_str(),
                    false,
                );

            let (deploy_buffer_linked, runtime_buffer_linked, object_format) =
                match era_compiler_llvm_context::evm_link(
                    (contract.deploy_identifier.as_str(), deploy_memory_buffer),
                    (contract.runtime_identifier.as_str(), runtime_memory_buffer),
                    &linker_symbols,
                ) {
                    Ok(result) => result,
                    Err(error) => {
                        self.messages
                            .push(solx_solc::StandardJsonOutputError::new_error(
                                error, None, None,
                            ));
                        continue;
                    }
                };

            contract.deploy_build = deploy_buffer_linked.as_slice().to_vec();
            contract.runtime_build = runtime_buffer_linked.as_slice().to_vec();
            contract.object_format = object_format;
        }

        self
    }

    ///
    /// Writes all contracts to the terminal.
    ///
    pub fn write_to_terminal(
        mut self,
        output_metadata: bool,
        output_binary: bool,
    ) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.exit_on_error();

        if !output_metadata && !output_binary {
            writeln!(
                std::io::stderr(),
                "Compiler run successful. No output requested. Use flags `--bin` and `--metadata`."
            )?;
            return Ok(());
        }

        for (path, build) in self.results.into_iter() {
            build
                .expect("Always valid")
                .write_to_terminal(path, output_metadata, output_binary)?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts to the specified directory.
    ///
    pub fn write_to_directory(
        mut self,
        output_directory: &Path,
        output_metadata: bool,
        output_binary: bool,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.exit_on_error();

        std::fs::create_dir_all(output_directory)?;

        for build in self.results.into_values() {
            build.expect("Always valid").write_to_directory(
                output_directory,
                output_metadata,
                output_binary,
                overwrite,
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
        standard_json: &mut solx_solc::StandardJsonOutput,
        solc_version: solx_solc::Version,
    ) -> anyhow::Result<()> {
        let mut errors = Vec::with_capacity(self.results.len());
        for result in self.results.into_values() {
            let build = match result {
                Ok(build) => build,
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
                    let mut contract = solx_solc::StandardJsonOutputContract::default();
                    build.write_to_standard_json(&mut contract)?;
                    contracts.insert(name.name.unwrap_or(name.path), contract);
                }
            }
        }

        standard_json.errors.extend(errors);
        standard_json.version = Some(solc_version.default.to_string());
        standard_json.long_version = Some(solc_version.long.to_owned());

        Ok(())
    }
}

impl solx_solc::CollectableError for Build {
    fn errors(&self) -> Vec<&solx_solc::StandardJsonOutputError> {
        let mut errors: Vec<&solx_solc::StandardJsonOutputError> = self
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

    fn take_warnings(&mut self) -> Vec<solx_solc::StandardJsonOutputError> {
        let warnings = self
            .messages
            .iter()
            .filter(|message| message.severity == "warning")
            .cloned()
            .collect();
        self.messages
            .retain(|message| message.severity != "warning");
        warnings
    }
}
