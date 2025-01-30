//!
//! The `solc --combined-json` contract.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

///
/// The contract.
///
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Contract {
    /// `solc` hashes output.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub hashes: BTreeMap<String, String>,
    /// `solc` ABI output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub abi: serde_json::Value,
    /// `solc` metadata output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// `solc` developer documentation output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub devdoc: serde_json::Value,
    /// `solc` user documentation output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub userdoc: serde_json::Value,
    /// `solc` storage layout output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub storage_layout: serde_json::Value,
    /// `solc` transient storage layout output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub transient_storage_layout: serde_json::Value,
    /// `solc` AST output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub ast: serde_json::Value,
    /// `solc` assembly output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub asm: serde_json::Value,

    /// LLVM-generated assembly.
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub assembly: Option<String>,
    /// Hexadecimal deploy bytecode segment output.
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub bin: Option<String>,
    /// Hexadecimal runtime bytecode segment output.
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub bin_runtime: Option<String>,

    /// The unlinked factory dependencies.
    #[serde(default, skip_deserializing)]
    pub factory_deps_unlinked: BTreeSet<String>,
    /// The factory dependencies.
    #[serde(default, skip_deserializing)]
    pub factory_deps: BTreeMap<String, String>,
    /// The missing libraries.
    #[serde(default, skip_deserializing)]
    pub missing_libraries: BTreeSet<String>,
    /// The binary object format.
    #[serde(default, skip_deserializing)]
    pub object_format: Option<era_compiler_common::ObjectFormat>,
}
