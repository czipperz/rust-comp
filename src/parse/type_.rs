use super::parser::Parser;
use super::Error;
use crate::ast::*;

pub fn expect_type<'a>(parser: &mut Parser<'a>) -> Result<&'a Type<'a>, Error> {
    parser
        .expect_label()
        .map(|name| parser.alloc(Type::Named(parser.alloc(NamedType { name }))))
}
