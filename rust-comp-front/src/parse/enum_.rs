use super::combinator::*;
use super::error::Error;
use super::parser::Parser;
use super::type_::expect_type;
use crate::ast::*;
use crate::token::TokenKind;

pub fn expect_enum<'a>(parser: &mut Parser<'a, '_>) -> Result<Enum<'a>, Error> {
    parser.expect_token(TokenKind::Enum)?;
    let name = parser.expect_label()?;
    parser.expect_token(TokenKind::OpenCurly)?;
    let variants = many_separator(parser, expect_variant, |p| p.expect_token(TokenKind::Comma))?;
    parser.expect_token(TokenKind::CloseCurly)?;
    Ok(Enum { name, variants })
}

fn expect_variant<'a>(parser: &mut Parser<'a, '_>) -> Result<Variant<'a>, Error> {
    let name = parser.expect_label()?;
    let data = if parser.peek() == Some(TokenKind::OpenParen) {
        match expect_type(parser)? {
            Type::Tuple(types) => VariantData::Tuple(types),
            _ => unreachable!(),
        }
    } else {
        VariantData::None
    };
    Ok(Variant { name, data })
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;

    #[test]
    fn test_expect_enum_0() {
        let (index, len, enum_) = parse(expect_enum, "enum X {}");
        let enum_ = enum_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            enum_,
            Enum {
                name: "X",
                variants: vec![],
            }
        );
    }

    #[test]
    fn test_expect_enum_2() {
        let (index, len, enum_) = parse(expect_enum, "enum X {Y, Z,}");
        let enum_ = enum_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            enum_,
            Enum {
                name: "X",
                variants: vec![
                    Variant {
                        name: "Y",
                        data: VariantData::None,
                    },
                    Variant {
                        name: "Z",
                        data: VariantData::None,
                    }
                ],
            }
        );
    }

    #[test]
    fn test_expect_enum_with_tuple_data() {
        let (index, len, enum_) = parse(expect_enum, "enum Ref {Ref(i32)}");
        let enum_ = enum_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            enum_,
            Enum {
                name: "Ref",
                variants: vec![Variant {
                    name: "Ref",
                    data: VariantData::Tuple(vec![Type::Named(NamedType { name: "i32" })])
                }],
            }
        );
    }
}
