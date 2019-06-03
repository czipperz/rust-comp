mod tagged_iter;
use self::tagged_iter::TaggedIter;

use crate::pos::*;
use crate::token::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenizerError {}

pub fn read_tokens<'a>(file: usize, contents: &str) -> Result<(Vec<Token>, Pos), TokenizerError> {
    let mut tagged_iter = TaggedIter::new(file, contents);
    let mut tokens = Vec::new();
    let mut span = Span {
        file: tagged_iter.pos.file,
        start: tagged_iter.pos.index,
        end: tagged_iter.pos.index,
    };

    loop {
        span.end = tagged_iter.pos.index;
        match tagged_iter.next() {
            None => {
                flush_temp(&mut tokens, tagged_iter.contents, span);
                break;
            }
            Some(ch) if ch.is_whitespace() => {
                flush_temp(&mut tokens, tagged_iter.contents, span);
                span.start = tagged_iter.pos.index;
            }
            Some(ch) if "(){}:,-=>;".contains(ch) => {
                // There are two cases here: we are parsing a label that is
                // terminated by a symbol, or we are parsing a symbol.  If start
                // == pos then the length before the symbol is 0 so we are
                // parsing a symbol
                if span.start == span.end {
                    span.end = tagged_iter.pos.index;
                }
                if "-=".contains(ch) {
                    if tagged_iter.peek() == Some('>') {
                        tagged_iter.next();
                        span.end = tagged_iter.pos.index;
                    }
                }
                flush_temp(&mut tokens, tagged_iter.contents, span);
                span.start = span.end;
            }
            Some(_) => (),
        }
    }

    Ok((tokens, tagged_iter.pos))
}

fn flush_temp(tokens: &mut Vec<Token>, file_contents: &str, span: Span) {
    const SYMBOLS: [(&str, TokenValue); 14] = [
        ("else", TokenValue::Else),
        ("fn", TokenValue::Fn),
        ("if", TokenValue::If),
        ("let", TokenValue::Let),
        ("(", TokenValue::OpenParen),
        (")", TokenValue::CloseParen),
        ("{", TokenValue::OpenCurly),
        ("}", TokenValue::CloseCurly),
        (":", TokenValue::Colon),
        (",", TokenValue::Comma),
        ("->", TokenValue::ThinArrow),
        ("=>", TokenValue::FatArrow),
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
    fn test_read_tokens_empty_file_number_1() {
        assert_eq!(read_tokens(1, ""), Ok((vec![], Pos { file: 1, index: 0 })));
    }

    #[test]
    fn test_read_tokens_whitespace_file() {
        assert_eq!(
            read_tokens(0, "  \n  "),
            Ok((vec![], Pos { file: 0, index: 5 }))
        );
    }

    #[test]
    fn test_read_tokens_else() {
        assert_eq!(
            read_tokens(0, "else"),
            Ok((
                vec![Token {
                    value: TokenValue::Else,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 4
                    },
                }],
                Pos { file: 0, index: 4 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_fn_eof() {
        assert_eq!(
            read_tokens(0, "fn"),
            Ok((
                vec![Token {
                    value: TokenValue::Fn,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 2
                    },
                }],
                Pos { file: 0, index: 2 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_fn_space() {
        assert_eq!(
            read_tokens(0, "fn "),
            Ok((
                vec![Token {
                    value: TokenValue::Fn,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 2
                    },
                }],
                Pos { file: 0, index: 3 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_fnx() {
        assert_eq!(
            read_tokens(0, "fnx"),
            Ok((
                vec![Token {
                    value: TokenValue::Label,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 3
                    },
                }],
                Pos { file: 0, index: 3 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_if() {
        assert_eq!(
            read_tokens(0, "if"),
            Ok((
                vec![Token {
                    value: TokenValue::If,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 2
                    },
                }],
                Pos { file: 0, index: 2 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_let() {
        assert_eq!(
            read_tokens(0, "let"),
            Ok((
                vec![Token {
                    value: TokenValue::Let,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 3
                    },
                }],
                Pos { file: 0, index: 3 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_individual_symbols() {
        assert_eq!(
            read_tokens(0, "(){}:,;"),
            Ok((
                vec![
                    Token {
                        value: TokenValue::OpenParen,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 1
                        },
                    },
                    Token {
                        value: TokenValue::CloseParen,
                        span: Span {
                            file: 0,
                            start: 1,
                            end: 2
                        },
                    },
                    Token {
                        value: TokenValue::OpenCurly,
                        span: Span {
                            file: 0,
                            start: 2,
                            end: 3
                        },
                    },
                    Token {
                        value: TokenValue::CloseCurly,
                        span: Span {
                            file: 0,
                            start: 3,
                            end: 4
                        },
                    },
                    Token {
                        value: TokenValue::Colon,
                        span: Span {
                            file: 0,
                            start: 4,
                            end: 5
                        },
                    },
                    Token {
                        value: TokenValue::Comma,
                        span: Span {
                            file: 0,
                            start: 5,
                            end: 6
                        },
                    },
                    Token {
                        value: TokenValue::Semicolon,
                        span: Span {
                            file: 0,
                            start: 6,
                            end: 7
                        },
                    },
                ],
                Pos { file: 0, index: 7 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_set_paren() {
        assert_eq!(
            read_tokens(0, "=("),
            Ok((
                vec![
                    Token {
                        value: TokenValue::Set,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 1
                        },
                    },
                    Token {
                        value: TokenValue::OpenParen,
                        span: Span {
                            file: 0,
                            start: 1,
                            end: 2
                        },
                    },
                ],
                Pos { file: 0, index: 2 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_fat_arrow() {
        assert_eq!(
            read_tokens(0, "=>"),
            Ok((
                vec![Token {
                    value: TokenValue::FatArrow,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 2
                    },
                }],
                Pos { file: 0, index: 2 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_thin_arrow() {
        assert_eq!(
            read_tokens(0, "->"),
            Ok((
                vec![Token {
                    value: TokenValue::ThinArrow,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 2
                    },
                }],
                Pos { file: 0, index: 2 }
            ))
        );
    }
}
