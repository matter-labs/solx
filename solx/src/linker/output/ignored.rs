//!
//! The ignored output bytecode object.
//!

///
/// The ignored output bytecode object.
///
#[derive(Debug, Default, serde::Serialize)]
pub struct Ignored {
    /// Contract bytecode.
    pub bytecode: String,
    /// Contract bytecode hash.
    pub hash: String,
}

impl Ignored {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(bytecode: String, hash: String) -> Self {
        Self { bytecode, hash }
    }
}
