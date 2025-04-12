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
        .into_iter()
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

    let mut solc_input = solx_standard_json::Input::try_from_solidity_sources(
        sources,
        libraries.clone(),
        remappings,
        solx_standard_json::InputOptimizer::default(),
        None,
        via_ir,
        solx_standard_json::InputSelection::new_compilation(true, true, Some(via_ir)),
        solx_standard_json::InputMetadata::default(),
        vec![],
    )?;

    let mut solc_output = {
        let _lock = UNIT_TEST_LOCK.lock();
        solc_compiler.standard_json(&mut solc_input, &mut vec![], None, vec![], None)
    }?;
    solc_output.check_errors()?;

    let linker_symbols = libraries.as_linker_symbols()?;

    let project = Project::try_from_solc_output(libraries, via_ir, &mut solc_output, None)?;
    solc_output.check_errors()?;

    let build = project.compile_to_evm(
        &mut vec![],
        true,
        metadata_hash_type,
        optimizer_settings,
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

    let build = build.link(linker_symbols, Some(cbor_data));
    build.write_to_standard_json(&mut solc_output)?;
    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the Yul standard JSON and returns the standard JSON output.
///
/// If `solc_compiler` is set, the standard JSON is validated with `solc`.
///
pub fn build_yul_standard_json(
    mut solc_input: solx_standard_json::Input,
) -> anyhow::Result<solx_standard_json::Output> {
    self::setup()?;

    let solc_compiler = solx_solc::Compiler::default();

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EVM);

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode,
    )?;

    let mut solc_output = {
        let _lock = UNIT_TEST_LOCK.lock();
        solc_compiler.validate_yul_standard_json(&mut solc_input, &mut vec![])
    }?;

    let project = Project::try_from_yul_sources(
        solc_input.sources,
        era_compiler_common::Libraries::default(),
        &solc_input.settings.output_selection,
        Some(&mut solc_output),
        None,
    )?;
    let build = project.compile_to_evm(
        &mut vec![],
        true,
        era_compiler_common::EVMMetadataHashType::IPFS,
        optimizer_settings,
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

    let build = build.link(BTreeMap::new(), Some(cbor_data));
    build.write_to_standard_json(&mut solc_output)?;
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

    let optimizer_settings =
        era_compiler_llvm_context::OptimizerSettings::try_from_cli(input.settings.optimizer.mode)?;

    let mut output = solx_standard_json::Output::new(&BTreeMap::new(), &mut vec![]);

    let project = Project::try_from_llvm_ir_sources(
        input.sources,
        input.settings.libraries,
        &input.settings.output_selection,
        Some(&mut output),
    )?;
    let build = project.compile_to_evm(
        &mut vec![],
        true,
        era_compiler_common::EVMMetadataHashType::IPFS,
        optimizer_settings,
        vec![],
        None,
    )?;
    build.check_errors()?;

    let cbor_data = vec![(
        solx::DEFAULT_EXECUTABLE_NAME.to_owned(),
        solx::version().parse().expect("Always valid"),
    )];

    let build = build.link(BTreeMap::new(), Some(cbor_data));
    build.write_to_standard_json(&mut output)?;
    output.check_errors()?;
    Ok(output)
}
