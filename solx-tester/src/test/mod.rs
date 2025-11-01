//!
//! The test.
//!

pub mod case;
pub mod context;
pub mod description;
pub mod instance;
pub mod selector;

use std::sync::Arc;
use std::sync::Mutex;

use crate::compilers::mode::Mode;
use crate::revm::REVM;
use crate::summary::Summary;
use crate::test::case::Case;
use crate::test::context::case::CaseContext;
use crate::test::context::input::InputContext;

///
/// The test.
///
#[derive(Debug)]
pub struct Test {
    /// The test name.
    name: String,
    /// The test cases.
    cases: Vec<Case>,
    /// The test mode.
    mode: Mode,
    /// The test group.
    group: Option<String>,
}

impl Test {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String, cases: Vec<Case>, mode: Mode, group: Option<String>) -> Self {
        Self {
            name,
            cases,
            mode,
            group,
        }
    }

    ///
    /// Runs the test on REVM.
    ///
    pub fn run_revm(self, summary: Arc<Mutex<Summary>>, enable_trace: bool) {
        for case in self.cases {
            let context = CaseContext {
                name: &self.name,
                mode: &self.mode,
                group: &self.group,
            };
            case.run_revm(summary.clone(), &context, REVM::new(enable_trace));
        }
    }
}
