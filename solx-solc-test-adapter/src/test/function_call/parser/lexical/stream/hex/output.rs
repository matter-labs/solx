//!
//! The lexical hex literal parser output.
//!

use crate::test::function_call::parser::lexical::token::lexeme::literal::hex::Hex;

///
/// The lexical hex literal parser output.
///
#[derive(Debug, PartialEq, Eq)]
pub struct Output {
    /// The number of characters in the hex.
    pub size: usize,
    /// The hex data.
    pub hex: Hex,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(size: usize, hex: Hex) -> Self {
        Self { size, hex }
    }
}
