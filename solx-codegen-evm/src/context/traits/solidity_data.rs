//!
//! The LLVM IR Solidity data trait.
//!

use std::collections::BTreeSet;

///
/// The LLVM IR Solidity data trait.
///
pub trait ISolidityData {
    ///
    /// Returns all runtime code offsets for the specified `id`.
    ///
    fn offsets(&mut self, id: &str) -> Option<BTreeSet<u64>>;
}
