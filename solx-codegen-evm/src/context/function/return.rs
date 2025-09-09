//!
//! The LLVM IR generator function return entity.
//!

use crate::context::pointer::Pointer;
use crate::context::traits::address_space::IAddressSpace;

///
/// The LLVM IR generator function return entity.
///
#[derive(Debug, Clone, Copy)]
pub enum Return<'ctx, AS>
where
    AS: IAddressSpace
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Into<inkwell::AddressSpace>
        + std::fmt::Debug,
{
    /// The function does not return a value.
    None,
    /// The function returns a primitive value.
    Primitive {
        /// The primitive value pointer allocated at the function entry.
        pointer: Pointer<'ctx, AS>,
    },
    /// The function returns a compound value.
    /// In this case, the return pointer is allocated on the stack by the callee.
    Compound {
        /// The structure pointer allocated at the function entry.
        pointer: Pointer<'ctx, AS>,
        /// The function return type size.
        size: usize,
    },
}

impl<'ctx, AS> Return<'ctx, AS>
where
    AS: IAddressSpace
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Into<inkwell::AddressSpace>
        + std::fmt::Debug,
{
    ///
    /// A shortcut constructor.
    ///
    pub fn none() -> Self {
        Self::None
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn primitive(pointer: Pointer<'ctx, AS>) -> Self {
        Self::Primitive { pointer }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn compound(pointer: Pointer<'ctx, AS>, size: usize) -> Self {
        Self::Compound { pointer, size }
    }

    ///
    /// Returns the pointer to the function return value.
    ///
    pub fn return_pointer(&self) -> Option<Pointer<'ctx, AS>> {
        match self {
            Return::None => None,
            Return::Primitive { pointer } => Some(pointer.to_owned()),
            Return::Compound { pointer, .. } => Some(pointer.to_owned()),
        }
    }

    ///
    /// Returns the return data size in bytes, based on the default stack alignment.
    ///
    pub fn return_data_size(&self) -> usize {
        era_compiler_common::BYTE_LENGTH_FIELD
            * match self {
                Self::None => 0,
                Self::Primitive { .. } => 1,
                Self::Compound { size, .. } => *size,
            }
    }
}
