//!
//! The `solc --standard-json` output contract EVM bytecode.
//!

pub mod link_reference;

use std::collections::BTreeMap;

use self::link_reference::LinkReference;

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
    pub link_references: Option<BTreeMap<String, BTreeMap<String, Vec<LinkReference>>>>,
    /// Immutable references placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable_references: Option<BTreeMap<String, Vec<String>>>,
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
        unlinked_symbols: Option<BTreeMap<String, Vec<u64>>>,
        immutable_references: Option<BTreeMap<String, Vec<String>>>,
    ) -> Self {
        let link_references = unlinked_symbols.map(|unlinked_symbols| {
            let mut link_references = BTreeMap::new();
            for (symbol, offsets) in unlinked_symbols.into_iter() {
                let parts = symbol.split(':').collect::<Vec<_>>();
                let path = parts[0].to_string();
                let name = parts[1].to_string();

                link_references
                    .entry(path)
                    .or_insert_with(BTreeMap::new)
                    .entry(name)
                    .or_insert(
                        offsets
                            .into_iter()
                            .map(LinkReference::new)
                            .collect::<Vec<LinkReference>>(),
                    );
            }
            link_references
        });

        Self {
            object,
            llvm_assembly,

            opcodes,
            source_map,
            link_references,
            immutable_references,
        }
    }

    ///
    /// Checks if all key fields are empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.object.is_none() && self.llvm_assembly.is_none()
    }
}
