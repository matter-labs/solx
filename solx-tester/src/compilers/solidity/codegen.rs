//!
//! The Solidity compiler codegen.
//!

use std::str::FromStr;

///
/// The Solidity compiler codegen.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Codegen {
    /// The Yul IR.
    Yul,
    /// The EVM legacy assembly IR.
    EVMLA,
}

impl FromStr for Codegen {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "yul" => Ok(Self::Yul),
            "evmla" => Ok(Self::EVMLA),
            string => anyhow::bail!(
                "Invalid codegen: `{string}`. Available options: {}.",
                [Self::EVMLA, Self::Yul]
                    .into_iter()
                    .map(|codegen| codegen.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl std::fmt::Display for Codegen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Yul => write!(f, "yul"),
            Self::EVMLA => write!(f, "evmla"),
        }
    }
}
