use super::combinator::*;
use super::fn_::expect_fn;
use super::parser::Parser;
use super::Error;
use crate::ast::*;
use crate::token::*;

pub fn expect_top_level<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevel<'a>, Error> {
    let visibility = expect_visibility(parser)?;
    let kind = one_of(
        parser,
        &mut [expect_toplevel_fn, expect_mod, expect_use][..],
        Error::Expected("expression", parser.span()),
    )?;
    Ok(TopLevel { kind, visibility })
}

fn expect_visibility<'a>(parser: &mut Parser<'a, '_>) -> Result<Visibility<'a>, Error> {
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

fn expect_toplevel_fn<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error> {
    expect_fn(parser).map(TopLevelKind::Function)
}

fn expect_mod<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error> {
    parser.expect_token(TokenKind::Mod)?;
    let name = parser.expect_label()?;
    parser.expect_token(TokenKind::Semicolon)?;
    Ok(TopLevelKind::ModFile(ModFile { mod_: name }))
}

fn expect_use<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error> {
    parser.expect_token(TokenKind::Use)?;
    let mut path = expect_path(parser)?;
    let item = path.path.pop().unwrap();
    parser.expect_token(TokenKind::Semicolon)?;
    Ok(TopLevelKind::Use(Use { path, item }))
}

fn expect_path<'a>(parser: &mut Parser<'a, '_>) -> Result<Path<'a>, Error> {
    let mut path = Vec::new();
    path.push(parser.expect_label()?);
    while parser.expect_token(TokenKind::ColonColon).is_ok() {
        path.push(parser.expect_label()?);
    }
    Ok(Path { path })
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;

    #[test]
    fn test_expect_mod() {
        let (index, len, mod_) = parse(expect_mod, "mod x;");
        let mod_ = mod_.unwrap();
        assert_eq!(index, len);
        assert_eq!(mod_, TopLevelKind::ModFile(ModFile { mod_: "x" }));
    }

    #[test]
    fn test_expect_use_label() {
        let (index, len, mod_) = parse(expect_use, "use x;");
        let mod_ = mod_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            mod_,
            TopLevelKind::Use(Use {
                path: Path { path: vec![] },
                item: "x",
            })
        );
    }

    #[test]
    fn test_expect_use_long_path() {
        let (index, len, mod_) = parse(expect_use, "use x::y::z;");
        let mod_ = mod_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            mod_,
            TopLevelKind::Use(Use {
                path: Path {
                    path: vec!["x", "y"]
                },
                item: "z",
            })
        );
    }

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
