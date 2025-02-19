//!
//! The default build script for `solx`.
//!

///
/// Links solc and Boost libraries statically.
///
fn main() {
    // Re-run if the `Boost` path environment variable is changed.
    println!("cargo:rerun-if-env-changed={}", env!("BOOST_PREFIX"));
    // Re-run if the `solc` path environment variable is changed.
    println!("cargo:rerun-if-env-changed={}", env!("SOLC_PREFIX"));
    // Re-run if the `solc` directory contents are changed.
    if let Ok(path) = std::env::var("SOLC_PREFIX") {
        println!("cargo:rerun-if-changed={}", path);
    }

    // Where to find Boost libraries.
    println!("cargo:rustc-link-search=native={}", env!("BOOST_PREFIX"));
    // Where to find solc libraries.
    for directory in [
        "libsolc",
        "libsolidity",
        "libsolutil",
        "liblangutil",
        "libevmasm",
        "libyul",
        "libsmtutil",
    ] {
        println!(
            "cargo:rustc-link-search=native={}/{directory}",
            env!("SOLC_PREFIX"),
        );
    }

    // Link with Boost libraries.
    for library in ["boost_filesystem", "boost_system", "boost_program_options"] {
        println!("cargo:rustc-link-lib=static={library}");
    }
    // Link with solc libraries.
    for library in [
        "solc", "solidity", "solutil", "langutil", "evmasm", "yul", "smtutil",
    ] {
        println!("cargo:rustc-link-lib=static={library}");
    }
}
