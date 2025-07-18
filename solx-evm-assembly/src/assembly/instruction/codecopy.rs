//!
//! Translates the CODECOPY use cases.
//!

use inkwell::values::BasicValue;

use era_compiler_llvm_context::IContext;

///
/// Translates the contract hash copying.
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
    let source_type = context.array_type(context.byte_type(), source.len());
    let source_global = context.module().add_global(
        source_type,
        Some(era_compiler_llvm_context::EVMAddressSpace::Code.into()),
        "codecopy_bytes_global",
    );
    source_global.set_initializer(
        &context
            .llvm()
            .const_string(source.as_bytes(), false)
            .as_basic_value_enum(),
    );
    let source_pointer = era_compiler_llvm_context::Pointer::new(
        source_type,
        era_compiler_llvm_context::EVMAddressSpace::Code,
        source_global.as_pointer_value(),
    );

    let destination_pointer = era_compiler_llvm_context::Pointer::new_with_offset(
        context,
        era_compiler_llvm_context::EVMAddressSpace::Heap,
        context.field_type(),
        destination,
        "codecopy_bytes_destination_pointer",
    )?;

    context.build_memcpy(
        context.intrinsics().memory_copy_from_code,
        destination_pointer,
        source_pointer,
        context.field_const(source.len() as u64),
        "codecopy_memcpy",
    )?;
    Ok(())
}
