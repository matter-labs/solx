//!
//! The EVM string attribute.
//!

///
/// The EVM string attribute.
///
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Attribute {
    /// The corresponding value.
    EVMEntryFunction,
}

impl std::str::FromStr for Attribute {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "evm-entry-function" => Ok(Attribute::EVMEntryFunction),
            _ => anyhow::bail!("Unknown attribute: {string}"),
        }
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::EVMEntryFunction => write!(f, "evm-entry-function"),
        }
    }
}
