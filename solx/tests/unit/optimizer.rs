//!
//! Unit tests for the optimizer.
//!

use std::collections::BTreeSet;

use test_case::test_case;

#[test_case(false)]
#[test_case(true)]
fn default(via_ir: bool) {
    let sources =
        crate::common::read_sources(&[crate::common::TEST_SOLIDITY_CONTRACT_OPTIMIZED_PATH]);

    let build_optimized_for_cycles = crate::common::build_solidity_standard_json(
        sources.clone(),
        era_compiler_common::Libraries::default(),
        era_compiler_common::EVMMetadataHashType::IPFS,
        BTreeSet::new(),
        via_ir,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Build failure");
    let build_optimized_for_size = crate::common::build_solidity_standard_json(
        sources,
        era_compiler_common::Libraries::default(),
        era_compiler_common::EVMMetadataHashType::IPFS,
        BTreeSet::new(),
        via_ir,
        era_compiler_llvm_context::OptimizerSettings::size(),
    )
    .expect("Build failure");

    let optimized_for_gas = build_optimized_for_cycles
        .contracts
        .get(crate::common::TEST_SOLIDITY_CONTRACT_OPTIMIZED_PATH)
        .expect("Missing file")
        .get("Optimized")
        .expect("Missing contract")
        .evm
        .as_ref()
        .expect("Missing EVM data")
        .bytecode
        .as_ref()
        .expect("Missing bytecode")
        .object
        .as_bytes();
    let optimized_for_size = build_optimized_for_size
        .contracts
        .get(crate::common::TEST_SOLIDITY_CONTRACT_OPTIMIZED_PATH)
        .expect("Missing file")
        .get("Optimized")
        .expect("Missing contract")
        .evm
        .as_ref()
        .expect("Missing EVM data")
        .bytecode
        .as_ref()
        .expect("Missing bytecode")
        .object
        .as_bytes();

    assert!(
        optimized_for_gas != optimized_for_size,
        "Expected gas-optimized bytecode to be different from size-optimized. Gas-optimized: {optimized_for_gas:?}, size-optimized: {optimized_for_size:?}",
    );
}
