use super::{Error, Parse, Parser};
use crate::ast::*;
use crate::token::TokenKind;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref TOP_LEVEL_KIND: HashMap<TokenKind, for<'a> fn(&mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error>> = {
        let mut table = HashMap::<
            TokenKind,
            for<'a> fn(&mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error>,
        >::new();
        table.insert(TokenKind::Fn, parse_toplevel_fn);
        table.insert(TokenKind::Struct, parse_toplevel_struct);
        table.insert(TokenKind::Enum, parse_toplevel_enum);
        table.insert(TokenKind::Mod, parse_mod);
        table.insert(TokenKind::Use, parse_use);
        table
    };
}

impl<'a> Parse<'a> for TopLevel<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<TopLevel<'a>, Error> {
        let visibility = Visibility::parse(parser)?;
        let kind = parser.dispatch(&TOP_LEVEL_KIND, "top level declaration")?;
        Ok(TopLevel { visibility, kind })
    }
}

fn parse_toplevel_fn<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error> {
    Function::parse(parser).map(TopLevelKind::Function)
}

fn parse_toplevel_struct<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error> {
    Struct::parse(parser).map(TopLevelKind::Struct)
}

fn parse_toplevel_enum<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error> {
    Enum::parse(parser).map(TopLevelKind::Enum)
}

fn parse_mod<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error> {
    parser.expect_token(TokenKind::Mod)?;
    let name = parser.expect_label()?;
    parser.expect_token(TokenKind::Semicolon)?;
    Ok(TopLevelKind::ModFile(ModFile { mod_: name }))
}

fn parse_use<'a>(parser: &mut Parser<'a, '_>) -> Result<TopLevelKind<'a>, Error> {
    parser.expect_token(TokenKind::Use)?;
    let mut path = Path::parse(parser)?;
    let item = path.path.pop().unwrap();
    parser.expect_token(TokenKind::Semicolon)?;
    Ok(TopLevelKind::Use(Use { path, item }))
}

#[cfg(test)]
mod tests {
    use super::super::{parse, parse_fn};
    use super::*;
    use crate::pos::Span;

    #[test]
    fn test_parse_top_level_works() {
        let (index, len, top_level) = parse("pub mod x;");
        assert_eq!(index, len);
        assert_eq!(
            top_level,
            Ok(TopLevel {
                visibility: Visibility::Public,
                kind: TopLevelKind::ModFile(ModFile { mod_: "x" }),
            })
        );
    }

    #[test]
    fn test_parse_top_level_fails() {
        let (index, len, top_level) = parse::<TopLevel>("pub");
        assert_eq!(index, len);
        assert_eq!(
            top_level,
            Err(Error::Expected(
                "top level declaration",
                Span {
                    file: 0,
                    start: 3,
                    end: 4
                }
            ))
        );
    }

    #[test]
    fn test_parse_mod() {
        let (index, len, mod_) = parse_fn(parse_mod, "mod x;");
        let mod_ = mod_.unwrap();
        assert_eq!(index, len);
        assert_eq!(mod_, TopLevelKind::ModFile(ModFile { mod_: "x" }));
    }

    #[test]
    fn test_parse_use_label() {
        let (index, len, mod_) = parse_fn(parse_use, "use x;");
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
    fn test_parse_use_long_path() {
        let (index, len, mod_) = parse_fn(parse_use, "use x::y::z;");
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
