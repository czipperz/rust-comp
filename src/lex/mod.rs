mod tagged_iter;
use self::tagged_iter::TaggedIter;

use crate::pos::*;
use crate::token::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenizerError {}

pub fn read_tokens(contents: &str) -> Result<(Vec<Token>, Pos), TokenizerError> {
    let mut tagged_iter = TaggedIter::new(contents);
    let mut tokens = Vec::new();
    let mut start = tagged_iter.pos;

    loop {
        let pos = tagged_iter.pos;
        let ch = tagged_iter.next();

        if ch.is_none() || ch.unwrap().is_whitespace() {
            flush_temp(&mut tokens, tagged_iter.contents, Span { start, end: pos });

            if ch.is_none() {
                break;
            }
            start = tagged_iter.pos;
        } else {
            let ch = ch.unwrap();
            if "(){}=;".contains(ch) {
                flush_temp(&mut tokens, tagged_iter.contents, Span { start, end: pos });
                start = pos;
            }
        }
    }

    Ok((tokens, tagged_iter.pos))
}

fn flush_temp(tokens: &mut Vec<Token>, file_contents: &str, span: Span) {
    const SYMBOLS: [(&str, TokenValue); 8] = [
        ("fn", TokenValue::Fn),
        ("let", TokenValue::Let),
        ("(", TokenValue::OpenParen),
        (")", TokenValue::CloseParen),
        ("{", TokenValue::OpenCurly),
        ("}", TokenValue::CloseCurly),
        ("=", TokenValue::Set),
        (";", TokenValue::Semicolon),
    ];
    if span.start != span.end {
        tokens.push(Token {
            value: if let Some(i) = SYMBOLS.iter().position(|(s, _)| **s == file_contents[span]) {
                SYMBOLS[i].1.clone()
            } else {
                TokenValue::Label
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
        assert_eq!(
            read_tokens(""),
            Ok((
                vec![],
                Pos {
                    line: 0,
                    column: 0,
                    index: 0,
                }
            ))
        );
    }

    #[test]
    fn test_read_tokens_whitespace_file() {
        assert_eq!(
            read_tokens("  \n  "),
            Ok((
                vec![],
                Pos {
                    line: 1,
                    column: 2,
                    index: 5,
                }
            ))
        );
    }

    #[test]
    fn test_read_tokens_fn_eof() {
        assert_eq!(
            read_tokens("fn"),
            Ok((
                vec![Token {
                    value: TokenValue::Fn,
                    span: Span::range(Pos::start(), "fn"),
                }],
                Pos {
                    line: 0,
                    column: 2,
                    index: 2,
                }
            ))
        );
    }

    #[test]
    fn test_read_tokens_fn_space() {
        assert_eq!(
            read_tokens("fn "),
            Ok((
                vec![Token {
                    value: TokenValue::Fn,
                    span: Span::range(Pos::start(), "fn"),
                }],
                Pos {
                    line: 0,
                    column: 3,
                    index: 3,
                }
            ))
        );
    }

    #[test]
    fn test_read_tokens_fnx() {
        assert_eq!(
            read_tokens("fnx"),
            Ok((
                vec![Token {
                    value: TokenValue::Label,
                    span: Span::range(Pos::start(), "fnx"),
                }],
                Pos {
                    line: 0,
                    column: 3,
                    index: 3,
                }
            ))
        );
    }

    #[test]
    fn test_read_tokens_let() {
        assert_eq!(
            read_tokens("let"),
            Ok((
                vec![Token {
                    value: TokenValue::Let,
                    span: Span::range(Pos::start(), "let"),
                }],
                Pos {
                    line: 0,
                    column: 3,
                    index: 3,
                }
            ))
        );
    }

    #[test]
    fn test_read_tokens_symbols() {
        assert_eq!(
            read_tokens("(){}=;"),
            Ok((
                vec![
                    Token {
                        value: TokenValue::OpenParen,
                        span: Span::range(Pos::start(), "("),
                    },
                    Token {
                        value: TokenValue::CloseParen,
                        span: Span::range(
                            Pos {
                                line: 0,
                                column: 1,
                                index: 1
                            },
                            ")"
                        ),
                    },
                    Token {
                        value: TokenValue::OpenCurly,
                        span: Span::range(
                            Pos {
                                line: 0,
                                column: 2,
                                index: 2
                            },
                            "{"
                        ),
                    },
                    Token {
                        value: TokenValue::CloseCurly,
                        span: Span::range(
                            Pos {
                                line: 0,
                                column: 3,
                                index: 3
                            },
                            "}",
                        ),
                    },
                    Token {
                        value: TokenValue::Set,
                        span: Span::range(
                            Pos {
                                line: 0,
                                column: 4,
                                index: 4
                            },
                            "="
                        )
                    },
                    Token {
                        value: TokenValue::Semicolon,
                        span: Span::range(
                            Pos {
                                line: 0,
                                column: 5,
                                index: 5
                            },
                            ";"
                        )
                    },
                ],
                Pos {
                    line: 0,
                    column: 6,
                    index: 6,
                }
            ))
        );
    }
}
