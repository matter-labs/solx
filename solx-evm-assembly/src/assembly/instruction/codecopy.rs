//!
//! Translates the CODECOPY use cases.
//!

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
    let pointer = era_compiler_llvm_context::Pointer::new_with_offset(
        context,
        era_compiler_llvm_context::EVMAddressSpace::Heap,
        context.field_type(),
        destination,
        "codecopy_bytes_destination_pointer",
    )?;

    context.build_call_metadata(
        context.intrinsics().codecopybytes,
        &[
            pointer.as_basic_value_enum().into(),
            context
                .llvm()
                .metadata_node(&[context.llvm().metadata_string(source).into()])
                .into(),
        ],
        "codecopy_bytes",
    )?;
    Ok(())
}
