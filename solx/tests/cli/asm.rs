//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;
// use test_case::test_case;

// TODO: fix when assembler is done
// #[test_case("cannot parse operand")]
// fn default(pattern: &str) -> anyhow::Result<()> {
//     crate::common::setup()?;

//     let args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "--asm"];
//     let invalid_args = &["--asm"];

//     // Valid command
//     let result = crate::cli::execute_solx(args)?;
//     let result_status_code = result
//         .success()
//         .stdout(predicate::str::contains(pattern))
//         .get_output()
//         .status
//         .code()
//         .expect("No exit code.");

//     // solc exit code == solx exit code
//     let solc_result = crate::cli::execute_solc(args)?;
//     solc_result.code(result_status_code);

//     // Run invalid: solx --asm
//     let invalid_result = crate::cli::execute_solx(invalid_args)?;
//     let invalid_result_status_code = invalid_result
//         .failure()
//         .stderr(
//             predicate::str::contains("No input sources specified")
//                 .or(predicate::str::contains("Compilation aborted")),
//         )
//         .get_output()
//         .status
//         .code()
//         .expect("No exit code.");

//     // Invalid solc exit code == Invalid solx exit code
//     let invalid_solc_result = crate::cli::execute_solc(invalid_args)?;
//     invalid_solc_result.code(invalid_result_status_code);

//     Ok(())
// }

#[test]
fn invalid_input() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[crate::common::TEST_YUL_CONTRACT_PATH, "--asm"];

    let result = crate::cli::execute_solx(args)?;
    let solc_result = crate::cli::execute_solc(args)?;

    let result_exit_code = result
        .failure()
        .stderr(predicate::str::contains(
            "Expected identifier but got 'StringLiteral'",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");
    solc_result.code(result_exit_code);

    Ok(())
}

#[test]
fn combined_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--asm",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "asm",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.failure().stderr(predicate::str::contains(
        "Cannot output data outside of JSON in combined JSON mode.",
    ));

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--asm",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Cannot output data outside of JSON in standard JSON mode.",
    ));

    Ok(())
}
