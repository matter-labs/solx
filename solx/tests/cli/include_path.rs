//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--base-path",
        crate::common::TEST_CONTRACTS_PATH,
        "--include-path",
        crate::common::TEST_CONTRACTS_PATH,
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test]
fn yul() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--base-path",
        crate::common::TEST_CONTRACTS_PATH,
        "--include-path",
        crate::common::TEST_CONTRACTS_PATH,
        "--yul",
        "--bin",
        crate::common::TEST_YUL_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "`include-path` is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test]
fn llvm_ir() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--base-path",
        crate::common::TEST_CONTRACTS_PATH,
        "--include-path",
        crate::common::TEST_CONTRACTS_PATH,
        "--llvm-ir",
        "--bin",
        crate::common::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "`include-path` is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test]
fn base_path_missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--include-path",
        crate::common::TEST_CONTRACTS_PATH,
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "--include-path option requires a non-empty base path",
    ));

    Ok(())
}
