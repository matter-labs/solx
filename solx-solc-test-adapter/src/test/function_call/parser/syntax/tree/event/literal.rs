//!
//! The event literal.
//!

use crate::test::function_call::parser::syntax::tree::literal::Literal;

///
/// The event literal.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventLiteral {
    /// The inner literal.
    pub inner: Literal,
    /// The indexed flag.
    pub indexed: bool,
}

impl EventLiteral {
    ///
    /// Creates an event literal.
    ///
    pub fn new(inner: Literal, indexed: bool) -> Self {
        Self { inner, indexed }
    }
}
