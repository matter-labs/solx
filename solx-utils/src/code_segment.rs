//!
//! Contract code segment.
//!

use std::str::FromStr;

///
/// Contract code segment.
///
/// On EraVM, the segments do not represent any entities in the final bytecode, but this separation is present
/// in IRs used for lowering.
///
/// On EVM, the segments represent deploy and runtime code segments without changes.
///
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub enum CodeSegment {
    /// The deploy code segment.
    Deploy,
    /// The runtime code segment.
    Runtime,
}

impl FromStr for CodeSegment {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "deploy" => Ok(Self::Deploy),
            "runtime" => Ok(Self::Runtime),
            string => anyhow::bail!("unknown code segment: `{string}`"),
        }
    }
}

impl std::fmt::Display for CodeSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deploy => write!(f, "deploy"),
            Self::Runtime => write!(f, "runtime"),
        }
    }
}
