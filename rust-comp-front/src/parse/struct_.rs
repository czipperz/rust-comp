use super::combinator::*;
use super::error::Error;
use super::parser::Parser;
use super::tree::*;
use super::type_::expect_type;
use super::visibility::expect_visibility;
use crate::token::TokenKind;

pub fn expect_struct<'a>(parser: &mut Parser) -> Result<Struct, Error> {
    let struct_span = parser.expect_token(TokenKind::Struct)?;
    let name = parser.expect_token(TokenKind::Label)?;
    let open_curly_span = parser.expect_token(TokenKind::OpenCurly)?;
    let (fields, comma_spans) = many_comma_separated(parser, expect_field)?;
    let close_curly_span = parser.expect_token(TokenKind::CloseCurly)?;
    Ok(Struct {
        struct_span,
        name,
        open_curly_span,
        fields,
        comma_spans,
        close_curly_span,
    })
}

fn expect_field<'a>(parser: &mut Parser) -> Result<Field, Error> {
    let visibility = expect_visibility(parser)?;
    let name = parser.expect_token(TokenKind::Label)?;
    let colon_span = parser.expect_token(TokenKind::Colon)?;
    let type_ = expect_type(parser)?;
    Ok(Field {
        visibility,
        name,
        colon_span,
        type_,
    })
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;
    use assert_matches::assert_matches;

    #[test]
    fn test_expect_struct_0() {
        let (index, len, struct_) = parse(expect_struct, "struct X {}");
        let struct_ = struct_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            struct_,
            Struct {
                struct_span: Span {
                    file: 0,
                    start: 0,
                    end: 6
                },
                name: Span {
                    file: 0,
                    start: 7,
                    end: 8
                },
                open_curly_span: Span {
                    file: 0,
                    start: 9,
                    end: 10
                },
                fields: vec![],
                comma_spans: vec![],
                close_curly_span: Span {
                    file: 0,
                    start: 10,
                    end: 11
                },
            }
        );
    }

    #[test]
    fn test_expect_struct_1() {
        let (index, len, struct_) = parse(expect_struct, "struct X {pub y: Y}");
        assert_eq!(index, len);
        assert_matches!(struct_, Ok(Struct {
            fields,
            comma_spans,
            ..
        }) =>
        {
            assert_eq!(fields.len(), 1);
            assert_eq!(
                fields[0].visibility,
                Visibility::Public(Span {
                    file: 0,
                    start: 10,
                    end: 13
                })
            );
            assert_eq!(comma_spans.len(), 0);
        });
    }

    #[test]
    fn test_expect_struct_1_trailing_comma() {
        let (index, len, struct_) = parse(expect_struct, "struct X {y: Y,}");
        assert_eq!(index, len);
        assert_matches!(struct_, Ok(Struct {
            fields,
            comma_spans,
            ..
        }) =>
        {
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].visibility, Visibility::Private);
            assert_eq!(comma_spans.len(), 1);
        });
    }

    #[test]
    fn test_expect_struct_2() {
        let (index, len, struct_) = parse(expect_struct, "struct X {y: Y, z: Z}");
        assert_eq!(index, len);
        assert_matches!(struct_, Ok(Struct {
            fields,
            comma_spans,
            ..
        }) =>
        {
            assert_eq!(fields.len(), 2);
            assert_eq!(comma_spans.len(), 1);
        });
    }
}
