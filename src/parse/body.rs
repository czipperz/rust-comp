use super::combinator::*;
use super::parser::Parser;
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
        &mut [expect_empty_statement, expect_expression_statement][..],
        Error::Expected("statement", parser.span()),
    )
}

fn expect_empty_statement(parser: &mut Parser) -> Result<Statement, Error> {
    parser
        .expect_token(TokenValue::Semicolon)
        .map(|_| Statement::Empty)
}

fn expect_expression_statement(parser: &mut Parser) -> Result<Statement, Error> {
    expect_expression(parser).map(Statement::Expression)
}

fn expect_expression(parser: &mut Parser) -> Result<Expression, Error> {
    parser
        .expect_label()
        .map(|label| Expression::Variable(label.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pos::*;

    #[test]
    fn test_expect_block_no_statements() {
        let tokens = make_tokens(vec![TokenValue::OpenCurly, TokenValue::CloseCurly]);
        let mut parser = Parser::new("{}", &tokens, Pos::start());
        let statements = expect_block(&mut parser).unwrap();
        assert_eq!(parser.index, 2);
        assert_eq!(statements.len(), 0);
    }

    #[test]
    fn test_expect_block_with_statements() {
        let tokens = make_tokens(vec![
            TokenValue::OpenCurly,
            TokenValue::Semicolon,
            TokenValue::Semicolon,
            TokenValue::CloseCurly,
        ]);
        let mut parser = Parser::new("{;;}", &tokens, Pos::start());
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
        let tokens = make_tokens(vec![TokenValue::Semicolon]);
        let mut parser = Parser::new(";", &tokens, Pos::start());
        let statement = expect_statement(&mut parser).unwrap();
        assert_eq!(parser.index, 1);
        assert_eq!(statement, Statement::Empty);
    }

    #[test]
    fn test_expect_expression_variable() {
        let tokens = [Token {
            value: TokenValue::Label,
            span: Span::range(Pos::start(), "ab"),
        }];
        let mut parser = Parser::new("ab", &tokens, Pos::start());
        let expression = expect_expression(&mut parser).unwrap();
        assert_eq!(parser.index, 1);
        assert_eq!(expression, Expression::Variable("ab".to_string()));
    }

    #[test]
    fn test_expect_expression_fn_should_error() {
        let tokens = [make_token(TokenValue::Fn)];
        let mut parser = Parser::new("ab", &tokens, Pos::start());
        let expression = expect_expression(&mut parser);
        assert_eq!(parser.index, 0);
        assert_eq!(
            expression,
            Err(Error::ExpectedToken(
                TokenValue::Label,
                Span {
                    start: Pos::start(),
                    end: Pos::start()
                }
            ))
        );
    }
}
