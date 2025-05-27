//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn bin() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_solx = TempDir::with_prefix("solx_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solx.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = crate::cli::execute_solx(args)?;
    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    assert!(tmp_dir_solx.path().exists());

    Ok(())
}

#[test]
fn bin_missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_solx = TempDir::with_prefix("solx_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solx.path().to_str().unwrap(),
    ];

    let _ = crate::cli::execute_solx(args)?;
    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Refusing to overwrite an existing file",
    ));

    assert!(tmp_dir_solx.path().exists());

    Ok(())
}

#[test]
fn asm() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_solx = TempDir::with_prefix("solx_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--asm",
        "--output-dir",
        tmp_dir_solx.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = crate::cli::execute_solx(args)?;
    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    assert!(tmp_dir_solx.path().exists());

    Ok(())
}

#[test]
fn asm_missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_solx = TempDir::with_prefix("solx_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--asm",
        "--output-dir",
        tmp_dir_solx.path().to_str().unwrap(),
    ];

    let _ = crate::cli::execute_solx(args)?;
    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Refusing to overwrite an existing file",
    ));

    assert!(tmp_dir_solx.path().exists());

    Ok(())
}

#[test]
fn metadata() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_solx = TempDir::with_prefix("solx_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata",
        "--output-dir",
        tmp_dir_solx.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = crate::cli::execute_solx(args)?;
    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    assert!(tmp_dir_solx.path().exists());

    Ok(())
}

#[test]
fn metadata_missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_solx = TempDir::with_prefix("solx_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata",
        "--output-dir",
        tmp_dir_solx.path().to_str().unwrap(),
    ];

    let _ = crate::cli::execute_solx(args)?;
    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Refusing to overwrite an existing file",
    ));

    assert!(tmp_dir_solx.path().exists());

    Ok(())
}

#[test]
fn all() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_solx = TempDir::with_prefix("solx_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_solx.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = crate::cli::execute_solx(args)?;
    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    assert!(tmp_dir_solx.path().exists());

    Ok(())
}

#[test]
fn all_missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_solx = TempDir::with_prefix("solx_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_solx.path().to_str().unwrap(),
    ];

    let _ = crate::cli::execute_solx(args)?;
    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Refusing to overwrite an existing file",
    ));

    assert!(tmp_dir_solx.path().exists());

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
        "--overwrite",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Overwriting flag cannot be used in standard JSON mode.",
    ));

    Ok(())
}
