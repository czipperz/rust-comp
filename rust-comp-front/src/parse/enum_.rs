use super::combinator::*;
use super::error::Error;
use super::parser::Parser;
use super::tree::*;
use super::type_::expect_type;
use crate::token::TokenKind;

pub fn expect_enum(parser: &mut Parser) -> Result<Enum, Error> {
    let enum_span = parser.expect_token(TokenKind::Enum)?;
    let name = parser.expect_token(TokenKind::Label)?;
    let open_curly_span = parser.expect_token(TokenKind::OpenCurly)?;
    let (variants, comma_spans) = many_comma_separated(parser, expect_variant)?;
    let close_curly_span = parser.expect_token(TokenKind::CloseCurly)?;
    Ok(Enum {
        enum_span,
        name,
        open_curly_span,
        variants,
        comma_spans,
        close_curly_span,
    })
}

fn expect_variant(parser: &mut Parser) -> Result<Variant, Error> {
    let name = parser.expect_token(TokenKind::Label)?;
    let data = if parser.peek_kind() == Some(TokenKind::OpenParen) {
        match expect_type(parser)? {
            Type::Tuple(tuple) => VariantData::Tuple(tuple),
            Type::Paren(paren) => VariantData::Tuple(TupleType {
                open_paren_span: paren.open_paren_span,
                types: vec![*paren.type_],
                comma_spans: Vec::new(),
                close_paren_span: paren.close_paren_span,
            }),
            _ => unreachable!(),
        }
    } else {
        VariantData::None
    };
    Ok(Variant { name, data })
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;

    #[test]
    fn test_expect_enum_0() {
        let (index, len, enum_) = parse(expect_enum, "enum X {}");
        assert_eq!(index, len);
        assert_eq!(
            enum_,
            Ok(Enum {
                enum_span: Span {
                    file: 0,
                    start: 0,
                    end: 4
                },
                name: Span {
                    file: 0,
                    start: 5,
                    end: 6
                },
                open_curly_span: Span {
                    file: 0,
                    start: 7,
                    end: 8
                },
                variants: vec![],
                comma_spans: vec![],
                close_curly_span: Span {
                    file: 0,
                    start: 8,
                    end: 9
                },
            })
        );
    }

    #[test]
    fn test_expect_enum_2() {
        let (index, len, enum_) = parse(expect_enum, "enum X {Y, Z,}");
        assert_eq!(index, len);
        let enum_ = enum_.unwrap();
        assert_eq!(
            &enum_.variants,
            &[
                Variant {
                    name: Span {
                        file: 0,
                        start: 8,
                        end: 9
                    },
                    data: VariantData::None,
                },
                Variant {
                    name: Span {
                        file: 0,
                        start: 11,
                        end: 12
                    },
                    data: VariantData::None,
                }
            ]
        );
        assert_eq!(
            &enum_.comma_spans,
            &[
                Span {
                    file: 0,
                    start: 9,
                    end: 10
                },
                Span {
                    file: 0,
                    start: 12,
                    end: 13
                },
            ]
        );
    }

    #[test]
    fn test_expect_enum_with_tuple_data() {
        let (index, len, variant) = parse(expect_variant, "Ref(i32)");
        assert_eq!(index, len);
        assert_eq!(
            variant,
            Ok(Variant {
                name: Span {
                    file: 0,
                    start: 0,
                    end: 3
                },
                data: VariantData::Tuple(TupleType {
                    open_paren_span: Span {
                        file: 0,
                        start: 3,
                        end: 4
                    },
                    types: vec![Type::Named(NamedType {
                        name: Span {
                            file: 0,
                            start: 4,
                            end: 7
                        }
                    })],
                    comma_spans: vec![],
                    close_paren_span: Span {
                        file: 0,
                        start: 7,
                        end: 8
                    },
                })
            })
        );
    }
}
