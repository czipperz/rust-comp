use super::parser::Parser;
use super::Error;
use crate::ast::*;

pub fn expect_type<'a>(parser: &mut Parser<'a>) -> Result<Type<'a>, Error> {
    parser
        .expect_label()
        .map(|name| Type::Named(NamedType { name }))
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
}
