//!
//! The Yul object.
//!

use crate::declare_wrapper;
use crate::yul::parser::dialect::era::EraDialect;
use crate::yul::parser::wrapper::Wrap;

declare_wrapper!(
    solx_yul::yul::parser::statement::object::Object<EraDialect>,
    Object
);

impl era_compiler_llvm_context::EVMWriteLLVM for Object {
    fn declare(
        &mut self,
        _context: &mut era_compiler_llvm_context::EVMContext,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn into_llvm(self, context: &mut era_compiler_llvm_context::EVMContext) -> anyhow::Result<()> {
        let mut entry = era_compiler_llvm_context::EVMEntryFunction::new(self.0.code.wrap());
        entry.declare(context)?;
        entry.into_llvm(context)?;
        Ok(())
    }
}
