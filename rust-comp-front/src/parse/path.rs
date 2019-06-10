use super::parser::Parser;
use super::Error;
use crate::ast::*;
use crate::token::*;

pub fn expect_path<'a>(parser: &mut Parser<'a, '_>) -> Result<Path<'a>, Error> {
    let mut path = Vec::new();
    path.push(parser.expect_label()?);
    while parser.expect_token(TokenKind::ColonColon).is_ok() {
        path.push(parser.expect_label()?);
    }
    Ok(Path { path })
}
