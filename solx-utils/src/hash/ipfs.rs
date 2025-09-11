//!
//! IPFS hash utilities.
//!

use base58::FromBase58;

///
/// IPFS hash utilities.
///
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IPFS {
    /// Binary representation.
    #[serde(with = "serde_arrays")]
    bytes: [u8; 2 + crate::BYTE_LENGTH_FIELD],
    /// Base58 string representation.
    string_base58: String,
    /// Hexadecimal string representation.
    string_hex: String,
}

impl IPFS {
    ///
    /// Computes the IPFS hash for `preimage`.
    ///
    pub fn from_slice(preimage: &[u8]) -> Self {
        let hasher = ipfs_hasher::IpfsHasher::default();
        let string_base58 = hasher.compute(preimage);
        let bytes = string_base58
            .from_base58()
            .expect("Base58 conversion is always valid")
            .try_into()
            .expect("The size is always correct");
        let string_hex = hex::encode(bytes);
        Self {
            bytes,
            string_base58,
            string_hex,
        }
    }

    ///
    /// Returns a reference to the 34-byte IPFS hash.
    ///
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    ///
    /// Returns a reference to the string representation of the IPFS hash.
    ///
    pub fn as_base58_str(&self) -> &str {
        self.string_base58.as_str()
    }

    ///
    /// Returns a reference to the hexadecimal string representation of the IPFS hash.
    ///
    pub fn as_hex_str(&self) -> &str {
        self.string_hex.as_str()
    }

    ///
    /// Extracts the binary representation.
    ///
    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
}

impl std::fmt::Display for IPFS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_hex_str())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn base58() {
        assert_eq!(
            super::IPFS::from_slice("LLVM is the Endgame".as_bytes()).as_base58_str(),
            "QmUriSv3NW33gtfyALPtD83tr1J88tQ4tGwLEVfH1wb92U"
        );
    }

    #[test]
    fn hexadecimal() {
        assert_eq!(
            super::IPFS::from_slice("LLVM is the Endgame".as_bytes()).as_hex_str(),
            "122060d9c5c201b8c2ed8ac8de1e21afc4ef115ad9f47863e21ffd29f272544b5125"
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            super::IPFS::from_slice("LLVM is the Endgame".as_bytes()).to_string(),
            "122060d9c5c201b8c2ed8ac8de1e21afc4ef115ad9f47863e21ffd29f272544b5125"
        );
    }
}
