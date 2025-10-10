//!
//! The identifier.
//!

use crate::test::function_call::parser::lexical::Location;

///
/// The identifier.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    /// The location of the syntax construction.
    pub location: Location,
    /// The identifier string contents.
    pub name: String,
}

impl Identifier {
    ///
    /// Creates an identifier.
    ///
    pub fn new(location: Location, name: String) -> Self {
        Self { location, name }
    }
}
