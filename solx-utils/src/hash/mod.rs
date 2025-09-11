//!
//! Hash utilities.
//!

pub mod ipfs;
pub mod keccak256;

use self::ipfs::IPFS;
use self::keccak256::Keccak256;

///
/// Hash enum to make encoding easier.
///
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Hash {
    /// IPFS hash.
    IPFS(IPFS),
    /// Keccak256 hash.
    Keccak256(Keccak256),
}

impl Hash {
    ///
    /// Returns a reference to the hash bytes.
    ///
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Hash::IPFS(hash) => hash.as_bytes(),
            Hash::Keccak256(hash) => hash.as_bytes(),
        }
    }

    ///
    /// Returns a reference to the hash string.
    ///
    pub fn as_str(&self) -> &str {
        match self {
            Hash::IPFS(hash) => hash.as_hex_str(),
            Hash::Keccak256(hash) => hash.as_str(),
        }
    }
}

impl From<IPFS> for Hash {
    fn from(hash: IPFS) -> Self {
        Self::IPFS(hash)
    }
}

impl From<Keccak256> for Hash {
    fn from(hash: Keccak256) -> Self {
        Self::Keccak256(hash)
    }
}
