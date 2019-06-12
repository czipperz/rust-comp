use super::{Error, Parse, Parser};
use crate::ast::*;
use crate::token::TokenKind;

impl<'a> Parse<'a> for Visibility<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Visibility<'a>, Error> {
        if parser.expect_token(TokenKind::Pub).is_ok() {
            if parser.expect_token(TokenKind::OpenParen).is_ok() {
                if parser.expect_token(TokenKind::CloseParen).is_ok() {
                    Ok(Visibility::Public)
                } else {
                    let path = Path::parse(parser)?;
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
}

#[cfg(test)]
mod tests {
    use super::super::parse;
    use super::*;
    use crate::ast::Path;
    use crate::pos::Span;

    #[test]
    fn test_parse_visibility_nothing() {
        let (index, _, visibility) = parse("fn");
        assert_eq!(index, 0);
        assert_eq!(visibility, Ok(Visibility::Private));
    }

    #[test]
    fn test_parse_visibility_pub() {
        let (index, len, visibility) = parse("pub");
        assert_eq!(index, len);
        assert_eq!(visibility, Ok(Visibility::Public));
    }

    #[test]
    fn test_parse_visibility_path() {
        let (index, len, visibility) = parse("pub(x::y)");
        assert_eq!(index, len);
        assert_eq!(
            visibility,
            Ok(Visibility::Path(Path {
                path: vec!["x", "y"]
            }))
        );
    }

    #[test]
    fn test_parse_visibility_path_no_closing_paren() {
        let (index, len, visibility) = parse::<Visibility>("pub(x::y");
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
    fn test_parse_visibility_nothing_in_parens() {
        let (index, len, visibility) = parse("pub()");
        assert_eq!(index, len);
        assert_eq!(visibility, Ok(Visibility::Public));
    }
}
