use super::combinator::*;
use super::error::Error;
use super::parser::Parser;
use crate::ast::*;
use crate::token::TokenKind;

pub fn expect_pattern<'a>(parser: &mut Parser<'a, '_>) -> Result<Pattern<'a>, Error> {
    if let Ok(name) = parser.expect_label() {
        if parser.expect_token(TokenKind::OpenParen).is_ok() {
            let patterns =
                many_separator(parser, expect_pattern, |p| p.expect_token(TokenKind::Comma))?;
            parser.expect_token(TokenKind::CloseParen)?;
            Ok(Pattern::NamedTuple(name, patterns))
        } else {
            Ok(Pattern::Named(name))
        }
    } else if parser.expect_token(TokenKind::OpenParen).is_ok() {
        let patterns =
            many_separator(parser, expect_pattern, |p| p.expect_token(TokenKind::Comma))?;
        parser.expect_token(TokenKind::CloseParen)?;
        Ok(Pattern::Tuple(patterns))
    } else {
        Err(Error::Expected("pattern", parser.span()))
    }
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;

    #[test]
    fn test_expect_pattern_open_curly_is_err() {
        let (index, _, pattern) = parse(expect_pattern, "{");
        let error = pattern.unwrap_err();
        assert_eq!(index, 0);
        assert_eq!(
            error,
            Error::Expected(
                "pattern",
                Span {
                    file: 0,
                    start: 0,
                    end: 1
                }
            )
        );
    }

    #[test]
    fn test_expect_pattern_label_eof() {
        let (index, len, pattern) = parse(expect_pattern, "abc");
        let pattern = pattern.unwrap();
        assert_eq!(index, len);
        assert_eq!(pattern, Pattern::Named("abc"));
    }

    #[test]
    fn test_expect_pattern_label_named_tuple() {
        let (index, len, pattern) = parse(expect_pattern, "abc(def)");
        let pattern = pattern.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            pattern,
            Pattern::NamedTuple("abc", vec![Pattern::Named("def")])
        );
    }

    #[test]
    fn test_expect_pattern_label_tuple() {
        let (index, len, pattern) = parse(expect_pattern, "(abc, def)");
        let pattern = pattern.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            pattern,
            Pattern::Tuple(vec![Pattern::Named("abc"), Pattern::Named("def")])
        );
    }
}
