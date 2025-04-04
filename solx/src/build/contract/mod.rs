//!
//! The Solidity contract build.
//!

pub mod object;

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
    pub deploy_object: Option<Object>,
    /// The runtime code object.
    pub runtime_object: Option<Object>,
    /// The metadata hash.
    pub metadata_hash: Option<era_compiler_common::Hash>,
    /// The combined `solc` and `solx` metadata.
    pub metadata: Option<String>,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: era_compiler_common::ContractName,
        deploy_object: Option<Object>,
        runtime_object: Option<Object>,
        metadata_hash: Option<era_compiler_common::Hash>,
        metadata: Option<String>,
    ) -> Self {
        Self {
            name,
            deploy_object,
            runtime_object,
            metadata_hash,
            metadata,
        }
    }

    ///
    /// Writes the contract text assembly and bytecode to terminal.
    ///
    pub fn write_to_terminal(self, path: String, output_metadata: bool) -> anyhow::Result<()> {
        writeln!(std::io::stdout(), "\n======= {path} =======")?;

        if self.deploy_object.is_some() || self.runtime_object.is_some() {
            let deploy_bytecode = self.deploy_object.map(|object| object.bytecode);
            let runtime_bytecode = self.runtime_object.map(|object| object.bytecode);
            writeln!(
                std::io::stdout(),
                "Binary:\n{}{}",
                hex::encode(deploy_bytecode.unwrap_or_default()),
                hex::encode(runtime_bytecode.unwrap_or_default()),
            )?;
        }

        if output_metadata {
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
        self,
        output_path: &Path,
        overwrite: bool,
        output_metadata: bool,
    ) -> anyhow::Result<()> {
        let file_path = PathBuf::from(self.name.path);
        let file_name = file_path
            .file_name()
            .expect("Always exists")
            .to_str()
            .expect("Always valid");

        let mut output_path = output_path.to_owned();
        output_path.push(file_name);
        std::fs::create_dir_all(output_path.as_path())?;

        if self.deploy_object.is_some() || self.runtime_object.is_some() {
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
                let deploy_bytecode = self.deploy_object.map(|object| object.bytecode);
                let runtime_bytecode = self.runtime_object.map(|object| object.bytecode);
                let bytecode = format!(
                    "{}{}",
                    hex::encode(deploy_bytecode.unwrap_or_default()),
                    hex::encode(runtime_bytecode.unwrap_or_default()),
                );
                std::fs::write(output_path.as_path(), bytecode)
                    .map_err(|error| anyhow::anyhow!("File {output_path:?} writing: {error}"))?;
            }
        }

        if output_metadata {
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
    ) -> anyhow::Result<()> {
        standard_json_contract.metadata = self.metadata;

        let evm = standard_json_contract
            .evm
            .get_or_insert_with(solx_standard_json::OutputContractEVM::default);
        evm.bytecode = self.deploy_object.map(|object| {
            solx_standard_json::OutputContractEVMBytecode::new(
                hex::encode(object.bytecode),
                object.unlinked_libraries,
                object.format,
            )
        });
        evm.deployed_bytecode = self.runtime_object.map(|object| {
            solx_standard_json::OutputContractEVMBytecode::new(
                hex::encode(object.bytecode),
                object.unlinked_libraries,
                object.format,
            )
        });

        Ok(())
    }
}
