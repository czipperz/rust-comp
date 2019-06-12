use super::combinator::*;
use super::{Error, Parse, Parser};
use crate::ast::*;
use crate::token::TokenKind;

impl<'a> Parse<'a> for Pattern<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Pattern<'a>, Error> {
        if let Ok(name) = parser.expect_label() {
            if parser.expect_token(TokenKind::OpenParen).is_ok() {
                let patterns = many_comma_separated(parser)?;
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(Pattern::NamedTuple(name, patterns))
            } else {
                Ok(Pattern::Named(name))
            }
        } else if parser.expect_token(TokenKind::OpenParen).is_ok() {
            let patterns = many_comma_separated(parser)?;
            parser.expect_token(TokenKind::CloseParen)?;
            Ok(Pattern::Tuple(patterns))
        } else {
            Err(Error::Expected("pattern", parser.span()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::parse;
    use super::*;
    use crate::pos::Span;

    #[test]
    fn test_parse_pattern_open_curly_is_err() {
        let (index, _, pattern) = parse::<Pattern>("{");
        assert_eq!(index, 0);
        assert_eq!(
            pattern,
            Err(Error::Expected(
                "pattern",
                Span {
                    file: 0,
                    start: 0,
                    end: 1
                }
            ))
        );
    }

    #[test]
    fn test_parse_pattern_label_eof() {
        let (index, len, pattern) = parse("abc");
        assert_eq!(index, len);
        assert_eq!(pattern, Ok(Pattern::Named("abc")));
    }

    #[test]
    fn test_parse_pattern_label_named_tuple() {
        let (index, len, pattern) = parse("abc(def)");
        assert_eq!(index, len);
        assert_eq!(
            pattern,
            Ok(Pattern::NamedTuple("abc", vec![Pattern::Named("def")]))
        );
    }

    #[test]
    fn test_parse_pattern_label_tuple() {
        let (index, len, pattern) = parse("(abc, def)");
        assert_eq!(index, len);
        assert_eq!(
            pattern,
            Ok(Pattern::Tuple(vec![
                Pattern::Named("abc"),
                Pattern::Named("def")
            ]))
        );
    }
}
