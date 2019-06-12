use super::{Error, Parse, Parser};
use crate::token::TokenKind;

pub fn many<'a, T>(parser: &mut Parser<'a, '_>) -> Result<Vec<T>, Error>
where
    T: Parse<'a>,
{
    let mut xs = Vec::new();
    loop {
        let old_index = parser.index;
        match Parse::parse(parser) {
            Ok(x) => xs.push(x),
            Err(_) if old_index == parser.index => return Ok(xs),
            Err(e) => return Err(e),
        }
    }
}

pub fn many_comma_separated<'a, T: Parse<'a>>(
    parser: &mut Parser<'a, '_>,
) -> Result<Vec<T>, Error> {
    let mut xs = Vec::new();
    loop {
        let old_index = parser.index;
        match Parse::parse(parser) {
            Ok(x) => xs.push(x),
            Err(_) if old_index == parser.index => return Ok(xs),
            Err(e) => return Err(e),
        }

        let old_index = parser.index;
        match parser.expect_token(TokenKind::Comma) {
            Ok(()) => (),
            Err(_) if old_index == parser.index => return Ok(xs),
            Err(e) => return Err(e),
        }
    }
}

pub fn maybe<'a, T: Parse<'a>>(parser: &mut Parser<'a, '_>) -> Result<Option<T>, Error> {
    let old_index = parser.index;
    match Parse::parse(parser) {
        Ok(x) => Ok(Some(x)),
        Err(_) if old_index == parser.index => Ok(None),
        Err(e) => Err(e),
    }
}
