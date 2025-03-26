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
