//!
//! The Solidity compiler.
//!

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::RwLock;

use crate::combined_json::selector::Selector as CombinedJsonSelector;
use crate::combined_json::CombinedJson;
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
    /// The binary executable name.
    pub executable: String,
    /// The `solc` compiler version.
    pub version: Version,
}

impl Compiler {
    /// The default executable name.
    pub const DEFAULT_EXECUTABLE_NAME: &'static str = "solc";

    /// The first version of `solc` with the support of standard JSON interface.
    pub const FIRST_SUPPORTED_VERSION: semver::Version = semver::Version::new(0, 4, 12);

    /// The first version of `solc`, where Yul codegen is considered robust enough.
    pub const FIRST_YUL_VERSION: semver::Version = semver::Version::new(0, 8, 0);

    /// The first version of `solc`, where `--via-ir` codegen option is supported.
    pub const FIRST_VIA_IR_VERSION: semver::Version = semver::Version::new(0, 8, 13);

    /// The first version of `solc`, where EVM Cancun is supported.
    pub const FIRST_CANCUN_VERSION: semver::Version = semver::Version::new(0, 8, 24);

    /// The last supported version of `solc`.
    pub const LAST_SUPPORTED_VERSION: semver::Version = semver::Version::new(0, 8, 28);

    ///
    /// A shortcut constructor lazily using a thread-safe cell.
    ///
    /// Different tools may use different `executable` names. For example, the integration tester
    /// uses `solc-<version>` format.
    ///
    pub fn try_from_path(executable: &str) -> anyhow::Result<Self> {
        if let Some(executable) = Self::executables()
            .read()
            .expect("Sync")
            .get(executable)
            .cloned()
        {
            return Ok(executable);
        }
        let mut executables = Self::executables().write().expect("Sync");

        if let Err(error) = which::which(executable) {
            anyhow::bail!("The `{executable}` executable not found: {error}. Please add it to ${{PATH}} or provide it explicitly with the `--solc` option.");
        }
        let version = Self::parse_version(executable)?;
        let compiler = Self {
            executable: executable.to_owned(),
            version,
        };

        executables.insert(executable.to_owned(), compiler.clone());
        Ok(compiler)
    }

    ///
    /// Initializes the Solidity compiler with the default executable name.
    ///
    pub fn try_from_default() -> anyhow::Result<Self> {
        Self::try_from_path(Self::DEFAULT_EXECUTABLE_NAME)
    }

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
        let mut command = std::process::Command::new(self.executable.as_str());
        command.stdin(std::process::Stdio::piped());
        command.stdout(std::process::Stdio::piped());
        command.stderr(std::process::Stdio::piped());
        command.arg("--standard-json");
        if let Some(base_path) = base_path {
            command.arg("--base-path");
            command.arg(base_path);
        }
        for include_path in include_paths.into_iter() {
            command.arg("--include-path");
            command.arg(include_path);
        }
        if let Some(allow_paths) = allow_paths {
            command.arg("--allow-paths");
            command.arg(allow_paths);
        }

        let mut process = command.spawn().map_err(|error| {
            anyhow::anyhow!("{} subprocess spawning: {:?}", self.executable, error)
        })?;
        let stdin = process
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("{} subprocess stdin getting error", self.executable))?;
        let stdin_input = serde_json::to_vec(&input).expect("Always valid");
        stdin.write_all(stdin_input.as_slice()).map_err(|error| {
            anyhow::anyhow!("{} subprocess stdin writing: {error:?}", self.executable)
        })?;

        let result = process.wait_with_output().map_err(|error| {
            anyhow::anyhow!("{} subprocess output reading: {error:?}", self.executable)
        })?;
        if !result.status.success() {
            anyhow::bail!(
                "{} subprocess failed with exit code {:?}:\n{}\n{}",
                self.executable,
                result.status.code(),
                String::from_utf8_lossy(result.stdout.as_slice()),
                String::from_utf8_lossy(result.stderr.as_slice()),
            );
        }

        let mut solc_output = match era_compiler_common::deserialize_from_slice::<StandardJsonOutput>(
            result.stdout.as_slice(),
        ) {
            Ok(solc_output) => solc_output,
            Err(error) => {
                anyhow::bail!(
                    "{} subprocess stdout parsing: {error:?} (stderr: {})",
                    self.executable,
                    String::from_utf8_lossy(result.stderr.as_slice()),
                );
            }
        };
        eprintln!("{}", serde_json::to_string_pretty(&solc_output).unwrap());

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
    /// The `solc --combined-json abi,hashes...` mirror.
    ///
    pub fn combined_json(
        &self,
        paths: &[PathBuf],
        mut selectors: HashSet<CombinedJsonSelector>,
    ) -> anyhow::Result<CombinedJson> {
        selectors.retain(|selector| selector.is_source_solc());
        if selectors.is_empty() {
            return Ok(CombinedJson::new(self.version.default.to_owned()));
        }

        let executable = self.executable.to_owned();

        let mut command = std::process::Command::new(executable.as_str());
        command.stdout(std::process::Stdio::piped());
        command.stderr(std::process::Stdio::piped());
        command.args(paths);
        command.arg("--combined-json");
        command.arg(
            selectors
                .into_iter()
                .map(|selector| selector.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );

        let process = command
            .spawn()
            .map_err(|error| anyhow::anyhow!("{} subprocess spawning: {:?}", executable, error))?;

        let result = process.wait_with_output().map_err(|error| {
            anyhow::anyhow!("{} subprocess output reading: {error:?}", self.executable)
        })?;

        if !result.status.success() {
            anyhow::bail!(
                "{} subprocess failed with exit code {:?}:\n{}\n{}",
                self.executable,
                result.status.code(),
                String::from_utf8_lossy(result.stdout.as_slice()),
                String::from_utf8_lossy(result.stderr.as_slice()),
            );
        }

        era_compiler_common::deserialize_from_slice::<CombinedJson>(result.stdout.as_slice())
            .map_err(|error| {
                anyhow::anyhow!(
                    "{} subprocess stdout parsing: {error:?} (stderr: {})",
                    self.executable,
                    String::from_utf8_lossy(result.stderr.as_slice()),
                )
            })
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
    /// Returns the global shared array of `solc` executables.
    ///
    fn executables() -> &'static RwLock<HashMap<String, Self>> {
        static EXECUTABLES: OnceLock<RwLock<HashMap<String, Compiler>>> = OnceLock::new();
        EXECUTABLES.get_or_init(|| RwLock::new(HashMap::new()))
    }

    ///
    /// The `solc --version` mini-parser.
    ///
    fn parse_version(executable: &str) -> anyhow::Result<Version> {
        let mut command = std::process::Command::new(executable);
        command.arg("--version");
        let output = command
            .output()
            .map_err(|error| anyhow::anyhow!("`{executable}` subprocess: {error:?}."))?;
        if !output.status.success() {
            anyhow::bail!(
                "`{executable}` version getting: {}",
                String::from_utf8_lossy(output.stderr.as_slice()).to_string()
            );
        }

        let stdout = String::from_utf8_lossy(output.stdout.as_slice());
        let long = stdout
            .lines()
            .nth(1)
            .ok_or_else(|| anyhow::anyhow!("`{executable}` version parsing: not enough lines."))?
            .split(' ')
            .nth(1)
            .ok_or_else(|| {
                anyhow::anyhow!("`{executable}` version parsing: not enough words in the 2nd line.")
            })?
            .to_owned();
        let default: semver::Version = long
            .split('+')
            .next()
            .expect("Always exists")
            .parse()
            .map_err(|error| anyhow::anyhow!("`{executable}` version parsing: {error}."))?;

        let l2_revision: semver::Version = stdout
            .lines()
            .nth(2)
            .ok_or_else(|| anyhow::anyhow!("`{executable}` ZKsync revision parsing: missing line."))
            .and_then(|line| {
                line.split(' ').nth(1).ok_or_else(|| {
                    anyhow::anyhow!("`{executable}` ZKsync revision parsing: missing version.")
                })
            })
            .and_then(|line| {
                line.split('-').nth(1).ok_or_else(|| {
                    anyhow::anyhow!("`{executable}` ZKsync revision parsing: missing revision.")
                })
            })
            .and_then(|version| {
                version.parse().map_err(|error| {
                    anyhow::anyhow!("`{executable}` ZKsync revision parsing: {error}.")
                })
            })
            .map_err(|error| {
                anyhow::anyhow!("Only the ZKsync fork of `solc` can be used: {error}")
            })?;

        let version = Version::new(long, default, l2_revision);
        if version.default < Self::FIRST_SUPPORTED_VERSION {
            anyhow::bail!(
                "`{executable}` versions older than {} are not supported, found {}. Please upgrade to the latest supported version.",
                Self::FIRST_SUPPORTED_VERSION,
                version.default
            );
        }
        if version.default > Self::LAST_SUPPORTED_VERSION {
            anyhow::bail!(
                "`{executable}` versions newer than {} are not supported, found {}. Please check if you are using the latest version of solx.",
                Self::LAST_SUPPORTED_VERSION,
                version.default
            );
        }

        Ok(version)
    }
}
