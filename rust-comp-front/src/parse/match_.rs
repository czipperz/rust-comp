use super::error::Error;
use super::expression::expect_expression;
use super::parser::Parser;
use super::pattern::expect_pattern;
use super::statement::needs_semicolon;
use super::tree::*;
use crate::token::TokenKind;

pub fn expect_match<'a>(parser: &mut Parser) -> Result<Match, Error> {
    let match_span = parser.expect_token(TokenKind::Match)?;
    let value = expect_expression(parser)?;
    let open_curly_span = parser.expect_token(TokenKind::OpenCurly)?;
    let mut matches = Vec::new();
    let mut comma_spans = Vec::new();
    while parser.peek_kind() != Some(TokenKind::CloseCurly) {
        let match_item = expect_match_item(parser)?;
        let need_semicolon = needs_semicolon(&match_item.value);
        matches.push(match_item);
        match parser.expect_token(TokenKind::Comma) {
            Ok(span) => comma_spans.push(Some(span)),
            Err(_) => {
                comma_spans.push(None);
                if need_semicolon {
                    break;
                }
            }
        }
    }
    let close_curly_span = parser.expect_token(TokenKind::CloseCurly)?;
    Ok(Match {
        match_span,
        value: Box::new(value),
        open_curly_span,
        matches,
        comma_spans,
        close_curly_span,
    })
}

fn expect_match_item<'a>(parser: &mut Parser) -> Result<MatchItem, Error> {
    let pattern = expect_pattern(parser)?;
    let fat_arrow_span = parser.expect_token(TokenKind::FatArrow)?;
    let value = expect_expression(parser)?;
    Ok(MatchItem {
        pattern,
        fat_arrow_span,
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;

    #[test]
    fn test_expect_match_0() {
        let (index, len, match_) = parse(expect_match, "match x {}");
        assert_eq!(index, len);
        let match_ = match_.unwrap();
        assert_eq!(match_.matches.len(), 0);
    }

    #[test]
    fn test_expect_match_1() {
        let (index, len, match_) = parse(expect_match, "match x { a => () }");
        assert_eq!(index, len);
        let match_ = match_.unwrap();
        assert_eq!(match_.matches.len(), 1);
        assert_eq!(match_.comma_spans.len(), 1);
        assert!(match_.comma_spans[0].is_none());
    }

    #[test]
    fn test_expect_match_1_trailing_comma() {
        let (index, len, match_) = parse(expect_match, "match x { a => (), }");
        assert_eq!(index, len);
        let match_ = match_.unwrap();
        assert_eq!(match_.matches.len(), 1);
        assert_eq!(match_.comma_spans.len(), 1);
        assert!(match_.comma_spans[0].is_some());
    }

    #[test]
    fn test_expect_match_2_no_comma_because_braces() {
        let (index, len, match_) = parse(expect_match, "match x { a => {} b => () }");
        assert_eq!(index, len);
        let match_ = match_.unwrap();
        assert_eq!(match_.matches.len(), 2);
        assert_eq!(match_.comma_spans.len(), 2);
        assert!(match_.comma_spans[0].is_none());
        assert!(match_.comma_spans[1].is_none());
    }
}
