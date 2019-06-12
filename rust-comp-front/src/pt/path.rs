use super::{Error, Parse, Parser};
use crate::ast::*;
use crate::token::TokenKind;

impl<'a> Parse<'a> for Path<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Path<'a>, Error> {
        let mut path = Vec::new();
        path.push(parser.expect_label()?);
        while parser.expect_token(TokenKind::ColonColon).is_ok() {
            path.push(parser.expect_label()?);
        }
        Ok(Path { path })
    }
}
