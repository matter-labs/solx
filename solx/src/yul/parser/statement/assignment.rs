//!
//! The assignment expression statement.
//!

use era_compiler_llvm_context::IContext;
use inkwell::types::BasicType;

use crate::declare_wrapper;
use crate::yul::parser::wrapper::Wrap;

declare_wrapper!(
    solx_yul::yul::parser::statement::assignment::Assignment,
    Assignment
);

impl era_compiler_llvm_context::EVMWriteLLVM for Assignment {
    fn into_llvm(
        mut self,
        context: &mut era_compiler_llvm_context::EVMContext,
    ) -> anyhow::Result<()> {
        let value = match self.0.initializer.wrap().into_llvm(context)? {
            Some(value) => value,
            None => return Ok(()),
        };

        if self.0.bindings.len() == 1 {
            let identifier = self.0.bindings.remove(0);
            let pointer = context
                .current_function()
                .borrow()
                .get_stack_pointer(identifier.inner.as_str())
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "{} Assignment to an undeclared variable `{}`",
                        identifier.location,
                        identifier.inner,
                    )
                })?;
            context.build_store(pointer, value.to_llvm())?;
            return Ok(());
        }

        let llvm_type = value.to_llvm().into_struct_value().get_type();
        let tuple_pointer = context.build_alloca(llvm_type, "assignment_pointer")?;
        context.build_store(tuple_pointer, value.to_llvm())?;

        for (index, binding) in self.0.bindings.into_iter().enumerate() {
            let field_pointer = context.build_gep(
                tuple_pointer,
                &[
                    context.field_const(0),
                    context
                        .integer_type(era_compiler_common::BIT_LENGTH_X32)
                        .const_int(index as u64, false),
                ],
                context.field_type().as_basic_type_enum(),
                format!("assignment_binding_{index}_gep_pointer").as_str(),
            )?;

            let binding_pointer = context
                .current_function()
                .borrow()
                .get_stack_pointer(binding.inner.as_str())
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "{} Assignment to an undeclared variable `{}`",
                        binding.location,
                        binding.inner,
                    )
                })?;
            let value = context.build_load(
                field_pointer,
                format!("assignment_binding_{index}_value").as_str(),
            )?;
            context.build_store(binding_pointer, value)?;
        }

        Ok(())
    }
}
