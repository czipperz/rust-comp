use super::block::expect_block;
use super::combinator::*;
use super::error::Error;
use super::parser::Parser;
use super::tree::*;
use super::type_::expect_type;
use crate::token::TokenKind;

pub fn expect_fn<'a>(parser: &mut Parser) -> Result<Function, Error> {
    let fn_span = parser.expect_token(TokenKind::Fn)?;
    let name = parser.expect_token(TokenKind::Label)?;
    let open_paren_span = parser.expect_token(TokenKind::OpenParen)?;
    let (parameters, comma_spans) = many_comma_separated(parser, expect_parameter)?;
    let close_paren_span = parser.expect_token(TokenKind::CloseParen)?;
    let return_type = expect_return_type(parser)?;
    let body = expect_block(parser)?;
    Ok(Function {
        fn_span,
        name,
        open_paren_span,
        parameters,
        comma_spans,
        close_paren_span,
        return_type,
        body,
    })
}

fn expect_parameter<'a>(parser: &mut Parser) -> Result<Parameter, Error> {
    let name = parser.expect_token(TokenKind::Label)?;
    let colon_span = parser.expect_token(TokenKind::Colon)?;
    let type_ = expect_type(parser)?;
    Ok(Parameter {
        name,
        colon_span,
        type_,
    })
}

fn expect_return_type<'a>(parser: &mut Parser) -> Result<Option<ReturnType>, Error> {
    if let Ok(thin_arrow_span) = parser.expect_token(TokenKind::ThinArrow) {
        Ok(Some(ReturnType {
            thin_arrow_span,
            type_: expect_type(parser)?,
        }))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::lex::read_tokens;
    use crate::pos::Span;
    use assert_matches::assert_matches;

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
        assert_eq!(index, len);
        assert_eq!(
            f,
            Ok(Function {
                fn_span: Span {
                    file: 0,
                    start: 0,
                    end: 2
                },
                name: Span {
                    file: 0,
                    start: 3,
                    end: 4
                },
                open_paren_span: Span {
                    file: 0,
                    start: 5,
                    end: 6
                },
                parameters: vec![],
                comma_spans: vec![],
                close_paren_span: Span {
                    file: 0,
                    start: 6,
                    end: 7
                },
                return_type: None,
                body: Block {
                    open_curly_span: Span {
                        file: 0,
                        start: 8,
                        end: 9
                    },
                    statements: vec![],
                    expression: None,
                    close_curly_span: Span {
                        file: 0,
                        start: 9,
                        end: 10
                    },
                }
            })
        );
    }

    #[test]
    fn test_expect_parameters_1_parameter() {
        let (index, len, function) = parse(expect_fn, "fn f(x: i32) {}");
        assert_eq!(index, len);
        let function = function.unwrap();
        assert_eq!(function.parameters.len(), 1);
        assert_eq!(function.comma_spans, []);
    }

    #[test]
    fn test_expect_parameters_2_parameters() {
        let (index, len, function) = parse(expect_fn, "fn f(x: i32, y: i32) {}");
        assert_eq!(index, len);
        let function = function.unwrap();
        assert_eq!(function.parameters.len(), 2);
        assert_eq!(
            function.comma_spans,
            [Span {
                file: 0,
                start: 11,
                end: 12
            }]
        );
    }

    #[test]
    fn test_expect_parameter() {
        let (index, len, parameter) = parse(expect_parameter, "x: i32");
        assert_eq!(index, len);
        assert_eq!(
            parameter,
            Ok(Parameter {
                name: Span {
                    file: 0,
                    start: 0,
                    end: 1
                },
                colon_span: Span {
                    file: 0,
                    start: 1,
                    end: 2
                },
                type_: Type::Named(NamedType {
                    name: Span {
                        file: 0,
                        start: 3,
                        end: 6
                    }
                })
            })
        );
    }

    #[test]
    fn test_expect_return_type_nothing() {
        let (index, _, return_type) = parse(expect_return_type, "{");
        assert_eq!(index, 0);
        assert_eq!(return_type, Ok(None));
    }

    #[test]
    fn test_expect_return_type_something() {
        let (index, len, return_type) = parse(expect_return_type, "-> &x {");
        assert_eq!(index, len - 1);
        assert_matches!(
            return_type,
            Ok(Some(ReturnType {
                type_: Type::Ref(_),
                ..
            }))
        );
    }
}
