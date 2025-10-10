//!
//! The lexical string literal parser.
//!

#[cfg(test)]
mod tests;

pub mod error;
pub mod output;

use self::error::Error;
use self::output::Output;
use crate::test::function_call::parser::lexical::token::lexeme::literal::string::String as LexicalString;

///
/// The parser state.
///
pub enum State {
    /// The initial state.
    DoubleQuoteOpen,
    /// The `"` has been parsed so far.
    Character,
}

///
/// Parses a string literal.
///
/// Example:
/// '"abc"'
///
pub fn parse(input: &str) -> Result<Output, Error> {
    let mut state = State::DoubleQuoteOpen;
    let mut length_chars = 0;
    let mut length_bytes = 0;
    let mut lines = 0;
    let mut column = 1;
    let mut value = String::new();
    loop {
        let character = input.chars().nth(length_chars);
        match state {
            State::DoubleQuoteOpen => match character {
                Some('\"') => {
                    length_chars += 1;
                    length_bytes += 1;
                    column += 1;
                    state = State::Character;
                }
                _ => return Err(Error::NotAString),
            },
            State::Character => match character {
                Some('\"') => {
                    length_chars += 1;
                    length_bytes += 1;
                    return Ok(Output::new(
                        length_bytes,
                        length_chars,
                        lines,
                        column,
                        LexicalString::new(value),
                    ));
                }
                Some('\n') => {
                    value.push('\n');
                    length_chars += 1;
                    length_bytes += 1;
                    lines += 1;
                    column += 1;
                }
                Some(character) => {
                    value.push(character);
                    length_chars += 1;
                    length_bytes += character.len_utf8();
                    column += 1;
                }
                None => return Err(Error::UnterminatedDoubleQuote { lines, column }),
            },
        }
    }
}
