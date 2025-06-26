//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;
use test_case::test_case;

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

#[test_case(crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH, vec!["__$733ff2b5a7b9002c636c19ae8206a21f88$__"])]
#[test_case(crate::common::TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_PATH, vec!["__$65ec92bf84627f42eab2cb5e40b5cc19ff$__"])]
#[test_case(crate::common::TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_MULTI_LEVEL_PATH, vec!["__$c1091a910937160002c95b60eab1fc9a86$__", "__$71eefe2b783075e8d047b21bbc2b61aa32$__"])]
fn deploy_time_linking(path: &str, placeholders: Vec<&str>) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[path, "--bin"];

    let mut result = crate::cli::execute_solx(args)?;

    result = result.success().stdout(predicate::str::contains("Binary"));
    for placeholder in placeholders.into_iter() {
        result = result.stdout(predicate::str::contains(placeholder));
    }

    Ok(())
}

#[test]
fn stack_too_deep_solc() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_STACK_TOO_DEEP_SOLC_PATH,
        "--bin",
    ];

    let result = crate::cli::execute_solx(args)?;

    result
        .success()
        .stdout(predicate::str::contains("Binary").count(1));

    Ok(())
}

#[test]
fn stack_too_deep_llvm() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_STACK_TOO_DEEP_LLVM_PATH,
        "--bin",
        "-Oz",
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
