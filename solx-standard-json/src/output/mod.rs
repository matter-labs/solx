//!
//! The `solc --standard-json` output.
//!

pub mod contract;
pub mod error;
pub mod source;

use std::collections::BTreeMap;

use crate::input::settings::selection::selector::Selector as InputSettingsSelector;
use crate::input::settings::selection::Selection as InputSettingsSelection;
use crate::input::source::Source as InputSource;

use self::contract::Contract;
use self::error::collectable::Collectable as CollectableError;
use self::error::source_location::SourceLocation as JsonOutputErrorSourceLocation;
use self::error::Error as JsonOutputError;
use self::source::Source;

///
/// The `solc --standard-json` output.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Output {
    /// The file-contract hashmap.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub contracts: BTreeMap<String, BTreeMap<String, Contract>>,
    /// The source code mapping data.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub sources: BTreeMap<String, Source>,
    /// The compilation errors and warnings.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<JsonOutputError>,
}

impl Output {
    ///
    /// Initializes a standard JSON output.
    ///
    /// Is used for projects compiled without `solc`.
    ///
    pub fn new(
        sources: &BTreeMap<String, InputSource>,
        messages: &mut Vec<JsonOutputError>,
    ) -> Self {
        let sources = sources
            .keys()
            .enumerate()
            .map(|(index, path)| (path.to_owned(), Source::new(index)))
            .collect::<BTreeMap<String, Source>>();

        Self {
            contracts: BTreeMap::new(),
            sources,
            errors: std::mem::take(messages),
        }
    }

    ///
    /// Initializes a standard JSON output with messages.
    ///
    /// Is used to emit errors in standard JSON mode.
    ///
    pub fn new_with_messages(messages: Vec<JsonOutputError>) -> Self {
        Self {
            contracts: BTreeMap::new(),
            sources: BTreeMap::new(),
            errors: messages,
        }
    }

    ///
    /// Prunes the output JSON and prints it to stdout.
    ///
    pub fn write_and_exit(mut self, output_selection: &InputSettingsSelection) -> ! {
        for (path, file) in self.contracts.iter_mut() {
            for (name, contract) in file.iter_mut() {
                if !output_selection.check_selection(
                    path.as_str(),
                    Some(name.as_str()),
                    InputSettingsSelector::Metadata,
                ) {
                    contract.metadata = None;
                }
                if !output_selection.check_selection(
                    path.as_str(),
                    Some(name.as_str()),
                    InputSettingsSelector::Yul,
                ) {
                    contract.ir_optimized = String::new();
                }
                if let Some(ref mut evm) = contract.evm {
                    if !output_selection.check_selection(
                        path.as_str(),
                        Some(name.as_str()),
                        InputSettingsSelector::EVMLA,
                    ) {
                        evm.legacy_assembly = serde_json::Value::Null;
                    }
                }
                if contract
                    .evm
                    .as_mut()
                    .map(|evm| evm.is_empty())
                    .unwrap_or_default()
                {
                    contract.evm = None;
                }
            }
        }

        self.contracts.retain(|_, contracts| {
            contracts.retain(|_, contract| !contract.is_empty());
            !contracts.is_empty()
        });

        serde_json::to_writer(std::io::stdout(), &self).expect("Stdout writing error");
        std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
    }

    ///
    /// Pushes an arbitrary error with path.
    ///
    /// Please do not push project-general errors without paths here.
    ///
    pub fn push_error(&mut self, path: Option<String>, error: anyhow::Error) {
        self.errors.push(JsonOutputError::new_error(
            None,
            error,
            path.map(JsonOutputErrorSourceLocation::new),
            None,
        ));
    }
}

impl CollectableError for Output {
    fn errors(&self) -> Vec<&JsonOutputError> {
        self.errors
            .iter()
            .filter(|error| error.severity == "error")
            .collect()
    }

    fn take_warnings(&mut self) -> Vec<JsonOutputError> {
        let warnings = self
            .errors
            .iter()
            .filter(|message| message.severity == "warning")
            .cloned()
            .collect();
        self.errors.retain(|message| message.severity != "warning");
        warnings
    }
}
