//!
//! Solidity compiler executable.
//!

pub mod arguments;

use std::io::Write;
use std::path::PathBuf;

use clap::Parser;

use self::arguments::Arguments;

/// The rayon worker stack size.
const RAYON_WORKER_STACK_SIZE: usize = 16 * 1024 * 1024;

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
            messages.push(solx_solc::StandardJsonOutputError::new_error(
                error, None, None,
            ));
        }
    }

    if is_standard_json {
        let output = solx_solc::StandardJsonOutput::new_with_messages(messages);
        output.write_and_exit(solx_solc::StandardJsonInputSelection::default());
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
    messages: &mut Vec<solx_solc::StandardJsonOutputError>,
) -> anyhow::Result<()> {
    if arguments.version {
        writeln!(
            std::io::stdout(),
            "{} v{} (LLVM build {})",
            env!("CARGO_PKG_DESCRIPTION"),
            solx::version(),
            inkwell::support::get_commit_id().to_string(),
        )?;
        return Ok(());
    }

    let mut thread_pool_builder = rayon::ThreadPoolBuilder::new();
    if let Some(threads) = arguments.threads {
        thread_pool_builder = thread_pool_builder.num_threads(threads);
    }
    thread_pool_builder
        .stack_size(RAYON_WORKER_STACK_SIZE)
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
    if arguments.fallback_to_optimizing_for_size {
        optimizer_settings.enable_fallback_to_size();
    }
    optimizer_settings.is_verify_each_enabled = arguments.llvm_verify_each;
    optimizer_settings.is_debug_logging_enabled = arguments.llvm_debug_logging;

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
        .unwrap_or(era_compiler_common::HashType::Keccak256);

    let build = if arguments.yul {
        solx::yul_to_evm(
            input_files.as_slice(),
            arguments.libraries.as_slice(),
            arguments.solc,
            messages,
            metadata_hash_type,
            optimizer_settings,
            llvm_options,
            debug_config,
        )
    } else if arguments.llvm_ir {
        solx::llvm_ir_to_evm(
            input_files.as_slice(),
            arguments.libraries.as_slice(),
            messages,
            metadata_hash_type,
            optimizer_settings,
            llvm_options,
            debug_config,
        )
    } else if arguments.link {
        anyhow::bail!("The EVM target does not support linking yet.");
    } else if let Some(standard_json) = arguments.standard_json {
        let solc_compiler = match arguments.solc.as_deref() {
            Some(executable) => Some(solx_solc::Compiler::try_from_path(executable)?),
            None => None,
        };
        return solx::standard_json_evm(
            solc_compiler,
            arguments.via_ir,
            standard_json.map(PathBuf::from),
            messages,
            arguments.base_path,
            arguments.include_path,
            arguments.allow_paths,
            debug_config,
        );
    } else if let Some(format) = arguments.combined_json {
        let solc_compiler = solx_solc::Compiler::try_from_path(
            arguments
                .solc
                .as_deref()
                .unwrap_or(solx_solc::Compiler::DEFAULT_EXECUTABLE_NAME),
        )?;
        return solx::combined_json_evm(
            format,
            input_files.as_slice(),
            arguments.libraries.as_slice(),
            &solc_compiler,
            messages,
            arguments.evm_version,
            arguments.via_ir,
            metadata_hash_type,
            arguments.metadata_literal,
            arguments.base_path,
            arguments.include_path,
            arguments.allow_paths,
            remappings,
            arguments.output_dir,
            arguments.overwrite,
            optimizer_settings,
            llvm_options,
            debug_config,
        );
    } else {
        let solc = solx_solc::Compiler::try_from_path(
            arguments
                .solc
                .as_deref()
                .unwrap_or(solx_solc::Compiler::DEFAULT_EXECUTABLE_NAME),
        )?;
        solx::standard_output_evm(
            input_files.as_slice(),
            arguments.libraries.as_slice(),
            &solc,
            messages,
            arguments.evm_version,
            arguments.via_ir,
            metadata_hash_type,
            arguments.metadata_literal,
            arguments.base_path,
            arguments.include_path,
            arguments.allow_paths,
            remappings,
            optimizer_settings,
            llvm_options,
            debug_config,
        )
    }?;

    if let Some(output_directory) = arguments.output_dir {
        build.write_to_directory(
            &output_directory,
            arguments.output_metadata,
            arguments.output_assembly,
            arguments.output_binary,
            arguments.overwrite,
        )?;
    } else {
        build.write_to_terminal(
            arguments.output_metadata,
            arguments.output_assembly,
            arguments.output_binary,
        )?;
    }

    Ok(())
}
