use super::combinator::*;
use super::{Error, Parse, Parser};
use crate::ast::*;
use crate::token::TokenKind;

impl<'a> Parse<'a> for Struct<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Struct<'a>, Error> {
        parser.expect_token(TokenKind::Struct)?;
        let name = parser.expect_label()?;
        parser.expect_token(TokenKind::OpenCurly)?;
        let fields = many_comma_separated(parser)?;
        parser.expect_token(TokenKind::CloseCurly)?;
        Ok(Struct { name, fields })
    }
}

impl<'a> Parse<'a> for Field<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Field<'a>, Error> {
        let visibility = Visibility::parse(parser)?;
        let name = parser.expect_label()?;
        parser.expect_token(TokenKind::Colon)?;
        let type_ = Type::parse(parser)?;
        Ok(Field {
            visibility,
            name,
            type_,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::parse;
    use super::*;

    #[test]
    fn test_expect_struct_0() {
        let (index, len, struct_) = parse("struct X {}");
        assert_eq!(index, len);
        assert_eq!(
            struct_,
            Ok(Struct {
                name: "X",
                fields: vec![],
            })
        );
    }

    #[test]
    fn test_expect_struct_1() {
        let (index, len, struct_) = parse("struct X {pub y: Y}");
        assert_eq!(index, len);
        assert_eq!(
            struct_,
            Ok(Struct {
                name: "X",
                fields: vec![Field {
                    visibility: Visibility::Public,
                    name: "y",
                    type_: Type::Named(NamedType { name: "Y" }),
                }],
            })
        );
    }

    #[test]
    fn test_expect_struct_1_trailing_comma() {
        let (index, len, struct_) = parse("struct X {y: Y,}");
        assert_eq!(index, len);
        assert_eq!(
            struct_,
            Ok(Struct {
                name: "X",
                fields: vec![Field {
                    visibility: Visibility::Private,
                    name: "y",
                    type_: Type::Named(NamedType { name: "Y" }),
                }],
            })
        );
    }

    #[test]
    fn test_expect_struct_2() {
        let (index, len, struct_) = parse("struct X {y: Y, z: Z}");
        assert_eq!(index, len);
        assert_eq!(
            struct_,
            Ok(Struct {
                name: "X",
                fields: vec![
                    Field {
                        visibility: Visibility::Private,
                        name: "y",
                        type_: Type::Named(NamedType { name: "Y" }),
                    },
                    Field {
                        visibility: Visibility::Private,
                        name: "z",
                        type_: Type::Named(NamedType { name: "Z" }),
                    }
                ],
            })
        );
    }
}
