//!
//! Solidity compiler constants.
//!

/// The worker thread stack size.
pub const WORKER_THREAD_STACK_SIZE: usize = 64 * 1024 * 1024;

///
/// The compiler version default function.
///
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}
