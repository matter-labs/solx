//!
//! The ZKsync LLVM builder lock file.
//!

use anyhow::Context;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

///
/// The lock file data.
///
/// This file describes the exact reference of the LLVM framework.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Lock {
    /// The LLVM repository URL.
    pub url: String,
    /// The LLVM repository branch.
    pub branch: String,
    /// The LLVM repository commit reference.
    pub r#ref: Option<String>,
}

impl TryFrom<&PathBuf> for Lock {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut config_str = String::new();
        let mut config_file =
            File::open(path).with_context(|| format!("Error opening {path:?} file"))?;
        config_file.read_to_string(&mut config_str)?;
        Ok(toml::from_str(&config_str)?)
    }
}
