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
pub mod evmla;
pub mod process;
pub mod project;
pub mod yul;

pub use self::build::contract::Contract as EVMContractBuild;
pub use self::build::Build as EVMBuild;
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
pub type Result<T> = std::result::Result<T, solx_standard_json::OutputError>;

///
/// Runs the Yul mode for the EVM target.
///
pub fn yul_to_evm(
    paths: &[PathBuf],
    libraries: &[String],
    output_bytecode: bool,
    output_assembly: bool,
    output_metadata: bool,
    messages: &mut Vec<solx_standard_json::OutputError>,
    metadata_hash_type: era_compiler_common::EVMMetadataHashType,
    append_cbor: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let libraries = era_compiler_common::Libraries::try_from(libraries)?;
    let output_selection = solx_standard_json::InputSelection::new_compilation(
        output_bytecode,
        output_metadata,
        Some(true),
    );
    let linker_symbols = libraries.as_linker_symbols()?;

    let solc_compiler = solx_solc::Compiler::default();
    solc_compiler.validate_yul_paths(paths, libraries.clone(), messages)?;

    let project = Project::try_from_yul_paths(
        paths,
        libraries,
        &output_selection,
        None,
        debug_config.as_ref(),
    )?;

    let mut build = project.compile_to_evm(
        messages,
        output_assembly,
        output_bytecode,
        metadata_hash_type,
        optimizer_settings,
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

    Ok(if output_bytecode {
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
    output_bytecode: bool,
    output_assembly: bool,
    output_metadata: bool,
    messages: &mut Vec<solx_standard_json::OutputError>,
    metadata_hash_type: era_compiler_common::EVMMetadataHashType,
    append_cbor: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let libraries = era_compiler_common::Libraries::try_from(libraries)?;
    let output_selection =
        solx_standard_json::InputSelection::new_compilation(output_bytecode, output_metadata, None);
    let linker_symbols = libraries.as_linker_symbols()?;

    let project = Project::try_from_llvm_ir_paths(paths, libraries, &output_selection, None)?;

    let mut build = project.compile_to_evm(
        messages,
        output_assembly,
        output_bytecode,
        metadata_hash_type,
        optimizer_settings,
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

    Ok(if output_bytecode {
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
    output_bytecode: bool,
    output_assembly: bool,
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
        solx_standard_json::InputSelection::new_compilation(output_bytecode, true, Some(via_ir)),
        solx_standard_json::InputMetadata::new(metadata_literal, append_cbor, metadata_hash_type),
        llvm_options.clone(),
    )?;

    let solc_compiler = solx_solc::Compiler::default();

    let mut solc_output = solc_compiler.standard_json(
        &mut solc_input,
        messages,
        use_import_callback,
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
        output_assembly,
        output_bytecode,
        metadata_hash_type,
        optimizer_settings,
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

    Ok(if output_bytecode {
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
    let output_bytecode = solc_input
        .settings
        .output_selection
        .is_set_for_any(solx_standard_json::InputSelector::Bytecode)
        || solc_input
            .settings
            .output_selection
            .is_set_for_any(solx_standard_json::InputSelector::RuntimeBytecode);
    let output_assembly = solc_input
        .settings
        .output_selection
        .is_set_for_any(solx_standard_json::InputSelector::DeployLLVMAssembly)
        || solc_input
            .settings
            .output_selection
            .is_set_for_any(solx_standard_json::InputSelector::RuntimeLLVMAssembly);
    let linker_symbols = solc_input.settings.libraries.as_linker_symbols()?;

    let mut optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode,
    )?;
    if solc_input.settings.optimizer.size_fallback {
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
                base_path,
                include_paths,
                allow_paths,
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(&solc_input.settings.output_selection);
            }

            let project = Project::try_from_solc_output(
                solc_input.settings.libraries,
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
                solc_input.sources,
                solc_input.settings.libraries,
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
                solc_input.sources,
                solc_input.settings.libraries,
                &solc_input.settings.output_selection,
                Some(&mut solc_output),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(&solc_input.settings.output_selection);
            }

            (solc_output, project)
        }
    };

    let build = project.compile_to_evm(
        messages,
        output_assembly,
        output_bytecode,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        debug_config,
    )?;
    if build.has_errors() {
        build.write_to_standard_json(&mut solc_output)?;
        solc_output.write_and_exit(&solc_input.settings.output_selection);
    }

    let build = if output_bytecode {
        build.link(linker_symbols, cbor_data)
    } else {
        build
    };
    build.write_to_standard_json(&mut solc_output)?;
    solc_output.write_and_exit(&solc_input.settings.output_selection);
}
