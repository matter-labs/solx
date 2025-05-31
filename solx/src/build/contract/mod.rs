//!
//! The Solidity contract build.
//!

pub mod object;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use normpath::PathExt;

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
    /// The solc EVM legacy assembly.
    pub legacy_assembly: Option<serde_json::Value>,
    /// The solc optimized Yul IR.
    pub ir_optimized: Option<String>,
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
            abi: None,
            method_identifiers: None,
            userdoc: None,
            devdoc: None,
            storage_layout: None,
            transient_storage_layout: None,
            legacy_assembly: None,
            ir_optimized: None,
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
            solx_standard_json::InputSelector::EVMLegacyAssembly,
        ) {
            let legacy_assembly = self.legacy_assembly.take().expect("Always exists");
            writeln!(std::io::stdout(), "EVM assembly:\n{legacy_assembly}")?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::BytecodeLLVMAssembly,
        ) {
            let deploy_assembly = self.deploy_object.assembly.take().expect("Always exists");
            writeln!(
                std::io::stdout(),
                "Deploy LLVM EVM assembly:\n{deploy_assembly}"
            )?;
        }
        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::RuntimeBytecodeLLVMAssembly,
        ) {
            let runtime_assembly = self.runtime_object.assembly.take().expect("Always exists");
            writeln!(
                std::io::stdout(),
                "Runtime LLVM EVM assembly:\n{runtime_assembly}"
            )?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::BytecodeObject,
        ) {
            let bytecode_hex = self
                .deploy_object
                .bytecode_hex
                .take()
                .expect("Always exists");
            writeln!(std::io::stdout(), "Binary:\n{bytecode_hex}")?;
        }
        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::RuntimeBytecodeObject,
        ) {
            let bytecode_hex = self
                .runtime_object
                .bytecode_hex
                .take()
                .expect("Always exists");
            writeln!(
                std::io::stdout(),
                "Binary of the runtime part:\n{bytecode_hex}"
            )?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::Yul,
        ) {
            let ir_optimized = self.ir_optimized.take().expect("Always exists");
            writeln!(std::io::stdout(), "Optimized IR:\n{ir_optimized}")?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::MethodIdentifiers,
        ) {
            writeln!(std::io::stdout(), "Function signatures:")?;
            for (signature, identifier) in
                self.method_identifiers.expect("Always exists").into_iter()
            {
                writeln!(std::io::stdout(), "{identifier}: {signature}")?;
            }
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

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::ABI,
        ) {
            writeln!(
                std::io::stdout(),
                "Contract JSON ABI:\n{}",
                self.abi.expect("Always exists")
            )?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::StorageLayout,
        ) {
            writeln!(
                std::io::stdout(),
                "Contract Storage Layout:\n{}",
                self.storage_layout.expect("Always exists")
            )?;
        }
        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::TransientStorageLayout,
        ) {
            writeln!(
                std::io::stdout(),
                "Contract Transient Storage Layout:\n{}",
                self.transient_storage_layout.expect("Always exists")
            )?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::DeveloperDocumentation,
        ) {
            writeln!(
                std::io::stdout(),
                "Developer Documentation:\n{}",
                self.devdoc.expect("Always exists")
            )?;
        }
        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::UserDocumentation,
        ) {
            writeln!(
                std::io::stdout(),
                "User Documentation:\n{}",
                self.userdoc.expect("Always exists")
            )?;
        }

        Ok(())
    }

    ///
    /// Writes the contract text assembly and bytecode to files.
    ///
    pub fn write_to_directory(
        mut self,
        output_directory: &Path,
        output_selection: &solx_standard_json::InputSelection,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        let contract_path = PathBuf::from(self.name.path.as_str());
        let contract_name = contract_path
            .file_name()
            .expect("Always exists")
            .to_str()
            .expect("Always valid");
        let contract_path = contract_path.normalize()?;
        let contract_path = if contract_path.starts_with(std::env::current_dir()?) {
            contract_path
                .as_path()
                .strip_prefix(std::env::current_dir()?)?
        } else {
            contract_path.as_path()
        }
        .to_string_lossy()
        .replace(['\\', '/', '.'], "_");

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::BytecodeObject,
        ) {
            let output_name = format!(
                "{contract_path}_{}.{}",
                self.name.name.as_deref().unwrap_or(contract_name),
                era_compiler_common::EXTENSION_EVM_BINARY
            );
            let mut output_path = output_directory.to_owned();
            output_path.push(output_name.as_str());

            let bytecode_hex = self
                .deploy_object
                .bytecode_hex
                .take()
                .expect("Always exists");
            Self::write_to_file(output_path.as_path(), bytecode_hex, overwrite)?;
        }
        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::RuntimeBytecodeObject,
        ) {
            let output_name = format!(
                "{contract_path}_{}.{}-{}",
                self.name.name.as_deref().unwrap_or(contract_name),
                era_compiler_common::EXTENSION_EVM_BINARY,
                era_compiler_common::CodeSegment::Runtime,
            );
            let mut output_path = output_directory.to_owned();
            output_path.push(output_name.as_str());

            let bytecode_hex = self
                .runtime_object
                .bytecode_hex
                .take()
                .expect("Always exists");
            Self::write_to_file(output_path.as_path(), bytecode_hex, overwrite)?;
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
                    "{contract_path}_{}_llvm.{}{}",
                    self.name.name.as_deref().unwrap_or(contract_name),
                    era_compiler_common::EXTENSION_EVM_ASSEMBLY,
                    match code_segment {
                        era_compiler_common::CodeSegment::Deploy => "".to_owned(),
                        era_compiler_common::CodeSegment::Runtime => format!("-{code_segment}"),
                    },
                );
                let mut output_path = output_directory.to_owned();
                output_path.push(output_name.as_str());

                let assembly = object.assembly.take().expect("Always exists");
                Self::write_to_file(output_path.as_path(), assembly, overwrite)?;
            }
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::Metadata,
        ) {
            let output_name = format!(
                "{contract_path}_{}_meta.{}",
                self.name.name.as_deref().unwrap_or(contract_name),
                era_compiler_common::EXTENSION_JSON,
            );
            let mut output_path = output_directory.to_owned();
            output_path.push(output_name.as_str());

            let metadata = self.metadata.take().expect("Always exists");
            Self::write_to_file(output_path.as_path(), metadata, overwrite)?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::ABI,
        ) {
            let output_name = format!(
                "{contract_path}_{}.abi",
                self.name.name.as_deref().unwrap_or(contract_name),
            );
            let mut output_path = output_directory.to_owned();
            output_path.push(output_name.as_str());

            let abi = self.abi.take().expect("Always exists").to_string();
            Self::write_to_file(output_path.as_path(), abi, overwrite)?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::MethodIdentifiers,
        ) {
            let output_name = format!(
                "{contract_path}_{}.signatures",
                self.name.name.as_deref().unwrap_or(contract_name),
            );
            let mut output_path = output_directory.to_owned();
            output_path.push(output_name.as_str());

            let mut output = "Function signatures:\n".to_owned();
            for (signature, identifier) in
                self.method_identifiers.expect("Always exists").into_iter()
            {
                output.push_str(format!("{identifier}: {signature}\n").as_str());
            }
            Self::write_to_file(output_path.as_path(), output, overwrite)?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::StorageLayout,
        ) {
            let output_name = format!(
                "{contract_path}_{}_storage.{}",
                self.name.name.as_deref().unwrap_or(contract_name),
                era_compiler_common::EXTENSION_JSON,
            );
            let mut output_path = output_directory.to_owned();
            output_path.push(output_name.as_str());

            let storage_layout = self.storage_layout.expect("Always exists").to_string();
            Self::write_to_file(output_path.as_path(), storage_layout, overwrite)?;
        }
        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::TransientStorageLayout,
        ) {
            let output_name = format!(
                "{contract_path}_{}_transient_storage.{}",
                self.name.name.as_deref().unwrap_or(contract_name),
                era_compiler_common::EXTENSION_JSON,
            );
            let mut output_path = output_directory.to_owned();
            output_path.push(output_name.as_str());

            let transient_storage_layout = self
                .transient_storage_layout
                .expect("Always exists")
                .to_string();

            Self::write_to_file(output_path.as_path(), transient_storage_layout, overwrite)?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::DeveloperDocumentation,
        ) {
            let output_name = format!(
                "{contract_path}_{}.docdev",
                self.name.name.as_deref().unwrap_or(contract_name),
            );
            let mut output_path = output_directory.to_owned();
            output_path.push(output_name.as_str());

            let devdoc = self.devdoc.expect("Always exists").to_string();
            Self::write_to_file(output_path.as_path(), devdoc, overwrite)?;
        }
        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::UserDocumentation,
        ) {
            let output_name = format!(
                "{contract_path}_{}.docuser",
                self.name.name.as_deref().unwrap_or(contract_name),
            );
            let mut output_path = output_directory.to_owned();
            output_path.push(output_name.as_str());

            let userdoc = self.userdoc.expect("Always exists").to_string();
            Self::write_to_file(output_path.as_path(), userdoc, overwrite)?;
        }

        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::EVMLegacyAssembly,
        ) {
            let output_name = format!(
                "{contract_path}_{}_evm.{}",
                self.name.name.as_deref().unwrap_or(contract_name),
                era_compiler_common::EXTENSION_JSON,
            );
            let mut output_path = output_directory.to_owned();
            output_path.push(output_name.as_str());

            let legacy_assembly = self.legacy_assembly.expect("Always exists").to_string();
            Self::write_to_file(output_path.as_path(), legacy_assembly, overwrite)?;
        }
        if output_selection.check_selection(
            self.name.path.as_str(),
            self.name.name.as_deref(),
            solx_standard_json::InputSelector::Yul,
        ) {
            let output_name = format!(
                "{contract_path}_{}_opt.{}",
                self.name.name.as_deref().unwrap_or(contract_name),
                era_compiler_common::EXTENSION_YUL,
            );
            let mut output_path = output_directory.to_owned();
            output_path.push(output_name.as_str());

            let ir_optimized = self.ir_optimized.expect("Always exists").to_string();
            Self::write_to_file(output_path.as_path(), ir_optimized, overwrite)?;
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
        standard_json_contract.abi = self.abi.filter(|_| {
            output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::ABI,
            )
        });
        standard_json_contract.userdoc = self.userdoc.filter(|_| {
            output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::UserDocumentation,
            )
        });
        standard_json_contract.devdoc = self.devdoc.filter(|_| {
            output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::DeveloperDocumentation,
            )
        });
        standard_json_contract.storage_layout = self.storage_layout.filter(|_| {
            output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::StorageLayout,
            )
        });
        standard_json_contract.transient_storage_layout =
            self.transient_storage_layout.filter(|_| {
                output_selection.check_selection(
                    self.name.path.as_str(),
                    self.name.name.as_deref(),
                    solx_standard_json::InputSelector::TransientStorageLayout,
                )
            });
        standard_json_contract.ir_optimized = self.ir_optimized.filter(|_| {
            output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::Yul,
            )
        });

        let evm = standard_json_contract
            .evm
            .get_or_insert_with(solx_standard_json::OutputContractEVM::default);
        evm.method_identifiers = self.method_identifiers.filter(|_| {
            output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::MethodIdentifiers,
            )
        });
        evm.legacy_assembly = self.legacy_assembly.filter(|_| {
            output_selection.check_selection(
                self.name.path.as_str(),
                self.name.name.as_deref(),
                solx_standard_json::InputSelector::EVMLegacyAssembly,
            )
        });
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

    ///
    /// Writes data to the file, checking the `overwrite` flag.
    ///
    pub fn write_to_file<C: AsRef<[u8]>>(
        output_path: &Path,
        data: C,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        if output_path.exists() && !overwrite {
            anyhow::bail!(
                "Refusing to overwrite an existing file {output_path:?} (use --overwrite to force)."
            );
        } else {
            std::fs::write(output_path, data)
                .map_err(|error| anyhow::anyhow!("File {output_path:?} writing: {error}"))?;
        }
        Ok(())
    }
}
