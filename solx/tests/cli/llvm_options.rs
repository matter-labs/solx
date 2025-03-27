//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--llvm-options='-eravm-disable-system-request-memoization 10'",
        "--bin",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains("Binary"));

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--llvm-options='-eravm-disable-system-request-memoization 10'",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "LLVM options must be specified in standard JSON input settings.",
    ));

    Ok(())
}
