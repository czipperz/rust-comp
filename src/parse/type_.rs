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
}
