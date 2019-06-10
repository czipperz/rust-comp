use super::combinator::*;
use super::error::Error;
use super::parser::Parser;
use super::type_::expect_type;
use super::visibility::expect_visibility;
use crate::ast::*;
use crate::token::TokenKind;

pub fn expect_struct<'a>(parser: &mut Parser<'a, '_>) -> Result<Struct<'a>, Error> {
    parser.expect_token(TokenKind::Struct)?;
    let name = parser.expect_label()?;
    parser.expect_token(TokenKind::OpenCurly)?;
    let fields = many_separator(parser, expect_field, |p| p.expect_token(TokenKind::Comma))?;
    parser.expect_token(TokenKind::CloseCurly)?;
    Ok(Struct { name, fields })
}

fn expect_field<'a>(parser: &mut Parser<'a, '_>) -> Result<Field<'a>, Error> {
    let visibility = expect_visibility(parser)?;
    let name = parser.expect_label()?;
    parser.expect_token(TokenKind::Colon)?;
    let type_ = expect_type(parser)?;
    Ok(Field {
        visibility,
        name,
        type_,
    })
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    //use crate::ast::*;

    #[test]
    fn test_expect_struct_0() {
        let (index, len, struct_) = parse(expect_struct, "struct X {}");
        let struct_ = struct_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            struct_,
            Struct {
                name: "X",
                fields: vec![],
            }
        );
    }

    #[test]
    fn test_expect_struct_1() {
        let (index, len, struct_) = parse(expect_struct, "struct X {pub y: Y}");
        let struct_ = struct_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            struct_,
            Struct {
                name: "X",
                fields: vec![Field {
                    visibility: Visibility::Public,
                    name: "y",
                    type_: Type::Named(NamedType { name: "Y" }),
                }],
            }
        );
    }

    #[test]
    fn test_expect_struct_1_trailing_comma() {
        let (index, len, struct_) = parse(expect_struct, "struct X {y: Y,}");
        let struct_ = struct_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            struct_,
            Struct {
                name: "X",
                fields: vec![Field {
                    visibility: Visibility::Private,
                    name: "y",
                    type_: Type::Named(NamedType { name: "Y" }),
                }],
            }
        );
    }

    #[test]
    fn test_expect_struct_2() {
        let (index, len, struct_) = parse(expect_struct, "struct X {y: Y, z: Z}");
        let struct_ = struct_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            struct_,
            Struct {
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
            }
        );
    }
}
