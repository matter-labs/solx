//!
//! The `solx` LLVM builder library.
//!

#![allow(clippy::too_many_arguments)]
#![allow(clippy::upper_case_acronyms)]

pub(crate) mod build_type;
pub(crate) mod ccache_variant;
pub(crate) mod llvm_path;
pub(crate) mod llvm_project;
pub(crate) mod lock;
pub(crate) mod platforms;
pub(crate) mod sanitizer;
pub(crate) mod utils;

pub use self::build_type::BuildType;
pub use self::ccache_variant::CcacheVariant;
pub use self::llvm_path::LLVMPath;
pub use self::llvm_project::LLVMProject;
pub use self::lock::Lock;
pub use self::sanitizer::Sanitizer;
pub use self::utils::exists as executable_exists;

use std::collections::HashSet;

///
/// Executes the building of the LLVM framework for the platform determined by the cfg macro.
/// Since cfg is evaluated at compile time, overriding the platform with a command-line
/// argument is not possible. So for cross-platform testing, comment out all but the
/// line to be tested, and perhaps also checks in the platform-specific build method.
///
pub fn build(
    build_type: BuildType,
    llvm_projects: HashSet<llvm_project::LLVMProject>,
    enable_rtti: bool,
    enable_tests: bool,
    enable_coverage: bool,
    extra_args: Vec<String>,
    ccache_variant: Option<ccache_variant::CcacheVariant>,
    enable_assertions: bool,
    sanitizer: Option<sanitizer::Sanitizer>,
    enable_valgrind: bool,
    valgrind_options: Vec<String>,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(LLVMPath::DIRECTORY_LLVM_TARGET)?;

    if cfg!(target_arch = "x86_64") {
        if cfg!(target_os = "linux") {
            platforms::x86_64_linux_gnu::build(
                build_type,
                llvm_projects,
                enable_rtti,
                enable_tests,
                enable_coverage,
                extra_args,
                ccache_variant,
                enable_assertions,
                sanitizer,
                enable_valgrind,
                valgrind_options,
            )?;
        } else if cfg!(target_os = "macos") {
            platforms::x86_64_macos::build(
                build_type,
                llvm_projects,
                enable_rtti,
                enable_tests,
                enable_coverage,
                extra_args,
                ccache_variant,
                enable_assertions,
                sanitizer,
            )?;
        } else if cfg!(target_os = "windows") {
            platforms::x86_64_windows_gnu::build(
                build_type,
                llvm_projects,
                enable_rtti,
                enable_tests,
                enable_coverage,
                extra_args,
                ccache_variant,
                enable_assertions,
                sanitizer,
            )?;
        } else {
            anyhow::bail!("Unsupported target OS for x86_64");
        }
    } else if cfg!(target_arch = "aarch64") {
        if cfg!(target_os = "linux") {
            platforms::aarch64_linux_gnu::build(
                build_type,
                llvm_projects,
                enable_rtti,
                enable_tests,
                enable_coverage,
                extra_args,
                ccache_variant,
                enable_assertions,
                sanitizer,
                enable_valgrind,
                valgrind_options,
            )?;
        } else if cfg!(target_os = "macos") {
            platforms::aarch64_macos::build(
                build_type,
                llvm_projects,
                enable_rtti,
                enable_tests,
                enable_coverage,
                extra_args,
                ccache_variant,
                enable_assertions,
                sanitizer,
            )?;
        } else {
            anyhow::bail!("Unsupported target OS for aarch64");
        }
    } else {
        anyhow::bail!("Unsupported target architecture");
    }

    Ok(())
}
