//!
//! Translates the CODECOPY use cases.
//!

use era_compiler_llvm_context::IContext;

///
/// Translates the contract hash copying.
///
/// TODO: support with dependencies
///
pub fn dependency<'ctx>(
    context: &mut era_compiler_llvm_context::EVMContext<'ctx>,
    offset: inkwell::values::IntValue<'ctx>,
    value: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<()> {
    let offset = context.builder().build_int_add(
        offset,
        context.field_const(
            (era_compiler_common::BYTE_LENGTH_X32 + era_compiler_common::BYTE_LENGTH_FIELD) as u64,
        ),
        "datacopy_dependency_offset",
    )?;

    era_compiler_llvm_context::evm_memory::store(context, offset, value)?;

    Ok(())
}

///
/// Translates the static data copying.
///
pub fn static_data<'ctx>(
    context: &mut era_compiler_llvm_context::EVMContext<'ctx>,
    destination: inkwell::values::IntValue<'ctx>,
    source: &str,
) -> anyhow::Result<()> {
    let mut offset = 0;
    for (index, chunk) in source
        .chars()
        .collect::<Vec<char>>()
        .chunks(era_compiler_common::BYTE_LENGTH_FIELD * 2)
        .enumerate()
    {
        let mut value_string = chunk.iter().collect::<String>();
        value_string.push_str(
            "0".repeat((era_compiler_common::BYTE_LENGTH_FIELD * 2) - chunk.len())
                .as_str(),
        );

        let datacopy_destination = context.builder().build_int_add(
            destination,
            context.field_const(offset as u64),
            format!("datacopy_destination_index_{index}").as_str(),
        )?;
        let datacopy_value = context.field_const_str_hex(value_string.as_str());
        era_compiler_llvm_context::evm_memory::store(
            context,
            datacopy_destination,
            datacopy_value,
        )?;
        offset += chunk.len() / 2;
    }

    Ok(())
}
