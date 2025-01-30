//!
//! Unit tests for the LLVM-based linker.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use test_case::test_case;

fn get_bytecode(
    path: &str,
    name: &str,
    libraries: solx_solc::StandardJsonInputLibraries,
    version: &semver::Version,
    via_ir: bool,
) -> Vec<u8> {
    let sources = crate::common::read_sources(&[path]);

    let build = crate::common::build_solidity_standard_json(
        sources,
        libraries,
        era_compiler_common::HashType::None,
        BTreeSet::new(),
        version,
        via_ir,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Build failure");
    let bytecode_hexadecimal = build
        .contracts
        .get(path)
        .expect("Missing file")
        .get(name)
        .expect("Missing contract")
        .evm
        .as_ref()
        .expect("Missing EVM data")
        .bytecode
        .as_ref()
        .expect("Missing bytecode")
        .object
        .as_str();
    hex::decode(bytecode_hexadecimal).expect("Invalid bytecode")
}

#[test_case(false)]
#[test_case(true)]
fn library_not_passed_compile_time(via_ir: bool) {
    let bytecode = get_bytecode(
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH,
        "SimpleContract",
        solx_solc::StandardJsonInputLibraries::default(),
        &solx_solc::Compiler::LAST_SUPPORTED_VERSION,
        via_ir,
    );

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    assert!(
        memory_buffer.is_elf_eravm(),
        "The bytecode is not an ELF file"
    );
}

#[test_case(false)]
#[test_case(true)]
fn library_not_passed_post_compile_time(via_ir: bool) {
    let bytecode = get_bytecode(
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH,
        "SimpleContract",
        solx_solc::StandardJsonInputLibraries::default(),
        &solx_solc::Compiler::LAST_SUPPORTED_VERSION,
        via_ir,
    );
    let full_path = format!(
        "{}:SimpleContract",
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH
    );
    let mut bytecodes = BTreeMap::new();
    bytecodes.insert(full_path.clone(), hex::encode(bytecode));

    let input = solx::LinkerInput::new(bytecodes, vec![]);
    let output = solx::Linker::link_eravm(input).expect("Linker failed");
    assert!(
        output.unlinked.contains_key(full_path.as_str()),
        "The bytecode is linked"
    );
}

#[test_case(false)]
#[test_case(true)]
fn library_passed_compile_time(via_ir: bool) {
    let libraries =
        vec!["tests/data/contracts/solidity/SimpleContract.sol:SimpleLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let libraries = solx_solc::StandardJsonInputLibraries::try_from(libraries.as_slice())
        .expect("Always valid");

    let bytecode = get_bytecode(
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH,
        "SimpleContract",
        libraries,
        &solx_solc::Compiler::LAST_SUPPORTED_VERSION,
        via_ir,
    );

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    assert!(!memory_buffer.is_elf_eravm(), "The bytecode is an ELF file");
}

#[test_case(false)]
#[test_case(true)]
fn library_passed_post_compile_time(via_ir: bool) {
    let libraries =
        vec!["tests/data/contracts/solidity/SimpleContract.sol:SimpleLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned()];

    let bytecode = get_bytecode(
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH,
        "SimpleContract",
        solx_solc::StandardJsonInputLibraries::default(),
        &solx_solc::Compiler::LAST_SUPPORTED_VERSION,
        via_ir,
    );
    let full_path = format!(
        "{}:SimpleContract",
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH
    );
    let mut bytecodes = BTreeMap::new();
    bytecodes.insert(full_path.clone(), hex::encode(bytecode));

    let input = solx::LinkerInput::new(bytecodes, libraries);
    let output = solx::Linker::link_eravm(input).expect("Linker failed");
    assert!(
        output.linked.contains_key(full_path.as_str()),
        "The bytecode is not linked"
    );
}

#[test_case(false)]
#[test_case(true)]
fn library_passed_post_compile_time_second_call(via_ir: bool) {
    let library_arguments =
        vec!["tests/data/contracts/solidity/SimpleContract.sol:SimpleLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let linker_symbols =
        solx_solc::StandardJsonInputLibraries::try_from(library_arguments.as_slice())
            .expect("Always valid")
            .as_linker_symbols()
            .expect("Always valid");

    let bytecode = get_bytecode(
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH,
        "SimpleContract",
        solx_solc::StandardJsonInputLibraries::default(),
        &solx_solc::Compiler::LAST_SUPPORTED_VERSION,
        via_ir,
    );

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    let memory_buffer_linked_empty = memory_buffer
        .link_module_eravm(&BTreeMap::new(), &BTreeMap::new())
        .expect("Link failure");
    let memory_buffer_linked = memory_buffer_linked_empty
        .link_module_eravm(&linker_symbols, &BTreeMap::new())
        .expect("Link failure");
    assert!(
        !memory_buffer_linked.is_elf_eravm(),
        "The bytecode is an ELF file"
    );
}

#[test_case(false)]
#[test_case(true)]
fn library_passed_post_compile_time_redundant_args(via_ir: bool) {
    let libraries = vec![
        "tests/data/contracts/solidity/fake.sol:Fake=0x0000000000000000000000000000000000000000".to_owned(),
        "tests/data/contracts/solidity/scam.sol:Scam=0x0000000000000000000000000000000000000000".to_owned(),
        "tests/data/contracts/solidity/SimpleContract.sol:SimpleLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned(),
    ];

    let bytecode = get_bytecode(
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH,
        "SimpleContract",
        solx_solc::StandardJsonInputLibraries::default(),
        &solx_solc::Compiler::LAST_SUPPORTED_VERSION,
        via_ir,
    );
    let full_path = format!(
        "{}:SimpleContract",
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH
    );
    let mut bytecodes = BTreeMap::new();
    bytecodes.insert(full_path.clone(), hex::encode(bytecode));

    let input = solx::LinkerInput::new(bytecodes, libraries);
    let output = solx::Linker::link_eravm(input).expect("Linker failed");
    assert!(
        output.linked.contains_key(full_path.as_str()),
        "The bytecode is not linked"
    );
}

#[test_case(false)]
#[test_case(true)]
#[should_panic(expected = "Input binary is not an EraVM ELF file")]
fn library_passed_post_compile_time_non_elf(via_ir: bool) {
    let library_arguments =
        vec!["tests/data/contracts/solidity/SimpleContract.sol:SimpleLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let libraries = solx_solc::StandardJsonInputLibraries::try_from(library_arguments.as_slice())
        .expect("Always valid")
        .as_linker_symbols()
        .expect("Always valid");

    let bytecode = get_bytecode(
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH,
        "SimpleContract",
        solx_solc::StandardJsonInputLibraries::default(),
        &solx_solc::Compiler::LAST_SUPPORTED_VERSION,
        via_ir,
    );

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    let memory_buffer_linked = memory_buffer
        .link_module_eravm(&libraries, &BTreeMap::new())
        .expect("Link failure");
    let _memory_buffer_linked_non_elf = memory_buffer_linked
        .link_module_eravm(&libraries, &BTreeMap::new())
        .expect("Link failure");
}

#[test_case(false)]
#[test_case(true)]
fn library_produce_equal_bytecode_in_both_cases(via_ir: bool) {
    let library_arguments =
        vec!["tests/data/contracts/solidity/SimpleContract.sol:SimpleLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let libraries = solx_solc::StandardJsonInputLibraries::try_from(library_arguments.as_slice())
        .expect("Always valid");
    let linker_symbols = libraries.as_linker_symbols().expect("Always valid");

    let bytecode_compile_time = get_bytecode(
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH,
        "SimpleContract",
        libraries,
        &solx_solc::Compiler::LAST_SUPPORTED_VERSION,
        via_ir,
    );
    let memory_buffer_compile_time = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode_compile_time.as_slice(),
        "bytecode_compile_time",
        false,
    );

    let bytecode_post_compile_time = get_bytecode(
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH,
        "SimpleContract",
        solx_solc::StandardJsonInputLibraries::default(),
        &solx_solc::Compiler::LAST_SUPPORTED_VERSION,
        via_ir,
    );
    let memory_buffer_post_compile_time =
        inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
            bytecode_post_compile_time.as_slice(),
            "bytecode_post_compile_time",
            false,
        );
    let memory_buffer_linked_post_compile_time = memory_buffer_post_compile_time
        .link_module_eravm(&linker_symbols, &BTreeMap::new())
        .expect("Link failure");

    assert!(
        memory_buffer_compile_time.as_slice() == memory_buffer_linked_post_compile_time.as_slice(),
        "The bytecodes are not equal"
    );
}

#[test_case(
    &[crate::common::TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_PATH],
    vec!["tests/data/contracts/solidity/LinkedMixedDeps.sol:UpperLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned()],
    false
)]
#[test_case(
    &[crate::common::TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_PATH],
    vec!["tests/data/contracts/solidity/LinkedMixedDeps.sol:UpperLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned()],
    true
)]
#[test_case(
    &[crate::common::TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_MULTI_LEVEL_PATH],
    vec![
        "tests/data/contracts/solidity/LinkedMixedDepsMultiLevel.sol:UpperLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned(),
        "tests/data/contracts/solidity/LinkedMixedDepsMultiLevel.sol:LowerLibrary=0x1234432112344321123443211234432112344321".to_owned(),
    ],
    false
)]
#[test_case(
    &[crate::common::TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_MULTI_LEVEL_PATH],
    vec![
        "tests/data/contracts/solidity/LinkedMixedDepsMultiLevel.sol:UpperLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned(),
        "tests/data/contracts/solidity/LinkedMixedDepsMultiLevel.sol:LowerLibrary=0x1234432112344321123443211234432112344321".to_owned(),
    ],
    true
)]
fn libraries_passed_post_compile_time_complex(
    sources: &[&str],
    libraries: Vec<String>,

    via_ir: bool,
) {
    let sources = crate::common::read_sources(sources);

    let build = crate::common::build_solidity_standard_json(
        sources,
        solx_solc::StandardJsonInputLibraries::default(),
        era_compiler_common::HashType::None,
        BTreeSet::new(),
        &solx_solc::Compiler::LAST_SUPPORTED_VERSION,
        via_ir,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Build failure");
    let bytecodes = build
        .contracts
        .into_iter()
        .map(|(path, contracts)| {
            contracts
                .into_iter()
                .map(|(name, contract)| {
                    let bytecode = contract
                        .evm
                        .expect("Missing EVM object")
                        .bytecode
                        .expect("Missing bytecode")
                        .object;
                    (format!("{path}:{name}"), bytecode)
                })
                .collect::<BTreeMap<String, String>>()
        })
        .flatten()
        .collect::<BTreeMap<String, String>>();

    let input = solx::LinkerInput::new(bytecodes, libraries);
    let output = solx::Linker::link_eravm(input).expect("Linker failed");
    assert!(!output.linked.is_empty(), "No linked objects found");
    assert!(
        !output.ignored.is_empty(),
        "No objects were linked at compile time"
    );
    assert!(output.unlinked.is_empty(), "Unlinked objects found");
}

#[test_case(
    &[crate::common::TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_PATH],
    false
)]
#[test_case(
    &[crate::common::TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_PATH],
    true
)]
#[test_case(
    &[crate::common::TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_MULTI_LEVEL_PATH],
    false
)]
#[test_case(
    &[crate::common::TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_MULTI_LEVEL_PATH],
    true
)]
fn libraries_not_passed_post_compile_time_complex(sources: &[&str], via_ir: bool) {
    let sources = crate::common::read_sources(sources);

    let build = crate::common::build_solidity_standard_json(
        sources,
        solx_solc::StandardJsonInputLibraries::default(),
        era_compiler_common::HashType::None,
        BTreeSet::new(),
        &solx_solc::Compiler::LAST_SUPPORTED_VERSION,
        via_ir,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Build failure");
    let bytecodes = build
        .contracts
        .into_iter()
        .map(|(path, contracts)| {
            contracts
                .into_iter()
                .map(|(name, contract)| {
                    let bytecode = contract
                        .evm
                        .expect("Missing EVM object")
                        .bytecode
                        .expect("Missing bytecode")
                        .object;
                    (format!("{path}:{name}"), bytecode)
                })
                .collect::<BTreeMap<String, String>>()
        })
        .flatten()
        .collect::<BTreeMap<String, String>>();

    let input = solx::LinkerInput::new(bytecodes, vec![]);
    let output = solx::Linker::link_eravm(input).expect("Linker failed");
    assert!(
        !output.ignored.is_empty(),
        "No objects were linked at compile time"
    );
    assert!(!output.unlinked.is_empty(), "No unlinked objects found");
}
