//!
//! The `solc --standard-json` output selection.
//!

pub mod file;

use serde::Serialize;

use crate::compilers::solidity::codegen::Codegen;

use self::file::File as FileSelection;

///
/// The `solc --standard-json` output selection.
///
#[derive(Debug, Default, Serialize)]
pub struct Selection {
    /// Only the 'all' wildcard is available for robustness reasons.
    #[serde(rename = "*", skip_serializing_if = "Option::is_none")]
    pub all: Option<FileSelection>,
}

impl Selection {
    ///
    /// Creates the selection required by EVM compilation process.
    ///
    pub fn new_required(codegen: Option<Codegen>) -> Self {
        Self {
            all: Some(FileSelection::new_required(codegen)),
        }
    }
}
