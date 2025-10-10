//!
//! Foundry size benchmark report format.
//!

pub mod contract;

use std::collections::BTreeMap;

use self::contract::ContractReport;

///
/// Foundry size benchmark report format.
///
#[derive(Debug, serde::Deserialize)]
pub struct FoundrySizeReport(pub BTreeMap<String, ContractReport>);
