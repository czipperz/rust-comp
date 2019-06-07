use super::block::expect_block;
use super::combinator::*;
use super::parser::Parser;
use super::type_::expect_type;
use super::Error;
use crate::arena::Allocator;
use crate::ast::*;
use crate::pos::Pos;
use crate::token::*;

pub fn parse<'a>(
    file_contents: &'a str,
    tokens: &'a [Token],
    eofpos: Pos,
    allocator: Allocator<'a>,
) -> Result<Vec<&'a TopLevel<'a>>, Error> {
    let mut parser = Parser::new(file_contents, tokens, eofpos, allocator);
    let top_levels = many(&mut parser, expect_top_level)?;

    if parser.index < tokens.len() {
        Err(Error::Expected("top level item", parser.span()))
    } else {
        Ok(top_levels)
    }
}

fn expect_top_level<'a>(parser: &mut Parser<'a>) -> Result<&'a TopLevel<'a>, Error> {
    one_of(
        parser,
        &mut [expect_toplevel_fn, expect_mod][..],
        Error::Expected("expression", parser.span()),
    )
}

fn expect_toplevel_fn<'a>(parser: &mut Parser<'a>) -> Result<&'a TopLevel<'a>, Error> {
    expect_fn(parser).map(|f| parser.alloc(TopLevel::Function(f)))
}

fn expect_fn<'a>(parser: &mut Parser<'a>) -> Result<&'a Function<'a>, Error> {
    parser.expect_token(TokenValue::Fn)?;
    let name = parser.expect_label()?;
    let parameters = expect_parameters(parser)?;
    let body = expect_block(parser)?;
    Ok(parser.alloc(Function {
        name,
        parameters,
        body,
    }))
}

fn expect_parameters<'a>(parser: &mut Parser<'a>) -> Result<Vec<&'a Parameter<'a>>, Error> {
    parser.expect_token(TokenValue::OpenParen)?;
    let parameters = many_separator(parser, expect_parameter, |parser| {
        parser.expect_token(TokenValue::Comma)
    })?;
    parser.expect_token(TokenValue::CloseParen)?;
    Ok(parameters)
}

fn expect_parameter<'a>(parser: &mut Parser<'a>) -> Result<&'a Parameter<'a>, Error> {
    let name = parser.expect_label()?;
    parser.expect_token(TokenValue::Colon)?;
    let type_ = expect_type(parser)?;
    Ok(parser.alloc(Parameter { name, type_ }))
}

fn expect_mod<'a>(parser: &mut Parser<'a>) -> Result<&'a TopLevel<'a>, Error> {
    parser.expect_token(TokenValue::Mod)?;
    let name = parser.expect_label()?;
    parser.expect_token(TokenValue::Semicolon)?;
    Ok(parser.alloc(TopLevel::ModFile(parser.alloc(ModFile { mod_: name }))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena::Arena;
    use crate::lex::read_tokens;

    #[test]
    fn test_parse_random_inputs_should_error() {
        let mut arena = Arena::new();
        let contents = "a b c";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let top_levels = parse(contents, &tokens, eofpos, arena.allocator());
        assert!(top_levels.is_err());
    }

    #[test]
    fn test_expect_fn_invalid() {
        let mut arena = Arena::new();
        let contents = "fn f () {";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        for i in 0..tokens.len() {
            dbg!(i);
            let mut parser = Parser::new(contents, &tokens[..i], eofpos, arena.allocator());
            assert!(expect_fn(&mut parser).is_err());
            assert_eq!(parser.index, i);
        }
    }

    #[test]
    fn test_expect_fn_matching() {
        let mut arena = Arena::new();
        let contents = "fn f () {}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let f = expect_fn(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(f.name, "f");
        assert_eq!(f.parameters.len(), 0);
        assert_eq!(f.body.statements.len(), 0);
    }

    #[test]
    fn test_expect_parameters_1_parameter() {
            let mut arena = Arena::new();
    let contents = "(x: i32)";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let parameters = expect_parameters(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            parameters,
            vec![&Parameter {
                name: "x",
                type_: &Type::Named(&NamedType { name: "i32" })
            }]
        );
    }

    #[test]
    fn test_expect_parameters_2_parameters() {
            let mut arena = Arena::new();
    let contents = "(x: i32, y: i32)";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let parameters = expect_parameters(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            parameters,
            vec![
                &Parameter {
                    name: "x",
                    type_: &Type::Named(&NamedType { name: "i32" })
                },
                &Parameter {
                    name: "y",
                    type_: &Type::Named(&NamedType { name: "i32" })
                }
            ]
        );
    }

    #[test]
    fn test_expect_parameter() {
            let mut arena = Arena::new();
    let contents = "x: i32";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let parameter = expect_parameter(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            parameter,
            &Parameter {
                name: "x",
                type_: &Type::Named(&NamedType { name: "i32" })
            }
        );
    }

    #[test]
    fn test_expect_mod() {
            let mut arena = Arena::new();
    let contents = "mod x;";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let mod_ = expect_mod(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(mod_, &TopLevel::ModFile(&ModFile { mod_: "x" }));
    }
}
