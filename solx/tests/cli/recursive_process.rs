//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn missing_input() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--recursive-process"];

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("Error: Stdin parsing error"));

    Ok(())
}

#[test]
fn excess_arguments() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--recursive-process",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "No other options are allowed in recursive mode.",
    ));

    Ok(())
}
