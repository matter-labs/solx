//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "--bin"];

    let result = crate::cli::execute_solx(args)?;

    result
        .success()
        .stdout(predicate::str::contains("Binary").count(1));

    Ok(())
}

#[test]
fn stdin() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--bin",
        solx_standard_json::InputSource::STDIN_INPUT_IDENTIFIER,
    ];

    let result =
        crate::cli::execute_solx_with_stdin(args, crate::common::TEST_SOLIDITY_CONTRACT_PATH)?;

    result
        .success()
        .stdout(predicate::str::contains("Binary").count(1));

    Ok(())
}

#[test]
fn deploy_time_linking() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH,
        "--bin",
    ];

    let result = crate::cli::execute_solx(args)?;

    result
        .success()
        .stdout(predicate::str::contains("Binary").count(2))
        .stdout(predicate::str::contains("__$733ff2b5a7b9002c636c19ae8206a21f88$__").count(1));

    Ok(())
}

#[test]
fn stack_too_deep() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_STACK_TOO_DEEP_PATH,
        "--bin",
    ];

    let result = crate::cli::execute_solx(args)?;

    result
        .success()
        .stdout(predicate::str::contains("Binary").count(1));

    Ok(())
}

#[test]
fn invalid_input() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[crate::common::TEST_YUL_CONTRACT_PATH, "--bin"];

    let result = crate::cli::execute_solx(args)?;

    result.failure().stderr(predicate::str::contains(
        "Expected identifier but got 'StringLiteral'",
    ));

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_PATH,
        "--bin",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Cannot output data outside of JSON in standard JSON mode.",
    ));

    Ok(())
}
