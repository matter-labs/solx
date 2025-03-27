//!
//! Process for compiling a single compilation unit.
//!
//! The EVM output data.
//!

use crate::build::contract::Contract as EVMContractBuild;

///
/// The EVM output data.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Output {
    /// The contract build.
    pub build: EVMContractBuild,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(build: EVMContractBuild) -> Self {
        Self { build }
    }
}
