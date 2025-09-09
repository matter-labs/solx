//!
//! CBOR utilities.
//!

///
/// CBOR payload.
///
/// Used for encoding IPFS contract metadata hash.
///
#[derive(Debug, Clone, PartialEq)]
pub struct CBOR<'a, S>
where
    S: ToString,
{
    /// Hash type name and hash itself in binary representation.
    pub hash: Option<(S, &'a [u8])>,
    /// Key of the version field.
    pub version_key: String,
    /// Version data to be encoded in the `version_key` field.
    pub version_data: Vec<(String, semver::Version)>,
}

impl<'a, S> CBOR<'a, S>
where
    S: ToString,
{
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        hash: Option<(S, &'a [u8])>,
        version_key: String,
        version_data: Vec<(String, semver::Version)>,
    ) -> Self {
        assert!(!version_data.is_empty(), "Version data cannot be empty");

        Self {
            hash,
            version_key,
            version_data,
        }
    }

    ///
    /// Returns a CBOR-encoded vector.
    ///
    pub fn to_vec(&self) -> Vec<u8> {
        let field_count = (self.hash.is_some() as usize) + 1;
        let mut cbor = Vec::with_capacity(64);
        cbor.push(0xA0_u8 + (field_count as u8));

        if let Some((r#type, hash)) = self.hash.as_ref() {
            cbor.push(0x64_u8);
            cbor.extend(r#type.to_string().as_bytes());
            cbor.push(0x58_u8);
            cbor.push(hash.len() as u8);
            cbor.extend_from_slice(hash);
        }

        cbor.push(0x60_u8 + (self.version_key.len() as u8));
        cbor.extend(self.version_key.as_bytes());
        let version_data = self
            .version_data
            .iter()
            .map(|(name, version)| format!("{name}:{version}"))
            .collect::<Vec<String>>()
            .join(";");
        cbor.push(0x78_u8);
        cbor.push(version_data.len() as u8);
        cbor.extend(version_data.as_bytes());

        cbor.extend((cbor.len() as u16).to_be_bytes());
        cbor
    }
}

#[cfg(test)]
mod tests {
    use crate::IPFSHash;
    use crate::MetadataHashType;

    #[test]
    #[should_panic(expected = "Version data cannot be empty")]
    fn none() {
        let cbor = super::CBOR::<'_, String>::new(None, "solc".to_owned(), vec![]);

        assert_eq!(hex::encode(cbor.to_vec()), "a164736f6c63600007");
    }

    #[test]
    fn none_and_solx_version_data() {
        let cbor = super::CBOR::<'_, String>::new(
            None,
            "solc".to_owned(),
            vec![("solx".to_owned(), semver::Version::new(0, 8, 29))],
        );

        assert_eq!(
            hex::encode(cbor.to_vec()),
            "a164736f6c63780b736f6c783a302e382e32390013"
        );
    }

    #[test]
    #[should_panic(expected = "Version data cannot be empty")]
    fn ipfs_hash_only() {
        let hash_type = MetadataHashType::IPFS.to_string();
        let ipfs = IPFSHash::from_slice("LLVM is the Endgame".as_bytes());

        let cbor = super::CBOR::new(
            Some((hash_type.as_str(), ipfs.as_bytes())),
            "solc".to_owned(),
            vec![],
        );

        assert_eq!(
            hex::encode(cbor.to_vec()),
            "a264697066735822122060d9c5c201b8c2ed8ac8de1e21afc4ef115ad9f47863e21ffd29f272544b512564736f6c63600030",
        );
    }

    #[test]
    fn ipfs_hash_and_zksolc_version_data() {
        let hash_type = MetadataHashType::IPFS.to_string();
        let ipfs = IPFSHash::from_slice("LLVM is the Endgame".as_bytes());

        let cbor = super::CBOR::new(
            Some((hash_type.as_str(), ipfs.as_bytes())),
            "solc".to_owned(),
            vec![("zksolc".to_owned(), semver::Version::new(0, 8, 29))],
        );

        assert_eq!(
            hex::encode(cbor.to_vec()),
            "a264697066735822122060d9c5c201b8c2ed8ac8de1e21afc4ef115ad9f47863e21ffd29f272544b512564736f6c63780d7a6b736f6c633a302e382e3239003e"
        );
    }

    #[test]
    fn ipfs_hash_and_zkvyper_version_data() {
        let hash_type = MetadataHashType::IPFS.to_string();
        let ipfs = IPFSHash::from_slice("LLVM is the Endgame".as_bytes());

        let cbor = super::CBOR::new(
            Some((hash_type.as_str(), ipfs.as_bytes())),
            "vyper".to_owned(),
            vec![
                ("zkvyper".to_owned(), semver::Version::new(1, 5, 10)),
                ("vyper".to_owned(), semver::Version::new(0, 4, 1)),
            ],
        );

        assert_eq!(
            hex::encode(cbor.to_vec()),
            "a264697066735822122060d9c5c201b8c2ed8ac8de1e21afc4ef115ad9f47863e21ffd29f272544b5125657679706572781a7a6b76797065723a312e352e31303b76797065723a302e342e31004c"
        );
    }

    #[test]
    fn ipfs_hash_and_extended_version_data() {
        let hash_type = MetadataHashType::IPFS.to_string();
        let ipfs = IPFSHash::from_slice("LLVM is the Endgame".as_bytes());

        let cbor = super::CBOR::new(
            Some((hash_type.as_str(), ipfs.as_bytes())),
            "solc".to_owned(),
            vec![
                ("solx".to_string(), semver::Version::new(0, 1, 0)),
                ("solc".to_string(), semver::Version::new(0, 8, 29)),
                ("llvm".to_string(), semver::Version::new(1, 0, 2)),
            ],
        );

        assert_eq!(
            hex::encode(cbor.to_vec()),
            "a264697066735822122060d9c5c201b8c2ed8ac8de1e21afc4ef115ad9f47863e21ffd29f272544b512564736f6c637821736f6c783a302e312e303b736f6c633a302e382e32393b6c6c766d3a312e302e320052"
        );
    }
}
