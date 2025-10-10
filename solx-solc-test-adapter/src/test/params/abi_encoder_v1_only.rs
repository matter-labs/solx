//!
//! ABIEncoderV1Only param values.
//!

///
/// ABIEncoderV1Only param values.
///
#[derive(Debug, PartialEq, Eq)]
pub enum ABIEncoderV1Only {
    /// `true` in the metadata.
    True,
    /// not specified in the metadata.
    Default,
}

impl TryFrom<&str> for ABIEncoderV1Only {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "true" => ABIEncoderV1Only::True,
            word => anyhow::bail!(r#"Expected "true" as ABIEncoderV1Only value, found: {word}"#),
        })
    }
}
