use super::parser::Parser;
use super::path::expect_path;
use super::Error;
use crate::ast::Visibility;
use crate::token::TokenKind;

pub fn expect_visibility<'a>(parser: &mut Parser<'a, '_>) -> Result<Visibility<'a>, Error> {
    if parser.expect_token(TokenKind::Pub).is_ok() {
        if parser.expect_token(TokenKind::OpenParen).is_ok() {
            if parser.expect_token(TokenKind::CloseParen).is_ok() {
                Ok(Visibility::Public)
            } else {
                let path = expect_path(parser)?;
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(Visibility::Path(path))
            }
        } else {
            Ok(Visibility::Public)
        }
    } else {
        Ok(Visibility::Private)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::ast::Path;
    use crate::pos::Span;

    #[test]
    fn test_expect_visibility_nothing() {
        let (index, _, visibility) = parse(expect_visibility, "fn");
        let visibility = visibility.unwrap();
        assert_eq!(index, 0);
        assert_eq!(visibility, Visibility::Private);
    }

    #[test]
    fn test_expect_visibility_pub() {
        let (index, len, visibility) = parse(expect_visibility, "pub");
        let visibility = visibility.unwrap();
        assert_eq!(index, len);
        assert_eq!(visibility, Visibility::Public);
    }

    #[test]
    fn test_expect_visibility_path() {
        let (index, len, visibility) = parse(expect_visibility, "pub(x::y)");
        let visibility = visibility.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            visibility,
            Visibility::Path(Path {
                path: vec!["x", "y"]
            })
        );
    }

    #[test]
    fn test_expect_visibility_path_no_closing_paren() {
        let (index, len, visibility) = parse(expect_visibility, "pub(x::y");
        let error = visibility.unwrap_err();
        assert_eq!(index, len);
        assert_eq!(
            error,
            Error::ExpectedToken(
                TokenKind::CloseParen,
                Span {
                    file: 0,
                    start: 8,
                    end: 9,
                }
            )
        );
    }

    #[test]
    fn test_expect_visibility_nothing_in_parens() {
        let (index, len, visibility) = parse(expect_visibility, "pub()");
        let visibility = visibility.unwrap();
        assert_eq!(index, len);
        assert_eq!(visibility, Visibility::Public);
    }
}
