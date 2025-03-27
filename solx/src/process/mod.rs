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
        solx_solc::StandardJsonOutputErrorSourceLocation::new(input.contract.name.path.clone());

    let result = Builder::new()
        .stack_size(crate::WORKER_THREAD_STACK_SIZE)
        .spawn(move || {
            input
                .contract
                .compile_to_evm(
                    input.identifier_paths,
                    input.output_bytecode,
                    input.deployed_libraries,
                    input.metadata_hash_type,
                    input.optimizer_settings,
                    input.llvm_options,
                    input.debug_config,
                )
                .map(EVMOutput::new)
                .map_err(|error| {
                    solx_solc::StandardJsonOutputError::new_error(
                        None,
                        error,
                        Some(source_location),
                        None,
                    )
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
            "{executable:?} subprocess failed with exit code {:?}:\n{}\n{}",
            result.status.code(),
            String::from_utf8_lossy(result.stdout.as_slice()),
            String::from_utf8_lossy(result.stderr.as_slice()),
        );
        return Err(solx_solc::StandardJsonOutputError::new_error(
            None,
            message,
            Some(solx_solc::StandardJsonOutputErrorSourceLocation::new(
                path.to_owned(),
            )),
            None,
        ));
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
