//!
//! CLI tests for empty input.
//!

use predicates::prelude::*;

#[test]
fn no_inputs() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[];

    let result = crate::cli::execute_solx(args)?;

    result
        .failure()
        .stderr(predicate::str::contains("No input files given. For standard input, specify `-` explicitly, or visit `--help` to see all options.").count(1));

    Ok(())
}

#[test]
fn no_inputs_yul() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--yul"];

    let result = crate::cli::execute_solx(args)?;

    result
        .failure()
        .stderr(predicate::str::contains("No input files given. For standard input, specify `-` explicitly, or visit `--help` to see all options.").count(1));

    Ok(())
}

#[test]
fn no_inputs_llvm_ir() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--yul"];

    let result = crate::cli::execute_solx(args)?;

    result
        .failure()
        .stderr(predicate::str::contains("No input files given. For standard input, specify `-` explicitly, or visit `--help` to see all options.").count(1));

    Ok(())
}
