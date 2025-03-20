//!
//! Bytecode object.
//!

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
    /// Dependencies.
    pub dependencies: solx_yul::Dependencies,
    /// Whether the object is already assembled.
    pub is_assembled: bool,
    /// The binary object format.
    pub object_format: era_compiler_common::ObjectFormat,
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
        dependencies: solx_yul::Dependencies,
    ) -> Self {
        Self {
            identifier,
            contract_name,
            bytecode,
            via_ir,
            code_segment,
            dependencies,
            is_assembled: false,
            object_format: era_compiler_common::ObjectFormat::ELF,
        }
    }

    ///
    /// Whether the object requires assebmling with its dependencies.
    ///
    pub fn requires_assembling(&self) -> bool {
        !self.is_assembled && !self.dependencies.inner.is_empty()
    }

    ///
    /// Checks whether the object name matches a dot-separated dependency name.
    ///
    /// This function is only useful for Yul codegen where object names like `A_25.A_25_deployed` are found.
    /// For EVM assembly codegen, it performs a simple comparison.
    ///
    pub fn matches_dependency(&self, dependency: &str) -> bool {
        let dependency = if self.via_ir {
            dependency.split('.').next().expect("Always exists")
        } else {
            dependency
        };

        self.identifier.as_str() == dependency
    }
}
