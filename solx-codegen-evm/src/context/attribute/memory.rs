//!
//! The LLVM memory attribute.
//!

///
/// The LLVM memory attribute.
///
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Memory {
    /// The corresponding value.
    None = 0,
    /// The corresponding value.
    Read = 1,
    /// The corresponding value.
    Write = 2,
    /// The corresponding value.
    ArgMemOnly = 3,
    /// The corresponding value.
    ArgMemReadOnly = 4,
    /// The corresponding value.
    ArgMemWriteOnly = 5,
    /// The corresponding value.
    InaccessibleMemOnly = 6,
    /// The corresponding value.
    InaccessibleMemReadOnly = 7,
    /// The corresponding value.
    InaccessibleMemWriteOnly = 8,
    /// The corresponding value.
    InaccessibleOrArgMemOnly = 9,
    /// The corresponding value.
    InaccessibleOrArgMemReadOnly = 10,
    /// The corresponding value.
    InaccessibleOrArgMemWriteOnly = 11,
}
