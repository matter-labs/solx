//!
//! CLI tests for the eponymous option.
//!

use std::path::PathBuf;

use predicates::prelude::*;
use tempfile::TempDir;
use test_case::test_case;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let output_directory = TempDir::with_prefix("solx_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_CALLER_MAIN_PATH,
        "--bin",
        "--bin-runtime",
        "--asm",
        "--metadata",
        "--ast-json",
        "--abi",
        "--hashes",
        "--userdoc",
        "--devdoc",
        "--storage-layout",
        "--transient-storage-layout",
        "--asm-solc-json",
        "--ir",
        "--output-dir",
        output_directory.path().to_str().expect("Always valid"),
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));
    assert!(output_directory.path().exists());

    Ok(())
}

#[test_case(format!(".{}", era_compiler_common::EXTENSION_EVM_BINARY))]
#[test_case(format!("_llvm.{}", era_compiler_common::EXTENSION_EVM_ASSEMBLY))]
#[test_case(format!("_meta.{}", era_compiler_common::EXTENSION_JSON))]
fn yul(extension: String) -> anyhow::Result<()> {
    crate::common::setup()?;

    let input_path = PathBuf::from(crate::common::TEST_YUL_CONTRACT_PATH);
    let output_directory = TempDir::with_prefix("solx_output")?;
    let mut output_file = input_path
        .join("Return")
        .to_string_lossy()
        .replace(['\\', '/', '.'], "_");
    output_file.push_str(extension.as_str());

    let args = &[
        input_path.to_str().expect("Always valid"),
        "--yul",
        "--bin",
        "--bin-runtime",
        "--asm",
        "--metadata",
        "--output-dir",
        output_directory.path().to_str().expect("Always valid"),
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    assert!(output_directory.path().exists());
    assert!(output_directory.path().join(output_file.as_str()).exists());

    Ok(())
}

#[test]
fn unusual_path_characters() -> anyhow::Result<()> {
    crate::common::setup()?;

    let output_directory = TempDir::with_prefix("File!and#$%-XXXXXX")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--bin-runtime",
        "--asm",
        "--metadata",
        "--output-dir",
        output_directory.path().to_str().expect("Always valid"),
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));
    assert!(output_directory.path().exists());

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_PATH,
        "--output-dir",
        "output",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Output directory cannot be used in standard JSON mode.",
    ));

    Ok(())
}
