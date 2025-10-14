//!
//! The lexical token symbol lexeme.
//!

use std::fmt;

///
/// The minimal logical character group, which is usually a delimiter, operator, or special symbol.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    /// The ( character
    ParenthesisLeft,
    /// The ) character
    ParenthesisRight,
    /// The [ character
    BracketSquareLeft,
    /// The ] character
    BracketSquareRight,
    /// The < character
    Lesser,
    /// The > character
    Greater,
    /// The : character
    Colon,
    /// The , character
    Comma,
    /// The ~ character
    Tilde,
    /// The -> character
    Arrow,
    /// The # character
    Number,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParenthesisLeft => write!(f, "("),
            Self::ParenthesisRight => write!(f, ")"),
            Self::BracketSquareLeft => write!(f, "["),
            Self::BracketSquareRight => write!(f, "]"),
            Self::Lesser => write!(f, "<"),
            Self::Greater => write!(f, ">"),
            Self::Colon => write!(f, ":"),
            Self::Comma => write!(f, ","),
            Self::Tilde => write!(f, "~"),
            Self::Arrow => write!(f, "->"),
            Self::Number => write!(f, "#"),
        }
    }
}
