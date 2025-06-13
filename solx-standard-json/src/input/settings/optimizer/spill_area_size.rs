//!
//! The `solc --standard-json` input settings optimizer spill area size.
//!

///
/// The `solc --standard-json` input settings optimizer spill area size.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpillAreaSize {
    /// Deploy code spill area size.
    pub creation: u64,
    /// Runtime code spill area size.
    pub runtime: u64,
}

impl SpillAreaSize {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(creation: u64, runtime: u64) -> Self {
        Self { creation, runtime }
    }
}
