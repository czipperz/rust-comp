use super::parser::Parser;
use super::path::expect_path;
use super::tree::*;
use super::Error;
use crate::token::TokenKind;

pub fn expect_visibility<'a>(parser: &mut Parser) -> Result<Visibility, Error> {
    if let Ok(pub_span) = parser.expect_token(TokenKind::Pub) {
        if let Ok(open_paren_span) = parser.expect_token(TokenKind::OpenParen) {
            let path = expect_path(parser)?;
            let close_paren_span = parser.expect_token(TokenKind::CloseParen)?;
            Ok(Visibility::Path(PathVisibility {
                pub_span,
                open_paren_span,
                path,
                close_paren_span,
            }))
        } else {
            Ok(Visibility::Public(pub_span))
        }
    } else {
        Ok(Visibility::Private)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;
    use assert_matches::assert_matches;

    #[test]
    fn test_expect_visibility_nothing() {
        let (index, _, visibility) = parse(expect_visibility, "fn");
        assert_eq!(index, 0);
        assert_eq!(visibility, Ok(Visibility::Private));
    }

    #[test]
    fn test_expect_visibility_pub() {
        let (index, len, visibility) = parse(expect_visibility, "pub");
        assert_eq!(index, len);
        assert_matches!(visibility, Ok(Visibility::Public(_)));
    }

    #[test]
    fn test_expect_visibility_path() {
        let (index, len, visibility) = parse(expect_visibility, "pub(x::y)");
        let visibility = visibility.unwrap();
        assert_eq!(index, len);
        assert_matches!(
            visibility,
            Visibility::Path(PathVisibility {
                path,
                ..
            }) => {
                assert_eq!(path.segments.len(), 2);
                assert_eq!(path.prefix_separator, None);
                assert_eq!(path.separator_spans.len(), 1);
            }
        );
    }

    #[test]
    fn test_expect_visibility_path_no_closing_paren() {
        let (index, len, visibility) = parse(expect_visibility, "pub(x::y");
        assert_eq!(index, len);
        assert_eq!(
            visibility,
            Err(Error::ExpectedToken(
                TokenKind::CloseParen,
                Span {
                    file: 0,
                    start: 8,
                    end: 9,
                }
            ))
        );
    }

    #[test]
    fn test_expect_visibility_nothing_in_parens() {
        let (index, len, visibility) = parse(expect_visibility, "pub()");
        assert_eq!(index, len - 1);
        assert!(visibility.is_err());
    }
}
