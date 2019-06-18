use super::parser::Parser;
use crate::lex::read_tokens;

pub fn parse<'a, F, T, E>(mut f: F, file_contents: &'a str) -> (usize, usize, Result<T, E>)
where
    F: FnMut(&mut Parser) -> Result<T, E>,
    T: 'a,
{
    let (tokens, eofpos) = read_tokens(0, file_contents).unwrap();
    let mut parser = Parser::new(file_contents, &tokens, eofpos);
    let res = f(&mut parser);
    (parser.index, tokens.len(), res)
}
