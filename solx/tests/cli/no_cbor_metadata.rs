//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::EVMMetadataHashType;
use predicates::prelude::*;

#[test]
fn none() -> anyhow::Result<()> {
    let _ = crate::common::setup();

    let hash_type = EVMMetadataHashType::None.to_string();
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
        "--no-cbor-metadata",
        "--bin",
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary"))
        .stdout(predicate::str::contains("a165").not())
        .stdout(predicate::str::ends_with("0023").not());

    Ok(())
}

#[test]
fn ipfs_solidity() -> anyhow::Result<()> {
    let _ = crate::common::setup();

    let hash_type = EVMMetadataHashType::IPFS.to_string();
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
        "--no-cbor-metadata",
        "--bin",
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary"))
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("0055").not());

    Ok(())
}

#[test]
fn ipfs_yul() -> anyhow::Result<()> {
    let _ = crate::common::setup();

    let hash_type = EVMMetadataHashType::IPFS.to_string();
    let args = &[
        "--yul",
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
        "--no-cbor-metadata",
        "--bin",
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary"))
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("003e").not());

    Ok(())
}

#[test]
fn ipfs_llvm_ir() -> anyhow::Result<()> {
    let _ = crate::common::setup();

    let hash_type = EVMMetadataHashType::IPFS.to_string();
    let args = &[
        "--llvm-ir",
        crate::common::TEST_LLVM_IR_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
        "--no-cbor-metadata",
        "--bin",
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary"))
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("003e").not());

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--standard-json", crate::common::TEST_JSON_NO_CBOR_METADATA];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("0055").not());

    Ok(())
}
