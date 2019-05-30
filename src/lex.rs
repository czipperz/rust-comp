use crate::pos::*;
use crate::tagged_iter::TaggedIter;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub value: TokenValue,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenValue {
    Fn,
    Label(String),
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Semicolon,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenizerError {}

pub fn read_tokens(mut tagged_iter: TaggedIter) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens = Vec::new();

    let mut start = tagged_iter.pos();
    let mut temp = String::new();

    loop {
        let pos = tagged_iter.pos();
        let ch = tagged_iter.next();

        if ch.is_none() || ch.unwrap().is_whitespace() {
            flush_temp(&mut tokens, &mut temp, Span { start, end: pos });

            if ch.is_none() {
                break;
            }
            start = tagged_iter.pos();
        } else {
            let ch = ch.unwrap();
            if "(){};".contains(ch) {
                flush_temp(&mut tokens, &mut temp, Span { start, end: pos });
                start = pos;
            }
            temp.push(ch);
        }
    }

    Ok(tokens)
}

fn flush_temp(tokens: &mut Vec<Token>, temp: &mut String, span: Span) {
    const SYMBOLS: [(&str, TokenValue); 6] = [
        ("fn", TokenValue::Fn),
        ("(", TokenValue::OpenParen),
        (")", TokenValue::CloseParen),
        ("{", TokenValue::OpenCurly),
        ("}", TokenValue::CloseCurly),
        (";", TokenValue::Semicolon),
    ];
    if !temp.is_empty() {
        tokens.push(Token {
            value: if let Some(i) = SYMBOLS.iter().position(|(s, _)| *s == temp) {
                SYMBOLS[i].1.clone()
            } else {
                TokenValue::Label(temp.clone())
            },
            span,
        });
        temp.clear();
    }
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_tokens_empty_file() {
        assert_eq!(read_tokens(TaggedIter::new("".to_string())), Ok(vec![]));
    }

    #[test]
    fn test_read_tokens_whitespace_file() {
        assert_eq!(read_tokens(TaggedIter::new("  ".to_string())), Ok(vec![]));
    }

    #[test]
    fn test_read_tokens_fn_eof() {
        assert_eq!(
            read_tokens(TaggedIter::new("fn".to_string())),
            Ok(vec![Token {
                value: TokenValue::Fn,
                span: Span {
                    start: Pos::start(),
                    end: Pos {
                        line: 0,
                        column: 2,
                        index: 2
                    }
                }
            }])
        );
    }

    #[test]
    fn test_read_tokens_fn_space() {
        assert_eq!(
            read_tokens(TaggedIter::new("fn ".to_string())),
            Ok(vec![Token {
                value: TokenValue::Fn,
                span: Span {
                    start: Pos::start(),
                    end: Pos {
                        line: 0,
                        column: 2,
                        index: 2
                    }
                }
            }])
        );
    }

    #[test]
    fn test_read_tokens_fnx() {
        assert_eq!(
            read_tokens(TaggedIter::new("fnx".to_string())),
            Ok(vec![Token {
                value: TokenValue::Label("fnx".to_string()),
                span: Span {
                    start: Pos::start(),
                    end: Pos {
                        line: 0,
                        column: 3,
                        index: 3
                    }
                }
            }])
        );
    }

    #[test]
    fn test_read_tokens_symbols() {
        assert_eq!(
            read_tokens(TaggedIter::new("(){};".to_string())),
            Ok(vec![
                Token {
                    value: TokenValue::OpenParen,
                    span: Span {
                        start: Pos {
                            line: 0,
                            column: 0,
                            index: 0
                        },
                        end: Pos {
                            line: 0,
                            column: 1,
                            index: 1
                        }
                    }
                },
                Token {
                    value: TokenValue::CloseParen,
                    span: Span {
                        start: Pos {
                            line: 0,
                            column: 1,
                            index: 1
                        },
                        end: Pos {
                            line: 0,
                            column: 2,
                            index: 2
                        }
                    }
                },
                Token {
                    value: TokenValue::OpenCurly,
                    span: Span {
                        start: Pos {
                            line: 0,
                            column: 2,
                            index: 2
                        },
                        end: Pos {
                            line: 0,
                            column: 3,
                            index: 3
                        }
                    }
                },
                Token {
                    value: TokenValue::CloseCurly,
                    span: Span {
                        start: Pos {
                            line: 0,
                            column: 3,
                            index: 3
                        },
                        end: Pos {
                            line: 0,
                            column: 4,
                            index: 4
                        }
                    }
                },
                Token {
                    value: TokenValue::Semicolon,
                    span: Span {
                        start: Pos {
                            line: 0,
                            column: 4,
                            index: 4
                        },
                        end: Pos {
                            line: 0,
                            column: 5,
                            index: 5
                        }
                    }
                },
            ])
        );
    }
}
