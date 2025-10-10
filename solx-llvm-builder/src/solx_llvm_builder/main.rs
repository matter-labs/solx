//!
//! The `solx` LLVM builder.
//!

pub(crate) mod arguments;

use std::collections::HashSet;
use std::str::FromStr;

use clap::Parser;

use self::arguments::Arguments;

/// The default path to the LLVM lock file.
pub const LLVM_LOCK_DEFAULT_PATH: &str = "LLVM.lock";

///
/// The entry.
///
fn main() {
    match main_inner() {
        Ok(()) => std::process::exit(0),
        Err(error) => {
            eprintln!("Error: {error:?}");
            std::process::exit(1)
        }
    }
}

///
/// The entry result wrapper.
///
fn main_inner() -> anyhow::Result<()> {
    let arguments = Arguments::parse();

    let extra_args_unescaped: Vec<String> = arguments
        .extra_args
        .iter()
        .map(|argument| {
            argument
                .strip_prefix('\\')
                .unwrap_or(argument.as_str())
                .to_owned()
        })
        .collect();
    if arguments.verbose {
        println!("\nextra_args: {:#?}", arguments.extra_args);
        println!("\nextra_args_unescaped: {extra_args_unescaped:#?}");
    }

    if let Some(ccache_variant) = arguments.ccache_variant {
        solx_llvm_builder::executable_exists(ccache_variant.to_string().as_str())?;
    }

    let mut projects = arguments
        .llvm_projects
        .into_iter()
        .map(|project| solx_llvm_builder::LLVMProject::from_str(project.to_string().as_str()))
        .collect::<Result<HashSet<solx_llvm_builder::LLVMProject>, String>>()
        .map_err(|project| anyhow::anyhow!("Unknown LLVM project `{project}`"))?;
    projects.insert(solx_llvm_builder::LLVMProject::LLD);

    solx_llvm_builder::build(
        arguments.build_type,
        projects,
        arguments.enable_rtti,
        arguments.enable_tests,
        arguments.enable_coverage,
        extra_args_unescaped,
        arguments.ccache_variant,
        arguments.enable_assertions,
        arguments.sanitizer,
        arguments.enable_valgrind,
        arguments.valgrind_options,
    )?;

    Ok(())
}
