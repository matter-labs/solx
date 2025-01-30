//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn invalid_input_yul() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--standard-json", crate::common::TEST_YUL_CONTRACT_PATH];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("parsing: expected value"));

    Ok(())
}

#[test]
fn invalid_input_solc_error() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_INVALID_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "ParserError: Expected identifier but got",
    ));

    Ok(())
}

// TODO: fix when recursion works with a static library
// #[test]
// fn recursion() -> anyhow::Result<()> {
//     crate::common::setup()?;

//     let args = &[
//         "--standard-json",
//         crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLX_RECURSION_PATH,
//     ];

//     let result = crate::cli::execute_solx(args)?;
//     result
//         .success()
//         .stdout(predicate::str::contains("bytecode"));

//     Ok(())
// }

#[test]
fn invalid_path() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains(
            "Standard JSON file \\\"tests/data/standard_json_input/non_existent.json\\\" reading",
        ))
        .code(era_compiler_common::EXIT_CODE_SUCCESS);

    Ok(())
}

#[test]
fn invalid_utf8() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_INVALID_UTF8_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Standard JSON parsing: expected value",
    ));

    Ok(())
}

#[test]
fn stdin_missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--standard-json"];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Standard JSON parsing: EOF while parsing",
    ));

    Ok(())
}

#[test]
fn empty_sources() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_EMPTY_SOURCES_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("No input sources specified."));

    Ok(())
}

#[test]
fn missing_sources() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_MISSING_SOURCES_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Standard JSON parsing: missing field `sources`",
    ));

    Ok(())
}

#[test]
fn yul() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_SOLC_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn both_urls_and_content() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_SOLX_BOTH_URLS_AND_CONTENT_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Both `content` and `urls` cannot be set",
    ));

    Ok(())
}

#[test]
fn neither_urls_nor_content() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_SOLX_NEITHER_URLS_NOR_CONTENT_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Invalid input source specified."));

    Ok(())
}
