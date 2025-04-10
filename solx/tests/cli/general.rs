//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;
use tempfile::TempDir;
use test_case::test_case;

#[test]
fn no_arguments() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args: &[&str] = &[];

    let result = crate::cli::execute_solx(args)?;

    result
        .failure()
        .stderr(predicate::str::contains("Usage: solx"));

    Ok(())
}

#[test_case(crate::common::SOLIDITY_BIN_OUTPUT_NAME)]
fn multiple_output_options(bin_output_file_name: &str) -> anyhow::Result<()> {
    crate::common::setup()?;
    let tmp_dir = TempDir::new()?;
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "-O3",
        "--bin",
        "--output-dir",
        tmp_dir.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful."));

    assert!(tmp_dir.path().exists());

    let bin_output_file = tmp_dir
        .path()
        .join(crate::common::TEST_SOLIDITY_CONTRACT_NAME)
        .join(bin_output_file_name);

    assert!(bin_output_file.exists());
    assert!(!crate::cli::is_file_empty(
        bin_output_file.to_str().unwrap()
    )?);

    Ok(())
}

#[test_case(crate::common::SOLIDITY_BIN_OUTPUT_NAME)]
fn same_output_directory_and_terminal(bin_output_file_name: &str) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir = TempDir::new()?;
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "-O3",
        "--bin",
        "--output-dir",
        tmp_dir.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful."));

    let bin_output_file = tmp_dir
        .path()
        .join(crate::common::TEST_SOLIDITY_CONTRACT_NAME)
        .join(bin_output_file_name);
    assert!(bin_output_file.exists());

    let cli_args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "-O3", "--bin"];
    let cli_result = crate::cli::execute_solx(cli_args)?;

    let stdout = String::from_utf8_lossy(cli_result.get_output().stdout.as_slice());
    assert!(crate::cli::is_output_same_as_file(
        bin_output_file.to_str().unwrap(),
        stdout.trim()
    )?);

    Ok(())
}
