//!
//! The `solc --standard-json` output error.
//!

pub mod collectable;
pub mod mapped_location;
pub mod source_location;

use std::collections::BTreeMap;

use crate::input::source::Source as InputSource;

use self::mapped_location::MappedLocation;
use self::source_location::SourceLocation;

///
/// The `solc --standard-json` output error.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    /// The component type.
    pub component: String,
    /// The error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// The formatted error message.
    pub formatted_message: String,
    /// The non-formatted error message.
    pub message: String,
    /// The error severity.
    pub severity: String,
    /// The error location data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location: Option<SourceLocation>,
    /// The error type.
    pub r#type: String,
}

impl Error {
    /// The list of ignored `solc` warnings that are strictly EVM-related.
    pub const IGNORED_WARNING_CODES: [&'static str; 5] = ["1699", "3860", "5159", "5574", "6417"];

    /// The memory-unsafe assembly warning code.
    pub const MEMORY_UNSAFE_ASSEMBLY_WARNING_CODE: &'static str = "5726";

    /// The environment variable to disable the memory-safe assembly check.
    pub const EVM_DISABLE_MEMORY_SAFE_ASM_CHECK_ENV: &'static str =
        "EVM_DISABLE_MEMORY_SAFE_ASM_CHECK";

    ///
    /// A shortcut constructor.
    ///
    pub fn new<S>(
        r#type: &str,
        error_code: Option<isize>,
        message: S,
        source_location: Option<SourceLocation>,
        sources: Option<&BTreeMap<String, InputSource>>,
    ) -> Self
    where
        S: std::fmt::Display,
    {
        let message = message.to_string();

        let message_trimmed = message.trim();
        let mut formatted_message = if message_trimmed.starts_with(r#type) {
            message_trimmed.to_owned()
        } else {
            format!("{type}: {message_trimmed}")
        };
        formatted_message.push('\n');
        if let Some(ref source_location) = source_location {
            let source_code = sources.and_then(|sources| {
                sources
                    .get(source_location.file.as_str())
                    .and_then(|source| source.content())
            });
            let mapped_location =
                MappedLocation::try_from_source_location(source_location, source_code);
            formatted_message.push_str(mapped_location.to_string().as_str());
            formatted_message.push('\n');
        }

        Self {
            component: "general".to_owned(),
            error_code: error_code.map(|code| code.to_string()),
            formatted_message,
            message,
            severity: r#type.to_lowercase(),
            source_location,
            r#type: r#type.to_owned(),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_error<S>(
        error_code: Option<isize>,
        message: S,
        source_location: Option<SourceLocation>,
        sources: Option<&BTreeMap<String, InputSource>>,
    ) -> Self
    where
        S: std::fmt::Display,
    {
        Self::new("Error", error_code, message, source_location, sources)
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_warning<S>(
        error_code: Option<isize>,
        message: S,
        source_location: Option<SourceLocation>,
        sources: Option<&BTreeMap<String, InputSource>>,
    ) -> Self
    where
        S: std::fmt::Display,
    {
        Self::new("Warning", error_code, message, source_location, sources)
    }

    ///
    /// Changes this message to a warning.
    ///
    /// It is useful when the user is confident that the error is not critical and can be ignored.
    ///
    pub fn make_warning(&mut self) {
        self.severity = "warning".to_owned();
        self.r#type = "Warning".to_owned();
    }

    ///
    /// Changes this message to a warning.
    ///
    /// It is useful when we want to abort the compilation when we consider the situation more critical than a warning.
    ///
    pub fn make_error(&mut self) {
        self.severity = "error".to_owned();
        self.r#type = "Error".to_owned();
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_message)
    }
}
