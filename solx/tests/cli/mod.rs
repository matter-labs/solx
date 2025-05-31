//!
//! The CLI/e2e tests entry module.
//!

use std::process::Command;

use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;

mod abi;
mod allow_paths;
mod asm;
mod asm_solc_json;
mod ast_json;
mod base_path;
mod bin;
mod bin_runtime;
mod debug_output_dir;
mod devdoc;
mod evm_version;
mod hashes;
mod include_path;
mod ir_optimized;
mod libraries;
mod llvm_ir;
mod llvm_options;
mod metadata;
mod metadata_hash;
mod metadata_literal;
mod no_cbor_metadata;
mod no_import_callback;
mod optimization;
mod optimization_size_fallback;
mod output_dir;
mod overwrite;
mod recursive_process;
mod remappings;
mod standard_json;
mod storage_layout;
mod threads;
mod transient_storage_layout;
mod userdoc;
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
