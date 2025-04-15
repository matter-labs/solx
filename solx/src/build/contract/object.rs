//!
//! Bytecode object.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

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
    /// Bytecode.
    pub bytecode: Vec<u8>,
    /// Whether IR codegen is used.
    pub via_ir: bool,
    /// Code segment.
    pub code_segment: era_compiler_common::CodeSegment,
    /// The metadata bytes. Only appended to runtime code.
    pub metadata_bytes: Option<Vec<u8>>,
    /// Dependencies.
    pub dependencies: solx_yul::Dependencies,
    /// The unlinked unlinked libraries.
    pub unlinked_libraries: BTreeSet<String>,
    /// Whether the object is already assembled.
    pub is_assembled: bool,
    /// Binary object format.
    pub format: era_compiler_common::ObjectFormat,
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
        bytecode: Vec<u8>,
        via_ir: bool,
        code_segment: era_compiler_common::CodeSegment,
        metadata_bytes: Option<Vec<u8>>,
        dependencies: solx_yul::Dependencies,
        unlinked_libraries: BTreeSet<String>,
        format: era_compiler_common::ObjectFormat,
        warnings: Vec<era_compiler_llvm_context::EVMWarning>,
    ) -> Self {
        Self {
            identifier,
            contract_name,
            bytecode,
            via_ir,
            code_segment,
            metadata_bytes,
            dependencies,
            unlinked_libraries,
            is_assembled: false,
            format,
            warnings,
        }
    }

    ///
    /// Appends metadata to the object.
    ///
    pub fn to_memory_buffer(
        &self,
        cbor_data: Option<Vec<(String, semver::Version)>>,
    ) -> anyhow::Result<inkwell::memory_buffer::MemoryBuffer> {
        let mut memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
            self.bytecode.as_slice(),
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
            let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
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
        era_compiler_llvm_context::evm_assemble(
            bytecode_buffers.as_slice(),
            bytecode_ids.as_slice(),
            self.code_segment,
        )
    }

    ///
    /// Links the object with its linker symbols.
    ///
    pub fn link(
        &mut self,
        linker_symbols: &BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>,
    ) -> anyhow::Result<()> {
        let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
            self.bytecode.as_slice(),
            self.identifier.as_str(),
            false,
        );

        let (linked_object, object_format) =
            era_compiler_llvm_context::evm_link(memory_buffer, linker_symbols)?;
        self.format = object_format;

        self.bytecode = linked_object.as_slice().to_owned();
        Ok(())
    }

    ///
    /// Whether the object requires assebmling with its dependencies.
    ///
    pub fn requires_assembling(&self) -> bool {
        !self.is_assembled
    }
}
