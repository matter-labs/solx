//!
//! The lexical string literal parser output.
//!

use crate::test::function_call::parser::lexical::token::lexeme::literal::string::String;

///
/// The lexical string literal parser output.
///
#[derive(Debug, PartialEq, Eq)]
pub struct Output {
    /// The number of bytes in the string.
    pub length_bytes: usize,
    /// The number of characters in the string.
    pub length_chars: usize,
    /// The numbers of lines in the string.
    pub lines: usize,
    /// The column where the string ends.
    pub column: usize,
    /// The string data.
    pub string: String,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        length_bytes: usize,
        length_chars: usize,
        lines: usize,
        column: usize,
        string: String,
    ) -> Self {
        Self {
            length_bytes,
            length_chars,
            lines,
            column,
            string,
        }
    }
}
