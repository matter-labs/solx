//!
//! The `solc --standard-json` input settings optimizer details.
//!

pub mod yul_details;

use self::yul_details::YulDetails;

///
/// The `solc --standard-json` input settings optimizer details.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    /// Yul optimizer.
    #[serde(default)]
    pub yul: bool,
    /// Yul optimizer details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yul_details: Option<YulDetails>,
}

impl Default for Details {
    fn default() -> Self {
        Self {
            yul: true,
            yul_details: Some(YulDetails::default()),
        }
    }
}
