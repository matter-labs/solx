//!
//! A run of a test with fixed compiler options (mode).
//!

use serde::Deserialize;
use serde::Serialize;

///
/// A run of a test with fixed compiler options (mode).
///
/// All fields are vectors to allow for multiple measurements with averaging capabilities.
///
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Run {
    /// Contract deploy code size.
    #[serde(default)]
    pub size: Vec<u64>,
    /// Contract runtime code size.
    #[serde(default)]
    pub runtime_size: Vec<u64>,
    /// Amount of EVM gas.
    #[serde(default)]
    pub gas: Vec<u64>,
}

impl Run {
    ///
    /// Extends the run with another run, averaging the values.
    ///
    pub fn extend(&mut self, other: &Self) {
        self.size.extend_from_slice(other.size.as_slice());
        self.runtime_size
            .extend_from_slice(other.runtime_size.as_slice());
        self.gas
            .extend(other.gas.iter().filter(|value| value < &&(u32::MAX as u64)));
    }

    ///
    /// Average contract size.
    ///
    pub fn average_size(&self) -> u64 {
        if self.size.is_empty() {
            return 0;
        }

        self.size.iter().sum::<u64>() / (self.size.len() as u64)
    }

    ///
    /// Average runtime code size.
    ///
    pub fn average_runtime_size(&self) -> u64 {
        if self.runtime_size.is_empty() {
            return 0;
        }

        self.runtime_size.iter().sum::<u64>() / (self.runtime_size.len() as u64)
    }

    ///
    /// Average amount of EVM gas.
    ///
    pub fn average_gas(&self) -> u64 {
        if self.gas.is_empty() {
            return 0;
        }

        self.gas.iter().sum::<u64>() / (self.gas.len() as u64)
    }
}
