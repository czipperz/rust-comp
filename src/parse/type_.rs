use super::combinator::*;
use super::parser::Parser;
use super::Error;
use crate::ast::*;
use crate::token::TokenKind;

pub fn expect_type<'a>(parser: &mut Parser<'a>) -> Result<Type<'a>, Error> {
    if parser.expect_token(TokenKind::Ampersand).is_ok() {
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
        let types = many_separator(parser, expect_type, |p| {
            p.expect_token(TokenKind::Comma)
        })?;
        parser.expect_token(TokenKind::CloseParen)?;
        Ok(Type::Tuple(types))
    } else {
        Ok(Type::Named(NamedType {
            name: parser.expect_label()?,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::read_tokens;
    use crate::pos::Span;

    #[test]
    fn test_expect_type_named_type() {
        let contents = "abc";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let type_ = expect_type(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(type_, Type::Named(NamedType { name: "abc" }));
    }

    #[test]
    fn test_expect_type_ref() {
        let contents = "&abc";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let type_ = expect_type(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            type_,
            Type::Ref(Box::new(Type::Named(NamedType { name: "abc" })))
        );
    }

    #[test]
    fn test_expect_type_ref_mut() {
        let contents = "&mut abc";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let type_ = expect_type(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            type_,
            Type::RefMut(Box::new(Type::Named(NamedType { name: "abc" })))
        );
    }

    #[test]
    fn test_expect_type_ptr_mut() {
        let contents = "*mut abc";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let type_ = expect_type(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            type_,
            Type::PtrMut(Box::new(Type::Named(NamedType { name: "abc" })))
        );
    }

    #[test]
    fn test_expect_type_ptr_const() {
        let contents = "*const abc";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let type_ = expect_type(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            type_,
            Type::PtrConst(Box::new(Type::Named(NamedType { name: "abc" })))
        );
    }

    #[test]
    fn test_expect_type_ptr_no_qualifier() {
        let contents = "*abc";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let err = expect_type(&mut parser).unwrap_err();
        assert_eq!(parser.index, 1);
        assert_eq!(
            err,
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
        let contents = "()";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let type_ = expect_type(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(type_, Type::Tuple(vec![]));
    }

    #[test]
    fn test_expect_type_tuple_1() {
        let contents = "(abc)";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let type_ = expect_type(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            type_,
            Type::Tuple(vec![Type::Named(NamedType { name: "abc" })])
        );
    }

    #[test]
    fn test_expect_type_tuple_3() {
        let contents = "(*const abc, def, &ghi)";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let type_ = expect_type(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            type_,
            Type::Tuple(vec![
                Type::PtrConst(Box::new(Type::Named(NamedType { name: "abc" }))),
                Type::Named(NamedType { name: "def" }),
                Type::Ref(Box::new(Type::Named(NamedType { name: "ghi" })))
            ])
        );
    }
}
