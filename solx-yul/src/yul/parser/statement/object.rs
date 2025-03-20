//!
//! The Yul object.
//!

use std::collections::BTreeSet;
use std::collections::HashSet;

use crate::yul::dependencies::Dependencies;
use crate::yul::error::Error;
use crate::yul::lexer::token::lexeme::keyword::Keyword;
use crate::yul::lexer::token::lexeme::literal::Literal;
use crate::yul::lexer::token::lexeme::symbol::Symbol;
use crate::yul::lexer::token::lexeme::Lexeme;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;
use crate::yul::lexer::Lexer;
use crate::yul::parser::dialect::Dialect;
use crate::yul::parser::error::Error as ParserError;
use crate::yul::parser::statement::code::Code;

///
/// The upper-level Yul object, representing the deploy code.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(bound = "P: serde::de::DeserializeOwned")]
pub struct Object<P>
where
    P: Dialect,
{
    /// The location.
    pub location: Location,
    /// The identifier.
    pub identifier: String,
    /// The code.
    pub code: Code<P>,
    /// The optional inner object, representing the runtime code.
    pub inner_object: Option<Box<Self>>,
    /// The factory dependency objects, which are represented by nested Yul object. The nested
    /// objects are duplicates of the upper-level objects describing the dependencies, so only
    /// their identifiers are preserved. The identifiers are used to address upper-level objects.
    pub factory_dependencies: HashSet<String>,
}

impl<P> Object<P>
where
    P: Dialect,
{
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let token = crate::yul::parser::take_or_next(initial, lexer)?;

        let location = match token {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Object),
                location,
                ..
            } => location,
            token => {
                return Err(ParserError::InvalidToken {
                    location: token.location,
                    expected: vec!["object"],
                    found: token.lexeme.to_string(),
                }
                .into());
            }
        };

        let identifier = match lexer.next()? {
            Token {
                lexeme: Lexeme::Literal(Literal::String(literal)),
                ..
            } => literal.inner,
            token => {
                return Err(ParserError::InvalidToken {
                    location: token.location,
                    expected: vec!["{string}"],
                    found: token.lexeme.to_string(),
                }
                .into());
            }
        };
        let is_runtime_code = identifier.ends_with("_deployed");

        match lexer.next()? {
            Token {
                lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                ..
            } => {}
            token => {
                return Err(ParserError::InvalidToken {
                    location: token.location,
                    expected: vec!["{"],
                    found: token.lexeme.to_string(),
                }
                .into());
            }
        }

        let code = Code::parse(lexer, None)?;
        let mut inner_object = None;
        let mut factory_dependencies = HashSet::new();

        if !is_runtime_code {
            inner_object = match lexer.peek()? {
                Token {
                    lexeme: Lexeme::Keyword(Keyword::Object),
                    ..
                } => {
                    let mut object = Self::parse(lexer, None)?;

                    if format!("{identifier}_deployed") != object.identifier {
                        return Err(ParserError::InvalidObjectName {
                            location: object.location,
                            expected: format!("{identifier}_deployed"),
                            found: object.identifier,
                        }
                        .into());
                    }

                    factory_dependencies.extend(object.factory_dependencies.drain());
                    Some(Box::new(object))
                }
                _ => None,
            };

            if let Token {
                lexeme: Lexeme::Identifier(identifier),
                ..
            } = lexer.peek()?
            {
                if identifier.inner.as_str() == "data" {
                    let _data = lexer.next()?;
                    let _identifier = lexer.next()?;
                    let _metadata = lexer.next()?;
                }
            };
        }

        loop {
            match lexer.next()? {
                Token {
                    lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                    ..
                } => break,
                token @ Token {
                    lexeme: Lexeme::Keyword(Keyword::Object),
                    ..
                } => {
                    let dependency = Self::parse(lexer, Some(token))?;
                    factory_dependencies.insert(dependency.identifier);
                }
                Token {
                    lexeme: Lexeme::Identifier(identifier),
                    ..
                } if identifier.inner.as_str() == "data" => {
                    let _identifier = lexer.next()?;
                    let _metadata = lexer.next()?;
                }
                token => {
                    return Err(ParserError::InvalidToken {
                        location: token.location,
                        expected: vec!["object", "}"],
                        found: token.lexeme.to_string(),
                    }
                    .into());
                }
            }
        }

        Ok(Self {
            location,
            identifier,
            code,
            inner_object,
            factory_dependencies,
        })
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> BTreeSet<String> {
        let mut missing_libraries = self.code.get_missing_libraries();
        if let Some(inner_object) = &self.inner_object {
            missing_libraries.extend(inner_object.get_missing_libraries());
        }
        missing_libraries
    }

    ///
    /// Get the list of EVM dependencies.
    ///
    pub fn get_evm_dependencies(&self, runtime_code: Option<&Self>) -> Dependencies {
        let mut dependencies = Dependencies::new(self.identifier.as_str());
        self.code.accumulate_evm_dependencies(&mut dependencies);

        if let Some(runtime_code) = runtime_code {
            if !dependencies.inner.contains(&runtime_code.identifier) {
                dependencies
                    .inner
                    .insert(0, runtime_code.identifier.to_owned());
            }
        }

        dependencies
    }
}

#[cfg(test)]
mod tests {
    use crate::yul::lexer::token::location::Location;
    use crate::yul::lexer::Lexer;
    use crate::yul::parser::dialect::DefaultDialect;
    use crate::yul::parser::error::Error;
    use crate::yul::parser::statement::object::Object;

    #[test]
    fn error_invalid_token_object() {
        let input = r#"
class "Test" {
    code {
        {
            return(0, 0)
        }
    }
    object "Test_deployed" {
        code {
            {
                return(0, 0)
            }
        }
    }
}
    "#;

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::<DefaultDialect>::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidToken {
                location: Location::new(2, 1),
                expected: vec!["object"],
                found: "class".to_owned(),
            }
            .into())
        );
    }

    #[test]
    fn error_invalid_token_identifier() {
        let input = r#"
object 256 {
    code {
        {
            return(0, 0)
        }
    }
    object "Test_deployed" {
        code {
            {
                return(0, 0)
            }
        }
    }
}
    "#;

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::<DefaultDialect>::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidToken {
                location: Location::new(2, 8),
                expected: vec!["{string}"],
                found: "256".to_owned(),
            }
            .into())
        );
    }

    #[test]
    fn error_invalid_token_bracket_curly_left() {
        let input = r#"
object "Test" (
    code {
        {
            return(0, 0)
        }
    }
    object "Test_deployed" {
        code {
            {
                return(0, 0)
            }
        }
    }
}
    "#;

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::<DefaultDialect>::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidToken {
                location: Location::new(2, 15),
                expected: vec!["{"],
                found: "(".to_owned(),
            }
            .into())
        );
    }

    #[test]
    fn error_invalid_token_object_inner() {
        let input = r#"
object "Test" {
    code {
        {
            return(0, 0)
        }
    }
    class "Test_deployed" {
        code {
            {
                return(0, 0)
            }
        }
    }
}
    "#;

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::<DefaultDialect>::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidToken {
                location: Location::new(8, 5),
                expected: vec!["object", "}"],
                found: "class".to_owned(),
            }
            .into())
        );
    }

    #[test]
    fn error_invalid_object_name() {
        let input = r#"
object "Test" {
    code {
        {
            return(0, 0)
        }
    }
    object "Invalid" {
        code {
            {
                return(0, 0)
            }
        }
    }
}
    "#;

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::<DefaultDialect>::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidObjectName {
                location: Location::new(8, 5),
                expected: "Test_deployed".to_owned(),
                found: "Invalid".to_owned(),
            }
            .into())
        );
    }
}
