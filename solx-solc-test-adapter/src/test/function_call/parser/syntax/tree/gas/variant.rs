//!
//! The gas option variant.
//!

///
/// The gas option variant.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variant {
    /// `irOptimized` in the source code.
    IrOptimized,
    /// `legacy` in the source code.
    Legacy,
    /// `legacyOptimized` in the source code.
    LegacyOptimized,
    /// `ir` in the source code.
    Ir,
}

impl Variant {
    ///
    /// A shortcut constructor.
    ///
    pub fn ir_optimized() -> Self {
        Self::IrOptimized
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn legacy() -> Self {
        Self::Legacy
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn legacy_optimized() -> Self {
        Self::LegacyOptimized
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn ir() -> Self {
        Self::Ir
    }
}
