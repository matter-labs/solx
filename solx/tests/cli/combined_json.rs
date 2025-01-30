//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

const JSON_ARGS: &[&str] = &[
    "abi",
    "hashes",
    "metadata",
    "devdoc",
    "userdoc",
    "storage-layout",
    "transient-storage-layout",
    "ast",
    "asm",
    "bin",
    "bin-runtime",
];

#[test]
fn all() -> anyhow::Result<()> {
    crate::common::setup()?;

    for selector in JSON_ARGS.into_iter() {
        let args = &[
            crate::common::TEST_SOLIDITY_CONTRACT_PATH,
            "--combined-json",
            selector,
        ];

        let result = crate::cli::execute_solx(args)?;
        let status_code = result
            .success()
            .stdout(predicate::str::contains("contracts"))
            .get_output()
            .status
            .code()
            .expect("No exit code.");

        let solc_result = crate::cli::execute_solc(args)?;
        solc_result.code(status_code);
    }

    Ok(())
}

#[test]
fn all_yul() -> anyhow::Result<()> {
    crate::common::setup()?;

    for selector in JSON_ARGS.into_iter() {
        let args = &[
            crate::common::TEST_YUL_CONTRACT_PATH,
            "--combined-json",
            selector,
        ];

        let result = crate::cli::execute_solx(args)?;
        let status_code = result
            .failure()
            .stderr(predicate::str::contains("Expected identifier"))
            .get_output()
            .status
            .code()
            .expect("No exit code.");

        let solc_result = crate::cli::execute_solc(args)?;
        solc_result.code(status_code);
    }

    Ok(())
}

#[test]
fn two_files() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        crate::common::TEST_SOLIDITY_CONTRACT_GREETER_PATH,
        "--combined-json",
        "bin",
    ];

    let result = crate::cli::execute_solx(args)?;
    let status_code = result
        .success()
        .stdout(
            predicate::str::is_match([r#""bin":"[0-9a-f]*""#; 2].join(".*")).expect("Always valid"),
        )
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(args)?;
    solc_result.code(status_code);

    Ok(())
}

#[test]
fn invalid_path() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "unknown",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stderr(predicate::str::contains(
        "The selector `unknown` is not supported, and therefore ignored.",
    ));

    Ok(())
}

#[test]
fn invalid_input() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&solx_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let selector = solx_solc::CombinedJsonSelector::Bytecode.to_string();
    let args = &[
        "--solc",
        solc_compiler.as_str(),
        crate::common::TEST_BROKEN_INPUT_PATH,
        "--combined-json",
        selector.as_str(),
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("subprocess failed with exit code"));

    Ok(())
}

#[test]
fn invalid_output() -> anyhow::Result<()> {
    crate::common::setup()?;

    let selector = solx_solc::CombinedJsonSelector::Bytecode.to_string();
    let args = &[
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_INVALID_OUTPUT_JSON,
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        selector.as_str(),
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("subprocess stdout parsing:"));

    Ok(())
}
