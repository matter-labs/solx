//!
//! Solidity compiler executable.
//!

pub mod arguments;

use std::collections::BTreeSet;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;

use self::arguments::Arguments;

///
/// The application entry point.
///
fn main() -> anyhow::Result<()> {
    let arguments = Arguments::try_parse()?;
    let is_standard_json = arguments.standard_json.is_some();
    let mut messages = arguments.validate();
    if messages.iter().all(|error| error.severity != "error") {
        if !is_standard_json {
            std::io::stderr()
                .write_all(
                    messages
                        .drain(..)
                        .map(|error| error.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                        .as_bytes(),
                )
                .expect("Stderr writing error");
        }
        if let Err(error) = main_inner(arguments, &mut messages) {
            messages.push(solx_standard_json::OutputError::new_error(
                None, error, None, None,
            ));
        }
    }

    if is_standard_json {
        let output = solx_standard_json::Output::new_with_messages(messages);
        output.write_and_exit(&solx_standard_json::InputSelection::default());
    }

    let exit_code = if messages.iter().any(|error| error.severity == "error") {
        era_compiler_common::EXIT_CODE_FAILURE
    } else {
        era_compiler_common::EXIT_CODE_SUCCESS
    };
    std::io::stderr()
        .write_all(
            messages
                .into_iter()
                .map(|error| error.to_string())
                .collect::<Vec<String>>()
                .join("\n")
                .as_bytes(),
        )
        .expect("Stderr writing error");
    std::process::exit(exit_code);
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner(
    arguments: Arguments,
    messages: &mut Vec<solx_standard_json::OutputError>,
) -> anyhow::Result<()> {
    if arguments.version {
        let solc = solx_solc::Compiler::default();
        writeln!(
            std::io::stdout(),
            "{}, {} v{}, LLVM revision: v{}, LLVM build: {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_VERSION"),
            solc.version.llvm_revision,
            inkwell::support::get_commit_id().to_string(),
        )?;
        writeln!(std::io::stdout(), "Version: {}", solc.version.long)?;
        return Ok(());
    }

    let mut thread_pool_builder = rayon::ThreadPoolBuilder::new();
    if let Some(threads) = arguments.threads {
        thread_pool_builder = thread_pool_builder.num_threads(threads);
    }
    thread_pool_builder
        .stack_size(solx::WORKER_THREAD_STACK_SIZE)
        .build_global()
        .expect("Thread pool configuration failure");

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EVM);

    if arguments.recursive_process {
        return solx::run_recursive();
    }

    let (input_files, remappings) = arguments.split_input_files_and_remappings()?;

    let mut optimizer_settings = match arguments.optimization {
        Some(mode) => era_compiler_llvm_context::OptimizerSettings::try_from_cli(mode)?,
        None => era_compiler_llvm_context::OptimizerSettings::cycles(),
    };
    if arguments.size_fallback {
        optimizer_settings.enable_fallback_to_size();
    }
    optimizer_settings.is_verify_each_enabled = arguments.llvm_verify_each;
    optimizer_settings.is_debug_logging_enabled = arguments.llvm_debug_logging;

    let mut selectors = BTreeSet::new();
    if arguments.output_bytecode {
        selectors.insert(solx_standard_json::InputSelector::BytecodeObject);
        selectors.insert(solx_standard_json::InputSelector::RuntimeBytecodeObject);
    }
    if arguments.output_assembly {
        selectors.insert(solx_standard_json::InputSelector::BytecodeLLVMAssembly);
        selectors.insert(solx_standard_json::InputSelector::RuntimeBytecodeLLVMAssembly);
    }
    if arguments.output_metadata {
        selectors.insert(solx_standard_json::InputSelector::Metadata);
    }
    let output_selection = solx_standard_json::InputSelection::new(selectors);

    let llvm_options: Vec<String> = arguments
        .llvm_options
        .as_ref()
        .map(|options| {
            options
                .split_whitespace()
                .map(|option| option.to_owned())
                .collect()
        })
        .unwrap_or_default();

    let debug_config = match arguments.debug_output_dir {
        Some(ref debug_output_directory) => {
            std::fs::create_dir_all(debug_output_directory.as_path())?;
            Some(era_compiler_llvm_context::DebugConfig::new(
                debug_output_directory.to_owned(),
            ))
        }
        None => None,
    };

    let metadata_hash_type = arguments
        .metadata_hash
        .unwrap_or(era_compiler_common::EVMMetadataHashType::IPFS);
    let append_cbor = !arguments.no_cbor_metadata;
    let use_import_callback = !arguments.no_import_callback;

    let build = if arguments.yul {
        solx::yul_to_evm(
            input_files.as_slice(),
            arguments.libraries.as_slice(),
            &output_selection,
            messages,
            metadata_hash_type,
            append_cbor,
            optimizer_settings,
            llvm_options,
            debug_config,
        )
    } else if arguments.llvm_ir {
        solx::llvm_ir_to_evm(
            input_files.as_slice(),
            arguments.libraries.as_slice(),
            &output_selection,
            messages,
            metadata_hash_type,
            append_cbor,
            optimizer_settings,
            llvm_options,
            debug_config,
        )
    } else if let Some(standard_json) = arguments.standard_json {
        return solx::standard_json_evm(
            standard_json.map(PathBuf::from),
            messages,
            arguments.base_path,
            arguments.include_path,
            arguments.allow_paths,
            use_import_callback,
            debug_config,
        );
    } else if !output_selection.is_empty() {
        solx::standard_output_evm(
            input_files.as_slice(),
            arguments.libraries.as_slice(),
            &output_selection,
            messages,
            arguments.evm_version,
            arguments.via_ir,
            metadata_hash_type,
            arguments.metadata_literal,
            append_cbor,
            arguments.base_path,
            arguments.include_path,
            arguments.allow_paths,
            use_import_callback,
            remappings,
            optimizer_settings,
            llvm_options,
            debug_config,
        )
    } else {
        writeln!(
            std::io::stdout(),
            "Compiler run successful. No output generated."
        )?;
        return Ok(());
    }?;

    if let Some(output_directory) = arguments.output_dir {
        build.write_to_directory(&output_directory, &output_selection, arguments.overwrite)?;
    } else {
        build.write_to_terminal(&output_selection)?;
    }

    Ok(())
}
