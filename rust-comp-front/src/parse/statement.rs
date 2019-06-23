use super::combinator::*;
use super::expression::expect_expression;
use super::parser::Parser;
use super::tree::*;
use super::type_::expect_type;
use super::Error;
use crate::token::*;

pub fn expect_statement<'a>(parser: &mut Parser) -> Result<Statement, Error> {
    one_of(
        parser,
        &mut [
            expect_let_statement,
            expect_empty_statement,
            expect_expression_statement,
        ][..],
        Error::Expected("statement", parser.span()),
    )
}

fn expect_let_statement<'a>(parser: &mut Parser) -> Result<Statement, Error> {
    let let_span = parser.expect_token(TokenKind::Let)?;

    let name = if let Ok(name) = parser.expect_token(TokenKind::Label) {
        Ok(name)
    } else if let Ok(underscore_span) = parser.expect_token(TokenKind::Underscore) {
        Err(underscore_span)
    } else {
        return Err(Error::ExpectedToken(TokenKind::Label, parser.span()));
    };

    let type_ = if let Ok(colon_span) = parser.expect_token(TokenKind::Colon) {
        Some(LetType {
            colon_span,
            type_: expect_type(parser)?,
        })
    } else {
        None
    };

    let value = if let Ok(set_span) = parser.expect_token(TokenKind::Set) {
        Some(LetValue {
            set_span,
            value: expect_expression(parser)?,
        })
    } else {
        None
    };

    let semicolon_span = parser.expect_token(TokenKind::Semicolon)?;

    Ok(Statement {
        kind: StatementKind::Let(Let {
            let_span,
            name,
            type_,
            value,
        }),
        semicolon_span: Some(semicolon_span),
    })
}

fn expect_empty_statement<'a>(parser: &mut Parser) -> Result<Statement, Error> {
    Ok(Statement {
        kind: StatementKind::Empty,
        semicolon_span: Some(parser.expect_token(TokenKind::Semicolon)?),
    })
}

fn expect_expression_statement<'a>(parser: &mut Parser) -> Result<Statement, Error> {
    let expression = expect_expression(parser)?;
    let semicolon_span = if needs_semicolon(&expression) {
        Some(parser.expect_token(TokenKind::Semicolon)?)
    } else {
        None
    };
    Ok(Statement {
        kind: StatementKind::Expression(expression),
        semicolon_span,
    })
}

pub fn needs_semicolon(expression: &Expression) -> bool {
    match *expression {
        Expression::Variable(_) => true,
        Expression::Paren(_) => true,
        Expression::Block(_) => false,
        Expression::If(_) => false,
        Expression::While(_) => false,
        Expression::Match(_) => false,
        Expression::Binary(_) => true,
        Expression::FunctionCall(_) => true,
        Expression::MemberAccess(_) => true,
        Expression::Bool(_) => true,
        Expression::Tuple(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;
    use assert_matches::assert_matches;

    #[test]
    fn test_expect_statement_empty() {
        let (index, len, statement) = parse(expect_statement, "");
        let error = statement.unwrap_err();
        assert_eq!(index, len);
        assert_eq!(
            error,
            Error::Expected(
                "statement",
                Span {
                    file: 0,
                    start: 0,
                    end: 1
                }
            )
        );
    }

    #[test]
    fn test_expect_statement_semicolon() {
        let (index, len, statement) = parse(expect_statement, ";");
        assert_eq!(index, len);
        let statement = statement.unwrap();
        assert_eq!(statement.kind, StatementKind::Empty);
    }

    #[test]
    fn test_let_statement_with_type_and_value() {
        let (index, len, statement) = parse(expect_let_statement, "let x: i32 = y;");
        assert_eq!(index, len);
        assert_matches!(statement, Ok(Statement {
            kind: StatementKind::Let(Let {
                name, type_, value, ..
            }),
            ..
        }) =>
        {
            assert!(name.is_ok());
            assert!(type_.is_some());
            assert!(value.is_some());
        });
    }

    #[test]
    fn test_let_statement_with_value_and_hole() {
        let (index, len, statement) = parse(expect_let_statement, "let _ = y;");
        assert_eq!(index, len);
        assert_matches!(statement, Ok(Statement {
            kind: StatementKind::Let(Let {
                name, type_, value, ..
            }),
            ..
        }) =>
        {
            assert!(name.is_err());
            assert!(type_.is_none());
            assert!(value.is_some());
        });
    }

    #[test]
    fn test_let_statement_without_value() {
        let (index, len, statement) = parse(expect_let_statement, "let x;");
        assert_eq!(index, len);
        assert_matches!(statement, Ok(Statement {
            kind: StatementKind::Let(Let {
                name, type_, value, ..
            }),
            ..
        }) =>
        {
            assert!(name.is_ok());
            assert!(type_.is_none());
            assert!(value.is_none());
        });
    }

    #[test]
    fn test_let_statement_let_if_else_error_no_semicolon() {
        let (index, len, statement) = parse(expect_let_statement, "let x = if b {} else {}");
        assert_eq!(index, len);
        assert_eq!(
            statement,
            Err(Error::ExpectedToken(
                TokenKind::Semicolon,
                Span {
                    file: 0,
                    start: 23,
                    end: 24,
                }
            ))
        );
    }

    #[test]
    fn test_expect_expression_statement_variable_no_semicolon_should_error() {
        let (index, len, statement) = parse(expect_expression_statement, "ab");
        let error = statement.unwrap_err();
        assert_eq!(index, len);
        assert_eq!(
            error,
            Error::ExpectedToken(
                TokenKind::Semicolon,
                Span {
                    file: 0,
                    start: 2,
                    end: 3,
                }
            )
        );
    }

    #[test]
    fn test_expect_expression_statement_paren_expression_no_semicolon_should_error() {
        let (index, len, statement) = parse(expect_expression_statement, "(ab)");
        let error = statement.unwrap_err();
        assert_eq!(index, len);
        assert_eq!(
            error,
            Error::ExpectedToken(
                TokenKind::Semicolon,
                Span {
                    file: 0,
                    start: 4,
                    end: 5,
                }
            )
        );
    }

    #[test]
    fn test_expect_expression_statement_binary_expression_no_semicolon_should_error() {
        let (index, len, statement) = parse(expect_expression_statement, "a + b");
        let error = statement.unwrap_err();
        assert_eq!(index, len);
        assert_eq!(
            error,
            Error::ExpectedToken(
                TokenKind::Semicolon,
                Span {
                    file: 0,
                    start: 5,
                    end: 6,
                }
            )
        );
    }

    #[test]
    fn test_expect_expression_statement_function_call_no_semicolon_should_error() {
        let (index, len, statement) = parse(expect_expression_statement, "f(a + b)");
        let error = statement.unwrap_err();
        assert_eq!(index, len);
        assert_eq!(
            error,
            Error::ExpectedToken(
                TokenKind::Semicolon,
                Span {
                    file: 0,
                    start: 8,
                    end: 9,
                }
            )
        );
    }

    #[test]
    fn test_expect_expression_statement_bool_no_semicolon_should_error() {
        let (index, len, statement) = parse(expect_expression_statement, "false");
        let error = statement.unwrap_err();
        assert_eq!(index, len);
        assert_eq!(
            error,
            Error::ExpectedToken(
                TokenKind::Semicolon,
                Span {
                    file: 0,
                    start: 5,
                    end: 6,
                }
            )
        );
    }

    #[test]
    fn test_expect_expression_statement_variable_semicolon() {
        let (index, len, statement) = parse(expect_expression_statement, "ab;");
        assert_eq!(index, len);
        assert!(statement.is_ok(),);
    }

    #[test]
    fn test_expect_expression_statement_block_doesnt_consume_semicolon() {
        let (index, len, statement) = parse(expect_expression_statement, "{ b; };");
        assert_eq!(index, len - 1);
        assert!(statement.is_ok());
    }

    #[test]
    fn test_expect_expression_statement_if_doesnt_consume_semicolon() {
        let (index, len, statement) = parse(expect_expression_statement, "if b {};");
        assert_eq!(index, len - 1);
        assert!(statement.is_ok());
    }

    #[test]
    fn test_expect_expression_statement_while_doesnt_consume_semicolon() {
        let (index, len, statement) = parse(expect_expression_statement, "while b {};");
        assert_eq!(index, len - 1);
        assert!(statement.is_ok());
    }

    #[test]
    fn test_expect_expression_statement_match_doesnt_consume_semicolon() {
        let (index, len, statement) = parse(expect_expression_statement, "match b {};");
        assert_eq!(index, len - 1);
        assert!(statement.is_ok());
    }
}
