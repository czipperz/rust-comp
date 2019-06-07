use super::block::expect_block;
use super::combinator::*;
use super::parser::Parser;
use super::type_::expect_type;
use super::Error;
use crate::ast::*;
use crate::pos::Pos;
use crate::token::*;

pub fn parse<'a>(
    file_contents: &'a str,
    tokens: &'a [Token],
    eofpos: Pos,
) -> Result<Vec<TopLevel<'a>>, Error> {
    let mut parser = Parser::new(file_contents, tokens, eofpos);
    let top_levels = many(&mut parser, expect_top_level)?;

    if parser.index < tokens.len() {
        Err(Error::Expected("top level item", parser.span()))
    } else {
        Ok(top_levels)
    }
}

fn expect_top_level<'a>(parser: &mut Parser<'a>) -> Result<TopLevel<'a>, Error> {
    one_of(
        parser,
        &mut [expect_toplevel_fn, expect_mod][..],
        Error::Expected("expression", parser.span()),
    )
}

fn expect_toplevel_fn<'a>(parser: &mut Parser<'a>) -> Result<TopLevel<'a>, Error> {
    expect_fn(parser).map(TopLevel::Function)
}

fn expect_fn<'a>(parser: &mut Parser<'a>) -> Result<Function<'a>, Error> {
    parser.expect_token(TokenValue::Fn)?;
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
    parser.expect_token(TokenValue::OpenParen)?;
    let parameters = many_separator(parser, expect_parameter, |parser| {
        parser.expect_token(TokenValue::Comma)
    })?;
    parser.expect_token(TokenValue::CloseParen)?;
    Ok(parameters)
}

fn expect_parameter<'a>(parser: &mut Parser<'a>) -> Result<Parameter<'a>, Error> {
    let name = parser.expect_label()?;
    parser.expect_token(TokenValue::Colon)?;
    let type_ = expect_type(parser)?;
    Ok(Parameter { name, type_ })
}

fn expect_mod<'a>(parser: &mut Parser<'a>) -> Result<TopLevel<'a>, Error> {
    parser.expect_token(TokenValue::Mod)?;
    let name = parser.expect_label()?;
    parser.expect_token(TokenValue::Semicolon)?;
    Ok(TopLevel::ModFile(ModFile { mod_: name }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::read_tokens;

    #[test]
    fn test_parse_random_inputs_should_error() {
        let contents = "a b c";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let top_levels = parse(contents, &tokens, eofpos);
        assert!(top_levels.is_err());
    }

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

    #[test]
    fn test_expect_mod() {
        let contents = "mod x;";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let mod_ = expect_mod(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(mod_, TopLevel::ModFile(ModFile { mod_: "x" }));
    }
}
