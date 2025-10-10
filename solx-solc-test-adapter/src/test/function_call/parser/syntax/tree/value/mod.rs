//!
//! The value option.
//!

pub mod builder;
pub mod unit;

use self::unit::Unit;
use crate::test::function_call::parser::lexical::Location;

///
/// The value option.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    /// The location of the syntax construction.
    pub location: Location,
    /// The unit.
    pub unit: Unit,
    /// The amount.
    pub amount: String,
}

impl Value {
    ///
    /// Creates a value option.
    ///
    pub fn new(location: Location, unit: Unit, amount: String) -> Self {
        Self {
            location,
            unit,
            amount,
        }
    }
}
