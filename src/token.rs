use crate::pos::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Else,
    Fn,
    If,
    Let,
    Label,
    Mod,
    Pub,
    Use,
    While,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Colon,
    Comma,
    ThinArrow,
    FatArrow,
    Equals,
    NotEquals,
    ForwardSlash,
    Minus,
    Plus,
    Set,
    Semicolon,
    Star,
}
