//!
//! The gas option.
//!

pub mod builder;
pub mod variant;

use self::variant::Variant;
use crate::test::function_call::parser::lexical::Location;

///
/// The gas option.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gas {
    /// The location of the syntax construction.
    pub location: Location,
    /// The gas option variant.
    pub variant: Variant,
    /// The gas value.
    pub value: String,
}

impl Gas {
    ///
    /// Creates a gas option.
    ///
    pub fn new(location: Location, variant: Variant, value: String) -> Self {
        Self {
            location,
            variant,
            value,
        }
    }
}
