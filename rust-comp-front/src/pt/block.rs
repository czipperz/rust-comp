use super::combinator::*;
use super::{Error, Parse, Parser};
use crate::ast::*;
use crate::token::TokenKind;

impl<'a> Parse<'a> for Block<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Block<'a>, Error> {
        parser.expect_token(TokenKind::OpenCurly)?;

        let mut statements = Vec::new();
        let expression;
        loop {
            let old_index = parser.index;
            match Statement::parse(parser) {
                Ok(x) => statements.push(x),
                Err(statement_err) => {
                    if old_index == parser.index {
                        expression = maybe(parser)?;
                    } else {
                        let statement_index = parser.index;
                        parser.index = old_index;
                        // If we cannot parse the expression, that is an error
                        expression = Some(Expression::parse(parser).map_err(|expression_err| {
                            // If statement parsing was more successful, use its error message
                            if parser.index < statement_index {
                                parser.index = statement_index;
                                statement_err
                            } else {
                                expression_err
                            }
                        })?);
                    }
                    break;
                }
            }
        }
        parser.expect_token(TokenKind::CloseCurly)?;
        Ok(Block {
            statements,
            expression: expression.map(Box::new),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::parse;
    use super::*;
    use crate::pos::Span;

    #[test]
    fn test_parse_block_no_statements() {
        let (index, len, block) = parse("{}");
        assert_eq!(index, len);
        assert_eq!(
            block,
            Ok(Block {
                statements: vec![],
                expression: None
            })
        );
    }

    #[test]
    fn test_parse_block_with_empty_statements() {
        let (index, len, block) = parse("{;;}");
        assert_eq!(index, len);
        assert_eq!(
            block,
            Ok(Block {
                statements: vec![Statement::Empty, Statement::Empty],
                expression: None
            })
        );
    }

    #[test]
    fn test_parse_block_with_statement_then_expression() {
        let (index, len, block) = parse("{x;y}");
        assert_eq!(index, len);
        assert_eq!(
            block,
            Ok(Block {
                statements: vec![Statement::Expression(Expression::Variable(Variable {
                    name: "x"
                }))],
                expression: Some(Box::new(Expression::Variable(Variable { name: "y" })))
            })
        );
    }

    #[test]
    fn test_parse_block_with_statement_then_malformed_expression() {
        let (index, len, block) = parse::<Block>("{x;(y}");
        assert_eq!(index, len - 1);
        assert_eq!(
            block,
            Err(Error::ExpectedToken(
                TokenKind::CloseParen,
                Span {
                    file: 0,
                    start: 5,
                    end: 6,
                }
            ))
        );
    }

    #[test]
    fn test_parse_block_with_statement_then_invalid_statement() {
        let (index, len, block) = parse::<Block>("{x;let}");
        assert_eq!(index, len - 1);
        assert_eq!(
            block,
            Err(Error::ExpectedToken(
                TokenKind::Label,
                Span {
                    file: 0,
                    start: 6,
                    end: 7,
                }
            ))
        );
    }

    #[test]
    fn test_parse_block_with_statement_then_if_no_closing_curly() {
        let (index, len, block) = parse::<Block>("{x;if x {");
        assert_eq!(index, len);
        assert_eq!(
            block,
            Err(Error::ExpectedToken(
                TokenKind::CloseCurly,
                Span {
                    file: 0,
                    start: 9,
                    end: 10,
                }
            ))
        );
    }
}
