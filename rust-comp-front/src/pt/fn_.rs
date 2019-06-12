use super::combinator::*;
use super::{Error, Parse, Parser};
use crate::ast::*;
use crate::token::TokenKind;

impl<'a> Parse<'a> for Function<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Function<'a>, Error> {
        parser.expect_token(TokenKind::Fn)?;
        let name = parser.expect_label()?;
        let parameters = parse_parameters(parser)?;
        let return_type = parse_return_type(parser)?;
        let body = Block::parse(parser)?;
        Ok(Function {
            name,
            parameters,
            return_type,
            body,
        })
    }
}

fn parse_parameters<'a>(parser: &mut Parser<'a, '_>) -> Result<Vec<Parameter<'a>>, Error> {
    parser.expect_token(TokenKind::OpenParen)?;
    let parameters = many_comma_separated(parser)?;
    parser.expect_token(TokenKind::CloseParen)?;
    Ok(parameters)
}

impl<'a> Parse<'a> for Parameter<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Parameter<'a>, Error> {
        let name = parser.expect_label()?;
        parser.expect_token(TokenKind::Colon)?;
        let type_ = Type::parse(parser)?;
        Ok(Parameter { name, type_ })
    }
}

fn parse_return_type<'a>(parser: &mut Parser<'a, '_>) -> Result<Type<'a>, Error> {
    if parser.expect_token(TokenKind::ThinArrow).is_ok() {
        Type::parse(parser)
    } else {
        Ok(Type::Tuple(vec![]))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{parse, parse_fn};
    use super::*;
    use crate::lex::read_tokens;

    #[test]
    fn test_parse_fn_invalid() {
        let contents = "fn f () {";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        for i in 0..tokens.len() {
            dbg!(i);
            let mut parser = Parser::new(contents, &tokens[..i], eofpos);
            assert!(Function::parse(&mut parser).is_err());
            assert_eq!(parser.index, i);
        }
    }

    #[test]
    fn test_parse_fn_matching() {
        let (index, len, f) = parse("fn f () {}");
        assert_eq!(index, len);
        assert_eq!(
            f,
            Ok(Function {
                name: "f",
                parameters: vec![],
                body: Block {
                    statements: vec![],
                    expression: None
                },
                return_type: Type::Tuple(vec![])
            })
        );
    }

    #[test]
    fn test_parse_parameters_1_parameter() {
        let (index, len, parameters) = parse_fn(parse_parameters, "(x: i32)");
        assert_eq!(index, len);
        assert_eq!(
            parameters,
            Ok(vec![Parameter {
                name: "x",
                type_: Type::Named(NamedType { name: "i32" })
            }])
        );
    }

    #[test]
    fn test_parse_parameters_2_parameters() {
        let (index, len, parameters) = parse_fn(parse_parameters, "(x: i32, y: i32)");
        assert_eq!(index, len);
        assert_eq!(
            parameters,
            Ok(vec![
                Parameter {
                    name: "x",
                    type_: Type::Named(NamedType { name: "i32" })
                },
                Parameter {
                    name: "y",
                    type_: Type::Named(NamedType { name: "i32" })
                }
            ])
        );
    }

    #[test]
    fn test_parse_parameter() {
        let (index, len, parameter) = parse("x: i32");
        assert_eq!(index, len);
        assert_eq!(
            parameter,
            Ok(Parameter {
                name: "x",
                type_: Type::Named(NamedType { name: "i32" })
            })
        );
    }

    #[test]
    fn test_parse_return_type_nothing() {
        let (index, _, return_type) = parse_fn(parse_return_type, "{");
        assert_eq!(index, 0);
        assert_eq!(return_type, Ok(Type::Tuple(vec![])));
    }

    #[test]
    fn test_parse_return_type_something() {
        let (index, len, return_type) = parse_fn(parse_return_type, "-> &x {");
        assert_eq!(index, len - 1);
        assert_eq!(
            return_type,
            Ok(Type::Ref(Box::new(Type::Named(NamedType { name: "x" }))))
        );
    }
}
