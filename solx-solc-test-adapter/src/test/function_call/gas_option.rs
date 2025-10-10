//!
//! The gas option.
//!

use crate::test::function_call::parser::Gas;
use crate::test::function_call::parser::GasVariant;

///
/// The gas option.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GasOption {
    /// `irOptimized` in the metadata.
    IrOptimized(web3::types::U256),
    /// `legacy` in the metadata.
    Legacy(web3::types::U256),
    /// `legacyOptimized` in the metadata.
    LegacyOptimized(web3::types::U256),
    /// `ir` in the metadata.
    Ir(web3::types::U256),
}

impl From<Gas> for GasOption {
    fn from(gas: Gas) -> Self {
        let value = web3::types::U256::from_dec_str(gas.value.as_str())
            .expect(super::VALIDATED_BY_THE_PARSER);
        match gas.variant {
            GasVariant::IrOptimized => Self::IrOptimized(value),
            GasVariant::Legacy => Self::Legacy(value),
            GasVariant::LegacyOptimized => Self::LegacyOptimized(value),
            GasVariant::Ir => Self::Ir(value),
        }
    }
}
