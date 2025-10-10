//!
//! The literal.
//!

pub mod alignment;
pub mod boolean;
pub mod builder;
pub mod hex;
pub mod integer;
pub mod string;

use self::boolean::Literal as BooleanLiteral;
use self::hex::Literal as HexLiteral;
use self::integer::Literal as IntegerLiteral;
use self::string::Literal as StringLiteral;

///
/// The literal.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    /// The boolean literal.
    Boolean(BooleanLiteral),
    /// The integer literal.
    Integer(IntegerLiteral),
    /// The string literal.
    String(StringLiteral),
    /// The hex literal.
    Hex(HexLiteral),
}

impl Literal {
    ///
    /// Converts literal to bytes.
    ///
    pub fn as_bytes_be(&self) -> anyhow::Result<Vec<u8>> {
        match self {
            Literal::Boolean(boolean) => Ok(boolean.as_bytes_be()),
            Literal::Integer(integer) => Ok(integer.as_bytes_be()),
            Literal::Hex(hex) => Ok(hex.as_bytes_be()),
            Literal::String(string) => string
                .as_bytes_be()
                .map_err(|error| anyhow::anyhow!("Failed to process string: {error}")),
        }
    }
}
