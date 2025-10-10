//!
//! Test selector, unambiously locating a test suite, or a specific input.
//!

use crate::test::case::input::identifier::InputIdentifier;

///
/// Test selector, unambiously locating a test suite, case, or input.
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestSelector {
    /// Path to the file containing test.
    pub path: String,
    /// Name of the case, if any. `None` means nameless case.
    pub case: Option<String>,
    /// Identifier of the specific input.
    pub input: Option<InputIdentifier>,
}

impl std::fmt::Display for TestSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            path: filename,
            case: case_name,
            input,
        } = self;
        f.write_fmt(format_args!("{filename}"))?;
        if let Some(case_name) = case_name {
            f.write_fmt(format_args!("::{case_name}"))?;
        }
        if let Some(input) = input {
            f.write_fmt(format_args!("[{input}]"))?;
        }
        Ok(())
    }
}
