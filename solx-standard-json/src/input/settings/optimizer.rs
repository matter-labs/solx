//!
//! The `solc --standard-json` input settings optimizer.
//!

///
/// The `solc --standard-json` input settings optimizer.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Optimizer {
    /// The optimization mode string.
    #[serde(
        default = "Optimizer::default_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub mode: Option<char>,
    /// Whether to try to recompile with -Oz if the bytecode is too large.
    #[serde(
        default = "Optimizer::default_size_fallback",
        skip_serializing_if = "Option::is_none"
    )]
    pub size_fallback: Option<bool>,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new(
            Self::default_mode().expect("Always exists"),
            Self::default_size_fallback().expect("Always exists"),
        )
    }
}

impl Optimizer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(mode: char, size_fallback: bool) -> Self {
        Self {
            mode: Some(mode),
            size_fallback: Some(size_fallback),
        }
    }

    ///
    /// The default optimization mode.
    ///
    pub fn default_mode() -> Option<char> {
        Some('3')
    }

    ///
    /// The default flag for the size fallback.
    ///
    pub fn default_size_fallback() -> Option<bool> {
        Some(false)
    }
}
