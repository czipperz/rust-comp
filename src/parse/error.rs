use crate::pos::Span;
use crate::token::TokenValue;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    ExpectedToken(TokenValue, Span),
    Expected(&'static str, Span),
}
