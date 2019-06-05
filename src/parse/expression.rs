use super::block::expect_block;
use super::combinator::*;
use super::parser::Parser;
use super::Error;
use crate::ast::*;
use crate::token::TokenValue;

pub fn expect_expression(parser: &mut Parser) -> Result<Expression, Error> {
    one_of(
        parser,
        &mut [
            expect_variable_expression,
            expect_block_expression,
            expect_if_expression,
            expect_while_expression,
        ][..],
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

fn expect_if_expression(parser: &mut Parser) -> Result<Expression, Error> {
    expect_if_expression_(parser).map(Expression::If)
}

fn expect_if_expression_(parser: &mut Parser) -> Result<IfExpression, Error> {
    parser.expect_token(TokenValue::If)?;
    let condition = expect_expression(parser)?;
    let then = expect_block(parser)?;
    let else_ = if parser.expect_token(TokenValue::Else).is_ok() {
        Some(Box::new(expect_else_expression(parser)?))
    } else {
        None
    };
    Ok(IfExpression {
        condition: Box::new(condition),
        then,
        else_,
    })
}

fn expect_else_expression(parser: &mut Parser) -> Result<ElseExpression, Error> {
    fn else_expression_if(parser: &mut Parser) -> Result<ElseExpression, Error> {
        expect_if_expression_(parser).map(ElseExpression::If)
    }
    fn else_expression_block(parser: &mut Parser) -> Result<ElseExpression, Error> {
        expect_block(parser).map(ElseExpression::Block)
    }

    one_of(
        parser,
        &mut [else_expression_if, else_expression_block][..],
        Error::Expected("else expression", parser.span()),
    )
}

fn expect_while_expression(parser: &mut Parser) -> Result<Expression, Error> {
    parser.expect_token(TokenValue::While)?;
    let condition = expect_expression(parser)?;
    let block = expect_block(parser)?;
    Ok(Expression::While(WhileExpression {
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
        assert_eq!(expression, Expression::Variable("ab".to_string()));
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
        assert_eq!(expression, Expression::Block(Block { statements: vec![] }));
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
            Expression::If(IfExpression {
                condition: Box::new(Expression::Variable("b".to_string())),
                then: Block { statements: vec![] },
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
            Expression::If(IfExpression {
                condition: Box::new(Expression::Variable("b".to_string())),
                then: Block { statements: vec![] },
                else_: Some(Box::new(ElseExpression::Block(Block {
                    statements: vec![]
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
            Expression::If(IfExpression {
                condition: Box::new(Expression::Variable("b".to_string())),
                then: Block { statements: vec![] },
                else_: Some(Box::new(ElseExpression::If(IfExpression {
                    condition: Box::new(Expression::Variable("c".to_string())),
                    then: Block { statements: vec![] },
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
            Expression::While(WhileExpression {
                condition: Box::new(Expression::Variable("b".to_string())),
                block: Block { statements: vec![] },
            })
        );
    }
}
