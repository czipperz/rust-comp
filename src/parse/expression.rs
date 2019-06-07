use super::block::expect_block;
use super::combinator::*;
use super::parser::Parser;
use super::Error;
use crate::ast::*;
use crate::token::TokenValue;

pub fn expect_expression<'a>(parser: &mut Parser<'a>) -> Result<&'a Expression<'a>, Error> {
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

fn expect_variable_expression<'a>(parser: &mut Parser<'a>) -> Result<&'a Expression<'a>, Error> {
    parser
        .expect_label()
        .map(|name| parser.alloc(Expression::Variable(parser.alloc(Variable { name }))))
}

fn expect_block_expression<'a>(parser: &mut Parser<'a>) -> Result<&'a Expression<'a>, Error> {
    expect_block(parser).map(|b| parser.alloc(Expression::Block(b)))
}

fn expect_if_expression<'a>(parser: &mut Parser<'a>) -> Result<&'a Expression<'a>, Error> {
    expect_if_expression_(parser).map(|i| parser.alloc(Expression::If(i)))
}

fn expect_if_expression_<'a>(parser: &mut Parser<'a>) -> Result<&'a If<'a>, Error> {
    parser.expect_token(TokenValue::If)?;
    let condition = expect_expression(parser)?;
    let then = expect_block(parser)?;
    let else_ = if parser.expect_token(TokenValue::Else).is_ok() {
        Some(expect_else_expression(parser)?)
    } else {
        None
    };
    Ok(parser.alloc(If {
        condition,
        then,
        else_,
    }))
}

fn expect_else_expression<'a>(parser: &mut Parser<'a>) -> Result<&'a Else<'a>, Error> {
    fn else_expression_if<'a>(parser: &mut Parser<'a>) -> Result<&'a Else<'a>, Error> {
        expect_if_expression_(parser).map(|i| parser.alloc(Else::If(i)))
    }
    fn else_expression_block<'a>(parser: &mut Parser<'a>) -> Result<&'a Else<'a>, Error> {
        expect_block(parser).map(|b| parser.alloc(Else::Block(b)))
    }

    one_of(
        parser,
        &mut [else_expression_if, else_expression_block][..],
        Error::Expected("else expression", parser.span()),
    )
}

fn expect_while_expression<'a>(parser: &mut Parser<'a>) -> Result<&'a Expression<'a>, Error> {
    parser.expect_token(TokenValue::While)?;
    let condition = expect_expression(parser)?;
    let block = expect_block(parser)?;
    Ok(parser.alloc(Expression::While(parser.alloc(While { condition, block }))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena::Arena;
    use crate::lex::read_tokens;
    use crate::pos::*;
    use crate::token::TokenValue;

    #[test]
    fn test_expect_variable_expression() {
        let mut arena = Arena::new();
        let contents = "ab";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let expression = expect_variable_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(expression, &Expression::Variable(&Variable { name: "ab" }));
    }

    #[test]
    fn test_expect_variable_expression_fn_should_error() {
        let mut arena = Arena::new();
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
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
        let mut arena = Arena::new();
        let contents = "{}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let expression = expect_block_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(expression, &Expression::Block(&Block { statements: vec![] }));
    }

    #[test]
    fn test_expect_if_expression() {
        let mut arena = Arena::new();
        let contents = "if b {}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let expression = expect_if_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            expression,
            &Expression::If(&If {
                condition: &Expression::Variable(&Variable { name: "b" }),
                then: &Block { statements: vec![] },
                else_: None,
            })
        );
    }

    #[test]
    fn test_expect_if_else_expression() {
        let mut arena = Arena::new();
        let contents = "if b {} else {}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let expression = expect_if_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            expression,
            &Expression::If(&If {
                condition: &Expression::Variable(&Variable { name: "b" }),
                then: &Block { statements: vec![] },
                else_: Some(&Else::Block(&Block { statements: vec![] }))
            })
        );
    }

    #[test]
    fn test_expect_if_else_if_expression() {
        let mut arena = Arena::new();
        let contents = "if b {} else if c {}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let expression = expect_if_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            expression,
            &Expression::If(&If {
                condition: &Expression::Variable(&Variable { name: "b" }),
                then: &Block { statements: vec![] },
                else_: Some(&Else::If(&If {
                    condition: &Expression::Variable(&Variable { name: "c" }),
                    then: &Block { statements: vec![] },
                    else_: None,
                })),
            })
        );
    }

    #[test]
    fn test_expect_while_expression() {
        let mut arena = Arena::new();
        let contents = "while b {}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let expression = expect_while_expression(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            expression,
            &Expression::While(&While {
                condition: &Expression::Variable(&Variable { name: "b" }),
                block: &Block { statements: vec![] },
            })
        );
    }
}
