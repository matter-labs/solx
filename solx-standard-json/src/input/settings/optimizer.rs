//!
//! The `solc --standard-json` input settings optimizer.
//!

///
/// The `solc --standard-json` input settings optimizer.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Optimizer {
    /// The optimization mode string.
    #[serde(default = "Optimizer::default_mode", skip_serializing)]
    pub mode: char,
    /// Whether to try to recompile with -Oz if the bytecode is too large.
    #[serde(default, skip_serializing)]
    pub size_fallback: bool,

    /// Enable the solc optimizer.
    /// Always `true` in order to allow library inlining.
    #[serde(default = "Optimizer::default_enabled")]
    pub enabled: bool,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new(Self::default_mode(), false)
    }
}

impl Optimizer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(mode: char, size_fallback: bool) -> Self {
        Self {
            mode,
            size_fallback,

            enabled: Self::default_enabled(),
        }
    }

    ///
    /// The default optimization mode.
    ///
    fn default_mode() -> char {
        '3'
    }

    ///
    /// The default flag to enable the `solc` optimizer.
    ///
    fn default_enabled() -> bool {
        true
    }
}
