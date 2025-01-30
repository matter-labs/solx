//!
//! The Solidity contract build.
//!

use std::collections::BTreeSet;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

///
/// The Solidity contract build.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The contract name.
    pub name: era_compiler_common::ContractName,
    /// The deploy bytecode identifier.
    pub deploy_identifier: String,
    /// The deploy bytecode.
    pub deploy_build: Vec<u8>,
    /// The runtime bytecode identifier.
    pub runtime_identifier: String,
    /// The runtime bytecode.
    pub runtime_build: Vec<u8>,
    /// The metadata hash.
    pub metadata_hash: Option<era_compiler_common::Hash>,
    /// The metadata JSON.
    pub metadata_json: serde_json::Value,
    /// The unlinked missing libraries.
    pub missing_libraries: BTreeSet<String>,
    /// The binary object format.
    pub object_format: era_compiler_common::ObjectFormat,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: era_compiler_common::ContractName,
        deploy_identifier: String,
        deploy_build: Vec<u8>,
        runtime_identifier: String,
        runtime_build: Vec<u8>,
        metadata_hash: Option<era_compiler_common::Hash>,
        metadata_json: serde_json::Value,
        missing_libraries: BTreeSet<String>,
        object_format: era_compiler_common::ObjectFormat,
    ) -> Self {
        Self {
            name,
            deploy_identifier,
            deploy_build,
            runtime_identifier,
            runtime_build,
            metadata_hash,
            metadata_json,
            missing_libraries,
            object_format,
        }
    }

    ///
    /// Writes the contract text assembly and bytecode to terminal.
    ///
    pub fn write_to_terminal(
        self,
        path: String,
        output_metadata: bool,
        output_assembly: bool,
        output_binary: bool,
    ) -> anyhow::Result<()> {
        writeln!(std::io::stdout(), "\n======= {path} =======")?;
        if output_assembly {
            writeln!(std::io::stdout(), "Assembly:\nTODO")?;
        }
        if output_metadata {
            writeln!(std::io::stdout(), "Metadata:\n{}", self.metadata_json)?;
        }
        if output_binary {
            writeln!(
                std::io::stdout(),
                "Binary:\n{}{}",
                hex::encode(self.deploy_build),
                hex::encode(self.runtime_build),
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
        output_assembly: bool,
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
                    self.metadata_json.to_string().as_bytes(),
                )
                .map_err(|error| anyhow::anyhow!("File {output_path:?} writing: {error}"))?;
            }
        }

        if output_assembly {
            let output_name = format!(
                "{}.{}",
                self.name.name.as_deref().unwrap_or(file_name),
                "asm"
            );
            let mut output_path = output_path.clone();
            output_path.push(output_name.as_str());

            if output_path.exists() && !overwrite {
                anyhow::bail!(
                    "Refusing to overwrite an existing file {output_path:?} (use --overwrite to force)."
                );
            } else {
                std::fs::write(output_path.as_path(), "TODO".as_bytes())
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
                let mut bytecode_hexadecimal = hex::encode(self.deploy_build.as_slice());
                bytecode_hexadecimal.push_str(hex::encode(self.runtime_build.as_slice()).as_str());
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
        let deploy_bytecode = hex::encode(self.deploy_build.as_slice());
        let runtime_bytecode = hex::encode(self.runtime_build.as_slice());

        standard_json_contract.metadata = self.metadata_json;
        standard_json_contract
            .evm
            .get_or_insert_with(solx_solc::StandardJsonOutputContractEVM::default)
            .modify_evm(deploy_bytecode, runtime_bytecode);
        standard_json_contract
            .missing_libraries
            .extend(self.missing_libraries);
        standard_json_contract.object_format = Some(self.object_format);

        Ok(())
    }

    ///
    /// Writes the contract text assembly and bytecode to the combined JSON.
    ///
    pub fn write_to_combined_json(
        self,
        combined_json_contract: &mut solx_solc::CombinedJsonContract,
    ) -> anyhow::Result<()> {
        if let Some(metadata) = combined_json_contract.metadata.as_mut() {
            *metadata = self.metadata_json.to_string();
        }

        combined_json_contract.bin = Some(hex::encode(self.deploy_build));
        combined_json_contract.bin_runtime = Some(hex::encode(self.runtime_build));

        combined_json_contract
            .missing_libraries
            .extend(self.missing_libraries);
        combined_json_contract.object_format = Some(self.object_format);

        Ok(())
    }
}
