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
    pub deploy_object: Object,
    /// The runtime code object.
    pub runtime_object: Object,
    /// The metadata hash.
    pub metadata_hash: Option<era_compiler_common::Hash>,
    /// The metadata string.
    pub metadata_string: String,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: era_compiler_common::ContractName,
        deploy_object: Object,
        runtime_object: Object,
        metadata_hash: Option<era_compiler_common::Hash>,
        metadata_string: String,
    ) -> Self {
        Self {
            name,
            deploy_object,
            runtime_object,
            metadata_hash,
            metadata_string,
        }
    }

    ///
    /// Writes the contract text assembly and bytecode to terminal.
    ///
    pub fn write_to_terminal(
        self,
        path: String,
        output_metadata: bool,
        output_binary: bool,
    ) -> anyhow::Result<()> {
        writeln!(std::io::stdout(), "\n======= {path} =======")?;
        if output_metadata {
            writeln!(std::io::stdout(), "Metadata:\n{}", self.metadata_string)?;
        }
        if output_binary {
            writeln!(
                std::io::stdout(),
                "Binary:\n{}{}",
                hex::encode(self.deploy_object.bytecode),
                hex::encode(self.runtime_object.bytecode),
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
        output_metadata: bool,
        output_binary: bool,
        overwrite: bool,
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
                std::fs::write(
                    output_path.as_path(),
                    self.metadata_string.to_string().as_bytes(),
                )
                .map_err(|error| anyhow::anyhow!("File {output_path:?} writing: {error}"))?;
            }
        }

        if output_binary {
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
                let mut bytecode_hexadecimal = hex::encode(self.deploy_object.bytecode);
                bytecode_hexadecimal.push_str(hex::encode(self.runtime_object.bytecode).as_str());
                std::fs::write(output_path.as_path(), bytecode_hexadecimal.as_bytes())
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
        standard_json_contract: &mut solx_solc::StandardJsonOutputContract,
    ) -> anyhow::Result<()> {
        standard_json_contract.metadata = self.metadata_string;
        standard_json_contract
            .evm
            .get_or_insert_with(solx_solc::StandardJsonOutputContractEVM::default)
            .modify(
                hex::encode(self.deploy_object.bytecode),
                self.deploy_object.format,
                self.deploy_object.unlinked_libraries,
                hex::encode(self.runtime_object.bytecode),
                self.runtime_object.format,
                self.runtime_object.unlinked_libraries,
            );
        Ok(())
    }
}
