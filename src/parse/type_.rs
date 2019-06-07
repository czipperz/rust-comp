use super::parser::Parser;
use super::Error;
use crate::ast::*;

pub fn expect_type<'a>(parser: &mut Parser<'a>) -> Result<Type<'a>, Error> {
    parser
        .expect_label()
        .map(|name| Type::Named(NamedType { name }))
}
