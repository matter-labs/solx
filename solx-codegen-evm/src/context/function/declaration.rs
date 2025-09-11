//!
//! The LLVM function declaration.
//!

///
/// The LLVM function declaration.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Declaration<'ctx> {
    /// The function type.
    pub r#type: inkwell::types::FunctionType<'ctx>,
    /// The function value.
    pub value: inkwell::values::FunctionValue<'ctx>,
}

impl<'ctx> Declaration<'ctx> {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        r#type: inkwell::types::FunctionType<'ctx>,
        value: inkwell::values::FunctionValue<'ctx>,
    ) -> Self {
        Self { r#type, value }
    }
}
