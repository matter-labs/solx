//!
//! The Ethereum tests directory.
//!

pub mod test;

use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::directories::Collection;
use crate::filters::Filters;
use crate::summary::Summary;

use self::test::EthereumTest;

///
/// The Ethereum tests directory.
///
pub struct EthereumDirectory;

impl EthereumDirectory {
    ///
    /// The upstream test index file name.
    ///
    /// This version if the index used for the REVM environment.
    ///
    const INDEX_NAME_UPSTREAM_SOLIDITY: &'static str = "solidity.yaml";

    ///
    /// Reads the Ethereum test index.
    ///
    pub fn read_index(index_path: &Path) -> anyhow::Result<solx_solc_test_adapter::FSEntity> {
        let index_data = std::fs::read_to_string(index_path)?;
        let index: solx_solc_test_adapter::FSEntity = serde_yaml::from_str(index_data.as_str())?;
        Ok(index)
    }
}

impl Collection for EthereumDirectory {
    type Test = EthereumTest;

    fn read_all(
        directory_path: &Path,
        _extension: &'static str,
        summary: Arc<Mutex<Summary>>,
        filters: &Filters,
    ) -> anyhow::Result<Vec<Self::Test>> {
        let index_path = PathBuf::from(Self::INDEX_NAME_UPSTREAM_SOLIDITY);

        Ok(Self::read_index(index_path.as_path())?
            .into_enabled_list(directory_path)
            .into_iter()
            .filter_map(|test| EthereumTest::new(test, summary.clone(), filters))
            .collect())
    }
}
