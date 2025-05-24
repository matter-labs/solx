//!
//! The `solc --standard-json` output contract EVM bytecode.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

///
/// The `solc --standard-json` output contract EVM bytecode.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bytecode {
    /// Bytecode object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// Text assembly from LLVM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llvm_assembly: Option<String>,

    /// Opcodes placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opcodes: Option<String>,
    /// Source maps placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_map: Option<String>,
    /// Link references placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_references: Option<BTreeMap<String, BTreeMap<String, Vec<String>>>>,
    /// Immutable references placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable_references: Option<BTreeMap<String, Vec<String>>>,

    /// Unlinked deployable references.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub unlinked_references: BTreeSet<String>,
}

impl Bytecode {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        object: Option<String>,
        llvm_assembly: Option<String>,

        opcodes: Option<String>,
        source_map: Option<String>,
        link_references: Option<BTreeMap<String, BTreeMap<String, Vec<String>>>>,
        immutable_references: Option<BTreeMap<String, Vec<String>>>,

        unlinked_references: BTreeSet<String>,
    ) -> Self {
        Self {
            object,
            llvm_assembly,

            opcodes,
            source_map,
            link_references,
            immutable_references,

            unlinked_references,
        }
    }

    ///
    /// Checks if all key fields are empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.object.is_none()
            && self.llvm_assembly.is_none()
            && self.opcodes.is_none()
            && self.source_map.is_none()
            && self.link_references.is_none()
            && self.immutable_references.is_none()
    }
}
