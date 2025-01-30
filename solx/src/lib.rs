//!
//! Solidity compiler library.
//!

#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::result_large_err)]

pub mod build_evm;
pub mod r#const;
pub mod evmla;
pub mod linker;
pub mod missing_libraries;
pub mod process;
pub mod project;
pub mod yul;

pub use self::build_evm::contract::Contract as EVMContractBuild;
pub use self::build_evm::Build as EVMBuild;
pub use self::linker::input::Input as LinkerInput;
pub use self::linker::output::Output as LinkerOutput;
pub use self::linker::Linker;
pub use self::process::input_evm::Input as EVMProcessInput;
pub use self::process::output_evm::Output as EVMProcessOutput;
pub use self::process::run as run_recursive;
pub use self::process::EXECUTABLE;
pub use self::project::contract::Contract as ProjectContract;
pub use self::project::Project;
pub use self::r#const::*;

use std::collections::BTreeSet;
use std::path::PathBuf;

use solx_solc::CollectableError;

/// The default error compatible with `solc` standard JSON output.
pub type Result<T> = std::result::Result<T, solx_solc::StandardJsonOutputError>;

///
/// Runs the Yul mode for the EVM target.
///
pub fn yul_to_evm(
    paths: &[PathBuf],
    libraries: &[String],
    messages: &mut Vec<solx_solc::StandardJsonOutputError>,
    metadata_hash_type: era_compiler_common::HashType,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let libraries = solx_solc::StandardJsonInputLibraries::try_from(libraries)?;
    let linker_symbols = libraries.as_linker_symbols()?;

    let solc_compiler = solx_solc::Compiler::default();
    solc_compiler.validate_yul_paths(paths, libraries.clone(), messages)?;

    let project = Project::try_from_yul_paths(paths, libraries, None, debug_config.as_ref())?;

    let mut build = project.compile_to_evm(
        messages,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        debug_config,
    )?;
    build.take_and_write_warnings();
    build.check_errors()?;

    let mut build = build.link(linker_symbols);
    build.take_and_write_warnings();
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the LLVM IR mode for the EVM target.
///
pub fn llvm_ir_to_evm(
    paths: &[PathBuf],
    libraries: &[String],
    messages: &mut Vec<solx_solc::StandardJsonOutputError>,
    metadata_hash_type: era_compiler_common::HashType,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let libraries = solx_solc::StandardJsonInputLibraries::try_from(libraries)?;
    let linker_symbols = libraries.as_linker_symbols()?;

    let project = Project::try_from_llvm_ir_paths(paths, libraries, None)?;

    let mut build = project.compile_to_evm(
        messages,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        debug_config,
    )?;
    build.take_and_write_warnings();
    build.check_errors()?;

    let mut build = build.link(linker_symbols);
    build.take_and_write_warnings();
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the standard output mode for the EVM target.
///
pub fn standard_output_evm(
    paths: &[PathBuf],
    libraries: &[String],
    messages: &mut Vec<solx_solc::StandardJsonOutputError>,
    evm_version: Option<era_compiler_common::EVMVersion>,
    via_ir: bool,
    metadata_hash_type: era_compiler_common::HashType,
    use_literal_content: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    remappings: BTreeSet<String>,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let mut solc_input = solx_solc::StandardJsonInput::try_from_solidity_paths(
        paths,
        libraries,
        remappings,
        solx_solc::StandardJsonInputOptimizer::default(),
        evm_version,
        via_ir,
        solx_solc::StandardJsonInputSelection::new_required(via_ir),
        solx_solc::StandardJsonInputMetadata::new(use_literal_content, metadata_hash_type),
        llvm_options.clone(),
    )?;

    let mut solc_output = solx_solc::Compiler::default().standard_json(
        &mut solc_input,
        messages,
        base_path,
        include_paths,
        allow_paths,
    )?;
    solc_output.take_and_write_warnings();
    solc_output.check_errors()?;

    let linker_symbols = solc_input.settings.libraries.as_linker_symbols()?;

    let project = Project::try_from_solc_output(
        solc_input.settings.libraries,
        via_ir,
        &mut solc_output,
        debug_config.as_ref(),
    )?;
    solc_output.take_and_write_warnings();
    solc_output.check_errors()?;

    let mut build = project.compile_to_evm(
        messages,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        debug_config,
    )?;
    build.take_and_write_warnings();
    build.check_errors()?;

    let mut build = build.link(linker_symbols);
    build.take_and_write_warnings();
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the standard JSON mode for the EVM target.
///
pub fn standard_json_evm(
    via_ir: bool,
    json_path: Option<PathBuf>,
    messages: &mut Vec<solx_solc::StandardJsonOutputError>,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<()> {
    let solc_compiler = solx_solc::Compiler::default();

    let mut solc_input = solx_solc::StandardJsonInput::try_from(json_path.as_deref())?;
    let language = solc_input.language;
    let prune_output = solc_input.settings.selection_to_prune();
    let linker_symbols = solc_input.settings.libraries.as_linker_symbols()?;

    let mut optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode,
    )?;
    if solc_input
        .settings
        .optimizer
        .fallback_to_optimizing_for_size
    {
        optimizer_settings.enable_fallback_to_size();
    }
    let llvm_options = solc_input.settings.llvm_options.clone();

    let metadata_hash_type = solc_input.settings.metadata.hash_type;

    let (mut solc_output, project) = match language {
        solx_solc::StandardJsonInputLanguage::Solidity => {
            solc_input
                .extend_selection(solx_solc::StandardJsonInputSelection::new_required(via_ir));

            let mut solc_output = solc_compiler.standard_json(
                &mut solc_input,
                messages,
                base_path,
                include_paths,
                allow_paths,
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            let project = Project::try_from_solc_output(
                solc_input.settings.libraries,
                via_ir,
                &mut solc_output,
                debug_config.as_ref(),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, project)
        }
        solx_solc::StandardJsonInputLanguage::Yul => {
            let mut solc_output =
                solc_compiler.validate_yul_standard_json(&mut solc_input, messages)?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            let project = Project::try_from_yul_sources(
                solc_input.sources,
                solc_input.settings.libraries,
                Some(&mut solc_output),
                debug_config.as_ref(),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, project)
        }
        solx_solc::StandardJsonInputLanguage::LLVMIR => {
            let mut solc_output = solx_solc::StandardJsonOutput::new(&solc_input.sources, messages);

            let project = Project::try_from_llvm_ir_sources(
                solc_input.sources,
                solc_input.settings.libraries,
                Some(&mut solc_output),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, project)
        }
    };

    let build = project.compile_to_evm(
        messages,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        debug_config,
    )?;
    if build.has_errors() {
        build.write_to_standard_json(&mut solc_output, solc_compiler.version)?;
        solc_output.write_and_exit(prune_output);
    }

    let build = build.link(linker_symbols);
    build.write_to_standard_json(&mut solc_output, solc_compiler.version)?;
    solc_output.write_and_exit(prune_output);
}
