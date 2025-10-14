//!
//! The lexical comment parser error.
//!

///
/// The lexical comment parser error.
///
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// The lexeme is not a comment, which means that another parser must be run.
    NotAComment,
}
