//!
//! The Yul code.
//!

use crate::declare_wrapper;
use crate::yul::parser::dialect::era::EraDialect;
use crate::yul::parser::wrapper::Wrap;

declare_wrapper!(
    solx_yul::yul::parser::statement::code::Code<EraDialect>,
    Code
);

impl era_compiler_llvm_context::EVMWriteLLVM for Code {
    fn into_llvm(self, context: &mut era_compiler_llvm_context::EVMContext) -> anyhow::Result<()> {
        self.0.block.wrap().into_llvm(context)?;

        Ok(())
    }
}
