use super::combinator::*;
use super::{Error, Parse, Parser};
use crate::ast::*;
use crate::token::TokenKind;

impl<'a> Parse<'a> for Enum<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Enum<'a>, Error> {
        parser.expect_token(TokenKind::Enum)?;
        let name = parser.expect_label()?;
        parser.expect_token(TokenKind::OpenCurly)?;
        let variants = many_comma_separated(parser)?;
        parser.expect_token(TokenKind::CloseCurly)?;
        Ok(Enum { name, variants })
    }
}

impl<'a> Parse<'a> for Variant<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Variant<'a>, Error> {
        let name = parser.expect_label()?;
        let data = if parser.peek() == Some(TokenKind::OpenParen) {
            match Type::parse(parser)? {
                Type::Tuple(types) => VariantData::Tuple(types),
                _ => unreachable!(),
            }
        } else {
            VariantData::None
        };
        Ok(Variant { name, data })
    }
}

#[cfg(test)]
mod tests {
    use super::super::parse::parse;
    use super::*;

    #[test]
    fn test_parse_enum_0() {
        let (index, len, enum_) = parse("enum X {}");
        assert_eq!(index, len);
        assert_eq!(
            enum_,
            Ok(Enum {
                name: "X",
                variants: vec![],
            })
        );
    }

    #[test]
    fn test_parse_enum_2() {
        let (index, len, enum_) = parse("enum X {Y, Z,}");
        assert_eq!(index, len);
        assert_eq!(
            enum_,
            Ok(Enum {
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
            })
        );
    }

    #[test]
    fn test_parse_enum_with_tuple_data() {
        let (index, len, enum_) = parse("enum Ref {Ref(i32)}");
        assert_eq!(index, len);
        assert_eq!(
            enum_,
            Ok(Enum {
                name: "Ref",
                variants: vec![Variant {
                    name: "Ref",
                    data: VariantData::Tuple(vec![Type::Named(NamedType { name: "i32" })])
                }],
            })
        );
    }
}
