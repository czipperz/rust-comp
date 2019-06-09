use super::block::expect_block;
use super::combinator::*;
use super::parser::Parser;
use super::Error;
use crate::ast::*;
use crate::token::TokenValue;

pub fn expect_expression<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, Error> {
    one_of(
        parser,
        &mut [
            expect_variable_expression,
            expect_paren_expression,
            expect_block_expression,
            expect_if_expression,
            expect_while_expression,
        ][..],
        Error::Expected("expression", parser.span()),
    )
}

fn expect_variable_expression<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, Error> {
    parser
        .expect_label()
        .map(|name| Expression::Variable(Variable { name }))
}

fn expect_paren_expression<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, Error> {
    parser.expect_token(TokenValue::OpenParen)?;
    let expression = expect_expression(parser)?;
    parser.expect_token(TokenValue::CloseParen)?;
    Ok(Expression::Paren(Box::new(expression)))
}

fn expect_block_expression<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, Error> {
    expect_block(parser).map(Expression::Block)
}

fn expect_if_expression<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, Error> {
    expect_if_expression_(parser).map(Expression::If)
}

fn expect_if_expression_<'a>(parser: &mut Parser<'a>) -> Result<If<'a>, Error> {
    parser.expect_token(TokenValue::If)?;
    let condition = expect_expression(parser)?;
    let then = expect_block(parser)?;
    let else_ = if parser.expect_token(TokenValue::Else).is_ok() {
        Some(Box::new(expect_else_expression(parser)?))
    } else {
        None
    };
    Ok(If {
        condition: Box::new(condition),
        then,
        else_,
    })
}

fn expect_else_expression<'a>(parser: &mut Parser<'a>) -> Result<Else<'a>, Error> {
    fn else_expression_if<'a>(parser: &mut Parser<'a>) -> Result<Else<'a>, Error> {
        expect_if_expression_(parser).map(Else::If)
    }
    fn else_expression_block<'a>(parser: &mut Parser<'a>) -> Result<Else<'a>, Error> {
        expect_block(parser).map(Else::Block)
    }

    one_of(
        parser,
        &mut [else_expression_if, else_expression_block][..],
        Error::Expected("else expression", parser.span()),
    )
}

fn expect_while_expression<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, Error> {
    parser.expect_token(TokenValue::While)?;
    let condition = expect_expression(parser)?;
    let block = expect_block(parser)?;
    Ok(Expression::While(While {
        condition: Box::new(condition),
        block,
    }))
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
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_variable_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(expression, Expression::Variable(Variable { name: "ab" }));
    }

    #[test]
    fn test_expect_paren_expression() {
        let contents = "(ab)";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_paren_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            expression,
            Expression::Paren(Box::new(Expression::Variable(Variable { name: "ab" })))
        );
    }

    #[test]
    fn test_expect_variable_expression_fn_should_error() {
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
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
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_block_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            expression,
            Expression::Block(Block {
                statements: vec![],
                expression: None,
            })
        );
    }

    #[test]
    fn test_expect_if_expression() {
        let contents = "if b {}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_if_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            expression,
            Expression::If(If {
                condition: Box::new(Expression::Variable(Variable { name: "b" })),
                then: Block {
                    statements: vec![],
                    expression: None,
                },
                else_: None,
            })
        );
    }

    #[test]
    fn test_expect_if_else_expression() {
        let contents = "if b {} else {}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_if_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            expression,
            Expression::If(If {
                condition: Box::new(Expression::Variable(Variable { name: "b" })),
                then: Block {
                    statements: vec![],
                    expression: None,
                },
                else_: Some(Box::new(Else::Block(Block {
                    statements: vec![],
                    expression: None,
                })))
            })
        );
    }

    #[test]
    fn test_expect_if_else_if_expression() {
        let contents = "if b {} else if c {}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_if_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            expression,
            Expression::If(If {
                condition: Box::new(Expression::Variable(Variable { name: "b" })),
                then: Block {
                    statements: vec![],
                    expression: None,
                },
                else_: Some(Box::new(Else::If(If {
                    condition: Box::new(Expression::Variable(Variable { name: "c" })),
                    then: Block {
                        statements: vec![],
                        expression: None,
                    },
                    else_: None,
                }))),
            })
        );
    }

    #[test]
    fn test_expect_while_expression() {
        let contents = "while b {}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let expression = expect_while_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            expression,
            Expression::While(While {
                condition: Box::new(Expression::Variable(Variable { name: "b" })),
                block: Block {
                    statements: vec![],
                    expression: None,
                },
            })
        );
    }
}
