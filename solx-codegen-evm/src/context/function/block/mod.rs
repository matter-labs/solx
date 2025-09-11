//!
//! The LLVM IR generator function block.
//!

pub mod evmla_data;
pub mod key;

use self::evmla_data::EVMLAData;

///
/// The LLVM IR generator function block.
///
#[derive(Debug, Clone)]
pub struct Block<'ctx> {
    /// The inner block.
    inner: inkwell::basic_block::BasicBlock<'ctx>,
    /// The EVM legacy assembly compiler data.
    evmla_data: Option<EVMLAData>,
}

impl<'ctx> Block<'ctx> {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(inner: inkwell::basic_block::BasicBlock<'ctx>) -> Self {
        Self {
            inner,
            evmla_data: None,
        }
    }

    ///
    /// Sets the EVM legacy assembly data.
    ///
    pub fn set_evmla_data(&mut self, data: EVMLAData) {
        self.evmla_data = Some(data);
    }

    ///
    /// The LLVM object reference.
    ///
    pub fn inner(&self) -> inkwell::basic_block::BasicBlock<'ctx> {
        self.inner
    }

    ///
    /// Returns the EVM data reference.
    ///
    /// # Panics
    /// If the EVM data has not been initialized.
    ///
    pub fn evm(&self) -> &EVMLAData {
        self.evmla_data
            .as_ref()
            .expect("The EVM data must have been initialized")
    }

    ///
    /// Returns the EVM data mutable reference.
    ///
    /// # Panics
    /// If the EVM data has not been initialized.
    ///
    pub fn evm_mut(&mut self) -> &mut EVMLAData {
        self.evmla_data
            .as_mut()
            .expect("The EVM data must have been initialized")
    }
}
