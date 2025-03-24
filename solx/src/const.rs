//!
//! Solidity compiler constants.
//!

/// The `keccak256` scratch space offset.
pub const OFFSET_SCRATCH_SPACE: usize = 0;

/// The memory pointer offset.
pub const OFFSET_MEMORY_POINTER: usize = 2 * era_compiler_common::BYTE_LENGTH_FIELD;

/// The empty slot offset.
pub const OFFSET_EMPTY_SLOT: usize = 3 * era_compiler_common::BYTE_LENGTH_FIELD;

/// The non-reserved memory offset.
pub const OFFSET_NON_RESERVED: usize = 4 * era_compiler_common::BYTE_LENGTH_FIELD;

///
/// The compiler version default function.
///
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}
