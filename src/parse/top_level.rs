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
    let visibility = if parser.expect_token(TokenKind::Pub).is_ok() {
        Visibility::Public
    } else {
        Visibility::Private
    };
    let kind = one_of(
        parser,
        &mut [expect_toplevel_fn, expect_mod, expect_use][..],
        Error::Expected("expression", parser.span()),
    )?;
    Ok(TopLevel { kind, visibility })
}

fn expect_toplevel_fn<'a>(parser: &mut Parser<'a>) -> Result<TopLevelKind<'a>, Error> {
    expect_fn(parser).map(TopLevelKind::Function)
}

fn expect_fn<'a>(parser: &mut Parser<'a>) -> Result<Function<'a>, Error> {
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

fn expect_mod<'a>(parser: &mut Parser<'a>) -> Result<TopLevelKind<'a>, Error> {
    parser.expect_token(TokenKind::Mod)?;
    let name = parser.expect_label()?;
    parser.expect_token(TokenKind::Semicolon)?;
    Ok(TopLevelKind::ModFile(ModFile { mod_: name }))
}

fn expect_use<'a>(parser: &mut Parser<'a>) -> Result<TopLevelKind<'a>, Error> {
    parser.expect_token(TokenKind::Use)?;
    let mut path = Vec::new();
    path.push(parser.expect_label()?);
    if parser.expect_token(TokenKind::ColonColon).is_ok() {
        path.push(parser.expect_label()?);
    }
    parser.expect_token(TokenKind::Semicolon)?;
    Ok(TopLevelKind::Use(Use { path }))
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
        assert_eq!(mod_, TopLevelKind::ModFile(ModFile { mod_: "x" }));
    }

    #[test]
    fn test_expect_use_label() {
        let contents = "use x;";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let mod_ = expect_use(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(mod_, TopLevelKind::Use(Use { path: vec!["x"] }));
    }

    #[test]
    fn test_expect_use_label_colon_colon_label() {
        let contents = "use x::y;";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let mod_ = expect_use(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            mod_,
            TopLevelKind::Use(Use {
                path: vec!["x", "y"]
            })
        );
    }
}
