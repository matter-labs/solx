//!
//! The call.
//!

pub mod builder;
pub mod variant;

use self::variant::Variant;
use crate::test::function_call::parser::lexical::Location;

///
/// The call.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    /// The location of the syntax construction.
    pub location: Location,
    /// The variant.
    pub variant: Variant,
}

impl Call {
    ///
    /// Creates a function call.
    ///
    pub fn new(location: Location, variant: Variant) -> Self {
        Self { location, variant }
    }
}
