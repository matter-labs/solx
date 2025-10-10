//!
//! The `solc --standard-json` input.
//!

pub mod language;
pub mod settings;
pub mod source;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use serde::Serialize;

use self::settings::debug::Debug as SolcStandardJsonInputDebug;
use self::settings::optimizer::Optimizer as SolcStandardJsonInputOptimizer;
use self::settings::selection::Selection as SolcStandardJsonInputSelection;

use self::language::Language;
use self::settings::Settings;
use self::source::Source;

///
/// The `solc --standard-json` input.
///
#[derive(Debug, Serialize)]
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
    /// A shortcut constructor from source code.
    ///
    /// Only for the integration test purposes.
    ///
    pub fn try_from_sources(
        language: Language,
        evm_version: Option<solx_utils::EVMVersion>,
        sources: BTreeMap<String, String>,
        libraries: solx_utils::Libraries,
        remappings: Option<BTreeSet<String>>,
        output_selection: SolcStandardJsonInputSelection,
        via_ir: bool,
        codegen: Option<String>,
        optimizer: SolcStandardJsonInputOptimizer,
        debug: Option<SolcStandardJsonInputDebug>,
    ) -> anyhow::Result<Self> {
        let sources = sources
            .into_par_iter()
            .map(|(path, content)| (path, Source::from(content)))
            .collect();

        Ok(Self {
            language,
            sources,
            settings: Settings::new(
                evm_version,
                libraries,
                remappings,
                output_selection,
                via_ir,
                codegen,
                optimizer,
                debug,
            ),
        })
    }
}
