//!
//! compileViaYul param values.
//!

///
/// compileViaYul param values.
///
#[derive(Debug, PartialEq, Eq)]
pub enum CompileViaYul {
    /// `also` in the metadata.
    Also,
    /// `true` in the metadata.
    True,
    /// `false` in the metadata.
    False,
    /// not specified in the metadata.
    Default,
}

impl TryFrom<&str> for CompileViaYul {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "also" => CompileViaYul::Also,
            "true" => CompileViaYul::True,
            "false" => CompileViaYul::False,
            word => anyhow::bail!(
                r#"Expected "also", "true", or "false" as compileViaYul, found: {word}"#
            ),
        })
    }
}
