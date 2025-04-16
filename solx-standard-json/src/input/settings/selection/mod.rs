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
    /// A shortcut constructor for compilation.
    ///
    pub fn new_compilation(bytecode: bool, metadata: bool, via_ir: Option<bool>) -> Self {
        let mut selectors = BTreeSet::new();
        if bytecode {
            selectors.insert(Selector::Bytecode);
            selectors.insert(Selector::RuntimeBytecode);
        }
        if metadata {
            selectors.insert(Selector::Metadata);
        }
        if let Some(via_ir) = via_ir {
            selectors.insert(via_ir.into());
        }
        Self::new(selectors)
    }

    ///
    /// Checks if the output element of the specified contract is selected.
    ///
    pub fn check_selection(&self, path: &str, name: Option<&str>, selector: Selector) -> bool {
        if let Some(file) = self.inner.get("*").or(self.inner.get(path)) {
            if let (Some(any), selector @ Selector::AST) = (file.get(""), selector) {
                return any.contains(&selector);
            }
            if let Some(name) = name {
                if let Some(contract) = file.get("*").or(file.get(name)) {
                    return contract.contains(&selector);
                }
            } else {
                return true;
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
                if contract.contains(&selector) {
                    return true;
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
