//!
//! Unit tests for IR artifacts.
//!
//! The tests check if the IR artifacts are kept in the final output.
//!

use std::collections::BTreeSet;

#[test]
fn evmla() {
    let sources = crate::common::read_sources(&[crate::common::TEST_SOLIDITY_CONTRACT_PATH]);

    let build = crate::common::build_solidity_standard_json(
        sources,
        era_compiler_common::Libraries::default(),
        era_compiler_common::EVMMetadataHashType::IPFS,
        BTreeSet::new(),
        false,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
    assert!(
        !build
            .contracts
            .get(crate::common::TEST_SOLIDITY_CONTRACT_PATH)
            .expect("Always exists")
            .get("Test")
            .expect("Always exists")
            .evm
            .as_ref()
            .expect("EVM object is missing")
            .legacy_assembly
            .is_null(),
        "EVM assembly is missing",
    );
    assert!(
        build
            .contracts
            .get(crate::common::TEST_SOLIDITY_CONTRACT_PATH)
            .expect("Always exists")
            .get("Test")
            .expect("Always exists")
            .ir_optimized
            .is_empty(),
        "Yul is present although not requested",
    );
}

#[test]
fn yul() {
    let sources = crate::common::read_sources(&[crate::common::TEST_SOLIDITY_CONTRACT_PATH]);

    let build = crate::common::build_solidity_standard_json(
        sources,
        era_compiler_common::Libraries::default(),
        era_compiler_common::EVMMetadataHashType::IPFS,
        BTreeSet::new(),
        true,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");

    assert!(
        !build
            .contracts
            .get(crate::common::TEST_SOLIDITY_CONTRACT_PATH)
            .expect("Always exists")
            .get("Test")
            .expect("Always exists")
            .ir_optimized
            .is_empty(),
        "Yul is missing"
    );
    assert!(
        build
            .contracts
            .get(crate::common::TEST_SOLIDITY_CONTRACT_PATH)
            .expect("Always exists")
            .get("Test")
            .expect("Always exists")
            .evm
            .as_ref()
            .expect("EVM object is missing")
            .legacy_assembly
            .is_null(),
        "EVM assembly is present although not requested"
    );
}

#[test]
fn yul_empty_solidity_interface() {
    let sources = crate::common::read_sources(&[
        crate::common::TEST_SOLIDITY_CONTRACT_INTERFACE_EMPTY_YUL_PATH,
    ]);

    let build = crate::common::build_solidity_standard_json(
        sources,
        era_compiler_common::Libraries::default(),
        era_compiler_common::EVMMetadataHashType::IPFS,
        BTreeSet::new(),
        true,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");

    assert_eq!(build.contracts.len(), 1, "More than one Yul object present");
}
