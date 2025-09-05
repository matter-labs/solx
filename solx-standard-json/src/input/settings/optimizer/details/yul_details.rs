//!
//! The `solc --standard-json` input settings optimizer Yul details.
//!

///
/// The `solc --standard-json` input settings optimizer Yul details.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulDetails {
    /// Yul stack allocation flag.
    #[serde(default)]
    pub stack_allocation: bool,
}
