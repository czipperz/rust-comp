use super::block::expect_block;
use super::combinator::*;
use super::error::Error;
use super::parser::Parser;
use super::type_::expect_type;
use crate::ast::*;
use crate::token::TokenKind;

pub fn expect_fn<'a>(parser: &mut Parser<'a, '_>) -> Result<Function<'a>, Error> {
    parser.expect_token(TokenKind::Fn)?;
    let name = parser.expect_label()?;
    let parameters = expect_parameters(parser)?;
    let return_type = expect_return_type(parser)?;
    let body = expect_block(parser)?;
    Ok(Function {
        name,
        parameters,
        return_type,
        body,
    })
}

fn expect_parameters<'a>(parser: &mut Parser<'a, '_>) -> Result<Vec<Parameter<'a>>, Error> {
    parser.expect_token(TokenKind::OpenParen)?;
    let parameters = many_separator(parser, expect_parameter, |parser| {
        parser.expect_token(TokenKind::Comma)
    })?;
    parser.expect_token(TokenKind::CloseParen)?;
    Ok(parameters)
}

fn expect_parameter<'a>(parser: &mut Parser<'a, '_>) -> Result<Parameter<'a>, Error> {
    let name = parser.expect_label()?;
    parser.expect_token(TokenKind::Colon)?;
    let type_ = expect_type(parser)?;
    Ok(Parameter { name, type_ })
}

fn expect_return_type<'a>(parser: &mut Parser<'a, '_>) -> Result<Type<'a>, Error> {
    if parser.expect_token(TokenKind::ThinArrow).is_ok() {
        expect_type(parser)
    } else {
        Ok(Type::Tuple(vec![]))
    }
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
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
        let (index, len, f) = parse(expect_fn, "fn f () {}");
        let f = f.unwrap();
        assert_eq!(index, len);
        assert_eq!(f.name, "f");
        assert_eq!(f.parameters.len(), 0);
        assert_eq!(f.body.statements.len(), 0);
    }

    #[test]
    fn test_expect_parameters_1_parameter() {
        let (index, len, parameters) = parse(expect_parameters, "(x: i32)");
        let parameters = parameters.unwrap();
        assert_eq!(index, len);
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
        let (index, len, parameters) = parse(expect_parameters, "(x: i32, y: i32)");
        let parameters = parameters.unwrap();
        assert_eq!(index, len);
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
        let (index, len, parameter) = parse(expect_parameter, "x: i32");
        let parameter = parameter.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            parameter,
            Parameter {
                name: "x",
                type_: Type::Named(NamedType { name: "i32" })
            }
        );
    }

    #[test]
    fn test_expect_return_type_nothing() {
        let (index, _, return_type) = parse(expect_return_type, "{");
        let return_type = return_type.unwrap();
        assert_eq!(index, 0);
        assert_eq!(return_type, Type::Tuple(vec![]));
    }

    #[test]
    fn test_expect_return_type_something() {
        let (index, len, return_type) = parse(expect_return_type, "-> &x {");
        let return_type = return_type.unwrap();
        assert_eq!(index, len - 1);
        assert_eq!(
            return_type,
            Type::Ref(Box::new(Type::Named(NamedType { name: "x" })))
        );
    }
}