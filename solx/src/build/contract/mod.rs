//!
//! The Solidity contract build.
//!

pub mod object;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use self::object::Object;

///
/// The Solidity contract build.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The contract name.
    pub name: era_compiler_common::ContractName,
    /// The deploy code object.
    pub deploy_object: Object,
    /// The runtime code object.
    pub runtime_object: Object,
    /// The combined `solc` and `solx` metadata.
    pub metadata: Option<String>,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: era_compiler_common::ContractName,
        deploy_object: Object,
        runtime_object: Object,
        metadata: Option<String>,
    ) -> Self {
        Self {
            name,
            deploy_object,
            runtime_object,
            metadata,
        }
    }

    ///
    /// Writes the contract text assembly and bytecode to terminal.
    ///
    pub fn write_to_terminal(
        mut self,
        output_selection: &solx_standard_json::InputSelection,
    ) -> anyhow::Result<()> {
        writeln!(
            std::io::stdout(),
            "\n======= {} =======",
            self.name.full_path
        )?;

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::BytecodeObject,
        ) {
            let mut deploy_bytecode_hex = self
                .deploy_object
                .bytecode_hex
                .take()
                .expect("Always exists");

            let runtime_bytecode_hex = self
                .runtime_object
                .bytecode_hex
                .take()
                .expect("Always exists");
            if deploy_bytecode_hex.len() > runtime_bytecode_hex.len() {
                deploy_bytecode_hex
                    .truncate(deploy_bytecode_hex.len() - runtime_bytecode_hex.len());
                deploy_bytecode_hex.push_str(runtime_bytecode_hex.as_str());
            }

            writeln!(std::io::stdout(), "Binary:\n{deploy_bytecode_hex}")?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::BytecodeLLVMAssembly,
        ) {
            let deploy_assembly = self.deploy_object.assembly.take().expect("Always exists");
            writeln!(std::io::stdout(), "Deploy assembly:\n{deploy_assembly}")?;
        }
        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::RuntimeBytecodeLLVMAssembly,
        ) {
            let runtime_assembly = self.runtime_object.assembly.take().expect("Always exists");
            writeln!(std::io::stdout(), "Runtime assembly:\n{runtime_assembly}")?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::Metadata,
        ) {
            writeln!(
                std::io::stdout(),
                "Metadata:\n{}",
                self.metadata.expect("Always exists")
            )?;
        }

        Ok(())
    }

    ///
    /// Writes the contract text assembly and bytecode to files.
    ///
    pub fn write_to_directory(
        mut self,
        output_path: &Path,
        output_selection: &solx_standard_json::InputSelection,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        let file_path = PathBuf::from(self.name.path.as_str());
        let file_name = file_path
            .file_name()
            .expect("Always exists")
            .to_str()
            .expect("Always valid");

        let mut output_path = output_path.to_owned();
        output_path.push(file_name);
        std::fs::create_dir_all(output_path.as_path())?;

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::BytecodeObject,
        ) {
            let output_name = format!(
                "{}.{}",
                self.name.name.as_deref().unwrap_or(file_name),
                era_compiler_common::EXTENSION_EVM_BINARY
            );
            let mut output_path = output_path.clone();
            output_path.push(output_name.as_str());

            if output_path.exists() && !overwrite {
                anyhow::bail!(
                    "Refusing to overwrite an existing file {output_path:?} (use --overwrite to force)."
                );
            } else {
                let mut deploy_bytecode_hex = self
                    .deploy_object
                    .bytecode_hex
                    .take()
                    .expect("Always exists");

                let runtime_bytecode_hex = self
                    .runtime_object
                    .bytecode_hex
                    .take()
                    .expect("Always exists");
                if deploy_bytecode_hex.len() > runtime_bytecode_hex.len() {
                    deploy_bytecode_hex
                        .truncate(deploy_bytecode_hex.len() - runtime_bytecode_hex.len());
                    deploy_bytecode_hex.push_str(runtime_bytecode_hex.as_str());
                }

                std::fs::write(output_path.as_path(), deploy_bytecode_hex)
                    .map_err(|error| anyhow::anyhow!("File {output_path:?} writing: {error}"))?;
            }
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::BytecodeLLVMAssembly,
        ) {
            for (object, code_segment) in [&mut self.deploy_object, &mut self.runtime_object]
                .iter_mut()
                .zip([
                    era_compiler_common::CodeSegment::Deploy,
                    era_compiler_common::CodeSegment::Runtime,
                ])
            {
                let output_name = format!(
                    "{}{}.{}",
                    self.name.name.as_deref().unwrap_or(file_name),
                    match code_segment {
                        era_compiler_common::CodeSegment::Deploy => "".to_owned(),
                        era_compiler_common::CodeSegment::Runtime => format!(".{code_segment}"),
                    },
                    era_compiler_common::EXTENSION_EVM_ASSEMBLY,
                );
                let mut output_path = output_path.clone();
                output_path.push(output_name.as_str());

                if output_path.exists() && !overwrite {
                    anyhow::bail!(
                        "Refusing to overwrite an existing file {output_path:?} (use --overwrite to force)."
                    );
                } else {
                    let assembly = object.assembly.take().expect("Always exists");
                    std::fs::write(output_path.as_path(), assembly).map_err(|error| {
                        anyhow::anyhow!("File {output_path:?} writing: {error}")
                    })?;
                }
            }
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::Metadata,
        ) {
            let output_name = format!(
                "{}_meta.{}",
                self.name.name.as_deref().unwrap_or(file_name),
                era_compiler_common::EXTENSION_JSON,
            );
            let mut output_path = output_path.clone();
            output_path.push(output_name.as_str());

            if output_path.exists() && !overwrite {
                anyhow::bail!(
                    "Refusing to overwrite an existing file {output_path:?} (use --overwrite to force)."
                );
            } else {
                std::fs::write(output_path.as_path(), self.metadata.expect("Always exists"))
                    .map_err(|error| anyhow::anyhow!("File {output_path:?} writing: {error}"))?;
            }
        }

        Ok(())
    }

    ///
    /// Writes the contract text assembly and bytecode to the standard JSON.
    ///
    pub fn write_to_standard_json(
        self,
        standard_json_contract: &mut solx_standard_json::OutputContract,
        output_selection: &solx_standard_json::InputSelection,
    ) {
        standard_json_contract.metadata = self.metadata.filter(|_| {
            output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::Metadata,
            )
        });

        let evm = standard_json_contract
            .evm
            .get_or_insert_with(solx_standard_json::OutputContractEVM::default);
        evm.bytecode = Some(solx_standard_json::OutputContractEVMBytecode::new(
            self.deploy_object.bytecode_hex.filter(|_| {
                output_selection.check_selection(
                    self.name.path.as_str(),
                    self.name.name.as_deref(),
                    solx_standard_json::InputSelector::BytecodeObject,
                )
            }),
            self.deploy_object.assembly.filter(|_| {
                output_selection.check_selection(
                    self.name.path.as_str(),
                    self.name.name.as_deref(),
                    solx_standard_json::InputSelector::BytecodeLLVMAssembly,
                )
            }),
            if output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::BytecodeLinkReferences,
            ) {
                Some(self.deploy_object.unlinked_symbols)
            } else {
                None
            },
            if output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::BytecodeOpcodes,
            ) {
                Some(String::new())
            } else {
                None
            },
            if output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::BytecodeSourceMap,
            ) {
                Some(String::new())
            } else {
                None
            },
            if output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::BytecodeGeneratedSources,
            ) {
                Some(Vec::new())
            } else {
                None
            },
            if output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::BytecodeFunctionDebugData,
            ) {
                Some(BTreeMap::new())
            } else {
                None
            },
            None,
        ));
        evm.deployed_bytecode = Some(solx_standard_json::OutputContractEVMBytecode::new(
            self.runtime_object.bytecode_hex.filter(|_| {
                output_selection.check_selection(
                    self.name.path.as_str(),
                    self.name.name.as_deref(),
                    solx_standard_json::InputSelector::RuntimeBytecodeObject,
                )
            }),
            self.runtime_object.assembly.filter(|_| {
                output_selection.check_selection(
                    self.name.path.as_str(),
                    self.name.name.as_deref(),
                    solx_standard_json::InputSelector::RuntimeBytecodeLLVMAssembly,
                )
            }),
            if output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::RuntimeBytecodeLinkReferences,
            ) {
                Some(self.runtime_object.unlinked_symbols)
            } else {
                None
            },
            if output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::RuntimeBytecodeOpcodes,
            ) {
                Some(String::new())
            } else {
                None
            },
            if output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::RuntimeBytecodeSourceMap,
            ) {
                Some(String::new())
            } else {
                None
            },
            if output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::RuntimeBytecodeGeneratedSources,
            ) {
                Some(Vec::new())
            } else {
                None
            },
            if output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::RuntimeBytecodeFunctionDebugData,
            ) {
                Some(BTreeMap::new())
            } else {
                None
            },
            if output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::RuntimeBytecodeImmutableReferences,
            ) {
                Some(serde_json::json!({}))
            } else {
                None
            },
        ));
    }
}
