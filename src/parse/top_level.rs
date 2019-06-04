use super::block::expect_block;
use super::combinator::*;
use super::parser::Parser;
use super::type_::expect_type;
use super::Error;
use crate::ast::*;
use crate::pos::Pos;
use crate::token::*;

pub fn parse(file_contents: &str, tokens: &[Token], eofpos: Pos) -> Result<Vec<TopLevel>, Error> {
    let mut parser = Parser::new(file_contents, tokens, eofpos);
    let top_levels = many(
        &mut parser,
        expect_top_level,
    )?;

    if parser.index < tokens.len() {
        Err(Error::Expected("top level item", parser.span()))
    } else {
        Ok(top_levels)
    }
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
    let parameters = many_separator(parser, expect_parameter, |parser| {
        parser.expect_token(TokenValue::Comma)
    })?;
    parser.expect_token(TokenValue::CloseParen)?;
    Ok(parameters)
}

fn expect_parameter(parser: &mut Parser) -> Result<Parameter, Error> {
    let name = parser.expect_label()?;
    parser.expect_token(TokenValue::Colon)?;
    let type_ = expect_type(parser)?;
    Ok(Parameter {
        name: name.to_string(),
        type_,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::read_tokens;

    #[test]
    fn test_parse_random_inputs_should_error() {
        let contents = "a b c";
        let (tokens, eofpos) = read_tokens(0, &contents).unwrap();
        let top_levels = parse(&contents, &tokens, eofpos);
        assert!(top_levels.is_err());
    }

    #[test]
    fn test_expect_fn_invalid() {
        let contents = "fn f () {";
        let (tokens, eofpos) = read_tokens(0, &contents).unwrap();
        for i in 0..tokens.len() {
            dbg!(i);
            let mut parser = Parser::new(&contents, &tokens[..i], eofpos);
            assert!(expect_fn(&mut parser).is_err());
            assert_eq!(parser.index, i);
        }
    }

    #[test]
    fn test_expect_fn_matching() {
        let contents = "fn f () {}";
        let (tokens, eofpos) = read_tokens(0, &contents).unwrap();
        let mut parser = Parser::new(&contents, &tokens, eofpos);
        let f = expect_fn(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(f.name, "f");
        assert_eq!(f.parameters.len(), 0);
        assert_eq!(f.body.statements.len(), 0);
    }

    #[test]
    fn test_expect_parameters_1_parameter() {
        let contents = "(x: i32)";
        let (tokens, eofpos) = read_tokens(0, &contents).unwrap();
        let mut parser = Parser::new(&contents, &tokens, eofpos);
        let parameters = expect_parameters(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            parameters,
            vec![Parameter {
                name: "x".to_string(),
                type_: Type::Named("i32".to_string())
            }]
        );
    }

    #[test]
    fn test_expect_parameters_2_parameters() {
        let contents = "(x: i32, y: i32)";
        let (tokens, eofpos) = read_tokens(0, &contents).unwrap();
        let mut parser = Parser::new(&contents, &tokens, eofpos);
        let parameters = expect_parameters(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            parameters,
            vec![
                Parameter {
                    name: "x".to_string(),
                    type_: Type::Named("i32".to_string())
                },
                Parameter {
                    name: "y".to_string(),
                    type_: Type::Named("i32".to_string())
                }
            ]
        );
    }

    #[test]
    fn test_expect_parameter() {
        let contents = "x: i32";
        let (tokens, eofpos) = read_tokens(0, &contents).unwrap();
        let mut parser = Parser::new(&contents, &tokens, eofpos);
        let parameter = expect_parameter(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            parameter,
            Parameter {
                name: "x".to_string(),
                type_: Type::Named("i32".to_string())
            }
        );
    }
}
