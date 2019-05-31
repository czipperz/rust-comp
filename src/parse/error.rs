use crate::lex::TokenValue;
use crate::pos::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    ExpectedToken(TokenValue, Span),
    Expected(&'static str, Span),
}
