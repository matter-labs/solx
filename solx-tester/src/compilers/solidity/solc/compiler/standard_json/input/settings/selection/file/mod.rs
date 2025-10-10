//!
//! The `solc --standard-json` output file selection.
//!

pub mod flag;

use std::collections::HashSet;

use serde::Serialize;

use crate::compilers::solidity::codegen::Codegen;

use self::flag::Flag as SelectionFlag;

///
/// The `solc --standard-json` output file selection.
///
#[derive(Debug, Default, Serialize)]
pub struct File {
    /// The per-file output selections.
    #[serde(rename = "", skip_serializing_if = "Option::is_none")]
    pub per_file: Option<HashSet<SelectionFlag>>,
    /// The per-contract output selections.
    #[serde(rename = "*", skip_serializing_if = "Option::is_none")]
    pub per_contract: Option<HashSet<SelectionFlag>>,
}

impl File {
    ///
    /// Creates the selection required by EVM compilation process.
    ///
    pub fn new_required(codegen: Option<Codegen>) -> Self {
        let mut per_contract = HashSet::new();
        per_contract.insert(SelectionFlag::Bytecode);
        per_contract.insert(SelectionFlag::MethodIdentifiers);
        if let Some(codegen) = codegen {
            per_contract.insert(SelectionFlag::from(codegen));
        }
        Self {
            per_file: Some(HashSet::from_iter([SelectionFlag::AST])),
            per_contract: Some(per_contract),
        }
    }
}
