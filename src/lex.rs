use crate::pos::*;
use crate::tagged_iter::TaggedIter;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    TFn,
    TLabel(String),
    TOpenParen,
    TCloseParen,
    TOpenCurly,
    TCloseCurly,
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
            flush_temp(
                &mut tokens,
                std::mem::replace(&mut temp, String::new()),
                Span { start, end: pos },
            );

            if ch.is_none() {
                break;
            }
            start = tagged_iter.pos();
        } else {
            let ch = ch.unwrap();
            if "(){}".contains(ch) {
                flush_temp(
                    &mut tokens,
                    std::mem::replace(&mut temp, String::new()),
                    Span { start, end: pos },
                );
                start = pos;
            }
            temp.push(ch);
        }
    }

    Ok(tokens)
}

fn flush_temp(tokens: &mut Vec<Token>, temp: String, span: Span) {
    const SYMBOLS: [(&str, TokenType); 5] = [
        ("fn", TokenType::TFn),
        ("(", TokenType::TOpenParen),
        (")", TokenType::TCloseParen),
        ("{", TokenType::TOpenCurly),
        ("}", TokenType::TCloseCurly),
    ];
    if !temp.is_empty() {
        tokens.push(Token {
            token_type: if let Some(i) = SYMBOLS.iter().position(|(s, _)| *s == temp) {
                SYMBOLS[i].1.clone()
            } else {
                TokenType::TLabel(temp)
            },
            span,
        });
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
                token_type: TokenType::TFn,
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
                token_type: TokenType::TFn,
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
                token_type: TokenType::TLabel("fnx".to_string()),
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
    fn test_read_tokens_parens() {
        assert_eq!(
            read_tokens(TaggedIter::new("(){}".to_string())),
            Ok(vec![
                Token {
                    token_type: TokenType::TOpenParen,
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
                    token_type: TokenType::TCloseParen,
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
                    token_type: TokenType::TOpenCurly,
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
                    token_type: TokenType::TCloseCurly,
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
                }
            ])
        );
    }
}
