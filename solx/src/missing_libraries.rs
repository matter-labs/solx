//!
//! The missing libraries.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

///
/// The missing libraries.
///
pub struct MissingLibraries {
    /// The libraries.
    pub contract_libraries: BTreeMap<String, BTreeSet<String>>,
}

impl MissingLibraries {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(contract_libraries: BTreeMap<String, BTreeSet<String>>) -> Self {
        Self { contract_libraries }
    }

    ///
    /// Writes the missing libraries to the standard JSON.
    ///
    pub fn write_to_standard_json(
        mut self,
        standard_json: &mut solx_solc::StandardJsonOutput,
        solc_version: Option<&solx_solc::Version>,
    ) {
        for (path, file) in standard_json.contracts.iter_mut() {
            for (name, contract) in file.iter_mut() {
                let full_name = format!("{path}:{name}");
                let missing_libraries = self.contract_libraries.remove(full_name.as_str());

                if let Some(missing_libraries) = missing_libraries {
                    contract.missing_libraries = missing_libraries;
                }
            }
        }

        if let Some(solc_version) = solc_version {
            standard_json.version = Some(solc_version.default.to_string());
            standard_json.long_version = Some(solc_version.long.to_owned());
        }
    }
}
