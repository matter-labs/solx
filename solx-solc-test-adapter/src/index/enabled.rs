//!
//! The enabled test entity description.
//!

use std::path::PathBuf;

///
/// The enabled test entity description.
///
#[derive(Debug, Clone)]
pub struct EnabledTest {
    /// The test path.
    pub path: PathBuf,
    /// The optimization modes which all the cases are enabled for.
    pub modes: Option<Vec<String>>,
    /// The compiler version the test must be run with.
    pub version: Option<semver::VersionReq>,
    /// The test group.
    pub group: Option<String>,
}

impl EnabledTest {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        path: PathBuf,
        modes: Option<Vec<String>>,
        version: Option<semver::VersionReq>,
        group: Option<String>,
    ) -> Self {
        Self {
            path,
            modes,
            version,
            group,
        }
    }
}
