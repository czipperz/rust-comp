use crate::lex;
use crate::pos::Span;
use crate::token::TokenKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    ExpectedToken(TokenKind, Span),
    Expected(&'static str, Span),
    Lex(lex::Error),
}

impl From<lex::Error> for Error {
    fn from(e: lex::Error) -> Self {
        Error::Lex(e)
    }
}
