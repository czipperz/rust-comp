use super::parser::Parser;
use super::Error;
use crate::ast::*;

pub fn expect_type(parser: &mut Parser) -> Result<Type, Error> {
    parser.expect_label().map(|label| {
        Type::Named(NamedType {
            name: label.to_string(),
        })
    })
}
