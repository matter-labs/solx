//!
//! The Ethereal IR block queue element.
//!

use crate::ethereal_ir::function::block::element::stack::Stack;

///
/// The Ethereal IR block queue element.
///
#[derive(Debug)]
pub struct QueueElement {
    /// The block key.
    pub block_key: solx_codegen_evm::BlockKey,
    /// The block predecessor.
    pub predecessor: Option<(solx_codegen_evm::BlockKey, usize)>,
    /// The predecessor's last stack state.
    pub stack: Stack,
}

impl QueueElement {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        block_key: solx_codegen_evm::BlockKey,
        predecessor: Option<(solx_codegen_evm::BlockKey, usize)>,
        stack: Stack,
    ) -> Self {
        Self {
            block_key,
            predecessor,
            stack,
        }
    }
}
