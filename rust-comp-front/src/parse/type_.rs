use super::combinator::*;
use super::parser::Parser;
use super::Error;
use crate::ast::*;
use crate::token::TokenKind;

pub fn expect_type<'a>(parser: &mut Parser<'a, '_>) -> Result<Type<'a>, Error> {
    if parser.expect_token(TokenKind::And).is_ok() {
        if parser.expect_token(TokenKind::Mut).is_ok() {
            Ok(Type::Ref(Box::new(Type::RefMut(Box::new(expect_type(
                parser,
            )?)))))
        } else {
            Ok(Type::Ref(Box::new(Type::Ref(Box::new(expect_type(
                parser,
            )?)))))
        }
    } else if parser.expect_token(TokenKind::Ampersand).is_ok() {
        if parser.expect_token(TokenKind::Mut).is_ok() {
            Ok(Type::RefMut(Box::new(expect_type(parser)?)))
        } else {
            Ok(Type::Ref(Box::new(expect_type(parser)?)))
        }
    } else if parser.expect_token(TokenKind::Star).is_ok() {
        if parser.expect_token(TokenKind::Mut).is_ok() {
            Ok(Type::PtrMut(Box::new(expect_type(parser)?)))
        } else if parser.expect_token(TokenKind::Const).is_ok() {
            Ok(Type::PtrConst(Box::new(expect_type(parser)?)))
        } else {
            Err(Error::Expected(
                "`const` or `mut` after pointer",
                parser.span(),
            ))
        }
    } else if parser.expect_token(TokenKind::OpenParen).is_ok() {
        let types = many_separator(parser, expect_type, |p| p.expect_token(TokenKind::Comma))?;
        parser.expect_token(TokenKind::CloseParen)?;
        Ok(Type::Tuple(types))
    } else if parser.expect_token(TokenKind::Underscore).is_ok() {
        Ok(Type::Hole)
    } else {
        Ok(Type::Named(NamedType {
            name: parser.expect_label()?,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;

    #[test]
    fn test_expect_type_named_type() {
        let (index, len, type_) = parse(expect_type, "abc");
        let type_ = type_.unwrap();
        assert_eq!(index, len);
        assert_eq!(type_, Type::Named(NamedType { name: "abc" }));
    }

    #[test]
    fn test_expect_type_ref() {
        let (index, len, type_) = parse(expect_type, "&abc");
        let type_ = type_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Type::Ref(Box::new(Type::Named(NamedType { name: "abc" })))
        );
    }

    #[test]
    fn test_expect_type_ref_ref() {
        let (index, len, type_) = parse(expect_type, "&&abc");
        let type_ = type_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Type::Ref(Box::new(Type::Ref(Box::new(Type::Named(NamedType {
                name: "abc"
            })))))
        );
    }

    #[test]
    fn test_expect_type_ref_ref_mut() {
        let (index, len, type_) = parse(expect_type, "&&mut abc");
        let type_ = type_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Type::Ref(Box::new(Type::RefMut(Box::new(Type::Named(NamedType {
                name: "abc"
            })))))
        );
    }

    #[test]
    fn test_expect_type_ref_mut() {
        let (index, len, type_) = parse(expect_type, "&mut abc");
        let type_ = type_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Type::RefMut(Box::new(Type::Named(NamedType { name: "abc" })))
        );
    }

    #[test]
    fn test_expect_type_ptr_mut() {
        let (index, len, type_) = parse(expect_type, "*mut abc");
        let type_ = type_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Type::PtrMut(Box::new(Type::Named(NamedType { name: "abc" })))
        );
    }

    #[test]
    fn test_expect_type_ptr_const() {
        let (index, len, type_) = parse(expect_type, "*const abc");
        let type_ = type_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Type::PtrConst(Box::new(Type::Named(NamedType { name: "abc" })))
        );
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
        let type_ = type_.unwrap();
        assert_eq!(index, len);
        assert_eq!(type_, Type::Tuple(vec![]));
    }

    #[test]
    fn test_expect_type_tuple_1() {
        let (index, len, type_) = parse(expect_type, "(abc)");
        let type_ = type_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Type::Tuple(vec![Type::Named(NamedType { name: "abc" })])
        );
    }

    #[test]
    fn test_expect_type_tuple_3() {
        let (index, len, type_) = parse(expect_type, "(*const abc, def, &ghi)");
        let type_ = type_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Type::Tuple(vec![
                Type::PtrConst(Box::new(Type::Named(NamedType { name: "abc" }))),
                Type::Named(NamedType { name: "def" }),
                Type::Ref(Box::new(Type::Named(NamedType { name: "ghi" })))
            ])
        );
    }

    #[test]
    fn test_expect_type_hole() {
        let (index, len, type_) = parse(expect_type, "_");
        let type_ = type_.unwrap();
        assert_eq!(index, len);
        assert_eq!(type_, Type::Hole);
    }
}
