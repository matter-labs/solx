//!
//! The lexical hex literal parser error.
//!

///
/// The lexical hex literal parser error.
///
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// The lexeme is not a hex, which means that another parser must be run.
    NotAHex,
    /// The hex has not been terminated, which ends up with an entire file treated as an unterminated hex.
    UnterminatedDoubleQuote {
        /// The position where the unterminated hex ends.
        offset: usize,
    },
    /// A non-hexadecimal character is found in a hex literal.
    ExpectedOneOfHexadecimal {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
}
