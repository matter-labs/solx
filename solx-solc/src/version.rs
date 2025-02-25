//!
//! The Solidity compiler version.
//!

///
/// The Solidity compiler version.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Version {
    /// The long version string.
    pub long: String,
    /// The short `semver`.
    pub default: semver::Version,
    /// The LLVM revision additional versioning.
    pub llvm_revision: semver::Version,
}

impl Version {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(long: String, default: semver::Version, llvm_revision: semver::Version) -> Self {
        Self {
            long,
            default,
            llvm_revision,
        }
    }
}
