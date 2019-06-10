use crate::pos::*;
use std::fmt;

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

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind::*;
        write!(
            f,
            "{}",
            match self {
                Const => "`const`",
                Else => "`else`",
                Fn => "`fn`",
                If => "`if`",
                Let => "`let`",
                Label => "a label",
                Mod => "`mod`",
                Mut => "`mut`",
                Pub => "`pub`",
                Use => "`use`",
                While => "`while`",
                OpenParen => "`(`",
                CloseParen => "`)`",
                OpenCurly => "`{`",
                CloseCurly => "`}`",
                Colon => "`:`",
                ColonColon => "`::`",
                Comma => "`,`",
                ThinArrow => "`->`",
                FatArrow => "`=>`",
                Equals => "`==`",
                NotEquals => "`!=`",
                Ampersand => "`&`",
                And => "`&&`",
                ForwardSlash => "`/`",
                Minus => "`-`",
                Or => "`||`",
                Plus => "`+`",
                Set => "`=`",
                Semicolon => "`;`",
                Star => "`*`",
            }
        )
    }
}
