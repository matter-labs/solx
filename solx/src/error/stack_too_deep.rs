//!
//! Stack-too-deep compilation error.
//!

///
/// Stack-too-deep compilation error.
///
#[derive(Debug, Clone, thiserror::Error, serde::Serialize, serde::Deserialize)]
#[error("Stack-too-deep error detected in {code_segment} code. Required spill area: {spill_area_size:?} bytes")]
pub struct StackTooDeep {
    /// Contract code segment.
    pub code_segment: era_compiler_common::CodeSegment,
    /// Spill area size in bytes.
    pub spill_area_size: u64,
}
