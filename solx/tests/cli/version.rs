//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--version"];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("LLVM-based Solidity compiler"));

    Ok(())
}

#[test]
fn excess_args() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--version", crate::common::TEST_SOLIDITY_CONTRACT_PATH];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "No other options are allowed while getting the compiler version.",
    ));

    Ok(())
}
