use super::block::expect_block;
use super::combinator::*;
use super::parser::Parser;
use super::Error;
use crate::ast::*;

pub fn expect_expression(parser: &mut Parser) -> Result<Expression, Error> {
    one_of(
        parser,
        &mut [expect_variable_expression, expect_block_expression][..],
        Error::Expected("expression", parser.span()),
    )
}

fn expect_variable_expression(parser: &mut Parser) -> Result<Expression, Error> {
    parser
        .expect_label()
        .map(|label| Expression::Variable(label.to_string()))
}

fn expect_block_expression(parser: &mut Parser) -> Result<Expression, Error> {
    expect_block(parser).map(Expression::Block)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::read_tokens;
    use crate::pos::*;
    use crate::token::TokenValue;

    #[test]
    fn test_expect_variable_expression() {
        let contents = "ab";
        let (tokens, eofpos) = read_tokens(0, &contents).unwrap();
        let mut parser = Parser::new(&contents, &tokens, eofpos);
        let expression = expect_variable_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(expression, Expression::Variable("ab".to_string()));
    }

    #[test]
    fn test_expect_variable_expression_fn_should_error() {
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(0, &contents).unwrap();
        let mut parser = Parser::new(&contents, &tokens, eofpos);
        let expression = expect_variable_expression(&mut parser);
        assert_eq!(parser.index, 0);
        assert_eq!(
            expression,
            Err(Error::ExpectedToken(
                TokenValue::Label,
                Span {
                    file: 0,
                    start: 0,
                    end: 2
                },
            ))
        );
    }

    #[test]
    fn test_expect_block_expression() {
        let contents = "{}";
        let (tokens, eofpos) = read_tokens(0, &contents).unwrap();
        let mut parser = Parser::new(&contents, &tokens, eofpos);
        let expression = expect_block_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(expression, Expression::Block(Block { statements: vec![] }));
    }
}
