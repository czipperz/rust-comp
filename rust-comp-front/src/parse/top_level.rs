use super::enum_::expect_enum;
use super::fn_::expect_fn;
use super::parser::Parser;
use super::path::expect_path;
use super::struct_::expect_struct;
use super::visibility::expect_visibility;
use super::Error;
use crate::ast::*;
use crate::token::*;

pub fn expect_top_level<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevel<'a>, Error> {
    let visibility = expect_visibility(parser)?;
    let kind = match parser.peek() {
        Some(TokenKind::Fn) => expect_toplevel_fn(parser),
        Some(TokenKind::Struct) => expect_toplevel_struct(parser),
        Some(TokenKind::Enum) => expect_toplevel_enum(parser),
        Some(TokenKind::Mod) => expect_mod(parser),
        Some(TokenKind::Use) => expect_use(parser),
        _ => Err(Error::Expected("top level declaration", parser.span())),
    }?;
    Ok(TopLevel { visibility, kind })
}

fn expect_toplevel_fn<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error> {
    expect_fn(parser).map(TopLevelKind::Function)
}

fn expect_toplevel_struct<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error> {
    expect_struct(parser).map(TopLevelKind::Struct)
}

fn expect_toplevel_enum<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error> {
    expect_enum(parser).map(TopLevelKind::Enum)
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

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;

    #[test]
    fn test_expect_top_level_works() {
        let (index, len, top_level) = parse(expect_top_level, "pub mod x;");
        let top_level = top_level.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            top_level,
            TopLevel {
                visibility: Visibility::Public,
                kind: TopLevelKind::ModFile(ModFile { mod_: "x" }),
            }
        );
    }

    #[test]
    fn test_expect_top_level_fails() {
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
}
