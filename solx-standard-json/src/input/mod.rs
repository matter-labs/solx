//!
//! The `solc --standard-json` input.
//!

pub mod language;
pub mod settings;
pub mod source;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::input::settings::metadata::Metadata as InputSettingsMetadata;
use crate::input::settings::optimizer::Optimizer as InputSettingsOptimizer;
use crate::input::settings::selection::Selection as InputSettingsSelection;

use self::language::Language;
use self::settings::Settings;
use self::source::Source;

///
/// The `solc --standard-json` input.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    /// The input language.
    pub language: Language,
    /// The input source code files hashmap.
    pub sources: BTreeMap<String, Source>,
    /// The compiler settings.
    pub settings: Settings,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    /// If the `path` is `None`, the input is read from the stdin.
    ///
    pub fn try_from(path: Option<&Path>) -> anyhow::Result<Self> {
        let input_json = match path {
            Some(path) => std::fs::read_to_string(path)
                .map_err(|error| anyhow::anyhow!("Standard JSON file {path:?} reading: {error}")),
            None => std::io::read_to_string(std::io::stdin())
                .map_err(|error| anyhow::anyhow!("Standard JSON reading from stdin: {error}")),
        }?;
        era_compiler_common::deserialize_from_str::<Self>(input_json.as_str())
            .map_err(|error| anyhow::anyhow!("Standard JSON parsing: {error}"))
    }

    ///
    /// A shortcut constructor from Solidity source paths.
    ///
    pub fn try_from_solidity_paths(
        paths: &[PathBuf],
        libraries: &[String],
        remappings: BTreeSet<String>,
        optimizer: InputSettingsOptimizer,
        evm_version: Option<era_compiler_common::EVMVersion>,
        via_ir: bool,
        output_selection: InputSettingsSelection,
        metadata: InputSettingsMetadata,
        llvm_options: Vec<String>,
    ) -> anyhow::Result<Self> {
        let mut paths: BTreeSet<PathBuf> = paths.iter().cloned().collect();
        let libraries = era_compiler_common::Libraries::try_from(libraries)?;
        for library_file in libraries.as_inner().keys() {
            paths.insert(PathBuf::from(library_file));
        }

        let sources = paths
            .into_par_iter()
            .map(|path| {
                let source = Source::try_read(path.as_path())?;
                Ok((path.to_string_lossy().to_string(), source))
            })
            .collect::<anyhow::Result<BTreeMap<String, Source>>>()?;

        Self::try_from_solidity_sources(
            sources,
            libraries,
            remappings,
            optimizer,
            evm_version,
            via_ir,
            output_selection,
            metadata,
            llvm_options,
        )
    }

    ///
    /// A shortcut constructor from Solidity source code.
    ///
    pub fn try_from_solidity_sources(
        sources: BTreeMap<String, Source>,
        libraries: era_compiler_common::Libraries,
        remappings: BTreeSet<String>,
        optimizer: InputSettingsOptimizer,
        evm_version: Option<era_compiler_common::EVMVersion>,
        via_ir: bool,
        output_selection: InputSettingsSelection,
        metadata: InputSettingsMetadata,
        llvm_options: Vec<String>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            language: Language::Solidity,
            sources,
            settings: Settings::new(
                optimizer,
                libraries,
                remappings,
                evm_version,
                via_ir,
                output_selection,
                metadata,
                llvm_options,
            ),
        })
    }

    ///
    /// A shortcut constructor from source code.
    ///
    pub fn from_yul_sources(
        sources: BTreeMap<String, Source>,
        libraries: era_compiler_common::Libraries,
        optimizer: InputSettingsOptimizer,
        output_selection: InputSettingsSelection,
        metadata: InputSettingsMetadata,
        llvm_options: Vec<String>,
    ) -> Self {
        Self {
            language: Language::Yul,
            sources,
            settings: Settings::new(
                optimizer,
                libraries,
                BTreeSet::new(),
                None,
                false,
                output_selection,
                metadata,
                llvm_options,
            ),
        }
    }

    ///
    /// A shortcut constructor from source code.
    ///
    pub fn from_yul_paths(
        paths: &[PathBuf],
        libraries: era_compiler_common::Libraries,
        optimizer: InputSettingsOptimizer,
        output_selection: InputSettingsSelection,
        metadata: InputSettingsMetadata,
        llvm_options: Vec<String>,
    ) -> Self {
        let sources = paths
            .iter()
            .map(|path| {
                (
                    path.to_string_lossy().to_string(),
                    Source::from(path.as_path()),
                )
            })
            .collect();
        Self::from_yul_sources(
            sources,
            libraries,
            optimizer,
            output_selection,
            metadata,
            llvm_options,
        )
    }

    ///
    /// Tries to resolve all sources.
    ///
    pub fn resolve_sources(&mut self) {
        self.sources
            .par_iter_mut()
            .map(|(_path, source)| {
                let _ = source.try_resolve();
            })
            .collect::<Vec<()>>();
    }
}
