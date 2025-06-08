//!
//! Unit test common utilities.
//!

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod r#const;

pub use self::r#const::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Once;

use assert_cmd::Command;

use solx::project::Project;
use solx_standard_json::CollectableError;

/// Shared lock for unit tests, as `solc` libraries are not thread-safe.
pub static UNIT_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

///
/// Setup required test dependencies.
///
pub fn setup() -> anyhow::Result<()> {
    // Set the `solx` binary path
    let solx_bin = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let _ = solx::process::EXECUTABLE.set(PathBuf::from(solx_bin.get_program()));

    // Enable LLVM pretty stack trace
    inkwell::support::enable_llvm_pretty_stack_trace();

    Ok(())
}

///
/// Reads source code files from the disk.
///
pub fn read_sources(paths: &[&str]) -> BTreeMap<String, String> {
    paths
        .iter()
        .map(|path| {
            let result = std::fs::read_to_string(path).map_err(|error| anyhow::anyhow!(error));
            result.map(|result| ((*path).to_owned(), result))
        })
        .collect::<anyhow::Result<BTreeMap<String, String>>>()
        .expect("Source reading failure")
}

///
/// Builds the Solidity project and returns the standard JSON output.
///
pub fn build_solidity_standard_json(
    sources: BTreeMap<String, String>,
    libraries: era_compiler_common::Libraries,
    metadata_hash_type: era_compiler_common::EVMMetadataHashType,
    remappings: BTreeSet<String>,
    via_ir: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<solx_standard_json::Output> {
    self::setup()?;

    let solc_compiler = solx_solc::Compiler::default();

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EVM);

    let sources: BTreeMap<String, solx_standard_json::InputSource> = sources
        .into_iter()
        .map(|(path, source)| (path, solx_standard_json::InputSource::from(source)))
        .collect();

    let mut selectors = BTreeSet::new();
    selectors.insert(solx_standard_json::InputSelector::BytecodeObject);
    selectors.insert(solx_standard_json::InputSelector::BytecodeLinkReferences);
    selectors.insert(solx_standard_json::InputSelector::BytecodeOpcodes);
    selectors.insert(solx_standard_json::InputSelector::BytecodeLLVMAssembly);
    selectors.insert(solx_standard_json::InputSelector::BytecodeSourceMap);
    selectors.insert(solx_standard_json::InputSelector::BytecodeGeneratedSources);
    selectors.insert(solx_standard_json::InputSelector::BytecodeFunctionDebugData);
    selectors.insert(solx_standard_json::InputSelector::RuntimeBytecodeObject);
    selectors.insert(solx_standard_json::InputSelector::RuntimeBytecodeLinkReferences);
    selectors.insert(solx_standard_json::InputSelector::RuntimeBytecodeImmutableReferences);
    selectors.insert(solx_standard_json::InputSelector::RuntimeBytecodeOpcodes);
    selectors.insert(solx_standard_json::InputSelector::RuntimeBytecodeLLVMAssembly);
    selectors.insert(solx_standard_json::InputSelector::RuntimeBytecodeSourceMap);
    selectors.insert(solx_standard_json::InputSelector::RuntimeBytecodeGeneratedSources);
    selectors.insert(solx_standard_json::InputSelector::RuntimeBytecodeFunctionDebugData);
    selectors.insert(solx_standard_json::InputSelector::AST);
    selectors.insert(solx_standard_json::InputSelector::ABI);
    selectors.insert(solx_standard_json::InputSelector::Metadata);
    selectors.insert(solx_standard_json::InputSelector::DeveloperDocumentation);
    selectors.insert(solx_standard_json::InputSelector::UserDocumentation);
    selectors.insert(solx_standard_json::InputSelector::StorageLayout);
    selectors.insert(solx_standard_json::InputSelector::TransientStorageLayout);
    selectors.insert(solx_standard_json::InputSelector::MethodIdentifiers);
    selectors.insert(if via_ir {
        solx_standard_json::InputSelector::Yul
    } else {
        solx_standard_json::InputSelector::EVMLegacyAssembly
    });
    let output_selection = solx_standard_json::InputSelection::new(selectors);

    let mut input = solx_standard_json::Input::try_from_solidity_sources(
        sources,
        libraries.clone(),
        remappings,
        solx_standard_json::InputOptimizer::default(),
        None,
        via_ir,
        &output_selection,
        solx_standard_json::InputMetadata::default(),
        vec![],
    )?;

    let mut output = {
        let _lock = UNIT_TEST_LOCK.lock();
        solc_compiler.standard_json(&mut input, &mut vec![], true, None, &[], None)
    }?;
    output.check_errors()?;

    let linker_symbols = libraries.as_linker_symbols()?;

    let project = Project::try_from_solc_output(libraries, via_ir, &mut output, None)?;
    output.check_errors()?;

    let build = project.compile_to_evm(
        &mut vec![],
        &input.settings.output_selection,
        metadata_hash_type,
        optimizer_settings,
        None,
        vec![],
        None,
    )?;
    build.check_errors()?;

    let cbor_data = vec![
        (
            solx::DEFAULT_EXECUTABLE_NAME.to_owned(),
            solx::version().parse().expect("Always valid"),
        ),
        (
            solx::SOLC_PRODUCTION_NAME.to_owned(),
            solc_compiler.version.default.to_owned(),
        ),
        (
            solx::SOLC_LLVM_REVISION_METADATA_TAG.to_owned(),
            solc_compiler.version.llvm_revision.to_owned(),
        ),
    ];

    let mut build = if input.settings.output_selection.is_bytecode_set_for_any() {
        build.link(linker_symbols, Some(cbor_data))
    } else {
        build
    };
    build.write_to_standard_json(&mut output, &input.settings.output_selection, true)?;
    output.check_errors()?;
    Ok(output)
}

///
/// Builds the Yul standard JSON and returns the standard JSON output.
///
/// If `solc_compiler` is set, the standard JSON is validated with `solc`.
///
pub fn build_yul_standard_json(
    mut input: solx_standard_json::Input,
) -> anyhow::Result<solx_standard_json::Output> {
    self::setup()?;

    let solc_compiler = solx_solc::Compiler::default();

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EVM);

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        input.settings.optimizer.mode.unwrap_or_else(|| {
            solx_standard_json::InputOptimizer::default_mode().expect("Always exists")
        }),
    )?;

    let mut solc_output = {
        let _lock = UNIT_TEST_LOCK.lock();
        solc_compiler.validate_yul_standard_json(&mut input, &mut vec![])
    }?;

    let project = Project::try_from_yul_sources(
        input.sources,
        era_compiler_common::Libraries::default(),
        &input.settings.output_selection,
        Some(&mut solc_output),
        None,
    )?;
    let build = project.compile_to_evm(
        &mut vec![],
        &input.settings.output_selection,
        era_compiler_common::EVMMetadataHashType::IPFS,
        optimizer_settings,
        None,
        vec![],
        None,
    )?;
    build.check_errors()?;

    let cbor_data = vec![
        (
            solx::DEFAULT_EXECUTABLE_NAME.to_owned(),
            solx::version().parse().expect("Always valid"),
        ),
        (
            solx::SOLC_PRODUCTION_NAME.to_owned(),
            solc_compiler.version.default.to_owned(),
        ),
        (
            solx::SOLC_LLVM_REVISION_METADATA_TAG.to_owned(),
            solc_compiler.version.llvm_revision.to_owned(),
        ),
    ];

    let mut build = if input.settings.output_selection.is_bytecode_set_for_any() {
        build.link(BTreeMap::new(), Some(cbor_data))
    } else {
        build
    };
    build.write_to_standard_json(&mut solc_output, &input.settings.output_selection, true)?;
    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the LLVM IR standard JSON and returns the standard JSON output.
///
pub fn build_llvm_ir_standard_json(
    input: solx_standard_json::Input,
) -> anyhow::Result<solx_standard_json::Output> {
    self::setup()?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EVM);

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        input.settings.optimizer.mode.unwrap_or_else(|| {
            solx_standard_json::InputOptimizer::default_mode().expect("Always exists")
        }),
    )?;

    let mut output = solx_standard_json::Output::new(&BTreeMap::new(), &mut vec![]);

    let project = Project::try_from_llvm_ir_sources(
        input.sources,
        input.settings.libraries,
        &input.settings.output_selection,
        Some(&mut output),
    )?;
    let build = project.compile_to_evm(
        &mut vec![],
        &input.settings.output_selection,
        era_compiler_common::EVMMetadataHashType::IPFS,
        optimizer_settings,
        None,
        vec![],
        None,
    )?;
    build.check_errors()?;

    let cbor_data = vec![(
        solx::DEFAULT_EXECUTABLE_NAME.to_owned(),
        solx::version().parse().expect("Always valid"),
    )];

    let mut build = if input.settings.output_selection.is_bytecode_set_for_any() {
        build.link(BTreeMap::new(), Some(cbor_data))
    } else {
        build
    };
    build.write_to_standard_json(&mut output, &input.settings.output_selection, true)?;
    output.check_errors()?;
    Ok(output)
}
