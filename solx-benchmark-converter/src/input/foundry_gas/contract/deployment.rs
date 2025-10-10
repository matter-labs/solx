//!
//! Foundry benchmark deployment report.
//!

///
/// Foundry benchmark deployment report.
///
#[derive(Debug, serde::Deserialize)]
pub struct Deployment {
    /// Contract size.
    pub size: u64,
    /// Gas amount spent on deployment.
    pub gas: u64,
}
