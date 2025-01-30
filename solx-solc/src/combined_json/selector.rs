//!
//! The `solc --combined-json` expected output selection flag.
//!

use std::str::FromStr;

///
/// The `solc --combined-json` expected output selection flag.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Selector {
    /// The ABI JSON.
    #[serde(rename = "abi")]
    ABI,
    /// The function signature hashes JSON.
    #[serde(rename = "hashes")]
    Hashes,
    /// The metadata.
    #[serde(rename = "metadata")]
    Metadata,
    /// The developer documentation.
    #[serde(rename = "devdoc")]
    Devdoc,
    /// The user documentation.
    #[serde(rename = "userdoc")]
    Userdoc,
    /// The storage layout.
    #[serde(rename = "storage-layout")]
    StorageLayout,
    /// The transient storage layout.
    #[serde(rename = "transient-storage-layout")]
    TransientStorageLayout,
    /// The AST JSON.
    #[serde(rename = "ast")]
    AST,
    /// The EVM assembly.
    #[serde(rename = "asm")]
    ASM,

    /// The assembly.
    #[serde(rename = "assembly", skip_serializing)]
    Assembly,

    /// The deploy bytecode.
    #[serde(rename = "bin", skip_serializing)]
    Bytecode,
    /// The runtime bytecode.
    #[serde(rename = "bin-runtime", skip_serializing)]
    BytecodeRuntime,
}

impl Selector {
    ///
    /// Converts the comma-separated CLI argument into an array of flags.
    ///
    pub fn from_cli(format: &str) -> Vec<anyhow::Result<Self>> {
        format
            .split(',')
            .map(|flag| Self::from_str(flag.trim()))
            .collect()
    }

    ///
    /// Whether the selector is available in `solc`.
    ///
    pub fn is_source_solc(&self) -> bool {
        !matches!(self, Self::Assembly)
    }
}

impl FromStr for Selector {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "abi" => Ok(Self::ABI),
            "hashes" => Ok(Self::Hashes),
            "metadata" => Ok(Self::Metadata),
            "devdoc" => Ok(Self::Devdoc),
            "userdoc" => Ok(Self::Userdoc),
            "storage-layout" => Ok(Self::StorageLayout),
            "transient-storage-layout" => Ok(Self::TransientStorageLayout),
            "ast" => Ok(Self::AST),
            "asm" => Ok(Self::ASM),
            "bin" => Ok(Self::Bytecode),
            "bin-runtime" => Ok(Self::BytecodeRuntime),

            "assembly" => Ok(Self::Assembly),

            selector => anyhow::bail!("{selector}"),
        }
    }
}

impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ABI => write!(f, "abi"),
            Self::Hashes => write!(f, "hashes"),
            Self::Metadata => write!(f, "metadata"),
            Self::Devdoc => write!(f, "devdoc"),
            Self::Userdoc => write!(f, "userdoc"),
            Self::StorageLayout => write!(f, "storage-layout"),
            Self::TransientStorageLayout => write!(f, "transient-storage-layout"),
            Self::AST => write!(f, "ast"),
            Self::ASM => write!(f, "asm"),
            Self::Bytecode => write!(f, "bin"),
            Self::BytecodeRuntime => write!(f, "bin-runtime"),

            Self::Assembly => write!(f, "assembly"),
        }
    }
}
