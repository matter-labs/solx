//!
//! The string literal lexeme.
//!

use crate::yul::lexer::token::lexeme::Lexeme;
use crate::yul::lexer::token::lexeme::Literal;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;

///
/// The string literal lexeme.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct String {
    /// The inner string contents.
    pub inner: std::string::String,
    /// Whether the string is hexadecimal.
    pub is_hexadecimal: bool,
}

impl String {
    ///
    /// Creates a string literal value.
    ///
    pub fn new(inner: ::std::string::String, is_hexadecimal: bool) -> Self {
        Self {
            inner,
            is_hexadecimal,
        }
    }

    ///
    /// Parses the value from the source code slice.
    ///
    pub fn parse(input: &str) -> Option<Token> {
        let mut length = 0;

        let is_string = input[length..].starts_with('"');
        let is_escaped_string = input[length..].starts_with(r#"\""#);
        let is_hex_string = input[length..].starts_with(r#"hex""#);

        if !is_string && !is_escaped_string && !is_hex_string {
            return None;
        }

        if is_string {
            length += 1;
        }
        if is_escaped_string {
            length += 2;
        }
        if is_hex_string {
            length += r#"hex""#.len();
        }

        let mut string = std::string::String::new();
        loop {
            if input[length..].starts_with('\\') {
                string.push_str(&input[length..length + 2]);
                length += 2;
                continue;
            }

            if input[length..].starts_with(r#"\""#) {
                length += 2;
                break;
            }
            if input[length..].starts_with('"') {
                length += 1;
                break;
            }

            string.push_str(&input[length..length + 1]);
            length += 1;
        }

        let literal = Self::new(string, is_hex_string);

        Some(Token::new(
            Location::new(0, length),
            Lexeme::Literal(Literal::String(literal)),
            length,
        ))
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
