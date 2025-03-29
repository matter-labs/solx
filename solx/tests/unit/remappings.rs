//!
//! Unit tests for remappings.
//!

use std::collections::BTreeSet;

use test_case::test_case;

#[test_case(false)]
#[test_case(true)]
fn default(via_ir: bool) {
    let sources = crate::common::read_sources(&[
        crate::common::TEST_SOLIDITY_CONTRACT_CALLER_MAIN_PATH,
        crate::common::TEST_SOLIDITY_CONTRACT_CALLER_CALLABLE_PATH,
    ]);

    let mut remappings = BTreeSet::new();
    remappings.insert("libraries/default/=./".to_owned());

    crate::common::build_solidity_standard_json(
        sources,
        era_compiler_common::Libraries::default(),
        era_compiler_common::HashType::Keccak256,
        remappings,
        via_ir,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}
