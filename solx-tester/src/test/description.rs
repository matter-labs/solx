//!
//! Test description with additional information such as the compiler mode and test group.
//!

use crate::Mode;

use crate::test::case::input::identifier::InputIdentifier;
use crate::test::context::input::InputContext;
use crate::test::selector::TestSelector;

///
/// Test description with additional information such as the compiler mode and test group.
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestDescription {
    /// Test group.
    pub group: Option<String>,
    /// Compiler mode.
    pub mode: Option<Mode>,
    /// Test selector, matching a precise input location or a case collecting several inputs.
    pub selector: TestSelector,
}

impl TestDescription {
    ///
    /// Creates a test description matching specified selector with no additonal
    /// information.
    ///
    pub fn default_for(test: TestSelector) -> Self {
        Self {
            group: None,
            mode: None,
            selector: test,
        }
    }

    ///
    /// Erase information about mode from this selector.
    ///
    pub fn with_erased_mode(self) -> Self {
        let Self {
            group,
            mode: _,
            selector: identifier,
        } = self;
        Self {
            group,
            mode: None,
            selector: identifier,
        }
    }

    ///
    /// Create a selector from accumulated input context and provided input
    /// identifier.
    ///
    pub fn from_context(ctx: InputContext<'_>, input: InputIdentifier) -> Self {
        Self {
            group: ctx.case_context.group.clone(),
            mode: Some(ctx.case_context.mode.clone()),
            selector: TestSelector {
                path: ctx.case_context.name.to_string(),
                case: ctx.case_name.clone(),
                input: Some(input),
            },
        }
    }
}
