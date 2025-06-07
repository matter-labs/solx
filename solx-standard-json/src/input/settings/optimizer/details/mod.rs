//!
//! The `solc --standard-json` input settings optimizer details.
//!

pub mod yul_details;

use self::yul_details::YulDetails;

///
/// The `solc --standard-json` input settings optimizer details.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    /// Yul optimizer details.
    #[serde(default)]
    pub yul_details: YulDetails,
}
