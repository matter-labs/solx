//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

// TODO: fix when Yul is fixed
// #[test]
// fn default() -> anyhow::Result<()> {
//     crate::common::setup()?;

//     let args = &[crate::common::TEST_YUL_CONTRACT_PATH, "--yul"];

//     let result = crate::cli::execute_solx(args)?;
//     result.success().stderr(predicate::str::contains(
//         "Compiler run successful. No output requested",
//     ));

//     Ok(())
// }

// TODO: fix when Yul is fixed
// #[test]
// fn solc() -> anyhow::Result<()> {
//     crate::common::setup()?;

//     let args = &[crate::common::TEST_YUL_CONTRACT_PATH, "--yul"];

//     let result = crate::cli::execute_solx(args)?;
//     result.success().stderr(predicate::str::contains(
//         "Compiler run successful. No output requested",
//     ));

//     Ok(())
// }

// TODO: fix when Yul is fixed
// #[test]
// fn invalid_input() -> anyhow::Result<()> {
//     crate::common::setup()?;

//     let args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "--yul"];

//     let result = crate::cli::execute_solx(args)?;
//     let solx_status = result
//         .failure()
//         .stderr(predicate::str::contains("Yul parsing"))
//         .get_output()
//         .status
//         .code()
//         .expect("No exit code.");

//     let solc_result = crate::cli::execute_solc(args)?;
//     solc_result.code(solx_status);

//     Ok(())
// }

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--standard-json",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Only one mode is allowed at the same time:",
    ));

    Ok(())
}

// TODO: fix when Yul is fixed
// #[test]
// fn invalid_solc_error() -> anyhow::Result<()> {
//     crate::common::setup()?;

//     let args = &[
//         "--standard-json",
//         crate::common::TEST_YUL_STANDARD_JSON_SOLC_INVALID_PATH,
//     ];

//     let result = crate::cli::execute_solx(args)?;
//     result.success().stdout(predicate::str::contains(
//         "DeclarationError: Function \\\"mdelete\\\" not found.",
//     ));

//     Ok(())
// }
