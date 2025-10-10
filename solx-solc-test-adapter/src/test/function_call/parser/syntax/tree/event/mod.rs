//!
//! The type.
//!

pub mod builder;
pub mod literal;
pub mod variant;

use self::literal::EventLiteral;
use self::variant::Variant;
use crate::test::function_call::parser::lexical::Location;

///
/// The type.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// The location of the syntax construction.
    pub location: Location,
    /// The signature variant.
    pub variant: Variant,
    /// The address.
    pub address: Option<String>,
    /// The expected values.
    pub expected: Option<Vec<EventLiteral>>,
}

impl Event {
    ///
    /// Creates a function call.
    ///
    pub fn new(
        location: Location,
        variant: Variant,
        address: Option<String>,
        expected: Option<Vec<EventLiteral>>,
    ) -> Self {
        Self {
            location,
            variant,
            address,
            expected,
        }
    }
}
