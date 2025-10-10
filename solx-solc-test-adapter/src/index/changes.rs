//!
//! The tests changes.
//!

use std::path::PathBuf;

///
/// The tests changes.
///
#[derive(Debug, Default)]
pub struct Changes {
    /// Created tests.
    pub created: Vec<PathBuf>,
    /// Deleted tests.
    pub deleted: Vec<PathBuf>,
    /// Updated tests.
    pub updated: Vec<PathBuf>,
    /// Tests updated with conflicts.
    pub conflicts: Vec<PathBuf>,
}
