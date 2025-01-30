//!
//! The token.
//!

pub mod lexeme;
pub mod location;

use self::lexeme::Lexeme;
use self::location::Location;

///
/// The token.
///
/// Contains a lexeme and its location.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The token location.
    pub location: Location,
    /// The lexeme.
    pub lexeme: Lexeme,
    /// The token length, including whitespaces.
    pub length: usize,
}

impl Token {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Location, lexeme: Lexeme, length: usize) -> Self {
        Self {
            location,
            lexeme,
            length,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.location, self.lexeme)
    }
}
