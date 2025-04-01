//!
//! The `solc --standard-json` input language.
//!

///
/// The `solc --standard-json` input language.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Language {
    /// Solidity language.
    Solidity,
    /// Yul IR.
    Yul,
    /// LLVM IR.
    #[serde(rename = "LLVM IR")]
    LLVMIR,
}
