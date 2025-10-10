//!
//! The lexical hex literal parser.
//!

#[cfg(test)]
mod tests;

pub mod error;
pub mod output;

use std::str;

use self::error::Error;
use self::output::Output;
use crate::test::function_call::parser::lexical::token::lexeme::literal::hex::Hex;

///
/// The parser state.
///
pub enum State {
    /// The initial state.
    LetterH,
    /// The `h` has been parsed so far.
    LetterE,
    /// The `he` has been parsed so far.
    LetterX,
    /// The `hex` has been parsed so far.
    DoubleQuoteOpen,
    /// The `"` has been parsed so far.
    Character,
}

///
/// Parses a hex literal.
///
/// Example:
/// 'hex"1234"'
///
pub fn parse(input: &str) -> Result<Output, Error> {
    let mut state = State::LetterH;
    let mut size = 0;
    let mut value = String::with_capacity(64);

    loop {
        let character = input.chars().nth(size);
        match state {
            State::LetterH => match character {
                Some('h') => {
                    size += 1;
                    state = State::LetterE;
                }
                _ => return Err(Error::NotAHex),
            },
            State::LetterE => match character {
                Some('e') => {
                    size += 1;
                    state = State::LetterX;
                }
                _ => return Err(Error::NotAHex),
            },
            State::LetterX => match character {
                Some('x') => {
                    size += 1;
                    state = State::DoubleQuoteOpen;
                }
                _ => return Err(Error::NotAHex),
            },
            State::DoubleQuoteOpen => match character {
                Some('\"') => {
                    size += 1;
                    state = State::Character;
                }
                _ => return Err(Error::NotAHex),
            },
            State::Character => match character {
                Some('\"') => {
                    size += 1;
                    return Ok(Output::new(size, Hex::new(value)));
                }
                Some(character) => {
                    size += 1;
                    if !Hex::CHARACTERS.contains(&character) {
                        return Err(Error::ExpectedOneOfHexadecimal {
                            found: character,
                            offset: size,
                        });
                    }
                    value.push(character);
                }
                None => return Err(Error::UnterminatedDoubleQuote { offset: size }),
            },
        }
    }
}
