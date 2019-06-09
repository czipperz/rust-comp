use super::block::expect_block;
use super::combinator::*;
use super::error::Error;
use super::parser::Parser;
use super::type_::expect_type;
use crate::ast::*;
use crate::token::TokenKind;

pub fn expect_fn<'a>(parser: &mut Parser<'a>) -> Result<Function<'a>, Error> {
    parser.expect_token(TokenKind::Fn)?;
    let name = parser.expect_label()?;
    let parameters = expect_parameters(parser)?;
    let body = expect_block(parser)?;
    Ok(Function {
        name,
        parameters,
        body,
    })
}

fn expect_parameters<'a>(parser: &mut Parser<'a>) -> Result<Vec<Parameter<'a>>, Error> {
    parser.expect_token(TokenKind::OpenParen)?;
    let parameters = many_separator(parser, expect_parameter, |parser| {
        parser.expect_token(TokenKind::Comma)
    })?;
    parser.expect_token(TokenKind::CloseParen)?;
    Ok(parameters)
}

fn expect_parameter<'a>(parser: &mut Parser<'a>) -> Result<Parameter<'a>, Error> {
    let name = parser.expect_label()?;
    parser.expect_token(TokenKind::Colon)?;
    let type_ = expect_type(parser)?;
    Ok(Parameter { name, type_ })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::read_tokens;

    #[test]
    fn test_expect_fn_invalid() {
        let contents = "fn f () {";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        for i in 0..tokens.len() {
            dbg!(i);
            let mut parser = Parser::new(contents, &tokens[..i], eofpos);
            assert!(expect_fn(&mut parser).is_err());
            assert_eq!(parser.index, i);
        }
    }

    #[test]
    fn test_expect_fn_matching() {
        let contents = "fn f () {}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let f = expect_fn(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(f.name, "f");
        assert_eq!(f.parameters.len(), 0);
        assert_eq!(f.body.statements.len(), 0);
    }

    #[test]
    fn test_expect_parameters_1_parameter() {
        let contents = "(x: i32)";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let parameters = expect_parameters(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            parameters,
            vec![Parameter {
                name: "x",
                type_: Type::Named(NamedType { name: "i32" })
            }]
        );
    }

    #[test]
    fn test_expect_parameters_2_parameters() {
        let contents = "(x: i32, y: i32)";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let parameters = expect_parameters(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            parameters,
            vec![
                Parameter {
                    name: "x",
                    type_: Type::Named(NamedType { name: "i32" })
                },
                Parameter {
                    name: "y",
                    type_: Type::Named(NamedType { name: "i32" })
                }
            ]
        );
    }

    #[test]
    fn test_expect_parameter() {
        let contents = "x: i32";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let parameter = expect_parameter(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            parameter,
            Parameter {
                name: "x",
                type_: Type::Named(NamedType { name: "i32" })
            }
        );
    }
}
