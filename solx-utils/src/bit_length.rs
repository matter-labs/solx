//!
//! Sizes in bits.
//!

/// The `bool` type bit-length.
pub const BIT_LENGTH_BOOLEAN: usize = 1;

/// The `u8` type or byte bit-length.
pub const BIT_LENGTH_BYTE: usize = 8;

/// The x86 word type (usually `u32`) bit-length.
pub const BIT_LENGTH_X32: usize = crate::byte_length::BYTE_LENGTH_X32 * BIT_LENGTH_BYTE;

/// The x86_64 word type (usually `u64`) bit-length.
pub const BIT_LENGTH_X64: usize = crate::byte_length::BYTE_LENGTH_X64 * BIT_LENGTH_BYTE;

/// The ETH address (usually `u160`) bit-length.
pub const BIT_LENGTH_ETH_ADDRESS: usize =
    crate::byte_length::BYTE_LENGTH_ETH_ADDRESS * BIT_LENGTH_BYTE;

/// The field (usually `u256` or `i256`) bit-length.
pub const BIT_LENGTH_FIELD: usize = crate::byte_length::BYTE_LENGTH_FIELD * BIT_LENGTH_BYTE;
