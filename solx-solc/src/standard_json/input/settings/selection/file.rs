//!
//! The `solc --standard-json` output file selection.
//!

use std::collections::HashSet;

use crate::standard_json::input::settings::selection::selector::Selector;

///
/// The `solc --standard-json` output file selection.
///
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct File {
    /// The per-file output selections.
    #[serde(default, rename = "", skip_serializing_if = "HashSet::is_empty")]
    pub per_file: HashSet<Selector>,
    /// The per-contract output selections.
    #[serde(default, rename = "*", skip_serializing_if = "HashSet::is_empty")]
    pub per_contract: HashSet<Selector>,
}

impl File {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(flags: Vec<Selector>) -> Self {
        let mut per_file = HashSet::new();
        let mut per_contract = HashSet::new();
        for flag in flags.into_iter() {
            match flag {
                Selector::AST => {
                    per_file.insert(Selector::AST);
                }
                flag => {
                    per_contract.insert(flag);
                }
            }
        }
        Self {
            per_file,
            per_contract,
        }
    }

    ///
    /// Extends the output selection with another one.
    ///
    pub fn extend(&mut self, other: Self) -> &mut Self {
        self.per_file.extend(other.per_file);
        self.per_contract.extend(other.per_contract);
        self
    }

    ///
    /// Returns flags that are going to be automatically added by the compiler,
    /// but were not explicitly requested by the user.
    ///
    /// Afterwards, the flags are used to prune JSON output before returning it.
    ///
    pub fn selection_to_prune(&self) -> Self {
        let required_per_file = vec![Selector::AST];
        let required_per_contract = vec![
            Selector::MethodIdentifiers,
            Selector::Metadata,
            Selector::Yul,
            Selector::EVMLA,
        ];

        let mut unset_per_file = HashSet::with_capacity(required_per_file.len());
        let mut unset_per_contract = HashSet::with_capacity(required_per_contract.len());

        for flag in required_per_file {
            if !self.per_file.contains(&flag) {
                unset_per_file.insert(flag);
            }
        }
        for flag in required_per_contract {
            if !self.per_contract.contains(&flag) {
                unset_per_contract.insert(flag);
            }
        }
        Self {
            per_file: unset_per_file,
            per_contract: unset_per_contract,
        }
    }

    ///
    /// Whether the flag is requested.
    ///
    pub fn contains(&self, flag: &Selector) -> bool {
        match flag {
            flag @ Selector::AST => self.per_file.contains(flag),
            flag => self.per_contract.contains(flag),
        }
    }

    ///
    /// Checks whether the selection is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.per_file.is_empty() && self.per_contract.is_empty()
    }
}
