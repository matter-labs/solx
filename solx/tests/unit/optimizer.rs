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

    let build_unoptimized = crate::common::build_solidity_standard_json(
        sources.clone(),
        solx_solc::StandardJsonInputLibraries::default(),
        era_compiler_common::HashType::Keccak256,
        BTreeSet::new(),
        via_ir,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Build failure");
    let build_optimized_for_cycles = crate::common::build_solidity_standard_json(
        sources.clone(),
        solx_solc::StandardJsonInputLibraries::default(),
        era_compiler_common::HashType::Keccak256,
        BTreeSet::new(),
        via_ir,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Build failure");
    let build_optimized_for_size = crate::common::build_solidity_standard_json(
        sources,
        solx_solc::StandardJsonInputLibraries::default(),
        era_compiler_common::HashType::Keccak256,
        BTreeSet::new(),
        via_ir,
        era_compiler_llvm_context::OptimizerSettings::size(),
    )
    .expect("Build failure");

    let size_when_unoptimized = build_unoptimized
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
        .len();
    let size_when_optimized_for_cycles = build_optimized_for_cycles
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
        .len();
    let size_when_optimized_for_size = build_optimized_for_size
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
        .len();

    assert!(
        size_when_optimized_for_cycles < size_when_unoptimized,
        "Expected cycles-optimized bytecode to be smaller than unoptimized. Optimized: {size_when_optimized_for_cycles}B, Unoptimized: {size_when_unoptimized}B",
    );
    assert!(
        size_when_optimized_for_size < size_when_unoptimized,
        "Expected size-optimized bytecode to be smaller than unoptimized. Optimized: {size_when_optimized_for_size}B, Unoptimized: {size_when_unoptimized}B",
    );
}
