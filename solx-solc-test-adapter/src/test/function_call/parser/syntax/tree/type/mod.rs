//!
//! The type.
//!

pub mod builder;
pub mod variant;

use self::variant::Variant;
use crate::test::function_call::parser::lexical::Location;

///
/// The type.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    /// The location of the syntax construction.
    pub location: Location,
    /// The type variant.
    pub variant: Variant,
}

impl Type {
    ///
    /// Creates a type.
    ///
    pub fn new(location: Location, variant: Variant) -> Self {
        Self { location, variant }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.variant {
            Variant::String => write!(f, "string"),
            Variant::Boolean => write!(f, "bool"),
            Variant::Address => write!(f, "address"),
            Variant::Function => write!(f, "function"),
            Variant::Bytes { byte_length } => match byte_length {
                Some(byte_length) => write!(f, "bytes{byte_length}"),
                None => write!(f, "bytes"),
            },
            Variant::IntegerUnsigned { bit_length } => write!(f, "uint{bit_length}"),
            Variant::IntegerSigned { bit_length } => write!(f, "int{bit_length}"),
            Variant::Array { inner, size } => match size {
                Some(size) => write!(f, "{inner}[{size}]"),
                None => write!(f, "{inner}[]"),
            },
            Variant::Tuple { inners } => {
                write!(
                    f,
                    "({})",
                    inners
                        .iter()
                        .map(|r#type| r#type.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
        }
    }
}
