use crate::pos::Span;
use crate::token::TokenKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    ExpectedToken(TokenKind, Span),
    Expected(&'static str, Span),
}
