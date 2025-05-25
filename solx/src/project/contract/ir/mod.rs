//!
//! The contract source code.
//!

pub mod evmla;
pub mod llvm_ir;
pub mod yul;

use self::evmla::EVMLA;
use self::llvm_ir::LLVMIR;
use self::yul::Yul;

///
/// The contract source code.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum IR {
    /// The Yul source code.
    Yul(Yul),
    /// The EVM legacy assembly source code.
    EVMLA(EVMLA),
    /// The LLVM IR source code.
    LLVMIR(LLVMIR),
}

impl From<Yul> for IR {
    fn from(inner: Yul) -> Self {
        Self::Yul(inner)
    }
}

impl From<EVMLA> for IR {
    fn from(inner: EVMLA) -> Self {
        Self::EVMLA(inner)
    }
}

impl From<LLVMIR> for IR {
    fn from(inner: LLVMIR) -> Self {
        Self::LLVMIR(inner)
    }
}
