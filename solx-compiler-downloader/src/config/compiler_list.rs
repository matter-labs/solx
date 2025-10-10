//!
//! The compiler JSON list metadata.
//!

use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;

use colored::Colorize;

///
/// The compiler JSON list metadata.
///
#[derive(Debug, serde::Deserialize)]
pub struct CompilerList {
    /// The collection of compiler releases.
    pub releases: BTreeMap<String, String>,
}

impl TryFrom<&Path> for CompilerList {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let url =
            reqwest::Url::from_str(path.to_str().expect("Always valid")).expect("Always valid");
        println!(
            " {} compiler bin JSON `{url}`",
            "Downloading".bright_green().bold(),
        );
        let list: CompilerList = reqwest::blocking::get(url)?.json()?;
        Ok(list)
    }
}
