//!
//! The ZKsync LLVM builder arguments.
//!

use clap::Parser;

///
/// The ZKsync LLVM builder arguments.
///
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Enables verbose output, e.g. to inspect extra flags.
    #[arg(short, long)]
    pub verbose: bool,

    /// LLVM build type (`Debug`, `Release`, `RelWithDebInfo`, or `MinSizeRel`).
    #[arg(long, default_value_t = solx_llvm_builder::BuildType::Release)]
    pub build_type: solx_llvm_builder::BuildType,

    /// LLVM projects to build LLVM with.
    #[arg(long)]
    pub llvm_projects: Vec<solx_llvm_builder::LLVMProject>,

    /// Whether to build LLVM with run-time type information (RTTI) enabled.
    #[arg(long)]
    pub enable_rtti: bool,

    /// Whether to build the LLVM tests.
    #[arg(long)]
    pub enable_tests: bool,

    /// Whether to build LLVM for source-based code coverage.
    #[arg(long)]
    pub enable_coverage: bool,

    /// Extra arguments to pass to CMake.  
    /// A leading backslash will be unescaped.
    #[arg(long, num_args = 1..)]
    pub extra_args: Vec<String>,

    /// Whether to use compiler cache (ccache) to speed-up builds.
    #[arg(long)]
    pub ccache_variant: Option<solx_llvm_builder::CcacheVariant>,

    /// Whether to build with assertions enabled or not.
    #[arg(long)]
    pub enable_assertions: bool,

    /// Build LLVM with sanitizer enabled (`Address`, `Memory`, `MemoryWithOrigins`, `Undefined`, `Thread`, `DataFlow`, or `Address;Undefined`).
    #[arg(long)]
    pub sanitizer: Option<solx_llvm_builder::Sanitizer>,

    /// Whether to run LLVM unit tests under valgrind or not.
    #[arg(long)]
    pub enable_valgrind: bool,

    /// Additional valgrind options to pass to the valgrind command.
    #[arg(long)]
    pub valgrind_options: Vec<String>,
}
