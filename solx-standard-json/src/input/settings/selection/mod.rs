//!
//! The `solc --standard-json` expected output selection.
//!

pub mod selector;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use self::selector::Selector;

///
/// The `solc --standard-json` expected output selection.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Selection {
    /// The inner selection map.
    #[serde(flatten)]
    inner: BTreeMap<String, BTreeMap<String, BTreeSet<Selector>>>,
}

impl Selection {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(selectors: BTreeSet<Selector>) -> Self {
        let mut file_level = BTreeMap::new();
        let mut contract_level = BTreeMap::new();

        let mut per_file_selectors = BTreeSet::new();
        if selectors.contains(&Selector::AST) {
            per_file_selectors.insert(Selector::AST);
        }
        let mut per_contract_selectors = selectors;
        per_contract_selectors.remove(&Selector::AST);

        if !per_file_selectors.is_empty() {
            contract_level.insert("".to_owned(), per_file_selectors);
        }
        if !per_contract_selectors.is_empty() {
            contract_level.insert("*".to_owned(), per_contract_selectors);
        }
        if !contract_level.is_empty() {
            file_level.insert("*".to_owned(), contract_level);
        }
        Self { inner: file_level }
    }

    ///
    /// Checks if the output element of the specified contract is selected.
    ///
    pub fn check_selection(&self, path: &str, name: Option<&str>, selector: Selector) -> bool {
        if let Some(file) = self.inner.get("*").or(self.inner.get(path)) {
            if let (Some(any), selector @ Selector::AST) = (file.get(""), selector) {
                return any.contains(&selector);
            }
            if let Some(contract) = file.get("*").or(name.and_then(|name| file.get(name))) {
                return contract.contains(&selector);
            }
        }
        false
    }

    ///
    /// Adds the specified selector to the output selection of all contracts.
    ///
    pub fn set_selector(&mut self, selector: Selector) {
        for file in self.inner.values_mut() {
            for contract in file.values_mut() {
                contract.insert(selector);
            }
        }
    }

    ///
    /// Retains only the selectors that request data from `solc`.
    ///
    pub fn retain_solc(&mut self) {
        for file in self.inner.values_mut() {
            for contract in file.values_mut() {
                contract.retain(Selector::is_received_from_solc);
            }
        }
    }

    ///
    /// Checks if the output element is requested for at least one contract.
    ///
    pub fn is_set_for_any(&self, selector: Selector) -> bool {
        for file in self.inner.values() {
            for contract in file.values() {
                match selector {
                    Selector::MethodIdentifiers | Selector::EVMLA
                        if contract.contains(&Selector::EVM) =>
                    {
                        return true
                    }
                    Selector::BytecodeObject
                    | Selector::BytecodeLLVMAssembly
                    | Selector::BytecodeOpcodes
                    | Selector::BytecodeLinkReferences
                    | Selector::BytecodeSourceMap
                    | Selector::BytecodeFunctionDebugData
                    | Selector::BytecodeGeneratedSources
                        if contract.contains(&Selector::Bytecode)
                            || contract.contains(&Selector::EVM) =>
                    {
                        return true
                    }
                    Selector::RuntimeBytecodeObject
                    | Selector::RuntimeBytecodeLLVMAssembly
                    | Selector::RuntimeBytecodeOpcodes
                    | Selector::RuntimeBytecodeLinkReferences
                    | Selector::RuntimeBytecodeImmutableReferences
                    | Selector::RuntimeBytecodeSourceMap
                    | Selector::RuntimeBytecodeFunctionDebugData
                    | Selector::RuntimeBytecodeGeneratedSources
                        if contract.contains(&Selector::RuntimeBytecode)
                            || contract.contains(&Selector::EVM) =>
                    {
                        return true
                    }
                    selector if contract.contains(&selector) => return true,
                    _ => continue,
                }
            }
        }
        false
    }

    ///
    /// Whether the selection is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
