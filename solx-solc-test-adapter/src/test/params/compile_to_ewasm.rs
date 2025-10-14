//!
//! compileToEwasm param values.
//!

///
/// compileToEwasm param values.
///
#[derive(Debug, PartialEq, Eq)]
pub enum CompileToEwasm {
    /// `also` in the metadata.
    Also,
    /// `false` in the metadata.
    False,
    /// not specified in the metadata.
    Default,
}

impl TryFrom<&str> for CompileToEwasm {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "also" => CompileToEwasm::Also,
            "false" => CompileToEwasm::False,
            word => anyhow::bail!(
                r#"Expected "also", or "false" as compileToEwasm value, found: {word}"#
            ),
        })
    }
}
