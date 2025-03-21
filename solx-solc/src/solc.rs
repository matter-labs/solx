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
        error_pointer: *mut *mut ::libc::c_char,
    ) -> *const std::os::raw::c_char;

    ///
    /// Get the Solidity compiler version.
    ///
    fn solidity_version_extended() -> *const std::os::raw::c_char;
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            version: Self::parse_version(),
        }
    }
}

impl Compiler {
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
        let input_c_string = CString::new(input_string).expect("Always valid");

        let base_path = base_path.map(|base_path| CString::new(base_path).expect("Always valid"));
        let base_path = match base_path.as_ref() {
            Some(base_path) => base_path.as_ptr(),
            None => std::ptr::null(),
        };

        let include_paths: Vec<CString> = include_paths
            .into_iter()
            .map(|path| CString::new(path).expect("Always valid"))
            .collect();
        let include_paths: Vec<*const ::libc::c_char> =
            include_paths.iter().map(|path| path.as_ptr()).collect();
        let include_paths_ptr = if include_paths.is_empty() {
            std::ptr::null()
        } else {
            include_paths.as_ptr()
        };

        let allow_paths = allow_paths
            .map(|allow_paths| {
                allow_paths
                    .split(',')
                    .map(|path| CString::new(path.to_owned()).expect("Always valid"))
                    .collect::<Vec<CString>>()
            })
            .unwrap_or_default();
        let allow_paths: Vec<*const ::libc::c_char> =
            allow_paths.iter().map(|path| path.as_ptr()).collect();
        let allow_paths_ptr = if allow_paths.is_empty() {
            std::ptr::null()
        } else {
            allow_paths.as_ptr()
        };

        let mut error_message = std::ptr::null_mut();
        let error_pointer = &mut error_message;
        let output_string = unsafe {
            let output_pointer = solidity_compile_default_callback(
                input_c_string.as_ptr(),
                base_path,
                include_paths.len() as u64,
                include_paths_ptr,
                allow_paths.len() as u64,
                allow_paths_ptr,
                error_pointer,
            );
            if !error_message.is_null() {
                let error_message = CStr::from_ptr(error_message).to_string_lossy().into_owned();
                anyhow::bail!("solc standard JSON I/O: {error_message}");
            }
            CStr::from_ptr(output_pointer)
                .to_string_lossy()
                .into_owned()
        };

        let mut solc_output = match era_compiler_common::deserialize_from_str::<StandardJsonOutput>(
            output_string.as_str(),
        ) {
            Ok(solc_output) => solc_output,
            Err(error) => {
                anyhow::bail!("solc standard JSON output parsing: {error:?}");
            }
        };

        input_json.resolve_sources();
        solc_output.errors.append(messages);
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
            let output_pointer = solidity_version_extended();
            CStr::from_ptr(output_pointer)
                .to_string_lossy()
                .into_owned()
        };

        let lines = output.lines().collect::<Vec<&str>>();

        let default: semver::Version = lines
            .get(1)
            .unwrap_or_else(|| panic!("solc version parsing: missing line 1."))
            .split(' ')
            .nth(1)
            .expect("solc version parsing: missing version.")
            .split('+')
            .next()
            .expect("solc version parsing: missing semver.")
            .parse::<semver::Version>()
            .unwrap_or_else(|error| panic!("solc version parsing: {error}."));

        let llvm_revision: semver::Version = lines
            .get(2)
            .expect("LLVM revision parsing: missing line 2.")
            .split(' ')
            .nth(1)
            .expect("LLVM revision parsing: missing version.")
            .split('-')
            .nth(1)
            .expect("LLVM revision parsing: missing revision.")
            .parse::<semver::Version>()
            .unwrap_or_else(|error| panic!("LLVM revision parsing: {error}."));

        Version::new(output, default, llvm_revision)
    }
}
