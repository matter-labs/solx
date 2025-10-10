//!
//! revertStrings param values.
//!

///
/// revertStrings param values.
///
#[derive(Debug, PartialEq, Eq)]
pub enum RevertStrings {
    /// `debug` in the metadata.
    Debug,
    /// not specified in the metadata.
    Default,
}

impl TryFrom<&str> for RevertStrings {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "debug" => RevertStrings::Debug,
            word => anyhow::bail!(r#"Expected "debug" as revertStrings value, found: {word}"#),
        })
    }
}

impl std::fmt::Display for RevertStrings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RevertStrings::Debug => write!(f, "debug"),
            RevertStrings::Default => write!(f, "default"),
        }
    }
}
