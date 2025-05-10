//!
//! Bytecode object.
//!

use std::collections::BTreeMap;

///
/// Bytecode object.
///
/// Can be either deploy and runtime code.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Object {
    /// Object identifier.
    pub identifier: String,
    /// Contract full name.
    pub contract_name: era_compiler_common::ContractName,
    /// Text assembly.
    pub assembly: Option<String>,
    /// Bytecode.
    pub bytecode: Option<Vec<u8>>,
    /// Whether IR codegen is used.
    pub via_ir: bool,
    /// Code segment.
    pub code_segment: era_compiler_common::CodeSegment,
    /// The metadata bytes. Only appended to runtime code.
    pub metadata_bytes: Option<Vec<u8>>,
    /// Dependencies.
    pub dependencies: solx_yul::Dependencies,
    /// The unlinked symbols, such as libraries.
    pub unlinked_symbols: Option<BTreeMap<String, Vec<u64>>>,
    /// Whether the object is already assembled.
    pub is_assembled: bool,
    /// Compilation warnings.
    pub warnings: Vec<era_compiler_llvm_context::EVMWarning>,
}

impl Object {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        identifier: String,
        contract_name: era_compiler_common::ContractName,
        assembly: Option<String>,
        bytecode: Option<Vec<u8>>,
        via_ir: bool,
        code_segment: era_compiler_common::CodeSegment,
        metadata_bytes: Option<Vec<u8>>,
        dependencies: solx_yul::Dependencies,
        unlinked_symbols: Option<BTreeMap<String, Vec<u64>>>,
        warnings: Vec<era_compiler_llvm_context::EVMWarning>,
    ) -> Self {
        Self {
            identifier,
            contract_name,
            assembly,
            bytecode,
            via_ir,
            code_segment,
            metadata_bytes,
            dependencies,
            unlinked_symbols,
            is_assembled: false,
            warnings,
        }
    }

    ///
    /// Appends metadata to the object.
    ///
    /// # Panics
    /// If bytecode is `None`.
    ///
    pub fn to_memory_buffer(
        &self,
        cbor_data: Option<Vec<(String, semver::Version)>>,
    ) -> anyhow::Result<inkwell::memory_buffer::MemoryBuffer> {
        let bytecode = self.bytecode.as_deref().expect("Bytecode is not set");

        let mut memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
            bytecode,
            self.identifier.as_str(),
            false,
        );

        if let (era_compiler_common::CodeSegment::Runtime, metadata_bytes) =
            (self.code_segment, &self.metadata_bytes)
        {
            memory_buffer = era_compiler_llvm_context::evm_append_metadata(
                memory_buffer,
                metadata_bytes.to_owned(),
                cbor_data
                    .map(|cbor_data| (crate::r#const::SOLC_PRODUCTION_NAME.to_owned(), cbor_data)),
            )?;
        }

        Ok(memory_buffer)
    }

    ///
    /// Assembles the object.
    ///
    /// # Panics
    /// If bytecode is `None`.
    ///
    pub fn assemble(
        &self,
        all_objects: &[&Self],
        cbor_data: Option<Vec<(String, semver::Version)>>,
    ) -> anyhow::Result<inkwell::memory_buffer::MemoryBuffer> {
        let memory_buffer = self.to_memory_buffer(cbor_data.clone())?;

        let mut memory_buffers = Vec::with_capacity(1 + self.dependencies.inner.len());
        memory_buffers.push((self.identifier.to_owned(), memory_buffer));

        memory_buffers.extend(self.dependencies.inner.iter().map(|dependency| {
            let original_dependency_identifier = dependency.to_owned();
            let dependency = all_objects
                .iter()
                .find(|object| object.identifier.as_str() == dependency.as_str())
                .expect("Dependency not found");
            let dependency_bytecode = dependency.bytecode.as_deref().expect("Bytecode is not set");
            let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                dependency_bytecode,
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
        era_compiler_llvm_context::evm_assemble(
            bytecode_buffers.as_slice(),
            bytecode_ids.as_slice(),
            self.code_segment,
        )
    }

    ///
    /// Links the object with its linker symbols.
    ///
    /// # Panics
    /// If bytecode is `None`.
    ///
    pub fn link(
        &mut self,
        linker_symbols: &BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>,
    ) -> anyhow::Result<()> {
        let bytecode = self.bytecode.as_deref().expect("Bytecode is not set");

        let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
            bytecode,
            self.identifier.as_str(),
            false,
        );

        let linked_object = era_compiler_llvm_context::evm_link(memory_buffer, linker_symbols)?;

        let unlinked_symbols = self
            .unlinked_symbols
            .as_ref()
            .map(|unlinked_symbols| {
                unlinked_symbols
                .keys()
                .map(|symbol| {
                    (symbol.to_owned(), [0u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS])
                })
                .collect::<BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>>()
            })
            .unwrap_or_default();
        let linked_object_with_placeholders =
            era_compiler_llvm_context::evm_link(linked_object, &unlinked_symbols)?;

        self.bytecode = Some(linked_object_with_placeholders.as_slice().to_owned());
        Ok(())
    }

    ///
    /// Whether the object requires assebmling with its dependencies.
    ///
    pub fn requires_assembling(&self) -> bool {
        !self.is_assembled
    }
}
