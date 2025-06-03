//!
//! The Ethereal IR function type.
//!

///
/// The Ethereal IR function type.
///
#[derive(Debug, Clone)]
pub enum Type {
    /// The initial function, combining deploy and runtime code.
    Initial,
    /// The recursive function with a specific block starting its recursive context.
    Recursive {
        /// The function name.
        name: String,
        /// The function initial block key.
        block_key: era_compiler_llvm_context::BlockKey,
        /// The size of stack input (in cells or 256-bit words).
        input_size: usize,
        /// The size of stack output (in cells or 256-bit words).
        output_size: usize,
    },
}

impl Type {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_initial() -> Self {
        Self::Initial
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_recursive(
        name: String,
        block_key: era_compiler_llvm_context::BlockKey,
        input_size: usize,
        output_size: usize,
    ) -> Self {
        Self::Recursive {
            name,
            block_key,
            input_size,
            output_size,
        }
    }
}
