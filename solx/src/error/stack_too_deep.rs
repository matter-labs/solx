//!
//! Stack-too-deep compilation error.
//!

///
/// Stack-too-deep compilation error.
///
#[derive(Debug, Clone, thiserror::Error, serde::Serialize, serde::Deserialize)]
#[error("Stack-too-deep error. Required spill area: {spill_area_size:?} bytes")]
pub struct StackTooDeep {
    /// Spill area size in bytes.
    pub spill_area_size: u64,
}
