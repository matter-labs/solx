//!
//! The missing libraries.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

///
/// The missing libraries.
///
pub struct MissingLibraries {
    /// The libraries.
    pub contract_libraries: BTreeMap<String, BTreeSet<String>>,
}

impl MissingLibraries {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(contract_libraries: BTreeMap<String, BTreeSet<String>>) -> Self {
        Self { contract_libraries }
    }
}
