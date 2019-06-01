use super::parser::Parser;
use super::Error;
use crate::ast::Type;

pub fn expect_type(parser: &mut Parser) -> Result<Type, Error> {
    parser.expect_label().map(|label| Type::Named(label.to_string()))
}
