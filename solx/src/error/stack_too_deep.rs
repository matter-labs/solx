//!
//! Stack-too-deep compilation error.
//!

///
/// Stack-too-deep compilation error.
///
#[derive(Debug, Clone, thiserror::Error, serde::Serialize, serde::Deserialize)]
#[error("Stack-too-deep error detected in {code_segment:?} code of {contract_name:?}. Required spill area: {spill_area_size:?} bytes")]
pub struct StackTooDeep {
    /// Spill area size in bytes.
    pub spill_area_size: u64,
    /// Optional contract identifier.
    pub contract_name: Option<era_compiler_common::ContractName>,
    /// Optional code segment.
    pub code_segment: Option<era_compiler_common::CodeSegment>,
}
