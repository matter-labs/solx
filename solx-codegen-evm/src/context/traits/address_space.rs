//!
//! The LLVM IR address space trait.
//!

///
/// The LLVM IR address space trait.
///
pub trait IAddressSpace {
    ///
    /// Returns the stack address space.
    ///
    fn stack() -> Self;
}
