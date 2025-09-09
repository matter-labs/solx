//!
//! The LLVM IR generator loop.
//!

///
/// The LLVM IR generator loop.
///
#[derive(Debug, Clone)]
pub struct Loop<'ctx> {
    /// The loop current block.
    pub body_block: inkwell::basic_block::BasicBlock<'ctx>,
    /// The increment block before the body.
    pub continue_block: inkwell::basic_block::BasicBlock<'ctx>,
    /// The join block after the body.
    pub join_block: inkwell::basic_block::BasicBlock<'ctx>,
}

impl<'ctx> Loop<'ctx> {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        body_block: inkwell::basic_block::BasicBlock<'ctx>,
        continue_block: inkwell::basic_block::BasicBlock<'ctx>,
        join_block: inkwell::basic_block::BasicBlock<'ctx>,
    ) -> Self {
        Self {
            body_block,
            continue_block,
            join_block,
        }
    }
}
