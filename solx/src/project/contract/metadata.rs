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
    /// If `solc` is used in the compilation process, its metadata is included here.
    pub source_metadata: serde_json::Value,
    /// The `solc` version if used.
    pub solc_version: Option<semver::Version>,
    /// The ZKsync `solc` edition.
    pub solc_zkvm_edition: Option<semver::Version>,
    /// The ZKsync compiler version.
    pub zk_version: semver::Version,
    /// The ZKsync compiler optimizer settings.
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
        solc_version: Option<semver::Version>,
        solc_zkvm_edition: Option<semver::Version>,
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
            solc_zkvm_edition,
            zk_version: crate::version().parse().expect("Always valid"),
            optimizer_settings,
            llvm_options,
        }
    }
}
