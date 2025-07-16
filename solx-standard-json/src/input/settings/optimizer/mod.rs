//!
//! The `solc --standard-json` input settings optimizer.
//!

pub mod spill_area_size;

use std::collections::BTreeMap;

use self::spill_area_size::SpillAreaSize;

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
    /// Spill area size for the LLVM stack-too-deep avoidance algorithm.
    /// It is specified per-contract using its fully qualified name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spill_area_size: Option<BTreeMap<String, SpillAreaSize>>,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new(
            Self::default_mode().expect("Always exists"),
            Self::default_size_fallback().expect("Always exists"),
            None,
        )
    }
}

impl Optimizer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        mode: char,
        size_fallback: bool,
        spill_area_size: Option<BTreeMap<String, SpillAreaSize>>,
    ) -> Self {
        Self {
            mode: Some(mode),
            size_fallback: Some(size_fallback),
            spill_area_size,
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
