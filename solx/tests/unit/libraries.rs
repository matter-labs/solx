//!
//! Unit tests for libraries.
//!

use std::collections::BTreeSet;

use test_case::test_case;

#[test_case(false)]
#[test_case(true)]
fn not_specified(via_ir: bool) {
    let sources =
        crate::common::read_sources(&[crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH]);

    let output = crate::common::build_solidity_standard_json(
        sources,
        era_compiler_common::Libraries::default(),
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        via_ir,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");

    assert!(
        output
            .contracts
            .get(crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH)
            .expect("Always exists")
            .get("SimpleContract")
            .expect("Always exists")
            .evm
            .as_ref()
            .expect("Always exists")
            .deployed_bytecode
            .as_ref()
            .expect("Always exists")
            .unlinked_references
            .contains(
                format!(
                    "{}:SimpleLibrary",
                    crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH
                )
                .as_str()
            ),
        "Missing library not detected"
    );
}

#[test_case(false)]
#[test_case(true)]
fn specified(via_ir: bool) {
    let sources =
        crate::common::read_sources(&[crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH]);

    let mut libraries = era_compiler_common::Libraries::default();
    libraries
        .as_inner_mut()
        .entry(crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH.to_string())
        .or_default()
        .entry("SimpleLibrary".to_string())
        .or_insert("0x00000000000000000000000000000000DEADBEEF".to_string());

    let output = crate::common::build_solidity_standard_json(
        sources,
        libraries,
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        via_ir,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
    assert!(
        output
            .contracts
            .get(crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH)
            .expect("Always exists")
            .get("SimpleContract")
            .expect("Always exists")
            .evm
            .as_ref()
            .expect("Always exists")
            .deployed_bytecode
            .as_ref()
            .expect("Always exists")
            .unlinked_references
            .is_empty(),
        "The list of unlinked libraries must be empty"
    );
}
