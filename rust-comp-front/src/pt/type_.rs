use super::combinator::many_comma_separated;
use super::{Error, Parse, Parser};
use crate::ast::*;
use crate::token::TokenKind;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref TYPE_KIND: HashMap<TokenKind, for<'a> fn(&mut Parser<'a, '_>) -> Result<Type<'a>, Error>> = {
        let mut table =
            HashMap::<TokenKind, for<'a> fn(&mut Parser<'a, '_>) -> Result<Type<'a>, Error>>::new();
        table.insert(TokenKind::And, parse_and_type);
        table.insert(TokenKind::Ampersand, parse_ampersand_type);
        table.insert(TokenKind::Star, parse_star_type);
        table.insert(TokenKind::OpenParen, parse_paren_type);
        table.insert(TokenKind::Underscore, parse_underscore_type);
        table.insert(TokenKind::Label, parse_named_type);
        table
    };
}

impl<'a> Parse<'a> for Type<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Type<'a>, Error> {
        parser.dispatch(&TYPE_KIND, "type")
    }
}

fn parse_and_type<'a>(parser: &mut Parser<'a, '_>) -> Result<Type<'a>, Error> {
    parser.expect_token(TokenKind::And)?;
    if parser.expect_token(TokenKind::Mut).is_ok() {
        Ok(Type::Ref(Box::new(Type::RefMut(Box::new(Type::parse(
            parser,
        )?)))))
    } else {
        Ok(Type::Ref(Box::new(Type::Ref(Box::new(Type::parse(
            parser,
        )?)))))
    }
}

fn parse_ampersand_type<'a>(parser: &mut Parser<'a, '_>) -> Result<Type<'a>, Error> {
    parser.expect_token(TokenKind::Ampersand)?;
    if parser.expect_token(TokenKind::Mut).is_ok() {
        Ok(Type::RefMut(Box::new(Type::parse(parser)?)))
    } else {
        Ok(Type::Ref(Box::new(Type::parse(parser)?)))
    }
}

fn parse_star_type<'a>(parser: &mut Parser<'a, '_>) -> Result<Type<'a>, Error> {
    parser.expect_token(TokenKind::Star)?;
    if parser.expect_token(TokenKind::Mut).is_ok() {
        Ok(Type::PtrMut(Box::new(Type::parse(parser)?)))
    } else if parser.expect_token(TokenKind::Const).is_ok() {
        Ok(Type::PtrConst(Box::new(Type::parse(parser)?)))
    } else {
        Err(Error::Expected(
            "`const` or `mut` after pointer",
            parser.span(),
        ))
    }
}

fn parse_paren_type<'a>(parser: &mut Parser<'a, '_>) -> Result<Type<'a>, Error> {
    parser.expect_token(TokenKind::OpenParen)?;
    let types = many_comma_separated(parser)?;
    parser.expect_token(TokenKind::CloseParen)?;
    Ok(Type::Tuple(types))
}

fn parse_underscore_type<'a>(parser: &mut Parser<'a, '_>) -> Result<Type<'a>, Error> {
    parser.expect_token(TokenKind::Underscore)?;
    Ok(Type::Hole)
}

fn parse_named_type<'a>(parser: &mut Parser<'a, '_>) -> Result<Type<'a>, Error> {
    Ok(Type::Named(NamedType {
        name: parser.expect_label()?,
    }))
}

#[cfg(test)]
mod tests {
    use super::super::parse;
    use super::*;
    use crate::pos::Span;

    #[test]
    fn test_expect_type_named_type() {
        let (index, len, type_) = parse("abc");
        assert_eq!(index, len);
        assert_eq!(type_, Ok(Type::Named(NamedType { name: "abc" })));
    }

    #[test]
    fn test_expect_type_ref() {
        let (index, len, type_) = parse("&abc");
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Ok(Type::Ref(Box::new(Type::Named(NamedType { name: "abc" }))))
        );
    }

    #[test]
    fn test_expect_type_ref_ref() {
        let (index, len, type_) = parse("&&abc");
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Ok(Type::Ref(Box::new(Type::Ref(Box::new(Type::Named(
                NamedType { name: "abc" }
            ))))))
        );
    }

    #[test]
    fn test_expect_type_ref_ref_mut() {
        let (index, len, type_) = parse("&&mut abc");
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Ok(Type::Ref(Box::new(Type::RefMut(Box::new(Type::Named(
                NamedType { name: "abc" }
            ))))))
        );
    }

    #[test]
    fn test_expect_type_ref_mut() {
        let (index, len, type_) = parse("&mut abc");
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Ok(Type::RefMut(Box::new(Type::Named(NamedType {
                name: "abc"
            }))))
        );
    }

    #[test]
    fn test_expect_type_ptr_mut() {
        let (index, len, type_) = parse("*mut abc");
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Ok(Type::PtrMut(Box::new(Type::Named(NamedType {
                name: "abc"
            }))))
        );
    }

    #[test]
    fn test_expect_type_ptr_const() {
        let (index, len, type_) = parse("*const abc");
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Ok(Type::PtrConst(Box::new(Type::Named(NamedType {
                name: "abc"
            }))))
        );
    }

    #[test]
    fn test_expect_type_ptr_no_qualifier() {
        let (index, _, type_) = parse::<Type>("*abc");
        assert_eq!(index, 1);
        assert_eq!(
            type_,
            Err(Error::Expected(
                "`const` or `mut` after pointer",
                Span {
                    file: 0,
                    start: 1,
                    end: 4,
                }
            ))
        );
    }

    #[test]
    fn test_expect_type_tuple_0() {
        let (index, len, type_) = parse("()");
        assert_eq!(index, len);
        assert_eq!(type_, Ok(Type::Tuple(vec![])));
    }

    #[test]
    fn test_expect_type_tuple_1() {
        let (index, len, type_) = parse("(abc)");
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Ok(Type::Tuple(vec![Type::Named(NamedType { name: "abc" })]))
        );
    }

    #[test]
    fn test_expect_type_tuple_3() {
        let (index, len, type_) = parse("(*const abc, def, &ghi)");
        assert_eq!(index, len);
        assert_eq!(
            type_,
            Ok(Type::Tuple(vec![
                Type::PtrConst(Box::new(Type::Named(NamedType { name: "abc" }))),
                Type::Named(NamedType { name: "def" }),
                Type::Ref(Box::new(Type::Named(NamedType { name: "ghi" })))
            ]))
        );
    }

    #[test]
    fn test_expect_type_hole() {
        let (index, len, type_) = parse("_");
        assert_eq!(index, len);
        assert_eq!(type_, Ok(Type::Hole));
    }
}
