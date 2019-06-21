use crate::pos::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Ampersand,
    And,
    CloseCurly,
    CloseParen,
    Colon,
    ColonColon,
    Comma,
    Const,
    Dot,
    Else,
    Enum,
    Equals,
    False,
    FatArrow,
    Fn,
    ForwardSlash,
    If,
    Label,
    Let,
    Match,
    Minus,
    Mod,
    Mut,
    NotEquals,
    OpenCurly,
    OpenParen,
    Or,
    Plus,
    Pub,
    Semicolon,
    Set,
    Star,
    Struct,
    ThinArrow,
    True,
    Underscore,
    Use,
    While,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind::*;
        write!(
            f,
            "{}",
            match self {
                Ampersand => "`&`",
                And => "`&&`",
                CloseCurly => "`}`",
                CloseParen => "`)`",
                Colon => "`:`",
                ColonColon => "`::`",
                Comma => "`,`",
                Const => "`const`",
                Dot => "`.`",
                Else => "`else`",
                Enum => "`enum`",
                Equals => "`==`",
                False => "`false`",
                FatArrow => "`=>`",
                Fn => "`fn`",
                ForwardSlash => "`/`",
                If => "`if`",
                Label => "a label",
                Let => "`let`",
                Match => "`match`",
                Minus => "`-`",
                Mod => "`mod`",
                Mut => "`mut`",
                NotEquals => "`!=`",
                OpenCurly => "`{`",
                OpenParen => "`(`",
                Or => "`||`",
                Plus => "`+`",
                Pub => "`pub`",
                Semicolon => "`;`",
                Set => "`=`",
                Star => "`*`",
                Struct => "`struct`",
                ThinArrow => "`->`",
                True => "`true`",
                Underscore => "`_`",
                Use => "`use`",
                While => "`while`",
            }
        )
    }
}
