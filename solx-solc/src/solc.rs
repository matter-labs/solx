//!
//! The Solidity compiler.
//!

use std::ffi::CStr;
use std::ffi::CString;
use std::path::PathBuf;

use crate::standard_json::input::settings::libraries::Libraries as StandardJsonInputSettingsLibraries;
use crate::standard_json::input::settings::optimizer::Optimizer as StandardJsonInputSettingsOptimizer;
use crate::standard_json::input::settings::selection::Selection as StandardJsonInputSettingsSelection;
use crate::standard_json::input::Input as StandardJsonInput;
use crate::standard_json::output::error::Error as StandardJsonOutputError;
use crate::standard_json::output::Output as StandardJsonOutput;
use crate::version::Version;

///
/// The Solidity compiler.
///
#[derive(Debug, Clone)]
pub struct Compiler {
    /// The `solc` compiler version.
    pub version: Version,
}

#[link(name = "solc", kind = "static")]
extern "C" {
    // Match the function signatures exactly.
    // If your return type is `const char*`, you might wrap it in Rust with `*const c_char`.
    // If you wrote a custom bridging function, rename to match that bridging function.

    fn solidity_compile(input: *const std::os::raw::c_char) -> *const std::os::raw::c_char;

    fn solidity_version() -> *const std::os::raw::c_char;
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            version: Self::parse_version(),
        }
    }
}

impl Compiler {
    /// The last ZKsync revision of `solc`.
    pub const LAST_ZKSYNC_REVISION: semver::Version = semver::Version::new(0, 1, 0);

    ///
    /// The Solidity `--standard-json` mirror.
    ///
    pub fn standard_json(
        &self,
        input: &mut StandardJsonInput,
        messages: &mut Vec<StandardJsonOutputError>,
        base_path: Option<String>,
        include_paths: Vec<String>,
        allow_paths: Option<String>,
    ) -> anyhow::Result<StandardJsonOutput> {
        let input_string = serde_json::to_string(input).expect("Always valid");
        let input_ffi = CString::new(input_string).expect("Always valid");
        let output_ffi = unsafe {
            let output_pointer = solidity_compile(input_ffi.as_ptr());
            CStr::from_ptr(output_pointer)
                .to_string_lossy()
                .into_owned()
        };

        let mut solc_output = match era_compiler_common::deserialize_from_str::<StandardJsonOutput>(
            output_ffi.as_str(),
        ) {
            Ok(solc_output) => solc_output,
            Err(error) => {
                anyhow::bail!("solc standard JSON output parsing: {error:?}");
            }
        };

        solc_output
            .errors
            .retain(|error| match error.error_code.as_deref() {
                Some(code) => !StandardJsonOutputError::IGNORED_WARNING_CODES.contains(&code),
                None => true,
            });
        solc_output.errors.append(messages);

        input.resolve_sources();
        solc_output.preprocess_ast(&input.sources, &self.version)?;
        solc_output.remove_evm_artifacts();

        Ok(solc_output)
    }

    ///
    /// Validates the Yul project as paths and libraries.
    ///
    pub fn validate_yul_paths(
        &self,
        paths: &[PathBuf],
        libraries: StandardJsonInputSettingsLibraries,
        messages: &mut Vec<StandardJsonOutputError>,
    ) -> anyhow::Result<StandardJsonOutput> {
        let mut solc_input = StandardJsonInput::from_yul_paths(
            paths,
            libraries,
            StandardJsonInputSettingsOptimizer::default(),
            vec![],
        );
        self.validate_yul_standard_json(&mut solc_input, messages)
    }

    ///
    /// Validates the Yul project as standard JSON input.
    ///
    pub fn validate_yul_standard_json(
        &self,
        solc_input: &mut StandardJsonInput,
        messages: &mut Vec<StandardJsonOutputError>,
    ) -> anyhow::Result<StandardJsonOutput> {
        solc_input.extend_selection(StandardJsonInputSettingsSelection::new_yul_validation());
        let solc_output = self.standard_json(solc_input, messages, None, vec![], None)?;
        Ok(solc_output)
    }

    ///
    /// The `solc` version parser.
    ///
    fn parse_version() -> Version {
        let output = unsafe {
            let output_pointer = solidity_version();
            CStr::from_ptr(output_pointer)
                .to_string_lossy()
                .into_owned()
        };

        let default: semver::Version = output
            .split('+')
            .next()
            .expect("Always exists")
            .parse()
            .expect("Always valid");

        Version::new(output, default, Self::LAST_ZKSYNC_REVISION)
    }
}
