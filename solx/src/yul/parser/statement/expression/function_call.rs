//!
//! The function call subexpression.
//!

use era_compiler_llvm_context::IContext;
use solx_yul::yul::parser::statement::expression::function_call::name::Name;

use crate::declare_wrapper;
use crate::yul::parser::wrapper::Wrap;

declare_wrapper!(
    solx_yul::yul::parser::statement::expression::function_call::FunctionCall,
    FunctionCall
);

impl FunctionCall {
    ///
    /// Converts the function call into an LLVM value.
    ///
    pub fn into_llvm<'ctx>(
        mut self,
        context: &mut era_compiler_llvm_context::EVMContext<'ctx>,
    ) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>> {
        let location = self.0.location;

        match self.0.name {
            Name::UserDefined(name) => {
                let mut values = Vec::with_capacity(self.0.arguments.len());
                for argument in self.0.arguments.into_iter().rev() {
                    let value = argument
                        .wrap()
                        .into_llvm(context)?
                        .expect("Always exists")
                        .value;
                    values.push(value);
                }
                values.reverse();
                let function = context.get_function(name.as_str()).ok_or_else(|| {
                    anyhow::anyhow!("{} Undeclared function `{}`", location, name)
                })?;

                let expected_arguments_count =
                    function.borrow().declaration().value.count_params() as usize;
                if expected_arguments_count != values.len() {
                    anyhow::bail!(
                        "{location} Function `{name}` expected {expected_arguments_count} arguments, found {}",
                        values.len()
                    );
                }

                let return_value = context.build_call(
                    function.borrow().declaration(),
                    values.as_slice(),
                    format!("{name}_call").as_str(),
                )?;

                Ok(return_value)
            }

            Name::Add => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_arithmetic::addition(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Sub => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_arithmetic::subtraction(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Mul => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_arithmetic::multiplication(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Div => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_arithmetic::division(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Mod => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_arithmetic::remainder(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Sdiv => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_arithmetic::division_signed(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Smod => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_arithmetic::remainder_signed(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }

            Name::Lt => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::ULT,
                )
                .map(Some)
            }
            Name::Gt => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::UGT,
                )
                .map(Some)
            }
            Name::Eq => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::EQ,
                )
                .map(Some)
            }
            Name::IsZero => {
                let arguments = self.pop_arguments_llvm::<1>(context)?;
                era_compiler_llvm_context::evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    context.field_const(0),
                    inkwell::IntPredicate::EQ,
                )
                .map(Some)
            }
            Name::Slt => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SLT,
                )
                .map(Some)
            }
            Name::Sgt => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SGT,
                )
                .map(Some)
            }

            Name::Or => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_bitwise::or(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Xor => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_bitwise::xor(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Not => {
                let arguments = self.pop_arguments_llvm::<1>(context)?;
                era_compiler_llvm_context::evm_bitwise::xor(
                    context,
                    arguments[0].into_int_value(),
                    context.field_type().const_all_ones(),
                )
                .map(Some)
            }
            Name::And => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_bitwise::and(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Shl => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_bitwise::shift_left(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Shr => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_bitwise::shift_right(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Sar => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_bitwise::shift_right_arithmetic(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Byte => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_bitwise::byte(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Pop => {
                let _arguments = self.pop_arguments_llvm::<1>(context)?;
                Ok(None)
            }

            Name::AddMod => {
                let arguments = self.pop_arguments_llvm::<3>(context)?;
                era_compiler_llvm_context::evm_math::add_mod(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(Some)
            }
            Name::MulMod => {
                let arguments = self.pop_arguments_llvm::<3>(context)?;
                era_compiler_llvm_context::evm_math::mul_mod(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(Some)
            }
            Name::Exp => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_math::exponent(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::SignExtend => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_math::sign_extend(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Keccak256 => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_math::keccak256(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }

            Name::MLoad => {
                let arguments = self.pop_arguments_llvm::<1>(context)?;
                era_compiler_llvm_context::evm_memory::load(context, arguments[0].into_int_value())
                    .map(Some)
            }
            Name::MStore => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_memory::store(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            Name::MStore8 => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_memory::store_byte(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            Name::MCopy => {
                let arguments = self.pop_arguments_llvm::<3>(context)?;
                let destination = era_compiler_llvm_context::Pointer::new_with_offset(
                    context,
                    era_compiler_llvm_context::EVMAddressSpace::Heap,
                    context.byte_type(),
                    arguments[0].into_int_value(),
                    "mcopy_destination",
                )?;
                let source = era_compiler_llvm_context::Pointer::new_with_offset(
                    context,
                    era_compiler_llvm_context::EVMAddressSpace::Heap,
                    context.byte_type(),
                    arguments[1].into_int_value(),
                    "mcopy_source",
                )?;

                context.build_memcpy(
                    context.intrinsics().memory_move_heap,
                    destination,
                    source,
                    arguments[2].into_int_value(),
                    "mcopy_size",
                )?;
                Ok(None)
            }

            Name::SLoad => {
                let arguments = self.pop_arguments_llvm::<1>(context)?;
                era_compiler_llvm_context::evm_storage::load(context, arguments[0].into_int_value())
                    .map(Some)
            }
            Name::SStore => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_storage::store(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            Name::TLoad => {
                let arguments = self.pop_arguments_llvm::<1>(context)?;
                era_compiler_llvm_context::evm_storage::transient_load(
                    context,
                    arguments[0].into_int_value(),
                )
                .map(Some)
            }
            Name::TStore => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_storage::transient_store(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            Name::LoadImmutable => {
                let mut arguments = self.pop_arguments::<1>(context)?;
                let id = arguments[0].original.take().ok_or_else(|| {
                    anyhow::anyhow!("{location} `loadimmutable` literal is missing")
                })?;
                era_compiler_llvm_context::evm_immutable::load(context, id.as_str()).map(Some)
            }
            Name::SetImmutable => {
                let mut arguments = self.pop_arguments::<3>(context)?;

                let id = arguments[1].original.take().ok_or_else(|| {
                    anyhow::anyhow!("{location} `setimmutable` literal is missing")
                })?;

                let base_offset = arguments[0].to_llvm().into_int_value();
                let value = arguments[2].to_llvm().into_int_value();
                era_compiler_llvm_context::evm_immutable::store(
                    context,
                    id.as_str(),
                    base_offset,
                    value,
                )
                .map(|_| None)
            }

            Name::CallDataLoad => {
                let arguments = self.pop_arguments_llvm::<1>(context)?;
                era_compiler_llvm_context::evm_calldata::load(
                    context,
                    arguments[0].into_int_value(),
                )
                .map(Some)
            }
            Name::CallDataSize => era_compiler_llvm_context::evm_calldata::size(context).map(Some),
            Name::CallDataCopy => {
                let arguments = self.pop_arguments_llvm::<3>(context)?;
                era_compiler_llvm_context::evm_calldata::copy(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )?;
                Ok(None)
            }

            Name::ReturnDataSize => {
                era_compiler_llvm_context::evm_return_data::size(context).map(Some)
            }
            Name::ReturnDataCopy => {
                let arguments = self.pop_arguments_llvm::<3>(context)?;
                era_compiler_llvm_context::evm_return_data::copy(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )?;
                Ok(None)
            }

            Name::CodeSize => era_compiler_llvm_context::evm_code::size(context).map(Some),
            Name::CodeCopy => {
                let arguments = self.pop_arguments_llvm::<3>(context)?;
                era_compiler_llvm_context::evm_code::copy(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )?;
                Ok(None)
            }
            Name::ExtCodeSize => {
                let arguments = self.pop_arguments_llvm::<1>(context)?;
                era_compiler_llvm_context::evm_code::ext_size(
                    context,
                    arguments[0].into_int_value(),
                )
                .map(Some)
            }
            Name::ExtCodeCopy => {
                let arguments = self.pop_arguments_llvm::<4>(context)?;
                era_compiler_llvm_context::evm_code::ext_copy(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                    arguments[3].into_int_value(),
                )
                .map(|_| None)
            }
            Name::ExtCodeHash => {
                let arguments = self.pop_arguments_llvm::<1>(context)?;
                era_compiler_llvm_context::evm_code::ext_hash(
                    context,
                    arguments[0].into_int_value(),
                )
                .map(Some)
            }

            Name::Return => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_return::r#return(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            Name::Revert => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_return::revert(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            Name::Stop => era_compiler_llvm_context::evm_return::stop(context).map(|_| None),
            Name::Invalid => era_compiler_llvm_context::evm_return::invalid(context).map(|_| None),

            Name::Log0 => {
                let arguments = self.pop_arguments_llvm::<2>(context)?;
                era_compiler_llvm_context::evm_event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    vec![],
                )?;
                Ok(None)
            }
            Name::Log1 => {
                let arguments = self.pop_arguments_llvm::<3>(context)?;
                era_compiler_llvm_context::evm_event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2..]
                        .iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )?;
                Ok(None)
            }
            Name::Log2 => {
                let arguments = self.pop_arguments_llvm::<4>(context)?;
                era_compiler_llvm_context::evm_event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2..]
                        .iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )?;
                Ok(None)
            }
            Name::Log3 => {
                let arguments = self.pop_arguments_llvm::<5>(context)?;
                era_compiler_llvm_context::evm_event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2..]
                        .iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )?;
                Ok(None)
            }
            Name::Log4 => {
                let arguments = self.pop_arguments_llvm::<6>(context)?;
                era_compiler_llvm_context::evm_event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2..]
                        .iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )?;
                Ok(None)
            }

            Name::Call => {
                let arguments = self.pop_arguments::<7>(context)?;

                let gas = arguments[0].value.into_int_value();
                let address = arguments[1].value.into_int_value();
                let value = arguments[2].value.into_int_value();
                let input_offset = arguments[3].value.into_int_value();
                let input_size = arguments[4].value.into_int_value();
                let output_offset = arguments[5].value.into_int_value();
                let output_size = arguments[6].value.into_int_value();

                Ok(Some(era_compiler_llvm_context::evm_call::call(
                    context,
                    gas,
                    address,
                    value,
                    input_offset,
                    input_size,
                    output_offset,
                    output_size,
                )?))
            }
            Name::StaticCall => {
                let arguments = self.pop_arguments::<6>(context)?;

                let gas = arguments[0].value.into_int_value();
                let address = arguments[1].value.into_int_value();
                let input_offset = arguments[2].value.into_int_value();
                let input_size = arguments[3].value.into_int_value();
                let output_offset = arguments[4].value.into_int_value();
                let output_size = arguments[5].value.into_int_value();

                Ok(Some(era_compiler_llvm_context::evm_call::static_call(
                    context,
                    gas,
                    address,
                    input_offset,
                    input_size,
                    output_offset,
                    output_size,
                )?))
            }
            Name::DelegateCall => {
                let arguments = self.pop_arguments::<6>(context)?;

                let gas = arguments[0].value.into_int_value();
                let address = arguments[1].value.into_int_value();
                let input_offset = arguments[2].value.into_int_value();
                let input_size = arguments[3].value.into_int_value();
                let output_offset = arguments[4].value.into_int_value();
                let output_size = arguments[5].value.into_int_value();

                Ok(Some(era_compiler_llvm_context::evm_call::delegate_call(
                    context,
                    gas,
                    address,
                    input_offset,
                    input_size,
                    output_offset,
                    output_size,
                )?))
            }

            Name::Create => {
                let arguments = self.pop_arguments_llvm::<3>(context)?;

                let value = arguments[0].into_int_value();
                let input_offset = arguments[1].into_int_value();
                let input_length = arguments[2].into_int_value();

                era_compiler_llvm_context::evm_create::create(
                    context,
                    value,
                    input_offset,
                    input_length,
                )
                .map(Some)
            }
            Name::Create2 => {
                let arguments = self.pop_arguments_llvm::<4>(context)?;

                let value = arguments[0].into_int_value();
                let input_offset = arguments[1].into_int_value();
                let input_length = arguments[2].into_int_value();
                let salt = arguments[3].into_int_value();

                era_compiler_llvm_context::evm_create::create2(
                    context,
                    value,
                    input_offset,
                    input_length,
                    salt,
                )
                .map(Some)
            }
            Name::DataOffset => {
                let mut arguments = self.pop_arguments::<1>(context)?;
                let object_name = arguments[0].original.take().ok_or_else(|| {
                    anyhow::anyhow!("{} `dataoffset` literal is missing", location)
                })?;
                let object_name = object_name.split('.').last().expect("Always exists");
                era_compiler_llvm_context::evm_code::data_offset(context, object_name).map(Some)
            }
            Name::DataSize => {
                let mut arguments = self.pop_arguments::<1>(context)?;
                let object_name = arguments[0]
                    .original
                    .take()
                    .ok_or_else(|| anyhow::anyhow!("{} `datasize` literal is missing", location))?;
                let object_name = object_name.split('.').last().expect("Always exists");
                era_compiler_llvm_context::evm_code::data_size(context, object_name).map(Some)
            }
            Name::DataCopy => {
                let arguments = self.pop_arguments_llvm::<3>(context)?;
                era_compiler_llvm_context::evm_code::copy(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )?;
                Ok(None)
            }

            Name::LinkerSymbol => {
                let mut arguments = self.pop_arguments::<1>(context)?;
                let path = arguments[0].original.take().ok_or_else(|| {
                    anyhow::anyhow!("{location} Linker symbol literal is missing")
                })?;
                era_compiler_llvm_context::evm_call::linker_symbol(context, path.as_str()).map(Some)
            }
            Name::MemoryGuard => {
                let arguments = self.pop_arguments_llvm::<1>(context)?;
                Ok(Some(arguments[0]))
            }

            Name::Address => context.build_call(context.intrinsics().address, &[], "address"),
            Name::Caller => context.build_call(context.intrinsics().caller, &[], "caller"),

            Name::CallValue => {
                era_compiler_llvm_context::evm_ether_gas::callvalue(context).map(Some)
            }
            Name::Gas => era_compiler_llvm_context::evm_ether_gas::gas(context).map(Some),
            Name::Balance => {
                let arguments = self.pop_arguments_llvm::<1>(context)?;

                let address = arguments[0].into_int_value();
                era_compiler_llvm_context::evm_ether_gas::balance(context, address).map(Some)
            }
            Name::SelfBalance => {
                era_compiler_llvm_context::evm_ether_gas::self_balance(context).map(Some)
            }

            Name::GasLimit => {
                era_compiler_llvm_context::evm_contract_context::gas_limit(context).map(Some)
            }
            Name::GasPrice => {
                era_compiler_llvm_context::evm_contract_context::gas_price(context).map(Some)
            }
            Name::Origin => {
                era_compiler_llvm_context::evm_contract_context::origin(context).map(Some)
            }
            Name::ChainId => {
                era_compiler_llvm_context::evm_contract_context::chain_id(context).map(Some)
            }
            Name::Timestamp => {
                era_compiler_llvm_context::evm_contract_context::block_timestamp(context).map(Some)
            }
            Name::Number => {
                era_compiler_llvm_context::evm_contract_context::block_number(context).map(Some)
            }
            Name::BlockHash => {
                let arguments = self.pop_arguments_llvm::<1>(context)?;
                let index = arguments[0].into_int_value();

                era_compiler_llvm_context::evm_contract_context::block_hash(context, index)
                    .map(Some)
            }
            Name::Difficulty | Name::Prevrandao => {
                era_compiler_llvm_context::evm_contract_context::difficulty(context).map(Some)
            }
            Name::CoinBase => {
                era_compiler_llvm_context::evm_contract_context::coinbase(context).map(Some)
            }
            Name::BaseFee => {
                era_compiler_llvm_context::evm_contract_context::basefee(context).map(Some)
            }
            Name::MSize => {
                era_compiler_llvm_context::evm_contract_context::msize(context).map(Some)
            }

            Name::CallCode => {
                let _arguments = self.pop_arguments_llvm::<7>(context)?;
                anyhow::bail!("{location} The `CALLCODE` instruction is not supported")
            }
            Name::Pc => anyhow::bail!("{location} The `PC` instruction is not supported"),
            Name::SelfDestruct => {
                let _arguments = self.pop_arguments_llvm::<1>(context)?;
                anyhow::bail!("{location} The `SELFDESTRUCT` instruction is not supported")
            }

            _ => Ok(None),
        }
    }

    ///
    /// Pops the specified number of arguments, converted into their LLVM values.
    ///
    fn pop_arguments_llvm<'ctx, const N: usize>(
        &mut self,
        context: &mut era_compiler_llvm_context::EVMContext<'ctx>,
    ) -> anyhow::Result<[inkwell::values::BasicValueEnum<'ctx>; N]> {
        let mut arguments = Vec::with_capacity(N);
        for expression in self.0.arguments.drain(0..N).rev() {
            arguments.push(
                expression
                    .wrap()
                    .into_llvm(context)?
                    .expect("Always exists")
                    .value,
            );
        }
        arguments.reverse();

        Ok(arguments.try_into().expect("Always successful"))
    }

    ///
    /// Pops the specified number of arguments.
    ///
    fn pop_arguments<'ctx, const N: usize>(
        &mut self,
        context: &mut era_compiler_llvm_context::EVMContext<'ctx>,
    ) -> anyhow::Result<[era_compiler_llvm_context::Value<'ctx>; N]> {
        let mut arguments = Vec::with_capacity(N);
        for expression in self.0.arguments.drain(0..N).rev() {
            arguments.push(
                expression
                    .wrap()
                    .into_llvm(context)?
                    .expect("Always exists"),
            );
        }
        arguments.reverse();

        Ok(arguments.try_into().expect("Always successful"))
    }
}
