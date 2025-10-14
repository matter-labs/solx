//!
//! The lexical token comment lexeme.
//!

use std::fmt;

///
/// The source code comment, which is dropped during the lexical analysis.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    /// The inner comment contents.
    inner: String,
}

impl Comment {
    ///
    /// Creates a comment.
    ///
    pub fn new(inner: String) -> Self {
        Self { inner }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
