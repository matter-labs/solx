//!
//! EVM metadata hash type.
//!

use std::str::FromStr;

///
/// EVM metadata  hash type.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MetadataHashType {
    /// Do not include bytecode hash.
    #[serde(rename = "none")]
    None,
    /// The `ipfs` hash.
    #[serde(rename = "ipfs")]
    IPFS,
}

impl FromStr for MetadataHashType {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "none" => Ok(Self::None),
            "ipfs" => Ok(Self::IPFS),
            string => anyhow::bail!("unknown bytecode hash mode: `{string}`"),
        }
    }
}

impl std::fmt::Display for MetadataHashType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::IPFS => write!(f, "ipfs"),
        }
    }
}
