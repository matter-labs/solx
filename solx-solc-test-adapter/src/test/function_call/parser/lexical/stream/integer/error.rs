//!
//! The lexical integer literal parser error.
//!

///
/// The lexical integer literal parser error.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The lexeme is not an integer, which means that another parser must be run.
    NotAnInteger,
    /// The lexeme is `0x`, which is not a valid hexadecimal literal.
    EmptyHexadecimalBody {
        /// The position where the literal ends.
        offset: usize,
    },
    /// A non-decimal or `x` character is found after a single `0`.
    ExpectedOneOfDecimalOrX {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
    /// A non-decimal character is found in a decimal literal.
    ExpectedOneOfDecimal {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
    /// A non-hexadecimal character is found in a hexadecimal literal.
    ExpectedOneOfHexadecimal {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
}
