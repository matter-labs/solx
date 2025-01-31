//!
//! The default build script for `solx`.
//!

///
/// Links solc and Boost libraries statically.
///
fn main() {
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

    // Link against the static libraries.
    for library in [
        "solc", "solidity", "solutil", "langutil", "evmasm", "yul", "smtutil",
    ] {
        println!("cargo:rustc-link-lib=static={library}");
    }

    // Link the Boost libraries.
    for library in ["boost_filesystem", "boost_system", "boost_program_options"] {
        println!("cargo:rustc-link-lib=static={library}");
    }
}
