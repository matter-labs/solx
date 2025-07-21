//!
//! Process for compiling a single compilation unit.
//!

pub mod input;
pub mod output;

use std::io::Read;
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
    let length_bytes = {
        let mut buffer = [0u8; 8];
        std::io::stdin()
            .read_exact(&mut buffer)
            .map_err(|error| anyhow::anyhow!("Input length prefix reading error: {error}"))?;
        usize::from_le_bytes(buffer)
    };
    let mut buffer = Vec::with_capacity(length_bytes);
    std::io::stdin()
        .read_to_end(&mut buffer)
        .map_err(|error| anyhow::anyhow!("Input reading error: {error}"))?;
    let input: EVMInput =
        ciborium::de::from_reader_with_recursion_limit(buffer.as_slice(), usize::MAX)
            .map_err(|error| anyhow::anyhow!("Input deserialziing error: {error}"))?;

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

    ciborium::into_writer(&result, &mut std::io::stdout())
        .map_err(|error| anyhow::anyhow!("Result serializing and writing error: {error}"))?;
    unsafe { inkwell::support::shutdown_llvm() };
    Ok(())
}

///
/// Runs this process recursively to compile a single contract.
///
pub fn call<I, O>(path: &str, input: &I) -> crate::Result<O>
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
        .unwrap_or_else(|error| panic!("{executable:?} subprocess spawning error: {error:?}"));

    let stdin = process
        .stdin
        .as_mut()
        .unwrap_or_else(|| panic!("{executable:?} subprocess stdin getting error"));
    let mut buffer = Vec::with_capacity(crate::r#const::DEFAULT_SERDE_BUFFER_SIZE);
    ciborium::into_writer(input, &mut buffer).unwrap_or_else(|error| {
        panic!("{executable:?} subprocess input serializing error: {error:?}")
    });
    stdin
        .write_all(buffer.len().to_le_bytes().as_slice())
        .unwrap_or_else(|error| {
            panic!("{executable:?} subprocess length prefix writing error: {error:?}")
        });
    stdin
        .write_all(buffer.as_slice())
        .unwrap_or_else(|error| panic!("{executable:?} subprocess input writing error: {error:?}"));

    let result = process.wait_with_output().unwrap_or_else(|error| {
        panic!("{executable:?} subprocess output reading error: {error:?}")
    });

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

    match ciborium::de::from_reader_with_recursion_limit(result.stdout.as_slice(), usize::MAX) {
        Ok(output) => output,
        Err(error) => {
            panic!(
                "{executable:?} subprocess stdout deserializing error: {error:?}\n{}\n{}",
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
/// This function is unsafe because it is called from the LLVM stackifier.
/// The function must terminate the process after handling the error.
///
pub unsafe extern "C" fn evm_stack_error_handler(spill_area_size: u64) {
    let result: Result<EVMOutput, Error> = Err(Error::stack_too_deep(
        spill_area_size,
        era_compiler_llvm_context::EVM_IS_SIZE_FALLBACK.load(std::sync::atomic::Ordering::Relaxed),
    ));
    let mut buffer = Vec::with_capacity(crate::r#const::DEFAULT_SERDE_BUFFER_SIZE);
    ciborium::into_writer(&result, &mut buffer)
        .unwrap_or_else(|error| panic!("Stdout stack-too-deep error serializing error: {error}"));
    std::io::stdout()
        .write_all(buffer.as_slice())
        .unwrap_or_else(|error| panic!("Stdout stack-too-deep error writing error: {error}"));
    unsafe { inkwell::support::shutdown_llvm() };
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
}
