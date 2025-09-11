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

impl solx_codegen_evm::WriteLLVM for Object {
    fn declare(&mut self, _context: &mut solx_codegen_evm::Context) -> anyhow::Result<()> {
        Ok(())
    }

    fn into_llvm(self, context: &mut solx_codegen_evm::Context) -> anyhow::Result<()> {
        let mut entry = solx_codegen_evm::EntryFunction::new(self.0.code.wrap());
        entry.declare(context)?;
        entry.into_llvm(context)?;
        Ok(())
    }
}
