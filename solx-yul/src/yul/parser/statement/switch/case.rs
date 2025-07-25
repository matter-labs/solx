//!
//! The switch statement case.
//!

use std::collections::BTreeSet;

use crate::dependencies::Dependencies;
use crate::yul::error::Error;
use crate::yul::lexer::token::lexeme::Lexeme;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;
use crate::yul::lexer::Lexer;
use crate::yul::parser::dialect::Dialect;
use crate::yul::parser::error::Error as ParserError;
use crate::yul::parser::statement::block::Block;
use crate::yul::parser::statement::expression::literal::Literal;

///
/// The Yul switch statement case.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(bound = "P: serde::de::DeserializeOwned")]
pub struct Case<P>
where
    P: Dialect,
{
    /// The location.
    pub location: Location,
    /// The matched constant.
    pub literal: Literal,
    /// The case block.
    pub block: Block<P>,
}

impl<P> Case<P>
where
    P: Dialect,
{
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let token = crate::yul::parser::take_or_next(initial, lexer)?;

        let (location, literal) = match token {
            token @ Token {
                lexeme: Lexeme::Literal(_),
                location,
                ..
            } => (location, Literal::parse(lexer, Some(token))?),
            token => {
                return Err(ParserError::InvalidToken {
                    location: token.location,
                    expected: vec!["{literal}"],
                    found: token.lexeme.to_string(),
                }
                .into());
            }
        };

        let block = Block::parse(lexer, None)?;

        Ok(Self {
            location,
            literal,
            block,
        })
    }

    ///
    /// Get the list of unlinked deployable libraries.
    ///
    pub fn get_unlinked_libraries(&self) -> BTreeSet<String> {
        self.block.get_unlinked_libraries()
    }

    ///
    /// Get the list of EVM dependencies.
    ///
    pub fn accumulate_evm_dependencies(&self, dependencies: &mut Dependencies) {
        self.block.accumulate_evm_dependencies(dependencies);
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
    fn error_invalid_token_literal() {
        let input = r#"
object "Test" {
    code {
        {
            return(0, 0)
        }
    }
    object "Test_deployed" {
        code {
            {
                switch 42
                    case x {}
                    default {}
                }
            }
        }
    }
}
    "#;

        let mut lexer = Lexer::new(input);
        let result = Object::<DefaultDialect>::parse(
            &mut lexer,
            None,
            era_compiler_common::CodeSegment::Deploy,
        );
        assert_eq!(
            result,
            Err(Error::InvalidToken {
                location: Location::new(12, 26),
                expected: vec!["{literal}"],
                found: "x".to_owned(),
            }
            .into())
        );
    }
}
