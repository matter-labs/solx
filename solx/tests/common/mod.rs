//!
//! Unit test common utilities.
//!

#![allow(dead_code)]

pub mod r#const;

pub use self::r#const::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Once;
use std::time::Duration;

use assert_cmd::Command;

use solx::project::Project;
use solx_solc::CollectableError;

/// Synchronization for `solc` downloads.
static DOWNLOAD_SOLC: Once = Once::new();

/// Synchronization for upstream `solc` downloads.
static DOWNLOAD_SOLC_UPSTREAM: Once = Once::new();

///
/// Setup required test dependencies.
///
pub fn setup() -> anyhow::Result<()> {
    // Download `solc` executables once
    DOWNLOAD_SOLC.call_once(|| {
        download_executables(SOLC_BIN_CONFIG_PATH, true)
            .expect("Unable to download `solc` executables");
    });

    // Set the `solx` binary path
    let solx_bin = Command::cargo_bin(solx::DEFAULT_EXECUTABLE_NAME)?;
    let _ = solx::process::EXECUTABLE.set(PathBuf::from(solx_bin.get_program()));

    // Enable LLVM pretty stack trace
    inkwell::support::enable_llvm_pretty_stack_trace();

    Ok(())
}

///
/// Downloads the necessary compiler executables.
///
pub fn download_executables(config_path: &str, create_alias: bool) -> anyhow::Result<()> {
    let mut http_client_builder = reqwest::blocking::ClientBuilder::new();
    http_client_builder = http_client_builder.connect_timeout(Duration::from_secs(60));
    http_client_builder = http_client_builder.pool_idle_timeout(Duration::from_secs(60));
    http_client_builder = http_client_builder.timeout(Duration::from_secs(60));
    let http_client = http_client_builder.build()?;

    let config_path = Path::new(config_path);
    era_compiler_downloader::Downloader::new(http_client.clone()).download(config_path)?;

    if create_alias {
        // Copy the latest `solc-*` binary to `solc` for CLI tests
        let latest_solc = PathBuf::from(
            get_solc_compiler(&solx_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable,
        );
        let mut solc = latest_solc.clone();
        solc.set_file_name(format!("solc{}", std::env::consts::EXE_SUFFIX));
        std::fs::copy(latest_solc, solc)?;
    }

    Ok(())
}

///
/// Returns the `solc` compiler for the given version.
///
pub fn get_solc_compiler(version: &semver::Version) -> anyhow::Result<solx_solc::Compiler> {
    let solc_path = PathBuf::from(SOLC_DOWNLOAD_DIRECTORY).join(format!(
        "{}-{version}{}",
        solx_solc::Compiler::DEFAULT_EXECUTABLE_NAME,
        std::env::consts::EXE_SUFFIX,
    ));
    solx_solc::Compiler::try_from_path(solc_path.to_str().expect("Always valid"))
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
    solc_version: &semver::Version,
    via_ir: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<solx_solc::StandardJsonOutput> {
    self::setup()?;

    let solc_compiler = get_solc_compiler(solc_version)?;

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

    let project =
        Project::try_from_solc_output(libraries, via_ir, &mut solc_output, &solc_compiler, None)?;
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

    build.write_to_standard_json(&mut solc_output, Some(&solc_compiler.version))?;
    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the Solidity project and returns the combined JSON output.
///
pub fn build_solidity_combined_json(
    sources: BTreeMap<String, String>,
    libraries: solx_solc::StandardJsonInputLibraries,
    selectors: Vec<solx_solc::CombinedJsonSelector>,
    metadata_hash_type: era_compiler_common::HashType,
    solc_version: &semver::Version,
    via_ir: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<solx_solc::CombinedJson> {
    self::setup()?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EVM);

    let solc_compiler = get_solc_compiler(solc_version)?;
    let paths: Vec<PathBuf> = sources.keys().map(PathBuf::from).collect();

    let mut solc_output = self::build_solidity_standard_json(
        sources,
        libraries.clone(),
        metadata_hash_type,
        BTreeSet::new(),
        solc_version,
        via_ir,
        optimizer_settings.clone(),
    )?;

    let project =
        Project::try_from_solc_output(libraries, via_ir, &mut solc_output, &solc_compiler, None)?;
    solc_output.check_errors()?;

    let build = project.compile_to_evm(
        &mut vec![],
        metadata_hash_type,
        optimizer_settings,
        vec![],
        None,
    )?;
    build.check_errors()?;

    let mut combined_json =
        solc_compiler.combined_json(paths.as_slice(), selectors.into_iter().collect())?;
    build.write_to_combined_json(&mut combined_json)?;
    Ok(combined_json)
}

///
/// Builds the Yul `sources` and returns the standard JSON output.
///
pub fn build_yul(
    sources: BTreeMap<String, String>,
) -> anyhow::Result<solx_solc::StandardJsonOutput> {
    self::setup()?;

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

    build.write_to_standard_json(&mut solc_output, None)?;
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
    solc_compiler: Option<&solx_solc::Compiler>,
) -> anyhow::Result<solx_solc::StandardJsonOutput> {
    self::setup()?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EVM);

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode,
    )?;

    let (solc_version, mut solc_output) = match solc_compiler {
        Some(solc_compiler) => {
            let solc_output =
                solc_compiler.validate_yul_standard_json(&mut solc_input, &mut vec![])?;
            (Some(&solc_compiler.version), solc_output)
        }
        None => (
            None,
            solx_solc::StandardJsonOutput::new(&solc_input.sources, &mut vec![]),
        ),
    };

    let project = Project::try_from_yul_sources(
        solc_input.sources,
        solx_solc::StandardJsonInputLibraries::default(),
        Some(&mut solc_output),
        solc_version,
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

    build.write_to_standard_json(&mut solc_output, solc_version)?;
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

    build.write_to_standard_json(&mut output, None)?;
    output.check_errors()?;
    Ok(output)
}
