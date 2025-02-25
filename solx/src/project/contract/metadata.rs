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
    /// The source code metadata.
    pub source_metadata: serde_json::Value,
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
        source_metadata: serde_json::Value,
        solc_version: semver::Version,
        solc_llvm_edition: semver::Version,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: &'a [String],
    ) -> Self {
        let source_metadata = match source_metadata {
            serde_json::Value::String(inner) => {
                let value = serde_json::from_str(inner.as_str()).expect("Always valid");
                serde_json::Value::Object(value)
            }
            value => value,
        };
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
