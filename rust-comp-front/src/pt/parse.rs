use super::combinator::many;
use super::{Error, Parser};
use crate::ast::TopLevel;
use crate::lex::read_tokens;

pub trait Parse<'a>
where
    Self: 'a + Sized,
{
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Self, Error>;
}

pub fn parse<'a, T>(file_contents: &'a str) -> (usize, usize, Result<T, Error>)
where
    T: Parse<'a>,
{
    parse_fn(Parse::parse, file_contents)
}

pub fn parse_fn<'a, T, F>(f: F, file_contents: &'a str) -> (usize, usize, Result<T, Error>)
where
    F: FnOnce(&mut Parser<'a, '_>) -> Result<T, Error>,
{
    let (tokens, eofpos) = read_tokens(0, file_contents).unwrap();
    let mut parser = Parser::new(file_contents, &tokens, eofpos);
    let res = f(&mut parser);
    (parser.index, tokens.len(), res)
}

pub fn parse_top_levels<'a>(file_contents: &'a str) -> Result<Vec<TopLevel>, Error> {
    let (tokens, eofpos) = read_tokens(0, file_contents)?;
    let mut parser = Parser::new(file_contents, &tokens, eofpos);
    let top_levels = many(&mut parser)?;

    if parser.index < tokens.len() {
        Err(Error::Expected("top level item", parser.span()))
    } else {
        Ok(top_levels)
    }
}
