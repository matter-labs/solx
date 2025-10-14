//!
//! Foundry benchmark function calling report.
//!

///
/// Foundry benchmark function calling report.
///
#[derive(Debug, serde::Deserialize)]
pub struct FunctionReport {
    /// Number of calls.
    pub calls: usize,
    /// Mean gas cost.
    pub mean: u64,
}
