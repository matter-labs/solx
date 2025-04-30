//!
//! The `solc --standard-json` output contract EVM bytecode.
//!

use std::collections::BTreeSet;

///
/// The `solc --standard-json` output contract EVM bytecode.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bytecode {
    /// Bytecode object.
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub object: Option<String>,
    /// Text assembly from LLVM.
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub llvm_assembly: Option<String>,

    /// Unlinked deployable references.
    #[serde(
        default,
        skip_serializing_if = "BTreeSet::is_empty",
        skip_deserializing
    )]
    pub unlinked_references: BTreeSet<String>,
    /// Binary object format.
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub format: Option<era_compiler_common::ObjectFormat>,
}

impl Bytecode {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        object: Option<String>,
        llvm_assembly: Option<String>,
        unlinked_references: BTreeSet<String>,
        format: era_compiler_common::ObjectFormat,
    ) -> Self {
        Self {
            object,
            llvm_assembly,
            unlinked_references,
            format: Some(format),
        }
    }

    ///
    /// Checks if all key fields are empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.object.is_none() && self.llvm_assembly.is_none()
    }
}
