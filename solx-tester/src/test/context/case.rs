//!
//! Context used to process test cases, consisting of a number of inputs.
//!

use crate::Mode;

///
/// Context used to process test cases, consisting of a number of inputs.
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaseContext<'a> {
    pub mode: &'a Mode,
    pub group: &'a Option<String>,
    pub name: &'a str,
}
