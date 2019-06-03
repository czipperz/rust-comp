use crate::pos::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub value: TokenValue,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenValue {
    Else,
    Fn,
    If,
    Let,
    Label,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Colon,
    Comma,
    ThinArrow,
    FatArrow,
    Set,
    Semicolon,
}
