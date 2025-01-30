//!
//! The default build script for `solx`.
//!

///
/// Links `libsolc` statically.
///
fn main() {
    // Tell Cargo where to find static libraries.
    println!("cargo:rustc-link-search=native=/Users/hedgarmac/src/era-solidity/build/libsolc/");
    println!("cargo:rustc-link-search=native=/Users/hedgarmac/src/era-solidity/build/libsolidity/");
    println!("cargo:rustc-link-search=native=/Users/hedgarmac/src/era-solidity/build/libsolutil/");
    println!("cargo:rustc-link-search=native=/Users/hedgarmac/src/era-solidity/build/liblangutil/");
    println!("cargo:rustc-link-search=native=/Users/hedgarmac/src/era-solidity/build/libevmasm/");
    println!("cargo:rustc-link-search=native=/Users/hedgarmac/src/era-solidity/build/libyul/");
    println!("cargo:rustc-link-search=native=/Users/hedgarmac/src/era-solidity/build/libsmtutil/");
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib/");

    // Link against the static libraries.
    println!("cargo:rustc-link-lib=static=solc");
    println!("cargo:rustc-link-lib=static=solidity");
    println!("cargo:rustc-link-lib=static=solutil");
    println!("cargo:rustc-link-lib=static=langutil");
    println!("cargo:rustc-link-lib=static=evmasm");
    println!("cargo:rustc-link-lib=static=yul");
    println!("cargo:rustc-link-lib=static=smtutil");

    // Link the Boost libraries.
    println!("cargo:rustc-link-lib=static=boost_filesystem");
    println!("cargo:rustc-link-lib=static=boost_system");
    println!("cargo:rustc-link-lib=static=boost_program_options");

    // Link the C++ standard library.
    // TODO: On MacOS + Clang, it is typically `c++`. On Linux with GCC, might be "stdc++".
    println!("cargo:rustc-link-lib=c++");
}
