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

#[cfg(test)]
pub fn make_tokens(values: Vec<TokenValue>) -> Vec<Token> {
    values.into_iter().map(make_token).collect()
}

#[cfg(test)]
pub fn make_token(value: TokenValue) -> Token {
    use crate::pos::*;
    Token {
        value,
        span: Span {
            start: Pos::start(),
            end: Pos::start(),
        },
    }
}
