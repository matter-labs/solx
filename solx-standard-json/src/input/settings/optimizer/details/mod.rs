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
    /// Yul optimizer.
    /// Always true to disable the `stackAllocation` pass.
    #[serde(default = "Details::default_yul")]
    pub yul: bool,
    /// Yul optimizer details.
    #[serde(default)]
    pub yul_details: YulDetails,
}

impl Details {
    /// 
    /// Returns the default value for `yul`.
    /// 
    fn default_yul() -> bool {
        false
    }
}
