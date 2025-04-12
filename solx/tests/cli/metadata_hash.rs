//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::EVMMetadataHashType;
use predicates::prelude::*;

#[test]
fn none() -> anyhow::Result<()> {
    crate::common::setup()?;

    let hash_type = EVMMetadataHashType::None.to_string();
    let args = &[
        "--metadata-hash",
        hash_type.as_str(),
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains("a164"));

    Ok(())
}

#[test]
fn ipfs() -> anyhow::Result<()> {
    crate::common::setup()?;

    let hash_type = EVMMetadataHashType::IPFS.to_string();
    let args = &[
        "--metadata-hash",
        hash_type.as_str(),
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains("a264"));

    Ok(())
}

#[test]
fn standard_json_cli_excess_arg() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--metadata-hash",
        "ipfs",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Metadata hash mode must be specified in standard JSON input settings.",
    ));

    Ok(())
}
