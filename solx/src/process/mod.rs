//!
//! Process for compiling a single compilation unit.
//!

pub mod input;
pub mod output;

use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;
use std::thread::Builder;

use crate::error::Error;
use crate::project::contract::Contract;

use self::input::Input as EVMInput;
use self::output::Output as EVMOutput;

/// The overridden executable name used when the compiler is run as a library.
pub static EXECUTABLE: OnceLock<PathBuf> = OnceLock::new();

///
/// Read input from `stdin`, compile a contract, and write the output to `stdout`.
///
pub fn run() -> anyhow::Result<()> {
    let input_json = std::io::read_to_string(std::io::stdin())
        .map_err(|error| anyhow::anyhow!("Stdin reading error: {error}"))?;
    let input: EVMInput = era_compiler_common::deserialize_from_str(input_json.as_str())
        .map_err(|error| anyhow::anyhow!("Stdin parsing error: {error}"))?;

    let source_location =
        solx_standard_json::OutputErrorSourceLocation::new(input.contract_name.path.clone());

    let result = Builder::new()
        .stack_size(crate::WORKER_THREAD_STACK_SIZE)
        .spawn(move || {
            Contract::compile_to_evm(
                input.contract_name,
                input.contract_ir,
                input.code_segment,
                input.identifier_paths,
                input.output_selection,
                input.immutables,
                input.metadata_bytes,
                input.optimizer_settings,
                input.llvm_options,
                input.debug_config,
            )
            .map(EVMOutput::new)
            .map_err(|error| match error {
                Error::Generic(error) => solx_standard_json::OutputError::new_error(
                    None,
                    error,
                    Some(source_location),
                    None,
                )
                .into(),
                error => error,
            })
        })
        .expect("Threading error")
        .join()
        .expect("Threading error");

    serde_json::to_writer(std::io::stdout(), &result)
        .map_err(|error| anyhow::anyhow!("Stdout writing error: {error}"))?;
    unsafe { inkwell::support::shutdown_llvm() };
    Ok(())
}

///
/// Runs this process recursively to compile a single contract.
///
pub fn call<I, O>(path: &str, input: I) -> crate::Result<O>
where
    I: serde::Serialize,
    O: serde::de::DeserializeOwned,
{
    let executable = EXECUTABLE
        .get()
        .cloned()
        .unwrap_or_else(|| std::env::current_exe().expect("Current executable path getting error"));

    let mut command = Command::new(executable.as_path());
    command.stdin(std::process::Stdio::piped());
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    command.arg("--recursive-process");
    command.arg(path);

    let mut process = command
        .spawn()
        .unwrap_or_else(|error| panic!("{executable:?} subprocess spawning: {error:?}"));

    let stdin = process
        .stdin
        .as_mut()
        .unwrap_or_else(|| panic!("{executable:?} subprocess stdin getting error"));
    let stdin_input = serde_json::to_vec(&input).expect("Always valid");
    stdin
        .write_all(stdin_input.as_slice())
        .unwrap_or_else(|error| panic!("{executable:?} subprocess stdin writing: {error:?}"));

    let result = process
        .wait_with_output()
        .unwrap_or_else(|error| panic!("{executable:?} subprocess output reading: {error:?}"));

    if result.status.code() != Some(era_compiler_common::EXIT_CODE_SUCCESS) {
        let message = format!(
            "{executable:?} subprocess failed {}:\n{}\n{}",
            match result.status.code() {
                Some(code) => format!("with exit code {code:?}"),
                None => "without exit code".to_owned(),
            },
            String::from_utf8_lossy(result.stdout.as_slice()),
            String::from_utf8_lossy(result.stderr.as_slice()),
        );
        Err(solx_standard_json::OutputError::new_error(
            None,
            message,
            Some(solx_standard_json::OutputErrorSourceLocation::new(
                path.to_owned(),
            )),
            None,
        ))?;
    }

    match era_compiler_common::deserialize_from_slice(result.stdout.as_slice()) {
        Ok(output) => output,
        Err(error) => {
            panic!(
                "{executable:?} subprocess stdout parsing error: {error:?}\n{}\n{}",
                String::from_utf8_lossy(result.stdout.as_slice()),
                String::from_utf8_lossy(result.stderr.as_slice()),
            );
        }
    }
}

///
/// Handles LLVM stack-too-deep errors.
///
/// # Safety
///
/// This function is unsafe because it is called from LLVM C API.
/// The function must terminate the process after handling the error.
///
pub unsafe extern "C" fn evm_stack_error_handler(spill_area_size: u64) {
    let result: Result<EVMOutput, Error> = Err(Error::stack_too_deep(spill_area_size));
    serde_json::to_writer(std::io::stdout(), &result)
        .unwrap_or_else(|error| panic!("Stdout writing error: {error}"));
    unsafe { inkwell::support::shutdown_llvm() };
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
}
