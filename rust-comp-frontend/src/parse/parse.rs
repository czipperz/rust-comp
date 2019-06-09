use super::combinator::many;
use super::error::Error;
use super::parser::Parser;
use super::top_level::expect_top_level;
use crate::ast::TopLevel;
use crate::pos::Pos;
use crate::token::Token;

pub fn parse<'a>(
    file_contents: &'a str,
    tokens: &'a [Token],
    eofpos: Pos,
) -> Result<Vec<TopLevel<'a>>, Error> {
    let mut parser = Parser::new(file_contents, tokens, eofpos);
    let top_levels = many(&mut parser, expect_top_level)?;

    if parser.index < tokens.len() {
        Err(Error::Expected("top level item", parser.span()))
    } else {
        Ok(top_levels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::read_tokens;

    #[test]
    fn test_parse_random_inputs_should_error() {
        let contents = "a b c";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let top_levels = parse(contents, &tokens, eofpos);
        assert!(top_levels.is_err());
    }
}
