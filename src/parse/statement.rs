use super::combinator::*;
use super::expression::expect_expression;
use super::parser::Parser;
use super::type_::expect_type;
use super::Error;
use crate::ast::*;
use crate::token::*;

pub fn expect_statement(parser: &mut Parser) -> Result<Statement, Error> {
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

fn expect_let_statement(parser: &mut Parser) -> Result<Statement, Error> {
    parser.expect_token(TokenValue::Let)?;
    let name = parser.expect_label()?.to_string();
    let type_ = if parser.expect_token(TokenValue::Colon).is_ok() {
        Some(expect_type(parser)?)
    } else {
        None
    };
    let value = if parser.expect_token(TokenValue::Set).is_ok() {
        Some(expect_expression(parser)?)
    } else {
        None
    };
    parser.expect_token(TokenValue::Semicolon)?;
    Ok(Statement::Let(Let { name, type_, value }))
}

fn expect_empty_statement(parser: &mut Parser) -> Result<Statement, Error> {
    parser
        .expect_token(TokenValue::Semicolon)
        .map(|_| Statement::Empty)
}

fn expect_expression_statement(parser: &mut Parser) -> Result<Statement, Error> {
    let expression = expect_expression(parser)?;
    match expression {
        Expression::Variable(_) => parser.expect_token(TokenValue::Semicolon)?,
        Expression::Block(_) => (),
        Expression::If(_) => (),
        Expression::While(_) => (),
    }
    Ok(Statement::Expression(expression))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::read_tokens;
    use crate::pos::*;

    #[test]
    fn test_expect_statement_empty() {
        let contents = "";
        let mut parser = Parser::new(contents, &[], Pos { file: 0, index: 0 });
        assert!(expect_statement(&mut parser).is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_statement_semicolon() {
        let contents = &";";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let statement = expect_statement(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(statement, Statement::Empty);
    }

    #[test]
    fn test_let_statement_with_type_and_value() {
        let contents = "let x: i32 = y;";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let statement = expect_let_statement(&mut parser);
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            statement,
            Ok(Statement::Let(Let {
                name: "x".to_string(),
                type_: Some(Type::Named(NamedType {
                    name: "i32".to_string()
                })),
                value: Some(Expression::Variable(Variable {
                    name: "y".to_string()
                })),
            }))
        );
    }

    #[test]
    fn test_let_statement_with_value() {
        let contents = "let x = y;";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let statement = expect_let_statement(&mut parser);
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            statement,
            Ok(Statement::Let(Let {
                name: "x".to_string(),
                type_: None,
                value: Some(Expression::Variable(Variable {
                    name: "y".to_string()
                })),
            }))
        );
    }

    #[test]
    fn test_let_statement_without_value() {
        let contents = "let x;";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let statement = expect_let_statement(&mut parser);
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            statement,
            Ok(Statement::Let(Let {
                name: "x".to_string(),
                type_: None,
                value: None
            }))
        );
    }

    #[test]
    fn test_let_statement_let_if_else_error_no_semicolon() {
        let contents = "let x = if b {} else {}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let statement = expect_let_statement(&mut parser);
        assert_eq!(parser.index, tokens.len());
        assert!(statement.is_err());
    }

    #[test]
    fn test_expect_expression_statement_variable_no_semicolon_should_error() {
        let contents = "ab";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let statement = expect_expression_statement(&mut parser);
        assert_eq!(parser.index, tokens.len());
        assert!(statement.is_err());
    }

    #[test]
    fn test_expect_expression_statement_variable_semicolon() {
        let contents = "ab;";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let statement = expect_expression_statement(&mut parser);
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            statement,
            Ok(Statement::Expression(Expression::Variable(Variable {
                name: "ab".to_string()
            })))
        );
    }

    #[test]
    fn test_expect_expression_statement_if_doesnt_consume_semicolon() {
        let contents = "if b {};";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let statement = expect_expression_statement(&mut parser);
        assert_eq!(parser.index, tokens.len() - 1);
        assert!(statement.is_ok());
    }

    #[test]
    fn test_expect_expression_statement_block_doesnt_consume_semicolon() {
        let contents = "{ b; };";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let statement = expect_expression_statement(&mut parser);
        assert_eq!(parser.index, tokens.len() - 1);
        assert!(statement.is_ok());
    }
}
