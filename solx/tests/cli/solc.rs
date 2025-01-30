//!
//! CLI tests for the eponymous option.
//!

use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&solx_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        solc_compiler.as_str(),
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test]
fn no_solc() -> anyhow::Result<()> {
    crate::common::setup()?;

    let mut solx = Command::cargo_bin(solx::DEFAULT_EXECUTABLE_NAME)?;

    let result = solx
        .arg(crate::common::TEST_SOLIDITY_CONTRACT_PATH)
        .env("PATH", "./solc-bin")
        .assert();
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&solx_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
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
fn standard_json_no_solc() -> anyhow::Result<()> {
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
fn llvm_ir() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&solx_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--llvm-ir",
        "--bin",
        crate::common::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "Using `solc` is only allowed in Solidity and Yul modes.",
    ));

    Ok(())
}

#[test]
fn invalid_path() -> anyhow::Result<()> {
    crate::common::setup()?;

    let path = "solc-not-found";
    let args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "--solc", path];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        format!("The `{path}` executable not found:").as_str(),
    ));

    Ok(())
}

#[test]
fn error_version_missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_OUTPUT_ERROR_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("version getting:"));

    Ok(())
}

#[test]
fn error_version_too_old() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_TOO_OLD_PATH,
    ];

    let version_supported = solx_solc::Compiler::FIRST_SUPPORTED_VERSION;
    let mut version_not_supported = version_supported.clone();
    version_not_supported.patch -= 1;

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains(format!("versions older than {version_supported} are not supported, found {version_not_supported}.").as_str()));

    Ok(())
}

#[test]
fn error_version_too_recent() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_TOO_NEW_PATH,
    ];

    let version_supported = solx_solc::Compiler::LAST_SUPPORTED_VERSION;
    let mut version_not_supported = version_supported.clone();
    version_not_supported.patch += 1;

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains(format!("versions newer than {version_supported} are not supported, found {version_not_supported}.").as_str()));

    Ok(())
}

#[test]
fn error_version_not_enough_lines() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_NOT_ENOUGH_LINES_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "version parsing: not enough lines.",
    ));

    Ok(())
}

#[test]
fn error_version_not_enough_words_in_2nd_line() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_NOT_ENOUGH_WORDS_IN_2ND_LINE_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "version parsing: not enough words in the 2nd line.",
    ));

    Ok(())
}

#[test]
fn error_version_parsing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_PARSING_ERROR_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("version parsing:"));

    Ok(())
}

#[test]
fn zksync_revision_missing_version() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_ZKSYNC_REVISION_MISSING_VERSION,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "ZKsync revision parsing: missing version.",
    ));

    Ok(())
}

#[test]
fn zksync_revision_missing_revision() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_ZKSYNC_REVISION_MISSING_REVISION,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "ZKsync revision parsing: missing revision.",
    ));

    Ok(())
}

#[test]
fn zksync_revision_parsing_revision_error() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_ZKSYNC_REVISION_PARSING_REVISION_ERROR,
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("ZKsync revision parsing:"));

    Ok(())
}

#[test]
fn exit_code_failed() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_EXIT_CODE_FAILED,
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("subprocess failed with exit code"));

    Ok(())
}

#[test]
fn invalid_output_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_INVALID_OUTPUT_JSON,
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("subprocess stdout parsing"));

    Ok(())
}
