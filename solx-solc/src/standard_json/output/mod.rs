//!
//! The `solc --standard-json` output.
//!

pub mod contract;
pub mod error;
pub mod source;

use std::collections::BTreeMap;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::standard_json::input::settings::selection::selector::Selector;
use crate::standard_json::input::settings::selection::Selection;
use crate::standard_json::input::source::Source as StandardJSONInputSource;
use crate::version::Version;

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

    /// The `solc` compiler version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The `solc` compiler long version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_version: Option<String>,
    /// The `solx` compiler version.
    #[serde(default = "crate::version")]
    pub zk_version: String,
}

impl Output {
    ///
    /// Initializes a standard JSON output.
    ///
    /// Is used for projects compiled without `solc`.
    ///
    pub fn new(
        sources: &BTreeMap<String, StandardJSONInputSource>,
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

            version: None,
            long_version: None,
            zk_version: crate::version(),
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

            version: None,
            long_version: None,
            zk_version: crate::version(),
        }
    }

    ///
    /// Prunes the output JSON and prints it to stdout.
    ///
    pub fn write_and_exit(mut self, selection_to_prune: Selection) -> ! {
        let contracts = self
            .contracts
            .values_mut()
            .flat_map(|contracts| contracts.values_mut())
            .collect::<Vec<&mut Contract>>();
        for contract in contracts.into_iter() {
            contract.metadata = String::new(); // TODO: fix metadata
            if selection_to_prune.contains(&Selector::Yul) {
                contract.ir_optimized = String::new();
            }
            if let Some(ref mut evm) = contract.evm {
                if selection_to_prune.contains(&Selector::EVMLA) {
                    evm.legacy_assembly = serde_json::Value::Null;
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
    /// Removes EVM artifacts to prevent their accidental usage.
    ///
    pub fn remove_evm_artifacts(&mut self) {
        for (_, file) in self.contracts.iter_mut() {
            for (_, contract) in file.iter_mut() {
                if let Some(evm) = contract.evm.as_mut() {
                    evm.bytecode = None;
                }
            }
        }
    }

    ///
    /// Pushes an arbitrary error with path.
    ///
    /// Please do not push project-general errors without paths here.
    ///
    pub fn push_error(&mut self, path: Option<String>, error: anyhow::Error) {
        self.errors.push(JsonOutputError::new_error(
            error,
            path.map(JsonOutputErrorSourceLocation::new),
            None,
        ));
    }

    ///
    /// Traverses the AST and returns the list of additional errors and warnings.
    ///
    pub fn preprocess_ast(
        &mut self,
        sources: &BTreeMap<String, StandardJSONInputSource>,
        version: &Version,
    ) -> anyhow::Result<()> {
        let id_paths: BTreeMap<usize, &String> = self
            .sources
            .iter()
            .map(|(path, source)| (source.id, path))
            .collect();

        let messages: Vec<JsonOutputError> = self
            .sources
            .par_iter()
            .map(|(_path, source)| {
                source
                    .ast
                    .as_ref()
                    .map(|ast| Source::get_messages(ast, &id_paths, sources, version))
                    .unwrap_or_default()
            })
            .flatten()
            .collect();
        self.errors.extend(messages);

        Ok(())
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
