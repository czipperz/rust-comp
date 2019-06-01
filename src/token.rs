use crate::pos::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub value: TokenValue,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenValue {
    Fn,
    Label,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Semicolon,
}
