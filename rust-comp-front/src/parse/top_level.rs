use super::enum_::expect_enum;
use super::fn_::expect_fn;
use super::parser::Parser;
use super::path::expect_path;
use super::struct_::expect_struct;
use super::tree::*;
use super::visibility::expect_visibility;
use super::Error;
use crate::token::*;

pub fn expect_top_level<'a>(parser: &mut Parser) -> Result<TopLevel, Error> {
    let visibility = expect_visibility(parser)?;
    let kind = match parser.peek_kind() {
        Some(TokenKind::Fn) => expect_toplevel_fn(parser),
        Some(TokenKind::Struct) => expect_toplevel_struct(parser),
        Some(TokenKind::Enum) => expect_toplevel_enum(parser),
        Some(TokenKind::Mod) => expect_mod(parser),
        Some(TokenKind::Use) => expect_use(parser),
        _ => Err(Error::Expected("top level declaration", parser.span())),
    }?;
    Ok(TopLevel { visibility, kind })
}

fn expect_toplevel_fn<'a>(parser: &mut Parser) -> Result<TopLevelKind, Error> {
    expect_fn(parser).map(TopLevelKind::Function)
}

fn expect_toplevel_struct<'a>(parser: &mut Parser) -> Result<TopLevelKind, Error> {
    expect_struct(parser).map(TopLevelKind::Struct)
}

fn expect_toplevel_enum<'a>(parser: &mut Parser) -> Result<TopLevelKind, Error> {
    expect_enum(parser).map(TopLevelKind::Enum)
}

fn expect_mod<'a>(parser: &mut Parser) -> Result<TopLevelKind, Error> {
    let mod_span = parser.expect_token(TokenKind::Mod)?;
    let name = parser.expect_token(TokenKind::Label)?;
    let semicolon_span = parser.expect_token(TokenKind::Semicolon)?;
    Ok(TopLevelKind::ModFile(ModFile {
        mod_span,
        name,
        semicolon_span,
    }))
}

fn expect_use<'a>(parser: &mut Parser) -> Result<TopLevelKind, Error> {
    let use_span = parser.expect_token(TokenKind::Use)?;
    let path = expect_use_path(parser)?;
    let semicolon_span = parser.expect_token(TokenKind::Semicolon)?;
    Ok(TopLevelKind::Use(Use {
        use_span,
        path,
        semicolon_span,
    }))
}

fn expect_use_path(parser: &mut Parser) -> Result<UsePath, Error> {
    let mut path = expect_path(parser)?;
    let item = path.segments.pop().unwrap();
    Ok(UsePath {
        segments: path.segments,
        prefix_separator: path.prefix_separator,
        separator_spans: path.separator_spans,
        suffix: UsePathSuffix::Item(item),
    })
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;
    use assert_matches::assert_matches;

    #[test]
    fn test_expect_top_level_mod() {
        let (index, len, top_level) = parse(expect_top_level, "pub mod x;");
        assert_eq!(index, len);
        assert_eq!(index, len);
        assert_matches!(top_level, Ok(TopLevel {
            visibility,
            kind: TopLevelKind::ModFile(_),
        }) =>
        {
            assert_eq!(
                visibility,
                Visibility::Public(Span {
                    file: 0,
                    start: 0,
                    end: 3
                })
            );
        });
    }

    #[test]
    fn test_expect_top_level_only_visibility_fails() {
        let (index, len, top_level) = parse(expect_top_level, "pub");
        let error = top_level.unwrap_err();
        assert_eq!(index, len);
        assert_eq!(
            error,
            Error::Expected(
                "top level declaration",
                Span {
                    file: 0,
                    start: 3,
                    end: 4
                }
            )
        );
    }

    #[test]
    fn test_expect_mod() {
        let (index, len, mod_) = parse(expect_mod, "mod x;");
        assert_eq!(index, len);
        assert!(mod_.is_ok());
    }

    #[test]
    fn test_expect_use_label() {
        let (index, len, use_) = parse(expect_use, "use x;");
        assert_eq!(index, len);
        assert!(use_.is_ok());
    }

    #[test]
    fn test_expect_use_long_path() {
        let (index, len, use_) = parse(expect_use, "use x::y::z;");
        assert_eq!(index, len);
        assert_matches!(use_, Ok(TopLevelKind::Use(use_)) => {
            assert_eq!(use_.path.segments.len(), 2);
            assert_eq!(
                use_.path.suffix,
                UsePathSuffix::Item(Span {
                    file: 0,
                    start: 10,
                    end: 11
                })
            );
        });
    }
}
