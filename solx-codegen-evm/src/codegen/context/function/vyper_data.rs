//!
//! The LLVM function Vyper data.
//!

use std::collections::HashMap;

///
/// The LLVM function Vyper data.
///
/// Describes some data that is only relevant to Vyper.
///
#[derive(Debug)]
pub struct VyperData {
    /// The block-local variables. They are still allocated at the beginning of the function,
    /// but their parent block must be known in order to pass the implicit arguments thereto.
    /// Is only used by the Vyper LLL IR compiler.
    label_arguments: HashMap<String, Vec<String>>,
}

impl Default for VyperData {
    fn default() -> Self {
        Self {
            label_arguments: HashMap::with_capacity(Self::LABEL_ARGUMENTS_HASHMAP_INITIAL_CAPACITY),
        }
    }
}

impl VyperData {
    /// The label arguments hashmap default capacity.
    const LABEL_ARGUMENTS_HASHMAP_INITIAL_CAPACITY: usize = 16;

    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::default()
    }

    ///
    /// Returns the list of a Vyper label arguments.
    ///
    pub fn label_arguments(&self, label_name: &str) -> Option<Vec<String>> {
        self.label_arguments.get(label_name).cloned()
    }

    ///
    /// Inserts arguments for the specified label.
    ///
    pub fn insert_label_arguments(&mut self, label_name: String, arguments: Vec<String>) {
        self.label_arguments.insert(label_name, arguments);
    }
}
