//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;
use test_case::test_case;

// TODO: #[test_case('0')] when -O0 is supported
#[test_case('1')]
#[test_case('2')]
#[test_case('3')]
#[test_case('s')]
#[test_case('z')]
fn all(level: char) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        &format!("-O{level}"),
        "--bin",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains("Binary"));

    Ok(())
}

// TODO: #[test_case('0')] when -O0 is supported
#[test_case('1')]
#[test_case('2')]
#[test_case('3')]
#[test_case('s')]
#[test_case('z')]
fn all_with_env_var(level: char) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "--bin"];
    let env_vars = vec![("SOLX_OPTIMIZATION", level.to_string())];

    let result = crate::cli::execute_solx_with_env_vars(args, env_vars)?;
    result.success().stdout(predicate::str::contains("Binary"));

    Ok(())
}

#[test]
fn invalid() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "-O", "99"];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(
        predicate::str::contains("Unexpected optimization option")
            .or(predicate::str::contains("error: invalid value \'99\' for \'--optimization <OPTIMIZATION>\': too many characters in string")),
    );

    Ok(())
}

#[test]
fn invalid_with_env_var() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH];
    let env_vars = vec![("SOLX_OPTIMIZATION", "99".to_string())];

    let result = crate::cli::execute_solx_with_env_vars(args, env_vars)?;
    result.failure().stderr(
        predicate::str::contains("Error: Invalid value `99` for environment variable \'SOLX_OPTIMIZATION\': only values 1, 2, 3, s, z are supported.")
    );

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_PATH,
        "-O",
        "3",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "LLVM optimizations must be specified in standard JSON input settings.",
    ));

    Ok(())
}

#[test]
fn standard_json_invalid_env_var() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_PATH,
    ];
    let env_vars = vec![("SOLX_OPTIMIZATION", "99".to_string())];

    let result = crate::cli::execute_solx_with_env_vars(args, env_vars)?;
    result.success().stdout(
    predicate::str::contains("Error: Invalid value `99` for environment variable \'SOLX_OPTIMIZATION\': only values 1, 2, 3, s, z are supported.")
    );

    Ok(())
}
