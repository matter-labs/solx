//!
//! The contract source code.
//!

pub mod evmla;
pub mod llvm_ir;
pub mod yul;

use std::collections::BTreeSet;

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

impl IR {
    ///
    /// Drains the list of factory dependencies.
    ///
    pub fn drain_factory_dependencies(&mut self) -> BTreeSet<String> {
        match self {
            IR::Yul(ref mut yul) => yul.object.0.factory_dependencies.drain().collect(),
            IR::EVMLA(ref mut evm) => evm.assembly.factory_dependencies.drain().collect(),
            IR::LLVMIR(_) => BTreeSet::new(),
        }
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> BTreeSet<String> {
        match self {
            Self::Yul(inner) => inner.get_missing_libraries(),
            Self::EVMLA(inner) => inner.get_missing_libraries(),
            Self::LLVMIR(_inner) => BTreeSet::new(),
        }
    }
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
