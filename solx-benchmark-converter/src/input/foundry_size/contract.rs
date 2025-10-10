//!
//! Foundry contract size benchmark report.
//!

///
/// Foundry contract size benchmark report.
///
#[derive(Debug, serde::Deserialize)]
pub struct ContractReport {
    /// Deployment code size, includes runtime code size.
    pub init_size: u64,
    /// Deployment code margin, that is the number of bytes before the EVM deploy code size limit.
    pub init_margin: i64,
    /// Runtime code size.
    pub runtime_size: u64,
    /// Runtime code margin, that is the number of bytes before the EVM runtime code size limit.
    pub runtime_margin: i64,
}
