use crate::lex::TokenValue;
use crate::pos::Pos;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    ExpectedToken(TokenValue, Pos),
    EOF,
}
