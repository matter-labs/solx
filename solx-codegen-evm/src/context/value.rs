//!
//! The LLVM compile-time value with metadata.
//!

///
/// The LLVM compile-time value with metadata.
///
#[derive(Debug, Clone)]
pub struct Value<'ctx> {
    /// The actual LLVM operand.
    pub value: inkwell::values::BasicValueEnum<'ctx>,
    /// The original AST value. Used mostly for string literals.
    pub original: Option<String>,
    /// The preserved constant value, if available.
    pub constant: Option<num::BigUint>,
}

impl<'ctx> Value<'ctx> {
    /// The calldata offset argument index.
    pub const ARGUMENT_INDEX_CALLDATA_OFFSET: usize = 0;

    /// The calldata length argument index.
    pub const ARGUMENT_INDEX_CALLDATA_LENGTH: usize = 1;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(value: inkwell::values::BasicValueEnum<'ctx>) -> Self {
        Self {
            value,
            original: None,
            constant: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_with_original(
        value: inkwell::values::BasicValueEnum<'ctx>,
        original: String,
    ) -> Self {
        Self {
            value,
            original: Some(original),
            constant: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_with_constant(
        value: inkwell::values::BasicValueEnum<'ctx>,
        constant: num::BigUint,
    ) -> Self {
        Self {
            value,
            original: None,
            constant: Some(constant),
        }
    }

    ///
    /// Returns the inner LLVM value.
    ///
    pub fn to_llvm(&self) -> inkwell::values::BasicValueEnum<'ctx> {
        self.value
    }
}

impl<'ctx> From<inkwell::values::BasicValueEnum<'ctx>> for Value<'ctx> {
    fn from(value: inkwell::values::BasicValueEnum<'ctx>) -> Self {
        Self::new(value)
    }
}
