//!
//! The linked output bytecode object.
//!

///
/// The linked output bytecode object.
///
#[derive(Debug, Default, serde::Serialize)]
pub struct Linked {
    /// Contract bytecode.
    pub bytecode: String,
    /// Contract bytecode hash.
    pub hash: String,
    /// Linked linker symbols (Solidity libraries).
    pub linker_symbols: Vec<String>,
    /// Linked factory dependencies (CREATE/CREATE2 dependencies).
    pub factory_dependencies: Vec<String>,
}

impl Linked {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        bytecode: String,
        hash: String,
        linker_symbols: Vec<String>,
        factory_dependencies: Vec<String>,
    ) -> Self {
        Self {
            bytecode,
            hash,
            linker_symbols,
            factory_dependencies,
        }
    }
}
