//!
//! The CLI/e2e tests entry module.
//!

use std::process::Command;

use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;

mod allow_paths;
mod base_path;
mod bin;
mod debug_output_dir;
mod evm_version;
mod general;
mod include_path;
mod libraries;
mod llvm_ir;
mod llvm_options;
mod metadata;
mod metadata_hash;
mod metadata_literal;
mod no_cbor_metadata;
mod optimization;
mod optimization_size_fallback;
mod output_dir;
mod overwrite;
mod recursive_process;
mod remappings;
mod standard_json;
mod threads;
mod version;
mod via_ir;
mod yul;

///
/// Execute `solx` with the given arguments and assert the result.
///
pub fn execute_solx(args: &[&str]) -> anyhow::Result<assert_cmd::assert::Assert> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    Ok(cmd.args(args).assert())
}

///
/// Check if the file at the given path is empty.
///
pub fn is_file_empty(file_path: &str) -> anyhow::Result<bool> {
    let metadata = std::fs::metadata(file_path)?;
    Ok(metadata.len() == 0)
}

///
/// Check if the output is the same as the file content.
///
pub fn is_output_same_as_file(file_path: &str, output: &str) -> anyhow::Result<bool> {
    let file_content = std::fs::read_to_string(file_path)?;
    Ok(file_content.trim().contains(output.trim()) || output.trim().contains(file_content.trim()))
}
