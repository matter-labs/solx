//!
//! The `solc --standard-json` debug settings.
//!

///
/// The `solc --standard-json` debug settings.
///
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Debug {
    /// The revert strings parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert_strings: Option<String>,
}

impl Debug {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(revert_strings: Option<String>) -> Self {
        Self { revert_strings }
    }
}
