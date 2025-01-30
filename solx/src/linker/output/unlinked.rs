//!
//! The information on an unlinked object.
//!

///
/// The information on an unlinked object.
///
#[derive(Debug, Default, serde::Serialize)]
pub struct Unlinked {
    /// Unlinked linker symbols (Solidity libraries).
    pub linker_symbols: Vec<String>,
    /// Unlinked factory dependencies (CREATE/CREATE2 dependencies).
    pub factory_dependencies: Vec<String>,
}

impl Unlinked {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(linker_symbols: Vec<String>, factory_dependencies: Vec<String>) -> Self {
        Self {
            linker_symbols,
            factory_dependencies,
        }
    }
}
