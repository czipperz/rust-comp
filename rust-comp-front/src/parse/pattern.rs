use super::combinator::*;
use super::error::Error;
use super::parser::Parser;
use super::tree::*;
use crate::token::TokenKind;

pub fn expect_pattern(parser: &mut Parser) -> Result<Pattern, Error> {
    match parser.peek_kind() {
        Some(TokenKind::Label) => expect_named_pattern(parser),
        Some(TokenKind::Underscore) => expect_hole_pattern(parser),
        Some(TokenKind::OpenParen) => expect_paren_pattern(parser),
        _ => Err(Error::Expected("pattern", parser.span())),
    }
}

fn expect_named_pattern(parser: &mut Parser) -> Result<Pattern, Error> {
    let name = parser.expect_token(TokenKind::Label)?;
    if parser.peek_kind() == Some(TokenKind::OpenParen) {
        Ok(Pattern::NamedTuple(name, expect_tuple_pattern(parser)?))
    } else {
        Ok(Pattern::Named(name))
    }
}

fn expect_hole_pattern(parser: &mut Parser) -> Result<Pattern, Error> {
    Ok(Pattern::Hole(parser.expect_token(TokenKind::Underscore)?))
}

fn expect_paren_pattern(parser: &mut Parser) -> Result<Pattern, Error> {
    let pattern = expect_tuple_pattern(parser)?;
    if pattern.patterns.len() == 1 && pattern.comma_spans.len() == 0 {
        Ok(Pattern::Paren(ParenPattern {
            open_paren_span: pattern.open_paren_span,
            pattern: Box::new(pattern.patterns.into_iter().next().unwrap()),
            close_paren_span: pattern.close_paren_span,
        }))
    } else {
        Ok(Pattern::Tuple(pattern))
    }
}

fn expect_tuple_pattern(parser: &mut Parser) -> Result<TuplePattern, Error> {
    let open_paren_span = parser.expect_token(TokenKind::OpenParen)?;
    let (patterns, comma_spans) = many_comma_separated(parser, expect_pattern)?;
    let close_paren_span = parser.expect_token(TokenKind::CloseParen)?;
    Ok(TuplePattern {
        open_paren_span,
        patterns,
        comma_spans,
        close_paren_span,
    })
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;
    use assert_matches::assert_matches;

    #[test]
    fn test_expect_pattern_open_curly_is_err() {
        let (index, _, pattern) = parse(expect_pattern, "{");
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
    fn test_expect_pattern_label_eof() {
        let (index, len, pattern) = parse(expect_pattern, "abc");
        assert_eq!(index, len);
        assert_eq!(
            pattern,
            Ok(Pattern::Named(Span {
                file: 0,
                start: 0,
                end: 3
            }))
        );
    }

    #[test]
    fn test_expect_pattern_hole() {
        let (index, len, pattern) = parse(expect_pattern, "_");
        assert_eq!(index, len);
        assert_eq!(
            pattern,
            Ok(Pattern::Hole(Span {
                file: 0,
                start: 0,
                end: 1
            }))
        );
    }

    #[test]
    fn test_expect_pattern_label_named_tuple() {
        let (index, len, pattern) = parse(expect_pattern, "abc(def)");
        assert_eq!(index, len);
        assert_matches!(pattern, Ok(Pattern::NamedTuple(name, tuple)) => {
            assert_eq!(
                name,
                Span {
                    file: 0,
                    start: 0,
                    end: 3
                }
            );
            assert_eq!(tuple.patterns.len(), 1);
        });
    }

    #[test]
    fn test_expect_pattern_tuple_2() {
        let (index, len, pattern) = parse(expect_pattern, "(abc, def)");
        assert_eq!(index, len);
        assert_matches!(pattern, Ok(Pattern::Tuple(tuple)) => {
            assert_eq!(tuple.patterns.len(), 2);
            assert_eq!(tuple.comma_spans.len(), 1);
        });
    }

    #[test]
    fn test_expect_pattern_paren_1_no_trailing_comma_should_be_paren() {
        let (index, len, pattern) = parse(expect_pattern, "(abc)");
        assert_eq!(index, len);
        assert_eq!(
            pattern,
            Ok(Pattern::Paren(ParenPattern {
                open_paren_span: Span {
                    file: 0,
                    start: 0,
                    end: 1
                },
                pattern: Box::new(Pattern::Named(Span {
                    file: 0,
                    start: 1,
                    end: 4
                })),
                close_paren_span: Span {
                    file: 0,
                    start: 4,
                    end: 5
                },
            }))
        );
    }
}
