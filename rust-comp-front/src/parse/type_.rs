use super::combinator::*;
use super::parser::Parser;
use super::tree::*;
use super::Error;
use crate::token::TokenKind;

pub fn expect_type<'a>(parser: &mut Parser) -> Result<Type, Error> {
    match parser.peek_kind() {
        Some(TokenKind::And) => expect_double_ref_type(parser),
        Some(TokenKind::Ampersand) => expect_ref_type(parser),
        Some(TokenKind::Star) => expect_pointer_type(parser),
        Some(TokenKind::OpenParen) => expect_paren_type(parser),
        Some(TokenKind::Underscore) => expect_hole_type(parser),
        Some(TokenKind::Label) => expect_named_type(parser),
        _ => Err(Error::Expected("type", parser.span())),
    }
}

fn expect_double_ref_type(parser: &mut Parser) -> Result<Type, Error> {
    use crate::pos::Span;
    let and_span = parser.expect_token(TokenKind::And)?;
    debug_assert_eq!(and_span.start + 2, and_span.end);
    let first_ref_span = Span {
        file: and_span.file,
        start: and_span.start,
        end: and_span.start + 1,
    };
    let second_ref_span = Span {
        file: and_span.file,
        start: and_span.start + 1,
        end: and_span.start + 2,
    };

    let inner_type = if let Ok(mut_span) = parser.expect_token(TokenKind::Mut) {
        Type::RefMut(RefMutType {
            ref_span: second_ref_span,
            mut_span,
            type_: Box::new(expect_type(parser)?),
        })
    } else {
        Type::Ref(RefType {
            ref_span: second_ref_span,
            type_: Box::new(expect_type(parser)?),
        })
    };

    Ok(Type::Ref(RefType {
        ref_span: first_ref_span,
        type_: Box::new(inner_type),
    }))
}

fn expect_ref_type(parser: &mut Parser) -> Result<Type, Error> {
    let ref_span = parser.expect_token(TokenKind::Ampersand)?;
    if let Ok(mut_span) = parser.expect_token(TokenKind::Mut) {
        Ok(Type::RefMut(RefMutType {
            ref_span,
            mut_span,
            type_: Box::new(expect_type(parser)?),
        }))
    } else {
        Ok(Type::Ref(RefType {
            ref_span,
            type_: Box::new(expect_type(parser)?),
        }))
    }
}

fn expect_pointer_type(parser: &mut Parser) -> Result<Type, Error> {
    let ptr_span = parser.expect_token(TokenKind::Star)?;
    if let Ok(mut_span) = parser.expect_token(TokenKind::Mut) {
        Ok(Type::PtrMut(PtrMutType {
            ptr_span,
            mut_span,
            type_: Box::new(expect_type(parser)?),
        }))
    } else if let Ok(const_span) = parser.expect_token(TokenKind::Const) {
        Ok(Type::PtrConst(PtrConstType {
            ptr_span,
            const_span,
            type_: Box::new(expect_type(parser)?),
        }))
    } else {
        Err(Error::Expected(
            "`const` or `mut` after pointer",
            parser.span(),
        ))
    }
}

fn expect_paren_type(parser: &mut Parser) -> Result<Type, Error> {
    let open_paren_span = parser.expect_token(TokenKind::OpenParen)?;
    let (types, comma_spans) = many_comma_separated(parser, expect_type)?;
    let close_paren_span = parser.expect_token(TokenKind::CloseParen)?;
    if types.len() == 1 && comma_spans.len() == 0 {
        Ok(Type::Paren(ParenType {
            open_paren_span,
            type_: Box::new(types.into_iter().next().unwrap()),
            close_paren_span,
        }))
    } else {
        Ok(Type::Tuple(TupleType {
            open_paren_span,
            types,
            comma_spans,
            close_paren_span,
        }))
    }
}

fn expect_hole_type(parser: &mut Parser) -> Result<Type, Error> {
    let underscore_span = parser.expect_token(TokenKind::Underscore)?;
    Ok(Type::Hole(HoleType { underscore_span }))
}

fn expect_named_type(parser: &mut Parser) -> Result<Type, Error> {
    Ok(Type::Named(NamedType {
        name: parser.expect_token(TokenKind::Label)?,
    }))
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;
    use assert_matches::assert_matches;

    #[test]
    fn test_expect_type_named_type() {
        let (index, len, type_) = parse(expect_type, "abc");
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Ok(Type::Named(NamedType {
                name: Span {
                    file: 0,
                    start: 0,
                    end: 3
                }
            }))
        );
    }

    #[test]
    fn test_expect_type_ref() {
        let (index, len, type_) = parse(expect_type, "&abc");
        assert_eq!(index, len);
        assert_matches!(type_, Ok(Type::Ref(_)));
    }

    #[test]
    fn test_expect_type_ref_ref() {
        let (index, len, type_) = parse(expect_type, "&&abc");
        assert_eq!(index, len);
        assert_matches!(type_, Ok(Type::Ref(RefType { type_, .. })) => {
            assert_matches!(*type_, Type::Ref(_));
        });
    }

    #[test]
    fn test_expect_type_ref_ref_mut() {
        let (index, len, type_) = parse(expect_type, "&&mut abc");
        assert_eq!(index, len);
        assert_matches!(type_, Ok(Type::Ref(RefType { type_, .. })) => {
            assert_matches!(*type_, Type::RefMut(_));
        });
    }

    #[test]
    fn test_expect_type_ref_mut() {
        let (index, len, type_) = parse(expect_type, "&mut abc");
        assert_eq!(index, len);
        assert_matches!(type_, Ok(Type::RefMut(_)));
    }

    #[test]
    fn test_expect_type_ptr_mut() {
        let (index, len, type_) = parse(expect_type, "*mut abc");
        assert_eq!(index, len);
        assert_matches!(type_, Ok(Type::PtrMut(_)));
    }

    #[test]
    fn test_expect_type_ptr_const() {
        let (index, len, type_) = parse(expect_type, "*const abc");
        assert_eq!(index, len);
        assert_matches!(type_, Ok(Type::PtrConst(_)));
    }

    #[test]
    fn test_expect_type_ptr_no_qualifier() {
        let (index, _, type_) = parse(expect_type, "*abc");
        let error = type_.unwrap_err();
        assert_eq!(index, 1);
        assert_eq!(
            error,
            Error::Expected(
                "`const` or `mut` after pointer",
                Span {
                    file: 0,
                    start: 1,
                    end: 4,
                }
            )
        );
    }

    #[test]
    fn test_expect_type_tuple_0() {
        let (index, len, type_) = parse(expect_type, "()");
        assert_eq!(index, len);
        assert_matches!(type_, Ok(Type::Tuple(tuple)) => {
            assert_eq!(tuple.types.len(), 0);
            assert_eq!(tuple.comma_spans.len(), 0);
        });
    }

    #[test]
    fn test_expect_type_tuple_1_is_paren() {
        let (index, len, type_) = parse(expect_type, "(abc)");
        assert_eq!(index, len);
        assert_matches!(type_, Ok(Type::Paren(_)));
    }

    #[test]
    fn test_expect_type_tuple_1_trailing_comma_is_tuple() {
        let (index, len, type_) = parse(expect_type, "(abc,)");
        assert_eq!(index, len);
        assert_matches!(type_, Ok(Type::Tuple(tuple)) => {
            assert_eq!(tuple.types.len(), 1);
            assert_eq!(tuple.comma_spans.len(), 1);
        });
    }

    #[test]
    fn test_expect_type_tuple_3() {
        let (index, len, type_) = parse(expect_type, "(*const abc, def, &ghi)");
        assert_eq!(index, len);
        assert_matches!(type_, Ok(Type::Tuple(tuple)) => {
            assert_eq!(tuple.types.len(), 3);
            assert_eq!(tuple.comma_spans.len(), 2);
        });
    }

    #[test]
    fn test_expect_type_hole() {
        let (index, len, type_) = parse(expect_type, "_");
        assert_eq!(index, len);
        assert_matches!(type_, Ok(Type::Hole(_)));
    }
}
