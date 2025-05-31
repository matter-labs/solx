//!
//! The `solc --asm-json` output.
//!

pub mod data;
pub mod instruction;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use era_compiler_llvm_context::IContext;

use crate::evmla::ethereal_ir::entry_link::EntryLink;
use crate::evmla::ethereal_ir::EtherealIR;

use self::data::Data;
use self::instruction::name::Name as InstructionName;
use self::instruction::Instruction;

///
/// The JSON assembly.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Assembly {
    /// The metadata string.
    #[serde(rename = ".auxdata")]
    pub auxdata: Option<String>,
    /// The deploy code instructions.
    #[serde(rename = ".code")]
    pub code: Option<Vec<Instruction>>,
    /// The runtime code.
    #[serde(rename = ".data")]
    pub data: Option<BTreeMap<String, Data>>,

    /// The full contract path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_path: Option<String>,
    /// The factory dependency paths.
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub factory_dependencies: HashSet<String>,
    /// The EVM legacy assembly extra metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_metadata: Option<solx_standard_json::OutputContractEVMExtraMetadata>,
}

impl Assembly {
    ///
    /// Sets the full contract path.
    ///
    pub fn set_full_path(&mut self, full_path: String) {
        self.full_path = Some(full_path);
    }

    ///
    /// Returns the full contract path if it is set, or `<undefined>` otherwise.
    ///
    /// # Panics
    /// If the `full_path` has not been set.
    ///
    pub fn full_path(&self) -> &str {
        self.full_path
            .as_deref()
            .unwrap_or_else(|| panic!("The full path of some contracts is unset"))
    }

    ///
    /// Returns a runtime code reference from the deploy code assembly.
    ///
    pub fn runtime_code(&self) -> anyhow::Result<&Assembly> {
        match self
            .data
            .as_ref()
            .and_then(|data| data.get("0"))
            .ok_or_else(|| anyhow::anyhow!("Runtime code data not found"))?
        {
            Data::Assembly(assembly) => Ok(assembly),
            Data::Hash(hash) => {
                anyhow::bail!("Expected runtime code, found hash `{hash}`");
            }
            Data::Path(path) => {
                anyhow::bail!("Expected runtime code, found path `{path}`");
            }
        }
    }

    ///
    /// Returns a runtime code mutable reference from the deploy code assembly.
    ///
    pub fn runtime_code_mut(&mut self) -> anyhow::Result<&mut Assembly> {
        match self
            .data
            .as_mut()
            .and_then(|data| data.get_mut("0"))
            .ok_or_else(|| anyhow::anyhow!("Runtime code data not found"))?
        {
            Data::Assembly(assembly) => Ok(assembly),
            Data::Hash(hash) => {
                anyhow::bail!("Expected runtime code, found hash `{hash}`");
            }
            Data::Path(path) => {
                anyhow::bail!("Expected runtime code, found path `{path}`");
            }
        }
    }

    ///
    /// Get the list of unlinked deployable libraries.
    ///
    pub fn get_unlinked_libraries(&self) -> BTreeSet<String> {
        let mut unlinked_libraries = BTreeSet::new();
        if let Some(code) = self.code.as_ref() {
            for instruction in code.iter() {
                if let InstructionName::PUSHLIB = instruction.name {
                    let library_path = instruction.value.to_owned().expect("Always exists");
                    unlinked_libraries.insert(library_path);
                }
            }
        }
        if let Some(data) = self.data.as_ref() {
            for (_, data) in data.iter() {
                unlinked_libraries.extend(data.get_unlinked_libraries());
            }
        }
        unlinked_libraries
    }

    ///
    /// Get the list of EVM dependencies.
    ///
    pub fn accumulate_evm_dependencies(&self, dependencies: &mut solx_yul::Dependencies) {
        if let Some(code) = self.code.as_ref() {
            for instruction in code.iter() {
                match instruction.name {
                    InstructionName::PUSH_DataOffset | InstructionName::PUSH_DataSize => {
                        let dependency = instruction.value.to_owned().expect("Always exists");
                        let is_runtime_code = dependencies.identifier
                            == dependency
                                .strip_suffix(
                                    format!(".{}", era_compiler_common::CodeSegment::Runtime)
                                        .as_str(),
                                )
                                .unwrap_or(dependencies.identifier.as_str());
                        dependencies.push(dependency, is_runtime_code);
                    }
                    _ => {}
                }
            }
        }
    }

    ///
    /// Returns the `keccak256` hash of the assembly representation.
    ///
    pub fn keccak256(&self) -> String {
        let json: Vec<u8> = serde_json::to_vec(self).expect("Always valid");
        era_compiler_common::Keccak256Hash::from_slice(json.as_slice()).to_string()
    }

    ///
    /// Replaces with dependency indexes with actual data.
    ///
    pub fn preprocess_dependencies(
        contracts: &mut BTreeMap<String, BTreeMap<String, solx_standard_json::OutputContract>>,
    ) -> anyhow::Result<()> {
        let mut hash_path_mapping = BTreeMap::new();

        for (path, file) in contracts.iter() {
            for (name, contract) in file.iter() {
                let deploy_code_path = format!("{path}:{name}");
                let deploy_code_assembly = match contract
                    .evm
                    .as_ref()
                    .and_then(|evm| evm.legacy_assembly.as_ref())
                    .filter(|json| json.is_object())
                {
                    Some(assembly) => serde_json::from_value::<Assembly>(assembly.to_owned())
                        .expect("Always valid"),
                    None => continue,
                };
                let deploy_code_hash = deploy_code_assembly.keccak256();

                let runtime_code_path = format!(
                    "{path}:{name}.{}",
                    era_compiler_common::CodeSegment::Runtime
                );
                let runtime_code_assembly = deploy_code_assembly.runtime_code()?;
                let runtime_code_hash = runtime_code_assembly.keccak256();

                hash_path_mapping.insert(deploy_code_hash, deploy_code_path);
                hash_path_mapping.insert(runtime_code_hash, runtime_code_path);
            }
        }

        let mut assemblies = BTreeMap::new();
        for (path, file) in contracts.iter_mut() {
            for (name, contract) in file.iter_mut() {
                let full_path = format!("{path}:{name}");
                let assembly = match contract
                    .evm
                    .as_mut()
                    .and_then(|evm| evm.legacy_assembly.as_mut())
                    .filter(|json| json.is_object())
                {
                    Some(assembly) => assembly,
                    None => continue,
                };
                assemblies.insert(full_path, assembly);
            }
        }
        assemblies
            .into_par_iter()
            .map(|(full_path, assembly_json)| {
                let mut assembly: Assembly =
                    serde_json::from_value(assembly_json.to_owned()).expect("Always valid");
                Self::preprocess_dependency_level(
                    full_path.as_str(),
                    &mut assembly,
                    &hash_path_mapping,
                )?;
                *assembly_json = serde_json::to_value(&assembly).expect("Always valid");
                Ok(())
            })
            .collect::<anyhow::Result<()>>()?;

        Ok(())
    }

    ///
    /// Preprocesses an assembly JSON structure dependency data map.
    ///
    fn preprocess_dependency_level(
        full_path: &str,
        assembly: &mut Assembly,
        hash_path_mapping: &BTreeMap<String, String>,
    ) -> anyhow::Result<()> {
        assembly.set_full_path(full_path.to_owned());

        let deploy_code_index_path_mapping =
            assembly.deploy_dependencies_pass(full_path, hash_path_mapping)?;
        if let Some(deploy_code_instructions) = assembly.code.as_deref_mut() {
            Instruction::replace_data_aliases(
                deploy_code_instructions,
                &deploy_code_index_path_mapping,
            )?;
        };

        let runtime_code_index_path_mapping =
            assembly.runtime_dependencies_pass(hash_path_mapping)?;
        if let Some(runtime_code_instructions) = assembly
            .data
            .as_mut()
            .and_then(|data_map| data_map.get_mut("0"))
            .and_then(|data| data.get_assembly_mut())
            .and_then(|assembly| assembly.code.as_deref_mut())
        {
            Instruction::replace_data_aliases(
                runtime_code_instructions,
                &runtime_code_index_path_mapping,
            )?;
        }

        Ok(())
    }

    ///
    /// Replaces the deploy code dependencies with full contract path and returns the list.
    ///
    fn deploy_dependencies_pass(
        &mut self,
        full_path: &str,
        hash_data_mapping: &BTreeMap<String, String>,
    ) -> anyhow::Result<BTreeMap<String, String>> {
        let mut index_path_mapping = BTreeMap::new();
        let index = "0".repeat(era_compiler_common::BYTE_LENGTH_FIELD * 2);
        index_path_mapping.insert(
            index,
            format!("{full_path}.{}", era_compiler_common::CodeSegment::Runtime),
        );

        let dependencies = match self.data.as_mut() {
            Some(dependencies) => dependencies,
            None => return Ok(index_path_mapping),
        };
        for (index, data) in dependencies.iter_mut() {
            if index == "0" {
                continue;
            }

            let mut index_extended =
                "0".repeat(era_compiler_common::BYTE_LENGTH_FIELD * 2 - index.len());
            index_extended.push_str(index.as_str());

            *data = match data {
                Data::Assembly(assembly) => {
                    let hash = Assembly::keccak256(assembly);
                    let full_path =
                        hash_data_mapping
                            .get(hash.as_str())
                            .cloned()
                            .ok_or_else(|| {
                                anyhow::anyhow!("Contract path not found for hash `{hash}`")
                            })?;
                    self.factory_dependencies.insert(full_path.to_owned());

                    index_path_mapping.insert(index_extended, full_path.clone());
                    Data::Path(full_path)
                }
                Data::Hash(hash) => {
                    index_path_mapping.insert(index_extended, hash.to_owned());
                    continue;
                }
                _ => continue,
            };
        }

        Ok(index_path_mapping)
    }

    ///
    /// Replaces the runtime code dependencies with full contract path and returns the list.
    ///
    fn runtime_dependencies_pass(
        &mut self,
        hash_data_mapping: &BTreeMap<String, String>,
    ) -> anyhow::Result<BTreeMap<String, String>> {
        let mut index_path_mapping = BTreeMap::new();

        let dependencies = match self
            .data
            .as_mut()
            .and_then(|data| data.get_mut("0"))
            .and_then(|data| data.get_assembly_mut())
            .and_then(|assembly| assembly.data.as_mut())
        {
            Some(dependencies) => dependencies,
            None => return Ok(index_path_mapping),
        };
        for (index, data) in dependencies.iter_mut() {
            let mut index_extended =
                "0".repeat(era_compiler_common::BYTE_LENGTH_FIELD * 2 - index.len());
            index_extended.push_str(index.as_str());

            *data = match data {
                Data::Assembly(assembly) => {
                    let hash = Assembly::keccak256(assembly);
                    let full_path =
                        hash_data_mapping
                            .get(hash.as_str())
                            .cloned()
                            .ok_or_else(|| {
                                anyhow::anyhow!("Contract path not found for hash `{hash}`")
                            })?;
                    self.factory_dependencies.insert(full_path.to_owned());

                    index_path_mapping.insert(index_extended, full_path.clone());
                    Data::Path(full_path)
                }
                Data::Hash(hash) => {
                    index_path_mapping.insert(index_extended, hash.to_owned());
                    continue;
                }
                _ => continue,
            };
        }

        Ok(index_path_mapping)
    }
}

impl era_compiler_llvm_context::EVMWriteLLVM for Assembly {
    fn declare(
        &mut self,
        _context: &mut era_compiler_llvm_context::EVMContext,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn into_llvm(self, context: &mut era_compiler_llvm_context::EVMContext) -> anyhow::Result<()> {
        let full_path = self.full_path().to_owned();

        let (code_segment, blocks) = if let Ok(runtime_code) = self.runtime_code() {
            if let Some(debug_config) = context.debug_config() {
                debug_config.dump_evmla(full_path.as_str(), self.to_string().as_str())?;
            }

            let deploy_code_blocks = EtherealIR::get_blocks(
                context.evmla().expect("Always exists").version.to_owned(),
                era_compiler_common::CodeSegment::Deploy,
                self.code
                    .as_deref()
                    .ok_or_else(|| anyhow::anyhow!("Deploy code instructions not found"))?,
            )?;

            let runtime_code_instructions = runtime_code
                .code
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Runtime code instructions not found"))?;
            let runtime_code_blocks = EtherealIR::get_blocks(
                context.evmla().expect("Always exists").version.to_owned(),
                era_compiler_common::CodeSegment::Runtime,
                runtime_code_instructions.as_slice(),
            )?;

            let mut blocks = deploy_code_blocks;
            blocks.extend(runtime_code_blocks);
            (era_compiler_common::CodeSegment::Deploy, blocks)
        } else {
            if let Some(debug_config) = context.debug_config() {
                debug_config.dump_evmla(
                    format!("{full_path}.{}", era_compiler_common::CodeSegment::Runtime).as_str(),
                    self.to_string().as_str(),
                )?;
            }

            let blocks = EtherealIR::get_blocks(
                context.evmla().expect("Always exists").version.to_owned(),
                era_compiler_common::CodeSegment::Runtime,
                self.code
                    .as_deref()
                    .ok_or_else(|| anyhow::anyhow!("Deploy code instructions not found"))?,
            )?;
            (era_compiler_common::CodeSegment::Runtime, blocks)
        };

        let mut ethereal_ir = EtherealIR::new(
            context.evmla().expect("Always exists").version.to_owned(),
            self.extra_metadata.unwrap_or_default(),
            Some(code_segment),
            blocks,
        )?;
        if let Some(debug_config) = context.debug_config() {
            let mut path = full_path.to_owned();
            if let era_compiler_common::CodeSegment::Runtime = code_segment {
                path.push_str(format!(".{}", code_segment).as_str());
            }
            debug_config.dump_ethir(path.as_str(), ethereal_ir.to_string().as_str())?;
        }
        ethereal_ir.declare(context)?;
        ethereal_ir.into_llvm(context)?;

        let mut entry =
            era_compiler_llvm_context::EVMEntryFunction::new(EntryLink::new(code_segment));
        entry.declare(context)?;
        entry.into_llvm(context)?;

        Ok(())
    }
}

impl std::fmt::Display for Assembly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(instructions) = self.code.as_ref() {
            for (index, instruction) in instructions.iter().enumerate() {
                match instruction.name {
                    InstructionName::Tag => writeln!(f, "{index:03} {instruction}")?,
                    _ => writeln!(f, "{index:03}     {instruction}")?,
                }
            }
        }

        Ok(())
    }
}
