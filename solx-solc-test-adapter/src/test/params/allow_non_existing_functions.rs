//!
//! allowNonExistingFunctions param values.
//!

///
/// allowNonExistingFunctions param values.
///
#[derive(Debug, PartialEq, Eq)]
pub enum AllowNonExistingFunctions {
    /// `true` in the metadata.
    True,
    /// not specified in the metadata.
    Default,
}

impl TryFrom<&str> for AllowNonExistingFunctions {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "true" => AllowNonExistingFunctions::True,
            word => anyhow::bail!(
                r#"Expected "true" as allowNonExistingFunctions value, found: {word}"#
            ),
        })
    }
}
