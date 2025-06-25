//!
//! Solidity compiler library.
//!

#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::result_large_err)]

pub mod build;
pub mod r#const;
pub mod error;
pub mod process;
pub mod project;
pub mod yul;

pub use self::build::contract::Contract as EVMContractBuild;
pub use self::build::Build as EVMBuild;
pub use self::error::stack_too_deep::StackTooDeep as StackTooDeepError;
pub use self::error::Error;
pub use self::process::input::Input as EVMProcessInput;
pub use self::process::output::Output as EVMProcessOutput;
pub use self::process::run as run_recursive;
pub use self::process::EXECUTABLE;
pub use self::project::contract::Contract as ProjectContract;
pub use self::project::Project;
pub use self::r#const::*;

use std::collections::BTreeSet;
use std::path::PathBuf;

use solx_standard_json::CollectableError;

/// The default error compatible with `solc` standard JSON output.
pub type Result<T> = std::result::Result<T, Error>;

///
/// Runs the Yul mode for the EVM target.
///
pub fn yul_to_evm(
    paths: &[PathBuf],
    libraries: &[String],
    output_selection: &solx_standard_json::InputSelection,
    messages: &mut Vec<solx_standard_json::OutputError>,
    metadata_hash_type: era_compiler_common::EVMMetadataHashType,
    append_cbor: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let libraries = era_compiler_common::Libraries::try_from(libraries)?;
    let linker_symbols = libraries.as_linker_symbols()?;

    let solc_compiler = solx_solc::Compiler::default();
    solc_compiler.validate_yul_paths(paths, libraries.clone(), messages)?;

    let project = Project::try_from_yul_paths(
        paths,
        libraries,
        output_selection,
        None,
        debug_config.as_ref(),
    )?;

    let mut build = project.compile_to_evm(
        messages,
        output_selection,
        metadata_hash_type,
        optimizer_settings,
        None,
        llvm_options,
        debug_config,
    )?;
    build.take_and_write_warnings();
    build.check_errors()?;

    let cbor_data = if append_cbor {
        Some(vec![
            (
                crate::r#const::DEFAULT_EXECUTABLE_NAME.to_owned(),
                crate::r#const::version().parse().expect("Always valid"),
            ),
            (
                crate::r#const::SOLC_PRODUCTION_NAME.to_owned(),
                solc_compiler.version.default.to_owned(),
            ),
            (
                crate::r#const::SOLC_LLVM_REVISION_METADATA_TAG.to_owned(),
                solc_compiler.version.llvm_revision.to_owned(),
            ),
        ])
    } else {
        None
    };

    Ok(if output_selection.is_bytecode_set_for_any() {
        let mut build = build.link(linker_symbols, cbor_data);
        build.take_and_write_warnings();
        build.check_errors()?;
        build
    } else {
        build
    })
}

///
/// Runs the LLVM IR mode for the EVM target.
///
pub fn llvm_ir_to_evm(
    paths: &[PathBuf],
    libraries: &[String],
    output_selection: &solx_standard_json::InputSelection,
    messages: &mut Vec<solx_standard_json::OutputError>,
    metadata_hash_type: era_compiler_common::EVMMetadataHashType,
    append_cbor: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let libraries = era_compiler_common::Libraries::try_from(libraries)?;
    let linker_symbols = libraries.as_linker_symbols()?;

    let project = Project::try_from_llvm_ir_paths(paths, libraries, output_selection, None)?;

    let mut build = project.compile_to_evm(
        messages,
        output_selection,
        metadata_hash_type,
        optimizer_settings,
        None,
        llvm_options,
        debug_config,
    )?;
    build.take_and_write_warnings();
    build.check_errors()?;

    let cbor_data = if append_cbor {
        Some(vec![(
            crate::r#const::DEFAULT_EXECUTABLE_NAME.to_owned(),
            crate::r#const::version().parse().expect("Always valid"),
        )])
    } else {
        None
    };

    Ok(if output_selection.is_bytecode_set_for_any() {
        let mut build = build.link(linker_symbols, cbor_data);
        build.take_and_write_warnings();
        build.check_errors()?;
        build
    } else {
        build
    })
}

///
/// Runs the standard output mode for the EVM target.
///
pub fn standard_output_evm(
    paths: &[PathBuf],
    libraries: &[String],
    output_selection: &solx_standard_json::InputSelection,
    messages: &mut Vec<solx_standard_json::OutputError>,
    evm_version: Option<era_compiler_common::EVMVersion>,
    via_ir: bool,
    metadata_hash_type: era_compiler_common::EVMMetadataHashType,
    metadata_literal: bool,
    append_cbor: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    use_import_callback: bool,
    remappings: BTreeSet<String>,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let mut solc_input = solx_standard_json::Input::try_from_solidity_paths(
        paths,
        libraries,
        remappings,
        solx_standard_json::InputOptimizer::default(),
        evm_version,
        via_ir,
        output_selection,
        solx_standard_json::InputMetadata::new(metadata_literal, append_cbor, metadata_hash_type),
        llvm_options.clone(),
    )?;

    let solc_compiler = solx_solc::Compiler::default();

    let cbor_data = if append_cbor {
        Some(vec![
            (
                crate::r#const::DEFAULT_EXECUTABLE_NAME.to_owned(),
                crate::r#const::version().parse().expect("Always valid"),
            ),
            (
                crate::r#const::SOLC_PRODUCTION_NAME.to_owned(),
                solc_compiler.version.default.to_owned(),
            ),
            (
                crate::r#const::SOLC_LLVM_REVISION_METADATA_TAG.to_owned(),
                solc_compiler.version.llvm_revision.to_owned(),
            ),
        ])
    } else {
        None
    };

    let mut solc_output = solc_compiler.standard_json(
        &mut solc_input,
        messages,
        use_import_callback,
        base_path.as_deref(),
        include_paths.as_slice(),
        allow_paths.as_deref(),
    )?;
    solc_output.take_and_write_warnings();
    solc_output.check_errors()?;

    let linker_symbols = solc_input.settings.libraries.as_linker_symbols()?;

    let project = Project::try_from_solc_output(
        solc_input.settings.libraries.clone(),
        via_ir,
        &mut solc_output,
        debug_config.as_ref(),
    )?;
    solc_output.take_and_write_warnings();
    solc_output.check_errors()?;

    let mut build = project.compile_to_evm(
        messages,
        &solc_input.settings.output_selection,
        metadata_hash_type,
        optimizer_settings.clone(),
        None,
        llvm_options,
        debug_config.clone(),
    )?;
    let stack_too_deep_errors = build.take_stack_too_deep_errors();
    build.take_and_write_warnings();
    build.check_errors()?;
    let output_selection = solc_input.settings.output_selection.clone();
    let build = if !stack_too_deep_errors.is_empty() {
        let (_solc_output_second_pass, mut build_second_pass) = standard_json_second_pass(
            &solc_compiler,
            solc_input,
            optimizer_settings,
            stack_too_deep_errors,
            messages,
            base_path.as_deref(),
            include_paths.as_slice(),
            allow_paths.as_deref(),
            use_import_callback,
            debug_config.as_ref(),
        )?;
        build_second_pass.take_and_write_warnings();
        build_second_pass.check_errors()?;
        build_second_pass
    } else {
        build
    };

    Ok(if output_selection.is_bytecode_set_for_any() {
        let mut build = build.link(linker_symbols, cbor_data);
        build.take_and_write_warnings();
        build.check_errors()?;
        build
    } else {
        build
    })
}

///
/// Runs the standard JSON mode for the EVM target.
///
pub fn standard_json_evm(
    json_path: Option<PathBuf>,
    messages: &mut Vec<solx_standard_json::OutputError>,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    use_import_callback: bool,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<()> {
    let solc_compiler = solx_solc::Compiler::default();

    let mut solc_input = solx_standard_json::Input::try_from(json_path.as_deref())?;
    let language = solc_input.language;
    let via_ir = solc_input.settings.via_ir;
    let linker_symbols = solc_input.settings.libraries.as_linker_symbols()?;

    let mut optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode.unwrap_or_else(|| {
            solx_standard_json::InputOptimizer::default_mode().expect("Always exists")
        }),
    )?;
    if solc_input
        .settings
        .optimizer
        .size_fallback
        .unwrap_or_default()
    {
        optimizer_settings.enable_fallback_to_size();
    }
    let llvm_options = solc_input.settings.llvm_options.clone();

    let metadata_hash_type = solc_input.settings.metadata.bytecode_hash;
    let append_cbor = solc_input.settings.metadata.append_cbor;

    let cbor_data = if append_cbor {
        let mut cbor_data = Vec::with_capacity(3);
        cbor_data.push((
            crate::r#const::DEFAULT_EXECUTABLE_NAME.to_owned(),
            crate::r#const::version().parse().expect("Always valid"),
        ));
        if let solx_standard_json::InputLanguage::Solidity
        | solx_standard_json::InputLanguage::Yul = language
        {
            cbor_data.push((
                crate::r#const::SOLC_PRODUCTION_NAME.to_owned(),
                solc_compiler.version.default.to_owned(),
            ));
            cbor_data.push((
                crate::r#const::SOLC_LLVM_REVISION_METADATA_TAG.to_owned(),
                solc_compiler.version.llvm_revision.to_owned(),
            ));
        };
        Some(cbor_data)
    } else {
        None
    };

    let (mut solc_output, project) = match language {
        solx_standard_json::InputLanguage::Solidity => {
            let mut solc_output = solc_compiler.standard_json(
                &mut solc_input,
                messages,
                use_import_callback,
                base_path.as_deref(),
                include_paths.as_slice(),
                allow_paths.as_deref(),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(&solc_input.settings.output_selection);
            }

            let project = Project::try_from_solc_output(
                solc_input.settings.libraries.clone(),
                via_ir,
                &mut solc_output,
                debug_config.as_ref(),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(&solc_input.settings.output_selection);
            }

            (solc_output, project)
        }
        solx_standard_json::InputLanguage::Yul => {
            let mut solc_output =
                solc_compiler.validate_yul_standard_json(&mut solc_input, messages)?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(&solc_input.settings.output_selection);
            }

            let project = Project::try_from_yul_sources(
                solc_input.sources.clone(),
                solc_input.settings.libraries.clone(),
                &solc_input.settings.output_selection,
                Some(&mut solc_output),
                debug_config.as_ref(),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(&solc_input.settings.output_selection);
            }

            (solc_output, project)
        }
        solx_standard_json::InputLanguage::LLVMIR => {
            let mut solc_output = solx_standard_json::Output::new(&solc_input.sources, messages);

            let project = Project::try_from_llvm_ir_sources(
                solc_input.sources.clone(),
                solc_input.settings.libraries.clone(),
                &solc_input.settings.output_selection,
                Some(&mut solc_output),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(&solc_input.settings.output_selection);
            }

            (solc_output, project)
        }
    };

    let mut build = project.compile_to_evm(
        messages,
        &solc_input.settings.output_selection,
        metadata_hash_type,
        optimizer_settings.clone(),
        None,
        llvm_options,
        debug_config.clone(),
    )?;
    let stack_too_deep_errors = build.take_stack_too_deep_errors();
    let output_selection = solc_input.settings.output_selection.clone();
    if build.has_errors() && stack_too_deep_errors.is_empty() {
        build.write_to_standard_json(
            &mut solc_output,
            &solc_input.settings.output_selection,
            false,
        )?;
        solc_output.write_and_exit(&solc_input.settings.output_selection);
    }
    let (build, mut solc_output) = if language == solx_standard_json::InputLanguage::Solidity
        && !stack_too_deep_errors.is_empty()
    {
        let (mut solc_output_second_pass, mut build_second_pass) = standard_json_second_pass(
            &solc_compiler,
            solc_input,
            optimizer_settings,
            stack_too_deep_errors,
            messages,
            base_path.as_deref(),
            include_paths.as_slice(),
            allow_paths.as_deref(),
            use_import_callback,
            debug_config.as_ref(),
        )?;
        if build_second_pass.has_errors() {
            build_second_pass.write_to_standard_json(
                &mut solc_output_second_pass,
                &output_selection,
                false,
            )?;
            solc_output_second_pass.write_and_exit(&output_selection);
        }
        (build_second_pass, solc_output_second_pass)
    } else {
        (build, solc_output)
    };
    let mut build = if output_selection.is_bytecode_set_for_any() {
        build.link(linker_symbols, cbor_data)
    } else {
        build
    };
    build.write_to_standard_json(&mut solc_output, &output_selection, true)?;
    solc_output.write_and_exit(&output_selection);
}

///
/// Runs the second pass that recompiles contracts that failed to compile in the first pass.
///
fn standard_json_second_pass(
    solc_compiler: &solx_solc::Compiler,
    mut solc_input: solx_standard_json::Input,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    stack_too_deep_errors: Vec<StackTooDeepError>,
    messages: &mut Vec<solx_standard_json::OutputError>,
    base_path: Option<&str>,
    include_paths: &[String],
    allow_paths: Option<&str>,
    use_import_callback: bool,
    debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<(solx_standard_json::Output, EVMBuild)> {
    let via_ir = solc_input.settings.via_ir;
    let llvm_options = solc_input.settings.llvm_options.clone();

    let metadata_hash_type = solc_input.settings.metadata.bytecode_hash;

    solc_input.settings.optimizer.spill_area_size = Some(
        stack_too_deep_errors
            .into_iter()
            .map(|error| {
                let (deploy_spill_area_size, runtime_spill_area_size) =
                    match error.code_segment.expect("Always exists") {
                        era_compiler_common::CodeSegment::Deploy => (error.spill_area_size, 0),
                        era_compiler_common::CodeSegment::Runtime => (0, error.spill_area_size),
                    };
                (
                    error.contract_name.expect("Always exists").full_path,
                    solx_standard_json::InputOptimizerSpillAreaSize::new(
                        deploy_spill_area_size,
                        runtime_spill_area_size,
                    ),
                )
            })
            .collect(),
    );
    let mut solc_output_second_pass = solc_compiler.standard_json(
        &mut solc_input,
        messages,
        use_import_callback,
        base_path,
        include_paths,
        allow_paths,
    )?;

    if solc_output_second_pass.has_errors() {
        solc_output_second_pass.write_and_exit(&solc_input.settings.output_selection);
    }

    let project_second_pass = Project::try_from_solc_output(
        solc_input.settings.libraries,
        via_ir,
        &mut solc_output_second_pass,
        debug_config,
    )?;
    if solc_output_second_pass.has_errors() {
        solc_output_second_pass.write_and_exit(&solc_input.settings.output_selection);
    }

    let build_second_pass = project_second_pass.compile_to_evm(
        messages,
        &solc_input.settings.output_selection,
        metadata_hash_type,
        optimizer_settings,
        solc_input.settings.optimizer.spill_area_size,
        llvm_options,
        debug_config.cloned(),
    )?;
    Ok((solc_output_second_pass, build_second_pass))
}
