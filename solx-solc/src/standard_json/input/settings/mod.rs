//!
//! The `solc --standard-json` input settings.
//!

pub mod metadata;
pub mod optimizer;
pub mod selection;

use std::collections::BTreeSet;

use self::metadata::Metadata;
use self::optimizer::Optimizer;
use self::selection::Selection;

///
/// The `solc --standard-json` input settings.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// The optimizer settings.
    #[serde(default)]
    pub optimizer: Optimizer,

    /// The linker library addresses.
    #[serde(
        default,
        skip_serializing_if = "era_compiler_common::Libraries::is_empty"
    )]
    pub libraries: era_compiler_common::Libraries,
    /// The sorted list of remappings.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub remappings: BTreeSet<String>,

    /// The target EVM version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_version: Option<era_compiler_common::EVMVersion>,
    /// Whether to compile Solidity via IR.
    #[serde(
        default,
        rename = "viaIR",
        skip_serializing_if = "Settings::is_via_ir_default"
    )]
    pub via_ir: bool,

    /// The output selection filters.
    #[serde(default, skip_serializing_if = "Selection::is_empty")]
    pub output_selection: Selection,
    /// The metadata settings.
    #[serde(default)]
    pub metadata: Metadata,

    /// The extra LLVM options.
    #[serde(default, skip_serializing)]
    pub llvm_options: Vec<String>,
}

impl Settings {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        optimizer: Optimizer,

        libraries: era_compiler_common::Libraries,
        remappings: BTreeSet<String>,

        evm_version: Option<era_compiler_common::EVMVersion>,
        via_ir: bool,

        mut output_selection: Selection,
        metadata: Metadata,

        llvm_options: Vec<String>,
    ) -> Self {
        output_selection.set_ir(via_ir);

        Self {
            optimizer,

            libraries,
            remappings,

            evm_version,
            via_ir,

            output_selection,
            metadata,
            llvm_options,
        }
    }

    ///
    /// Whether the via IR flag is the default.
    ///
    fn is_via_ir_default(via_ir: &bool) -> bool {
        !via_ir
    }
}
