//!
//! Solidity compiler arguments.
//!

use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use path_slash::PathExt;

///
/// Solidity compiler arguments.
///
#[derive(Debug, Parser)]
#[command(about, long_about = None, arg_required_else_help = true)]
pub struct Arguments {
    /// Print the version and exit.
    #[arg(long)]
    pub version: bool,

    /// Specify the input paths and remappings.
    /// If an argument contains a '=', it is considered a remapping.
    pub inputs: Vec<String>,

    /// Set the given path as the root of the source tree instead of the root of the filesystem.
    /// Passed to `solc` without changes.
    #[arg(long)]
    pub base_path: Option<String>,

    /// Make an additional source directory available to the default import callback.
    /// Can be used multiple times. Can only be used if the base path has a non-empty value.
    /// Passed to `solc` without changes.
    #[arg(long, num_args = 1..)]
    pub include_path: Vec<String>,

    /// Allow a given path for imports. A list of paths can be supplied by separating them with a comma.
    /// Passed to `solc` without changes.
    #[arg(long)]
    pub allow_paths: Option<String>,

    /// Create one file per component and contract/file at the specified directory, if given.
    #[arg(short, long)]
    pub output_dir: Option<PathBuf>,

    /// Overwrite existing files (used together with -o).
    #[arg(long)]
    pub overwrite: bool,

    /// Set the optimization parameter -O[0 | 1 | 2 | 3 | s | z].
    /// Use `3` for best performance and `z` for minimal size.
    #[arg(short = 'O', long)]
    pub optimization: Option<char>,

    /// Try to recompile with -Oz if the bytecode is too large.
    #[arg(long = "optimization-size-fallback")]
    pub size_fallback: bool,

    /// Pass arbitrary space-separated options to LLVM.
    /// The argument must be a single-quoted string following a `=` separator.
    /// Example: `--llvm-options='arg1 arg2 arg3 ... argN'`.
    #[arg(long)]
    pub llvm_options: Option<String>,

    /// EVM version `solc` will produce Yul or EVM assembly for.
    /// The default is chosen by `solc`.
    #[arg(long)]
    pub evm_version: Option<era_compiler_common::EVMVersion>,

    /// Specify addresses of deployable libraries. Syntax: `<libraryFullPath1>=<address1> ... <libraryFullPathN>=<addressN>`.
    /// Addresses are interpreted as hexadecimal strings prefixed with `0x`.
    #[arg(short, long, num_args = 1..)]
    pub libraries: Vec<String>,

    /// Switch to standard JSON input/output mode. Read from stdin or specified file, write the result to stdout.
    /// This is the default used by the Hardhat plugin.
    #[arg(long)]
    pub standard_json: Option<Option<String>>,

    /// Sets the number of threads, where each thread compiles its own translation unit in a child process.
    #[arg(short, long)]
    pub threads: Option<usize>,

    /// Switch to Yul mode.
    /// Only one input Yul file is allowed.
    /// Cannot be used with standard JSON mode.
    #[arg(long, alias = "strict-assembly")]
    pub yul: bool,

    /// Switch to LLVM IR mode.
    /// Only one input LLVM IR file is allowed.
    /// Cannot be used with standard JSON mode.
    /// Use this mode at your own risk, as LLVM IR input validation is not implemented.
    #[arg(long)]
    pub llvm_ir: bool,

    /// Enable the `solc` IR codegen.
    #[arg(long)]
    pub via_ir: bool,

    /// Set the metadata hash type.
    /// Available types: `none`, `ipfs`.
    /// The default is `ipfs`.
    #[arg(long)]
    pub metadata_hash: Option<era_compiler_common::EVMMetadataHashType>,

    /// Sets the literal content flag for contract metadata.
    /// If enabled, the metadata will contain the literal content of the source files.
    #[arg(long)]
    pub metadata_literal: bool,

    /// Turn off CBOR metadata at the end of bytecode.
    #[arg(long)]
    pub no_cbor_metadata: bool,

    /// Turn off the default `solc` import resolution callback.
    #[arg(long)]
    pub no_import_callback: bool,

    /// Emit bytecode of the compiled contracts.
    #[arg(long = "bin")]
    pub output_bytecode: bool,

    /// Emit deployed bytecode of the compiled contracts.
    #[arg(long = "bin-runtime")]
    pub output_bytecode_runtime: bool,

    /// Emit assembly of the compiled contracts.
    #[arg(long = "asm")]
    pub output_assembly: bool,

    /// Emit metadata of the compiled project.
    #[arg(long = "metadata")]
    pub output_metadata: bool,

    /// Emit ABI specification of the compiled project.
    #[arg(long = "abi")]
    pub output_abi: bool,

    /// Emit function signature hashes of the compiled project.
    #[arg(long = "hashes")]
    pub output_hashes: bool,

    /// Emit user documentation of the compiled project.
    #[arg(long = "userdoc")]
    pub output_userdoc: bool,

    /// Emit developer documentation of the compiled project.
    #[arg(long = "devdoc")]
    pub output_devdoc: bool,

    /// Emit storage layout of the compiled project.
    #[arg(long = "storage-layout")]
    pub output_storage_layout: bool,

    /// Emit storage layout of the compiled project.
    #[arg(long = "transient-storage-layout")]
    pub output_transient_storage_layout: bool,

    /// Emit AST of the compiled project.
    #[arg(long = "ast-json")]
    pub output_ast_json: bool,

    /// Emit solc's EVM assembly of the compiled project.
    #[arg(long = "asm-solc-json")]
    pub output_asm_solc_json: bool,

    /// Emit solc's optimized Yul IR of the compiled project.
    #[arg(long = "ir-optimized")]
    pub output_ir_optimized: bool,

    /// Dump all IRs to files in the specified directory.
    /// Only for testing and debugging.
    #[arg(long)]
    pub debug_output_dir: Option<PathBuf>,

    /// Set the verify-each option in LLVM.
    /// Only for testing and debugging.
    #[arg(long)]
    pub llvm_verify_each: bool,

    /// Set the debug-logging option in LLVM.
    /// Only for testing and debugging.
    #[arg(long)]
    pub llvm_debug_logging: bool,

    /// Run this process recursively and provide JSON input to compile a single contract.
    /// Only for usage from within the compiler.
    #[arg(long)]
    pub recursive_process: bool,
}

impl Arguments {
    ///
    /// Validates the arguments.
    ///
    pub fn validate(&self) -> Vec<solx_standard_json::OutputError> {
        let mut messages = vec![];

        if self.version && std::env::args().count() > 2 {
            messages.push(solx_standard_json::OutputError::new_error(
                None,
                "No other options are allowed while getting the compiler version.",
                None,
                None,
            ));
        }

        let modes_count = [self.yul, self.llvm_ir, self.standard_json.is_some()]
            .iter()
            .filter(|&&x| x)
            .count();
        if modes_count > 1 {
            messages.push(solx_standard_json::OutputError::new_error(
                None,
                "Only one mode is allowed at the same time: Yul, LLVM IR, standard JSON.",
                None,
                None,
            ));
        }

        if self.yul || self.llvm_ir {
            if self.base_path.is_some() {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "`base-path` is only allowed in Solidity mode.",
                    None,
                    None,
                ));
            }
            if !self.include_path.is_empty() {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "`include-path` is only allowed in Solidity mode.",
                    None,
                    None,
                ));
            }
            if self.allow_paths.is_some() {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "`allow-paths` is only allowed in Solidity mode.",
                    None,
                    None,
                ));
            }

            if self.output_abi
                || self.output_hashes
                || self.output_userdoc
                || self.output_devdoc
                || self.output_storage_layout
                || self.output_transient_storage_layout
                || self.output_ast_json
                || self.output_asm_solc_json
                || self.output_ir_optimized
            {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "ABI, hashes, userdoc, devdoc, storage layout, transient storage layout, AST, EVM assembly, Yul can be only emitted for Solidity contracts.",
                    None,
                    None,
                ));
            }

            if self.evm_version.is_some() {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "EVM version is only allowed in Solidity mode.",
                    None,
                    None,
                ));
            }

            if self.via_ir {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "IR codegen settings are only available in Solidity mode.",
                    None,
                    None,
                ));
            }
        }

        if self.standard_json.is_some() {
            if self.output_bytecode
                || self.output_bytecode_runtime
                || self.output_assembly
                || self.output_metadata
                || self.output_abi
                || self.output_hashes
                || self.output_userdoc
                || self.output_devdoc
                || self.output_storage_layout
                || self.output_transient_storage_layout
                || self.output_ast_json
                || self.output_asm_solc_json
                || self.output_ir_optimized
            {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "Cannot output data outside of JSON in standard JSON mode.",
                    None,
                    None,
                ));
            }

            if !self.inputs.is_empty() {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "Input files must be passed via standard JSON input.",
                    None,
                    None,
                ));
            }
            if !self.libraries.is_empty() {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "Libraries must be passed via standard JSON input.",
                    None,
                    None,
                ));
            }

            if self.via_ir {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "IR codegen must be passed via standard JSON input.",
                    None,
                    None,
                ));
            }
            if self.evm_version.is_some() {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "EVM version must be passed via standard JSON input.",
                    None,
                    None,
                ));
            }

            if self.output_dir.is_some() {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "Output directory cannot be used in standard JSON mode.",
                    None,
                    None,
                ));
            }
            if self.overwrite {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "Overwriting flag cannot be used in standard JSON mode.",
                    None,
                    None,
                ));
            }
            if self.optimization.is_some() {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "LLVM optimizations must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.size_fallback {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "Size optimization fallback must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.llvm_options.is_some() {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "LLVM options must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.metadata_hash.is_some() {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "Metadata hash mode must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.metadata_literal {
                messages.push(solx_standard_json::OutputError::new_error(
                    None,
                    "Metadata literal content flag must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
        }

        messages
    }

    ///
    /// Returns remappings from input paths.
    ///
    pub fn split_input_files_and_remappings(
        &self,
    ) -> anyhow::Result<(Vec<PathBuf>, BTreeSet<String>)> {
        let mut input_files = Vec::with_capacity(self.inputs.len());
        let mut remappings = BTreeSet::new();

        for input in self.inputs.iter() {
            if input.contains('=') {
                let mut parts = Vec::with_capacity(2);
                for path in input.trim().split('=') {
                    let path = PathBuf::from(path);
                    parts.push(
                        Self::path_to_posix(path.as_path())?
                            .to_string_lossy()
                            .to_string(),
                    );
                }
                if parts.len() != 2 {
                    anyhow::bail!(
                        "Invalid remapping `{}`: expected two parts separated by '='.",
                        input
                    );
                }
                remappings.insert(parts.join("="));
            } else {
                let path = PathBuf::from(input.trim());
                let path = Self::path_to_posix(path.as_path())?;
                input_files.push(path);
            }
        }

        Ok((input_files, remappings))
    }

    ///
    /// Normalizes an input path by converting it to POSIX format.
    ///
    fn path_to_posix(path: &Path) -> anyhow::Result<PathBuf> {
        let path = path
            .to_slash()
            .ok_or_else(|| anyhow::anyhow!("Input path {:?} POSIX conversion error.", path))?
            .to_string();
        let path = PathBuf::from(path.as_str());
        Ok(path)
    }
}
