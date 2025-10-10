//!
//! Identifier for the test input. Describes the input type and position but not the actual contents.
//!

///
/// Identifier for the test input. Describes the input type and position but not the actual contents.
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InputIdentifier {
    /// The contract deploy, regardless of target.
    Deployer { contract_identifier: String },
    /// The fallback method.
    Fallback { input_index: usize },
    /// The contract call.
    Runtime { input_index: usize, name: String },
    /// The storage empty check.
    StorageEmpty { input_index: usize },
    /// Check account balance.
    Balance { input_index: usize },
}

impl std::fmt::Display for InputIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputIdentifier::Deployer {
                contract_identifier,
            } => f.write_fmt(format_args!("#deployer:{contract_identifier}")),
            InputIdentifier::Runtime { input_index, name } => {
                f.write_fmt(format_args!("{name}:{input_index}"))
            }
            InputIdentifier::StorageEmpty { input_index } => {
                f.write_fmt(format_args!("#storage_empty_check:{input_index}"))
            }
            InputIdentifier::Balance { input_index } => {
                f.write_fmt(format_args!("#balance_check:{input_index}"))
            }
            InputIdentifier::Fallback { input_index } => {
                f.write_fmt(format_args!("#fallback:{input_index}"))
            }
        }
    }
}
