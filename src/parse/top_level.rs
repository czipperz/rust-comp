use super::body::expect_block;
use super::parser::*;
use super::Error;
use crate::ast::*;
use crate::lex::{Token, TokenValue};

pub fn parse(file_contents: &str, tokens: &[Token]) -> Result<Vec<TopLevel>, Error> {
    Parser::new(file_contents, tokens).many(expect_top_level)
}

fn expect_top_level(parser: &mut Parser) -> Result<TopLevel, Error> {
    expect_fn(parser).map(TopLevel::Function)
}

fn expect_fn(parser: &mut Parser) -> Result<Function, Error> {
    parser.expect_token(TokenValue::Fn)?;
    let name = parser.expect_label()?;
    let parameters = expect_parameters(parser)?;
    let body = expect_block(parser)?;
    Ok(Function {
        name: name.to_string(),
        parameters,
        body,
    })
}

fn expect_parameters(parser: &mut Parser) -> Result<Vec<Parameter>, Error> {
    parser.expect_token(TokenValue::OpenParen)?;
    parser.expect_token(TokenValue::CloseParen)?;
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenValue::*;

    #[test]
    fn test_expect_fn_invalid() {
        let tokens = make_tokens(vec![Fn, Label, OpenParen, CloseParen, OpenCurly]);
        for i in 0..tokens.len() {
            dbg!(i);
            let mut parser = Parser::new("fn f () {", &tokens[0..i]);
            assert!(expect_fn(&mut parser).is_err());
            assert_eq!(parser.index, i);
        }
    }

    #[test]
    fn test_expect_fn_matching() {
        use crate::pos::*;
        let tokens = [
            make_token(Fn),
            Token {
                value: Label,
                span: Span {
                    start: Pos {
                        line: 0,
                        column: 3,
                        index: 3,
                    },
                    end: Pos {
                        line: 0,
                        column: 4,
                        index: 4,
                    },
                },
            },
            make_token(OpenParen),
            make_token(CloseParen),
            make_token(OpenCurly),
            make_token(CloseCurly),
        ];
        let mut parser = Parser::new("fn f () {}", &tokens);
        let f = expect_fn(&mut parser).unwrap();
        assert_eq!(parser.index, 6);
        assert_eq!(f.name, "f");
        assert_eq!(f.parameters.len(), 0);
        assert_eq!(f.body.len(), 0);
    }
}
