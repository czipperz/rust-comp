use super::combinator::*;
use super::parser::Parser;
use super::type_::expect_type;
use super::Error;
use crate::ast::*;
use crate::token::*;

pub fn expect_block(parser: &mut Parser) -> Result<Vec<Statement>, Error> {
    parser.expect_token(TokenValue::OpenCurly)?;
    let statements = many(parser, expect_statement)?;
    parser.expect_token(TokenValue::CloseCurly)?;
    Ok(statements)
}

fn expect_statement(parser: &mut Parser) -> Result<Statement, Error> {
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
    Ok(Statement::Let(name, type_, value))
}

fn expect_empty_statement(parser: &mut Parser) -> Result<Statement, Error> {
    parser
        .expect_token(TokenValue::Semicolon)
        .map(|_| Statement::Empty)
}

fn expect_expression_statement(parser: &mut Parser) -> Result<Statement, Error> {
    let expression = expect_expression(parser)?;
    parser.expect_token(TokenValue::Semicolon)?;
    Ok(Statement::Expression(expression))
}

fn expect_expression(parser: &mut Parser) -> Result<Expression, Error> {
    parser
        .expect_label()
        .map(|label| Expression::Variable(label.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::read_tokens;
    use crate::pos::*;

    #[test]
    fn test_expect_block_no_statements() {
        let contents = "{}";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let statements = expect_block(&mut parser).unwrap();
        assert_eq!(parser.index, 2);
        assert_eq!(statements.len(), 0);
    }

    #[test]
    fn test_expect_block_with_empty_statements() {
        let contents = "{;;}";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let statements = expect_block(&mut parser).unwrap();
        assert_eq!(parser.index, 4);
        assert_eq!(statements, [Statement::Empty, Statement::Empty]);
    }

    #[test]
    fn test_expect_statement_empty() {
        let mut parser = Parser::new("", &[], Pos::start());
        assert!(expect_statement(&mut parser).is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_statement_semicolon() {
        let contents = ";";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let statement = expect_statement(&mut parser).unwrap();
        assert_eq!(parser.index, 1);
        assert_eq!(statement, Statement::Empty);
    }

    #[test]
    fn test_let_statement_with_type_and_value() {
        let contents = "let x: i32 = y;";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_let_statement(&mut parser);
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            expression,
            Ok(Statement::Let(
                "x".to_string(),
                Some(Type::Named("i32".to_string())),
                Some(Expression::Variable("y".to_string()))
            ))
        );
    }

    #[test]
    fn test_let_statement_with_value() {
        let contents = "let x = y;";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_let_statement(&mut parser);
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            expression,
            Ok(Statement::Let(
                "x".to_string(),
                None,
                Some(Expression::Variable("y".to_string()))
            ))
        );
    }

    #[test]
    fn test_let_statement_without_value() {
        let contents = "let x;";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_let_statement(&mut parser);
        assert_eq!(parser.index, tokens.len());
        assert_eq!(expression, Ok(Statement::Let("x".to_string(), None, None)));
    }

    #[test]
    fn test_expect_expression_statement_variable_no_semicolon_should_error() {
        let contents = "ab";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_expression_statement(&mut parser);
        assert_eq!(parser.index, 1);
        assert!(expression.is_err());
    }

    #[test]
    fn test_expect_expression_statement_variable_semicolon() {
        let contents = "ab;";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_expression_statement(&mut parser);
        assert_eq!(parser.index, 2);
        assert_eq!(
            expression,
            Ok(Statement::Expression(Expression::Variable(
                "ab".to_string()
            )))
        );
    }

    #[test]
    fn test_expect_expression_variable() {
        let contents = "ab";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_expression(&mut parser).unwrap();
        assert_eq!(parser.index, 1);
        assert_eq!(expression, Expression::Variable("ab".to_string()));
    }

    #[test]
    fn test_expect_expression_fn_should_error() {
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_expression(&mut parser);
        assert_eq!(parser.index, 0);
        assert_eq!(
            expression,
            Err(Error::ExpectedToken(
                TokenValue::Label,
                Span::range(Pos::start(), "fn"),
            ))
        );
    }
}
