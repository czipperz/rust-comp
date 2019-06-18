use super::combinator::*;
use super::expression::expect_expression;
use super::parser::Parser;
use super::statement::expect_statement;
use super::tree::*;
use super::Error;
use crate::token::TokenKind;

pub fn expect_block(parser: &mut Parser) -> Result<Block, Error> {
    let open_curly_span = parser.expect_token(TokenKind::OpenCurly)?;

    let mut statements = Vec::new();
    let expression;
    loop {
        let old_index = parser.index;
        match expect_statement(parser) {
            Ok(x) => statements.push(x),
            Err(statement_err) => {
                if old_index == parser.index {
                    expression = maybe(parser, expect_expression)?;
                } else {
                    let statement_index = parser.index;
                    parser.index = old_index;
                    // If we cannot parse the expression, that is an error
                    expression = Some(expect_expression(parser).map_err(|expression_err| {
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
    let close_curly_span = parser.expect_token(TokenKind::CloseCurly)?;

    Ok(Block {
        open_curly_span,
        statements,
        expression: expression.map(Box::new),
        close_curly_span,
    })
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;

    #[test]
    fn test_expect_block_no_statements() {
        let (index, len, block) = parse(expect_block, "{}");
        let block = block.unwrap();
        assert_eq!(index, len);
        assert_eq!(block.statements.len(), 0);
        assert_eq!(block.expression, None);
    }

    #[test]
    fn test_expect_block_with_empty_statements() {
        let (index, len, block) = parse(expect_block, "{;;}");
        let block = block.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            block.statements,
            [
                Statement {
                    kind: StatementKind::Empty,
                    semicolon_span: Some(Span {
                        file: 0,
                        start: 1,
                        end: 2
                    }),
                },
                Statement {
                    kind: StatementKind::Empty,
                    semicolon_span: Some(Span {
                        file: 0,
                        start: 2,
                        end: 3
                    }),
                },
            ]
        );
        assert_eq!(block.expression, None);
    }

    #[test]
    fn test_expect_block_with_statement_then_expression() {
        let (index, len, block) = parse(expect_block, "{x;y}");
        let block = block.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            block.statements,
            [Statement {
                kind: StatementKind::Expression(Expression::Variable(Variable {
                    name: Span {
                        file: 0,
                        start: 1,
                        end: 2
                    }
                })),
                semicolon_span: Some(Span {
                    file: 0,
                    start: 2,
                    end: 3
                }),
            }]
        );
        assert_eq!(
            block.expression,
            Some(Box::new(Expression::Variable(Variable {
                name: Span {
                    file: 0,
                    start: 3,
                    end: 4
                }
            })))
        );
    }

    #[test]
    fn test_expect_block_with_statement_then_malformed_expression() {
        let (index, len, block) = parse(expect_block, "{x;(y}");
        let error = block.unwrap_err();
        assert_eq!(index, len - 1);
        assert_eq!(
            error,
            Error::ExpectedToken(
                TokenKind::CloseParen,
                Span {
                    file: 0,
                    start: 5,
                    end: 6,
                }
            )
        );
    }

    #[test]
    fn test_expect_block_with_statement_then_invalid_statement() {
        let (index, len, block) = parse(expect_block, "{x;let}");
        let error = block.unwrap_err();
        assert_eq!(index, len - 1);
        assert_eq!(
            error,
            Error::ExpectedToken(
                TokenKind::Label,
                Span {
                    file: 0,
                    start: 6,
                    end: 7,
                }
            )
        );
    }

    #[test]
    fn test_expect_block_with_statement_then_if_no_closing_curly() {
        let (index, len, block) = parse(expect_block, "{x;if x {");
        let error = block.unwrap_err();
        assert_eq!(index, len);
        assert_eq!(
            error,
            Error::ExpectedToken(
                TokenKind::CloseCurly,
                Span {
                    file: 0,
                    start: 9,
                    end: 10,
                }
            )
        );
    }
}
