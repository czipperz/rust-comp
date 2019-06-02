mod tagged_iter;
#[cfg(test)]
pub use self::tagged_iter::lines;
use self::tagged_iter::TaggedIter;

use crate::pos::*;
use crate::token::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenizerError {}

pub fn read_tokens<'a>(contents: &[String]) -> Result<(Vec<Token>, Pos), TokenizerError> {
    let mut tagged_iter = TaggedIter::new(contents);
    let mut tokens = Vec::new();
    let mut span = Span {
        start: tagged_iter.pos,
        end: tagged_iter.pos,
    };

    loop {
        span.end = tagged_iter.pos;
        match tagged_iter.next() {
            None => {
                flush_temp(&mut tokens, tagged_iter.contents, span);
                break;
            }
            Some(ch) if ch.is_whitespace() => {
                flush_temp(&mut tokens, tagged_iter.contents, span);
                span.start = tagged_iter.pos;
            }
            Some(ch) if "(){}:;-=>".contains(ch) => {
                // There are two cases here: we are parsing a label that is
                // terminated by a symbol, or we are parsing a symbol.  If start
                // == pos then the length before the symbol is 0 so we are
                // parsing a symbol
                if span.start == span.end {
                    span.end = tagged_iter.pos;
                }
                if "-=".contains(ch) {
                    if tagged_iter.peek() == Some('>') {
                        tagged_iter.next();
                        span.end = tagged_iter.pos;
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

fn flush_temp(tokens: &mut Vec<Token>, file_contents: &[String], span: Span) {
    const SYMBOLS: [(&str, TokenValue); 11] = [
        ("fn", TokenValue::Fn),
        ("let", TokenValue::Let),
        ("(", TokenValue::OpenParen),
        (")", TokenValue::CloseParen),
        ("{", TokenValue::OpenCurly),
        ("}", TokenValue::CloseCurly),
        (":", TokenValue::Colon),
        ("->", TokenValue::ThinArrow),
        ("=>", TokenValue::FatArrow),
        ("=", TokenValue::Set),
        (";", TokenValue::Semicolon),
    ];

    if span.start != span.end {
        tokens.push(Token {
            value: if let Some(i) = SYMBOLS.iter().position(|(s, _)| {
                **s == file_contents[span.start.line][span.start.column..span.end.column]
            }) {
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
        assert_eq!(read_tokens(&[]), Ok((vec![], Pos { line: 0, column: 0 })));
    }

    #[test]
    fn test_read_tokens_whitespace_file() {
        assert_eq!(
            read_tokens(&lines("  \n  ")),
            Ok((vec![], Pos { line: 1, column: 2 }))
        );
    }

    #[test]
    fn test_read_tokens_fn_eof() {
        assert_eq!(
            read_tokens(&lines("fn")),
            Ok((
                vec![Token {
                    value: TokenValue::Fn,
                    span: Span::range(Pos::start(), "fn"),
                }],
                Pos { line: 0, column: 2 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_fn_space() {
        assert_eq!(
            read_tokens(&lines("fn ")),
            Ok((
                vec![Token {
                    value: TokenValue::Fn,
                    span: Span::range(Pos::start(), "fn"),
                }],
                Pos { line: 0, column: 3 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_fnx() {
        assert_eq!(
            read_tokens(&lines("fnx")),
            Ok((
                vec![Token {
                    value: TokenValue::Label,
                    span: Span::range(Pos::start(), "fnx"),
                }],
                Pos { line: 0, column: 3 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_let() {
        assert_eq!(
            read_tokens(&lines("let")),
            Ok((
                vec![Token {
                    value: TokenValue::Let,
                    span: Span::range(Pos::start(), "let"),
                }],
                Pos { line: 0, column: 3 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_individual_symbols() {
        assert_eq!(
            read_tokens(&lines("(){};")),
            Ok((
                vec![
                    Token {
                        value: TokenValue::OpenParen,
                        span: Span::range(Pos::start(), "("),
                    },
                    Token {
                        value: TokenValue::CloseParen,
                        span: Span::range(Pos { line: 0, column: 1 }, ")"),
                    },
                    Token {
                        value: TokenValue::OpenCurly,
                        span: Span::range(Pos { line: 0, column: 2 }, "{"),
                    },
                    Token {
                        value: TokenValue::CloseCurly,
                        span: Span::range(Pos { line: 0, column: 3 }, "}",),
                    },
                    Token {
                        value: TokenValue::Semicolon,
                        span: Span::range(Pos { line: 0, column: 4 }, ";")
                    },
                ],
                Pos { line: 0, column: 5 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_set() {
        assert_eq!(
            read_tokens(&lines("a =(b)")),
            Ok((
                vec![
                    Token {
                        value: TokenValue::Label,
                        span: Span::range(Pos::start(), "a"),
                    },
                    Token {
                        value: TokenValue::Set,
                        span: Span::range(Pos { line: 0, column: 2 }, "="),
                    },
                    Token {
                        value: TokenValue::OpenParen,
                        span: Span::range(Pos { line: 0, column: 3 }, "("),
                    },
                    Token {
                        value: TokenValue::Label,
                        span: Span::range(Pos { line: 0, column: 4 }, "b"),
                    },
                    Token {
                        value: TokenValue::CloseParen,
                        span: Span::range(Pos { line: 0, column: 5 }, ")"),
                    },
                ],
                Pos { line: 0, column: 6 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_fat_arrow() {
        assert_eq!(
            read_tokens(&lines("=>")),
            Ok((
                vec![Token {
                    value: TokenValue::FatArrow,
                    span: Span::range(Pos::start(), "=>"),
                }],
                Pos { line: 0, column: 2 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_thin_arrow() {
        assert_eq!(
            read_tokens(&lines("->")),
            Ok((
                vec![Token {
                    value: TokenValue::ThinArrow,
                    span: Span::range(Pos::start(), "->"),
                }],
                Pos { line: 0, column: 2 }
            ))
        );
    }
}
