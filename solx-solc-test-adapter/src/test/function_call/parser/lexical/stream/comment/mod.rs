//!
//! The lexical comment parser.
//!

#[cfg(test)]
mod tests;

pub mod error;
pub mod output;

use self::error::Error;
use self::output::Output;
use crate::test::function_call::parser::lexical::token::lexeme::comment::Comment;

///
/// The parser state.
///
pub enum State {
    /// The initial state.
    Start,
    /// The `#` has been parsed so far.
    NumberCharacter,
    /// The `#{comment}#` has been parsed so far.
    SecondNumberCharacter,
}

///
/// Parses a comment.
///
pub fn parse(input: &str) -> Result<Output, Error> {
    let mut state = State::Start;
    let mut length_chars = 0;
    let mut length_bytes = 0;
    let mut lines = 0;
    let mut column = 1;

    loop {
        let character = input.chars().nth(length_chars);
        match state {
            State::Start => match character {
                Some('#') => {
                    length_chars += 1;
                    length_bytes += 1;
                    column += 1;
                    state = State::NumberCharacter;
                }
                _ => return Err(Error::NotAComment),
            },
            State::NumberCharacter => match character {
                Some('#') => {
                    length_chars += 1;
                    length_bytes += 1;
                    column += 1;
                    state = State::SecondNumberCharacter;
                }
                Some('\n') => {
                    length_chars += 1;
                    length_bytes += 1;
                    column = 1;
                    lines += 1;
                }
                Some(char) => {
                    length_chars += 1;
                    length_bytes += char.len_utf8();
                    column += 1;
                }
                None => return Err(Error::NotAComment),
            },
            State::SecondNumberCharacter => match character {
                Some(char) => {
                    if !char.is_ascii_whitespace() {
                        return Err(Error::NotAComment);
                    }
                    length_chars += 1;
                    length_bytes += 1;
                    if char == '\n' {
                        lines += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                    let comment = Comment::new(input[1..length_bytes - 2].to_owned());
                    return Ok(Output::new(
                        length_bytes,
                        length_chars,
                        lines,
                        column,
                        comment,
                    ));
                }
                _ => return Err(Error::NotAComment),
            },
        }
    }
}
