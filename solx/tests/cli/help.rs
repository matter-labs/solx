//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn no_error() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--help"];

    let result = crate::cli::execute_solx(args)?;

    result
        .failure()
        .stderr(predicate::str::starts_with("Error").not());

    Ok(())
}
