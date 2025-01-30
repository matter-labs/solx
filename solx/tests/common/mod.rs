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
use solx_solc::CollectableError;

///
/// Setup required test dependencies.
///
pub fn setup() -> anyhow::Result<()> {
    // Set the `solx` binary path
    let solx_bin = Command::cargo_bin(solx::DEFAULT_EXECUTABLE_NAME)?;
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
    libraries: solx_solc::StandardJsonInputLibraries,
    metadata_hash_type: era_compiler_common::HashType,
    remappings: BTreeSet<String>,
    via_ir: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<solx_solc::StandardJsonOutput> {
    self::setup()?;

    let solc_compiler = solx_solc::Compiler::default();

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EVM);

    let sources: BTreeMap<String, solx_solc::StandardJsonInputSource> = sources
        .into_iter()
        .map(|(path, source)| (path, solx_solc::StandardJsonInputSource::from(source)))
        .collect();

    let mut solc_input = solx_solc::StandardJsonInput::try_from_solidity_sources(
        sources,
        libraries.clone(),
        remappings,
        solx_solc::StandardJsonInputOptimizer::default(),
        None,
        via_ir,
        solx_solc::StandardJsonInputSelection::new_required(via_ir),
        solx_solc::StandardJsonInputMetadata::default(),
        vec![],
    )?;

    let mut solc_output =
        solc_compiler.standard_json(&mut solc_input, &mut vec![], None, vec![], None)?;
    solc_output.check_errors()?;

    let linker_symbols = libraries.as_linker_symbols()?;

    let project = Project::try_from_solc_output(libraries, via_ir, &mut solc_output, None)?;
    solc_output.check_errors()?;

    let build = project.compile_to_evm(
        &mut vec![],
        metadata_hash_type,
        optimizer_settings,
        vec![],
        None,
    )?;
    build.check_errors()?;

    let build = build.link(linker_symbols);
    build.check_errors()?;

    build.write_to_standard_json(&mut solc_output, solc_compiler.version)?;
    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the Yul `sources` and returns the standard JSON output.
///
pub fn build_yul(
    sources: BTreeMap<String, String>,
) -> anyhow::Result<solx_solc::StandardJsonOutput> {
    self::setup()?;

    let solc_compiler = solx_solc::Compiler::default();

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EVM);

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::none();

    let sources = sources
        .into_iter()
        .map(|(path, source)| (path, solx_solc::StandardJsonInputSource::from(source)))
        .collect();

    let mut solc_output = solx_solc::StandardJsonOutput::new(&sources, &mut vec![]);

    let project = Project::try_from_yul_sources(
        sources,
        solx_solc::StandardJsonInputLibraries::default(),
        Some(&mut solc_output),
        None,
    )?;
    let build = project.compile_to_evm(
        &mut vec![],
        era_compiler_common::HashType::Ipfs,
        optimizer_settings,
        vec![],
        None,
    )?;
    build.check_errors()?;

    let build = build.link(BTreeMap::new());
    build.check_errors()?;

    build.write_to_standard_json(&mut solc_output, solc_compiler.version)?;
    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the Yul standard JSON and returns the standard JSON output.
///
/// If `solc_compiler` is set, the standard JSON is validated with `solc`.
///
pub fn build_yul_standard_json(
    mut solc_input: solx_solc::StandardJsonInput,
) -> anyhow::Result<solx_solc::StandardJsonOutput> {
    self::setup()?;

    let solc_compiler = solx_solc::Compiler::default();

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EVM);

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode,
    )?;

    let mut solc_output = solc_compiler.validate_yul_standard_json(&mut solc_input, &mut vec![])?;

    let project = Project::try_from_yul_sources(
        solc_input.sources,
        solx_solc::StandardJsonInputLibraries::default(),
        Some(&mut solc_output),
        None,
    )?;
    let build = project.compile_to_evm(
        &mut vec![],
        era_compiler_common::HashType::Ipfs,
        optimizer_settings,
        vec![],
        None,
    )?;
    build.check_errors()?;

    let build = build.link(BTreeMap::new());
    build.check_errors()?;

    build.write_to_standard_json(&mut solc_output, solc_compiler.version)?;
    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the LLVM IR standard JSON and returns the standard JSON output.
///
pub fn build_llvm_ir_standard_json(
    input: solx_solc::StandardJsonInput,
) -> anyhow::Result<solx_solc::StandardJsonOutput> {
    self::setup()?;

    let solc_compiler = solx_solc::Compiler::default();

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EVM);

    let optimizer_settings =
        era_compiler_llvm_context::OptimizerSettings::try_from_cli(input.settings.optimizer.mode)?;

    let mut output = solx_solc::StandardJsonOutput::new(&BTreeMap::new(), &mut vec![]);

    let project = Project::try_from_llvm_ir_sources(
        input.sources,
        solx_solc::StandardJsonInputLibraries::default(),
        Some(&mut output),
    )?;
    let build = project.compile_to_evm(
        &mut vec![],
        era_compiler_common::HashType::Ipfs,
        optimizer_settings,
        vec![],
        None,
    )?;
    build.check_errors()?;

    let build = build.link(BTreeMap::new());
    build.check_errors()?;

    build.write_to_standard_json(&mut output, solc_compiler.version)?;
    output.check_errors()?;
    Ok(output)
}
