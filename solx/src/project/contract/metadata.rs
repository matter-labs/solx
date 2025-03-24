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
    /// The `solc` version.
    pub solc_version: semver::Version,
    /// The LLVM `solc` revision.
    pub solc_llvm_revision: semver::Version,
    /// The `solx` compiler version.
    pub solx_version: semver::Version,
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
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: &'a [String],
    ) -> Self {
        let solc_version = solx_solc::Compiler::default().version;
        Self {
            solc_version: solc_version.default,
            solc_llvm_revision: solc_version.llvm_revision,
            solx_version: crate::version().parse().expect("Always valid"),
            optimizer_settings,
            llvm_options,
        }
    }

    ///
    /// Inserts the metadata into the original `solc` object.
    ///
    pub fn insert_into(self, metadata_string: String) -> String {
        if metadata_string.is_empty() {
            return metadata_string;
        }

        let mut object: serde_json::Value =
            serde_json::from_str(metadata_string.as_str()).expect("Always valid");
        object.as_object_mut().expect("Always valid").insert(
            env!("CARGO_PKG_NAME").to_owned(),
            serde_json::to_value(self).expect("Always valid"),
        );
        serde_json::to_string(&object).expect("Always valid")
    }
}
