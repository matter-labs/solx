//!
//! The lexical token hex literal lexeme.
//!

use std::fmt;

///
/// The lexical hex literal.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hex {
    /// The inner contents.
    pub inner: std::string::String,
}

impl Hex {
    /// Characters allowed in the hex literal.
    pub const CHARACTERS: [char; 22] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B',
        'C', 'D', 'E', 'F',
    ];

    ///
    /// Creates a hex literal value.
    ///
    pub fn new(inner: std::string::String) -> Self {
        Self { inner }
    }
}

#[allow(clippy::from_over_into)]
impl Into<std::string::String> for Hex {
    fn into(self) -> std::string::String {
        let mut string = "hex\"".to_owned();
        string.push_str(&self.inner);
        string.push('"');
        string
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: String = self.to_owned().into();
        write!(f, "{string}")
    }
}
