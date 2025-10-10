//!
//! The lexical comment parser output.
//!

use crate::test::function_call::parser::lexical::token::lexeme::comment::Comment;

///
/// The lexical comment parser output.
///
#[derive(Debug, PartialEq, Eq)]
pub struct Output {
    /// The number of bytes in the comment.
    pub length_bytes: usize,
    /// The number of characters in the comment.
    pub length_chars: usize,
    /// The numbers of lines in the comment.
    pub lines: usize,
    /// The column where the comment ends.
    pub column: usize,
    /// The comment data.
    pub comment: Comment,
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
        comment: Comment,
    ) -> Self {
        Self {
            length_bytes,
            length_chars,
            lines,
            column,
            comment,
        }
    }
}
