//!
//! The `solc --standard-json` output selection.
//!

pub mod file;
pub mod selector;

use self::file::File as FileSelection;
use self::selector::Selector;

///
/// The `solc --standard-json` output selection.
///
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Selection {
    /// Only the 'all' wildcard is available for robustness reasons.
    #[serde(default, rename = "*", skip_serializing_if = "FileSelection::is_empty")]
    pub all: FileSelection,
}

impl Selection {
    ///
    /// Creates the selection with arbitrary flags.
    ///
    pub fn new(flags: Vec<Selector>) -> Self {
        Self {
            all: FileSelection::new(flags),
        }
    }

    ///
    /// Creates the selection required by EraVM compilation process.
    ///
    pub fn new_required(via_ir: bool) -> Self {
        Self::new(vec![via_ir.into(), Selector::ABI]) // TODO: investigate why ABI is required
    }

    ///
    /// Creates the selection required by Yul validation process.
    ///
    pub fn new_yul_validation() -> Self {
        Self::new(vec![Selector::EVM])
    }

    ///
    /// Extends the output selection with another one.
    ///
    pub fn extend(&mut self, other: Self) -> &mut Self {
        self.all.extend(other.all);
        self
    }

    ///
    /// Returns flags that are going to be automatically added by the compiler,
    /// but were not explicitly requested by the user.
    ///
    /// Afterwards, the flags are used to prune JSON output before returning it.
    ///
    pub fn selection_to_prune(&self) -> Self {
        Self {
            all: self.all.selection_to_prune(),
        }
    }

    ///
    /// Whether the flag is requested.
    ///
    pub fn contains(&self, flag: &Selector) -> bool {
        self.all.contains(flag)
    }
}
