//!
//! EVM version param values.
//!

use regex::Regex;

///
/// EVM version param values.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVMVersion {
    /// Equals specified.
    Equals(EVM),
    /// Greater than specified.
    Greater(EVM),
    /// Lesser than specified.
    Lesser(EVM),
    /// Greater or equals than specified.
    GreaterEquals(EVM),
    /// Lesser or equals than specified.
    LesserEquals(EVM),
    /// Not specified.
    Default,
}

impl EVMVersion {
    ///
    /// Checks whether the specified version matches the requirement.
    ///
    pub fn matches(&self, version: &EVM) -> bool {
        match self {
            Self::Equals(inner) => version == inner,
            Self::Greater(inner) => version > inner,
            Self::Lesser(inner) => version < inner,
            Self::GreaterEquals(inner) => version >= inner,
            Self::LesserEquals(inner) => version <= inner,
            Self::Default => true,
        }
    }

    ///
    /// Checks whether the specified versions matches the requirement.
    ///
    pub fn matches_any(&self, versions: &[EVM]) -> bool {
        for version in versions.iter() {
            if self.matches(version) {
                return true;
            }
        }

        false
    }
}

impl TryFrom<&str> for EVMVersion {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let regex = Regex::new(r"^(=|>|<|>=|<=)(\w*)$").expect("Always valid");

        let captures = regex
            .captures(value)
            .ok_or_else(|| anyhow::anyhow!("Invalid EVM version description: {value}"))?;

        let symbol = captures.get(1).expect("Always exists").as_str();
        let version = captures.get(2).expect("Always exists").as_str();

        let version: EVM = version.try_into()?;

        Ok(match symbol {
            "=" => EVMVersion::Equals(version),
            ">" => EVMVersion::Greater(version),
            "<" => EVMVersion::Lesser(version),
            ">=" => EVMVersion::GreaterEquals(version),
            "<=" => EVMVersion::LesserEquals(version),
            _ => anyhow::bail!("Invalid symbol before EVM version: {symbol}"),
        })
    }
}

///
/// EVM version.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum EVM {
    /// Homestead EVM version.
    Homestead,
    /// TangerineWhistle EVM version.
    TangerineWhistle,
    /// SpuriousDragon EVM version.
    SpuriousDragon,
    /// Byzantium EVM version.
    Byzantium,
    /// Constantinople EVM version.
    Constantinople,
    /// Petersburg EVM version.
    Petersburg,
    /// Istanbul EVM version.
    Istanbul,
    /// Berlin EVM version.
    Berlin,
    /// London EVM version.
    London,
    /// Paris EVM version.
    Paris,
    /// Shanghai EVM version.
    Shanghai,
    /// Cancun EVM version.
    Cancun,
    /// Prague EVM version.
    Prague,
}

impl TryFrom<&str> for EVM {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "homestead" => EVM::Homestead,
            "tangerineWhistle" => EVM::TangerineWhistle,
            "spuriousDragon" => EVM::SpuriousDragon,
            "byzantium" => EVM::Byzantium,
            "constantinople" => EVM::Constantinople,
            "petersburg" => EVM::Petersburg,
            "istanbul" => EVM::Istanbul,
            "berlin" => EVM::Berlin,
            "london" => EVM::London,
            "paris" => EVM::Paris,
            "shanghai" => EVM::Shanghai,
            "cancun" => EVM::Cancun,
            _ => anyhow::bail!("Invalid EVM version: {value}"),
        })
    }
}
