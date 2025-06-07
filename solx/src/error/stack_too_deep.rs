//!
//! Stack-too-deep compilation error.
//!

///
/// Stack-too-deep compilation error.
///
#[derive(Debug, Clone, thiserror::Error, serde::Serialize, serde::Deserialize)]
#[error("Stack-too-deep error detected. Required spill areas: {deploy_spill_area_size:?} bytes for deploy code, {runtime_spill_area_size:?} bytes for runtime code")]
pub struct StackTooDeep {
    /// Contract full name.
    pub contract_name: era_compiler_common::ContractName,
    /// Deploy code spill area size in bytes.
    pub deploy_spill_area_size: Option<u64>,
    /// Runtime code spill area size in bytes.
    pub runtime_spill_area_size: Option<u64>,
}
