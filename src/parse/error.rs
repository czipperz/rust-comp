use crate::lex::TokenType;
use crate::pos::Pos;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    ExpectedToken(TokenType, Pos),
    EOF,
}
