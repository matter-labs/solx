//!
//! Foundry gas benchmark report format.
//!

pub mod contract;

use self::contract::ContractReport;

///
/// Foundry gas benchmark report format.
///
#[derive(Debug, serde::Deserialize)]
pub struct FoundryGasReport(pub Vec<ContractReport>);
