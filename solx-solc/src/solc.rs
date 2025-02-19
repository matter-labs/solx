//!
//! The Solidity compiler.
//!

use std::borrow::Cow;
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
    ///
    /// Pass standard JSON input to the Solidity compiler.
    ///
    /// Passes `--base-path`, `--include-paths`, and `--allow-paths` just like it is done with the CLI.
    ///
    fn solidity_compile_default_callback(
        input: *const ::libc::c_char,
        base_path: *const ::libc::c_char,
        include_paths_size: u64,
        include_paths: *const *const ::libc::c_char,
        allow_paths_size: u64,
        allow_paths: *const *const ::libc::c_char,
    ) -> *const std::os::raw::c_char;

    ///
    /// Get the Solidity compiler version.
    ///
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
    pub const LAST_ZKSYNC_REVISION: semver::Version = semver::Version::new(1, 0, 1);

    ///
    /// The Solidity `--standard-json` mirror.
    ///
    pub fn standard_json(
        &self,
        input_json: &mut StandardJsonInput,
        messages: &mut Vec<StandardJsonOutputError>,
        base_path: Option<String>,
        include_paths: Vec<String>,
        allow_paths: Option<String>,
    ) -> anyhow::Result<StandardJsonOutput> {
        let input_string = serde_json::to_string(input_json).expect("Always valid");
        let input_string = Self::to_null_terminated_owned(input_string.as_str());
        let input_c_string = Self::to_c_str(input_string.as_str());

        let base_path = base_path.as_deref().map(Self::to_null_terminated_owned);
        let base_path = base_path
            .as_ref()
            .map(|base_path| Self::to_c_str(base_path.as_str()));
        let base_path = match base_path {
            Some(base_path) => base_path.as_ptr(),
            None => std::ptr::null(),
        };

        let include_paths: Vec<String> = include_paths
            .iter()
            .map(|path| Self::to_null_terminated_owned(path))
            .collect();
        let include_paths: Vec<Cow<CStr>> = include_paths
            .iter()
            .map(|path| Self::to_c_str(path.as_str()))
            .collect();
        let include_paths: Vec<*const ::libc::c_char> =
            include_paths.iter().map(|path| path.as_ptr()).collect();
        let include_paths_ptr = if include_paths.is_empty() {
            std::ptr::null()
        } else {
            include_paths.as_ptr()
        };

        let allow_paths = allow_paths
            .as_deref()
            .map(|allow_paths| {
                allow_paths
                    .split(',')
                    .map(Self::to_null_terminated_owned)
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default();
        let allow_paths: Vec<Cow<CStr>> = allow_paths
            .iter()
            .map(|path| Self::to_c_str(path.as_str()))
            .collect();
        let allow_paths: Vec<*const ::libc::c_char> =
            allow_paths.iter().map(|path| path.as_ptr()).collect();
        let allow_paths_ptr = if allow_paths.is_empty() {
            std::ptr::null()
        } else {
            allow_paths.as_ptr()
        };

        let output_ffi = unsafe {
            let output_pointer = solidity_compile_default_callback(
                input_c_string.as_ptr(),
                base_path,
                include_paths.len() as u64,
                include_paths_ptr,
                allow_paths.len() as u64,
                allow_paths_ptr,
            );
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

        input_json.resolve_sources();
        solc_output.preprocess_ast(&input_json.sources, &self.version)?;
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

    fn to_c_str(mut s: &str) -> Cow<CStr> {
        if s.is_empty() {
            s = "\0";
        }

        if !s.chars().rev().any(|ch| ch == '\0') {
            return Cow::from(CString::new(s).expect("unreachable since null bytes are checked"));
        }

        unsafe { Cow::from(CStr::from_ptr(s.as_ptr() as *const _)) }
    }

    fn to_null_terminated_owned(s: &str) -> String {
        if let Some(p) = s.rfind('\0') {
            s[..=p].to_string()
        } else {
            format!("{s}\0")
        }
    }
}
