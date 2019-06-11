use super::combinator::*;
use super::expression::expect_expression;
use super::parser::Parser;
use super::type_::expect_type;
use super::Error;
use crate::ast::*;
use crate::token::*;

pub fn expect_statement<'a>(parser: &mut Parser<'a, '_>) -> Result<Statement<'a>, Error> {
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

fn expect_let_statement<'a>(parser: &mut Parser<'a, '_>) -> Result<Statement<'a>, Error> {
    parser.expect_token(TokenKind::Let)?;
    let name = if let Ok(name) = parser.expect_label() {
        Some(name)
    } else if parser.expect_token(TokenKind::Underscore).is_ok() {
        None
    } else {
        return Err(Error::ExpectedToken(TokenKind::Label, parser.span()));
    };
    let type_ = if parser.expect_token(TokenKind::Colon).is_ok() {
        Some(expect_type(parser)?)
    } else {
        None
    };
    let value = if parser.expect_token(TokenKind::Set).is_ok() {
        Some(expect_expression(parser)?)
    } else {
        None
    };
    parser.expect_token(TokenKind::Semicolon)?;
    Ok(Statement::Let(Let { name, type_, value }))
}

fn expect_empty_statement<'a>(parser: &mut Parser<'a, '_>) -> Result<Statement<'a>, Error> {
    parser
        .expect_token(TokenKind::Semicolon)
        .map(|_| Statement::Empty)
}

fn expect_expression_statement<'a>(parser: &mut Parser<'a, '_>) -> Result<Statement<'a>, Error> {
    let expression = expect_expression(parser)?;
    if needs_semicolon(&expression) {
        parser.expect_token(TokenKind::Semicolon)?;
    }
    Ok(Statement::Expression(expression))
}

pub fn needs_semicolon(expression: &Expression<'_>) -> bool {
    match *expression {
        Expression::Variable(_) => true,
        Expression::Paren(_) => true,
        Expression::Block(_) => false,
        Expression::If(_) => false,
        Expression::While(_) => false,
        Expression::Match(_) => false,
        Expression::Binary(_) => true,
        Expression::FunctionCall(_) => true,
        Expression::Bool(_) => true,
        Expression::Tuple(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;

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
        let statement = statement.unwrap();
        assert_eq!(index, len);
        assert_eq!(statement, Statement::Empty);
    }

    #[test]
    fn test_let_statement_with_type_and_value() {
        let (index, len, statement) = parse(expect_let_statement, "let x: i32 = y;");
        let statement = statement.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            statement,
            Statement::Let(Let {
                name: Some("x"),
                type_: Some(Type::Named(NamedType { name: "i32" })),
                value: Some(Expression::Variable(Variable { name: "y" })),
            })
        );
    }

    #[test]
    fn test_let_statement_with_value_and_hole() {
        let (index, len, statement) = parse(expect_let_statement, "let _ = y;");
        let statement = statement.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            statement,
            Statement::Let(Let {
                name: None,
                type_: None,
                value: Some(Expression::Variable(Variable { name: "y" })),
            })
        );
    }

    #[test]
    fn test_let_statement_without_value() {
        let (index, len, statement) = parse(expect_let_statement, "let x;");
        let statement = statement.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            statement,
            Statement::Let(Let {
                name: Some("x"),
                type_: None,
                value: None
            })
        );
    }

    #[test]
    fn test_let_statement_let_if_else_error_no_semicolon() {
        let (index, len, statement) = parse(expect_let_statement, "let x = if b {} else {}");
        let error = statement.unwrap_err();
        assert_eq!(index, len);
        assert_eq!(
            error,
            Error::ExpectedToken(
                TokenKind::Semicolon,
                Span {
                    file: 0,
                    start: 23,
                    end: 24,
                }
            )
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
        let statement = statement.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            statement,
            Statement::Expression(Expression::Variable(Variable { name: "ab" }))
        );
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
