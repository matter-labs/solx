//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;
// use test_case::test_case;

/// TODO: when LLVM IR is replaced
// #[test_case(crate::common::TEST_LLVM_IR_CONTRACT_PATH)]
// fn default(path: &str) -> anyhow::Result<()> {
//     crate::common::setup()?;
//     let args = &[path, "--llvm-ir"];

//     let result = crate::cli::execute_solx(args)?;
//     result.success().stderr(predicate::str::contains(
//         "Compiler run successful. No output requested.",
//     ));

//     Ok(())
// }

#[test]
fn invalid_input_text() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--llvm-ir", crate::common::TEST_BROKEN_INPUT_PATH];

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("error: expected top-level entity"));

    Ok(())
}

#[test]
fn invalid_input_solidity() -> anyhow::Result<()> {
    crate::common::setup()?;
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--llvm-ir",
        "--bin",
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("expected top-level entity"));

    Ok(())
}

#[test]
fn invalid_input_llvm_ir() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--llvm-ir",
        crate::common::TEST_LLVM_IR_CONTRACT_INVALID_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "error: use of undefined value \'%runtime\'",
    ));

    Ok(())
}

#[test]
fn missing_file() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--llvm-ir", "--bin", crate::common::TEST_NON_EXISTENT_PATH];

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("reading:"));

    Ok(())
}

/// TODO: when linker is implemented
// #[test]
// fn linker_error() -> anyhow::Result<()> {
//     crate::common::setup()?;

//     let args = &[
//         "--llvm-ir",
//         crate::common::TEST_LLVM_IR_CONTRACT_LINKER_ERROR_PATH,
//     ];

//     let result = crate::cli::execute_solx(args)?;
//     result.failure().stderr(predicate::str::contains(
//         "ld.lld: error: undefined symbol: foo",
//     ));

//     Ok(())
// }

#[test]
fn excess_mode_combined_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_LLVM_IR_CONTRACT_PATH,
        "--llvm-ir",
        "--combined-json",
        "anyarg",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "Only one mode is allowed at the same time",
    ));

    Ok(())
}

#[test]
fn excess_mode_standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_LLVM_IR_CONTRACT_PATH,
        "--llvm-ir",
        "--standard-json",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Only one mode is allowed at the same time",
    ));

    Ok(())
}

#[test]
fn standard_json_invalid() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_LLVM_IR_STANDARD_JSON_INVALID_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("error: use of undefined value"));

    Ok(())
}

#[test]
fn standard_json_missing_file() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_LLVM_IR_STANDARD_JSON_MISSING_FILE_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Error: File \\\"tests/data/contracts/llvm_ir/Missing.ll\\\" reading:",
    ));

    Ok(())
}

#[test]
fn standard_json_excess_solc() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&solx_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        crate::common::TEST_LLVM_IR_STANDARD_JSON_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "LLVM IR projects cannot be compiled with `solc`.",
    ));

    Ok(())
}
