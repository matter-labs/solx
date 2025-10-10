//!
//! The `solc --standard-json` output contract EVM data.
//!

pub mod bytecode;

use std::collections::BTreeMap;

use serde::Deserialize;

use self::bytecode::Bytecode;

///
/// The `solc --standard-json` output contract EVM data.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EVM {
    /// Contract deploy bytecode.
    pub bytecode: Option<Bytecode>,
    /// Contract runtime bytecode.
    pub deployed_bytecode: Option<Bytecode>,
    /// Contract function signatures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method_identifiers: Option<BTreeMap<String, String>>,
}
