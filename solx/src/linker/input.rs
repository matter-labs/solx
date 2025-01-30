//!
//! The linker input.
//!

use std::collections::BTreeMap;

///
/// The linker input.
///
#[derive(Debug, serde::Deserialize)]
pub struct Input {
    /// Bytecode files as a mapping from paths to content.
    pub bytecodes: BTreeMap<String, String>,
    /// Library linking specifiers in format `<path>:<library>=<address>`.
    pub libraries: Vec<String>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(bytecodes: BTreeMap<String, String>, libraries: Vec<String>) -> Self {
        Self {
            bytecodes,
            libraries,
        }
    }
}
