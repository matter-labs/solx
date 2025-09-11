//!
//! The LLVM optimizer settings size level.
//!

///
/// The LLVM optimizer settings size level.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SizeLevel {
    /// No size optimizations.
    Zero,
    /// The default size optimizations.
    S,
    /// The aggresize size optimizations.
    Z,
}

impl From<SizeLevel> for u32 {
    fn from(level: SizeLevel) -> Self {
        match level {
            SizeLevel::Zero => 0,
            SizeLevel::S => 1,
            SizeLevel::Z => 2,
        }
    }
}

impl std::fmt::Display for SizeLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SizeLevel::Zero => write!(f, "0"),
            SizeLevel::S => write!(f, "s"),
            SizeLevel::Z => write!(f, "z"),
        }
    }
}
