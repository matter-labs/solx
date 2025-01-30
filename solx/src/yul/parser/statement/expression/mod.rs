//!
//! The expression statement.
//!

pub mod function_call;
pub mod literal;

use crate::declare_wrapper;
use era_compiler_llvm_context::IContext;

use crate::yul::parser::wrapper::Wrap;

declare_wrapper!(
    solx_yul::yul::parser::statement::expression::Expression,
    Expression
);

impl Expression {
    ///
    /// Converts the expression into an LLVM value.
    ///
    pub fn into_llvm<'ctx>(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<'ctx>,
    ) -> anyhow::Result<Option<era_compiler_llvm_context::Value<'ctx>>> {
        match self.0 {
            solx_yul::yul::parser::statement::expression::Expression::Literal(literal) => literal
                .clone()
                .wrap()
                .into_llvm(context)
                .map_err(|error| {
                    anyhow::anyhow!(
                        "{} Invalid literal `{}`: {}",
                        literal.location,
                        literal.inner.to_string(),
                        error
                    )
                })
                .map(Some),
            solx_yul::yul::parser::statement::expression::Expression::Identifier(identifier) => {
                let pointer = context
                    .current_function()
                    .borrow()
                    .get_stack_pointer(identifier.inner.as_str())
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "{} Undeclared variable `{}`",
                            identifier.location,
                            identifier.inner,
                        )
                    })?;

                let value = context.build_load(pointer, identifier.inner.as_str())?;
                Ok(Some(value.into()))
            }
            solx_yul::yul::parser::statement::expression::Expression::FunctionCall(call) => {
                Ok(call
                    .wrap()
                    .into_llvm(context)?
                    .map(era_compiler_llvm_context::Value::new))
            }
        }
    }
}
