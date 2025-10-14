//!
//! The lexical symbol parser output.
//!

use crate::test::function_call::parser::lexical::token::lexeme::symbol::Symbol;

///
/// The lexical symbol parser output.
///
#[derive(Debug, PartialEq, Eq)]
pub struct Output {
    /// The number of characters in the symbol.
    pub size: usize,
    /// The symbol data.
    pub symbol: Symbol,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(size: usize, symbol: Symbol) -> Self {
        Self { size, symbol }
    }
}
