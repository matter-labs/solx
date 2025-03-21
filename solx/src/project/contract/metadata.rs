//!
//! The contract metadata.
//!

///
/// The contract metadata.
///
/// Is used to append the metadata hash to the contract bytecode.
///
#[derive(Debug, serde::Serialize)]
pub struct Metadata<'a> {
    /// The original `solc` metadata.
    pub source_metadata: String,
    /// The `solc` version.
    pub solc_version: semver::Version,
    /// The LLVM `solc` edition.
    pub solc_llvm_edition: semver::Version,
    /// The LLVM compiler version.
    pub version: semver::Version,
    /// The LLVM compiler optimizer settings.
    pub optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    /// The LLVM extra arguments.
    pub llvm_options: &'a [String],
}

impl<'a> Metadata<'a> {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        source_metadata: String,
        solc_version: semver::Version,
        solc_llvm_edition: semver::Version,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: &'a [String],
    ) -> Self {
        Self {
            source_metadata,
            solc_version,
            solc_llvm_edition,
            version: crate::version().parse().expect("Always valid"),
            optimizer_settings,
            llvm_options,
        }
    }
}
