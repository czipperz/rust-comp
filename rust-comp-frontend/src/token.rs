use crate::pos::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Const,
    Else,
    Fn,
    If,
    Let,
    Label,
    Mod,
    Mut,
    Pub,
    Use,
    While,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Colon,
    ColonColon,
    Comma,
    ThinArrow,
    FatArrow,
    Equals,
    NotEquals,
    Ampersand,
    And,
    ForwardSlash,
    Minus,
    Or,
    Plus,
    Set,
    Semicolon,
    Star,
}
