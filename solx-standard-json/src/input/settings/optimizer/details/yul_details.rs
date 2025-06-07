//!
//! The `solc --standard-json` input settings optimizer Yul details.
//!

///
/// The `solc --standard-json` input settings optimizer Yul details.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulDetails {
    /// Stack allocation.
    #[serde(default = "YulDetails::default_stack_allocation")]
    pub stack_allocation: bool,
}

impl Default for YulDetails {
    fn default() -> Self {
        Self::new(Self::default_stack_allocation())
    }
}

impl YulDetails {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(stack_allocation: bool) -> Self {
        Self { stack_allocation }
    }

    ///
    /// The default flag to enable the stack allocation.
    ///
    pub fn default_stack_allocation() -> bool {
        true
    }
}
