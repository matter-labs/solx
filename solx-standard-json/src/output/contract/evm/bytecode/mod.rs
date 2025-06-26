//!
//! The `solc --standard-json` output contract EVM bytecode.
//!

pub mod link_reference;

use std::collections::BTreeMap;

use self::link_reference::LinkReference;

///
/// The `solc --standard-json` output contract EVM bytecode.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bytecode {
    /// Bytecode object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// Text assembly from LLVM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llvm_assembly: Option<String>,
    /// Link references placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_references: Option<BTreeMap<String, BTreeMap<String, Vec<LinkReference>>>>,

    /// Opcodes placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opcodes: Option<String>,
    /// Source maps placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_map: Option<String>,
    /// Generated sources placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_sources: Option<Vec<serde_json::Value>>,
    /// Function debug data placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_debug_data: Option<BTreeMap<String, serde_json::Value>>,
    /// Immutable generated_sources placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable_references: Option<serde_json::Value>,
}

impl Bytecode {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        object: Option<String>,
        llvm_assembly: Option<String>,
        unlinked_symbols: Option<BTreeMap<String, Vec<u64>>>,

        opcodes: Option<String>,
        source_map: Option<String>,
        generated_sources: Option<Vec<serde_json::Value>>,
        function_debug_data: Option<BTreeMap<String, serde_json::Value>>,
        immutable_references: Option<serde_json::Value>,
    ) -> Self {
        let link_references = unlinked_symbols.map(|unlinked_symbols| {
            let mut link_references = BTreeMap::new();
            for (symbol, offsets) in unlinked_symbols.into_iter() {
                let parts = symbol.split(':').collect::<Vec<_>>();
                let path = parts[0].to_owned();
                let name = parts[1].to_owned();

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
            link_references,

            opcodes,
            source_map,
            generated_sources,
            function_debug_data,
            immutable_references,
        }
    }

    ///
    /// Checks if all key fields are empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.object.is_none()
            && self.llvm_assembly.is_none()
            && self.link_references.is_none()
            && self.opcodes.is_none()
            && self.source_map.is_none()
            && self.generated_sources.is_none()
            && self.function_debug_data.is_none()
            && self.immutable_references.is_none()
    }
}
