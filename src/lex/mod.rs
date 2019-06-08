mod tagged_iter;
use self::tagged_iter::TaggedIter;

use crate::pos::*;
use crate::token::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    UnterminatedBlockComment(Pos),
}

pub fn read_tokens<'a>(file: usize, contents: &str) -> Result<(Vec<Token>, Pos), Error> {
    let mut tagged_iter = TaggedIter::new(file, contents);
    let mut tokens = Vec::new();
    let mut span = Span {
        file: tagged_iter.pos.file,
        start: tagged_iter.pos.index,
        end: tagged_iter.pos.index,
    };

    loop {
        skip_comments(&mut tokens, &mut tagged_iter, &mut span)?;

        span.end = tagged_iter.pos.index;

        match tagged_iter.peek() {
            None => {
                flush_temp(&mut tokens, tagged_iter.contents, span);
                break;
            }
            Some(ch) if ch.is_whitespace() => {
                // end the current token
                tagged_iter.advance();
                flush_temp(&mut tokens, tagged_iter.contents, span);
                span.start = tagged_iter.pos.index;
            }
            Some(ch) if is_symbol(ch) => {
                if span.start == span.end {
                    // start a new symbol token
                    tagged_iter.advance();
                    span.end = tagged_iter.pos.index;
                    if "-=".contains(ch) {
                        if tagged_iter.peek() == Some('>') {
                            tagged_iter.advance();
                            span.end = tagged_iter.pos.index;
                        }
                    }
                }
                flush_temp(&mut tokens, tagged_iter.contents, span);
                span.start = span.end;
            }
            Some(_) => tagged_iter.advance(),
        }
    }

    Ok((tokens, tagged_iter.pos))
}

fn is_symbol(ch: char) -> bool {
    let symbols = "(),-:;=>{}";
    ch.is_ascii() && symbols.as_bytes().binary_search(&(ch as u8)).is_ok()
}

fn skip_comments(
    tokens: &mut Vec<Token>,
    tagged_iter: &mut TaggedIter,
    span: &mut Span,
) -> Result<(), Error> {
    loop {
        if tagged_iter.peek() == Some('/') && tagged_iter.peek2() == Some('/') {
            span.end = tagged_iter.pos.index;
            flush_temp(tokens, tagged_iter.contents, *span);

            tagged_iter.advance();
            tagged_iter.advance();
            while tagged_iter.peek().is_some() && tagged_iter.peek() != Some('\n') {
                tagged_iter.advance();
            }
            tagged_iter.advance();

            span.start = tagged_iter.pos.index;
            continue;
        }

        if tagged_iter.peek() == Some('/') && tagged_iter.peek2() == Some('*') {
            span.end = tagged_iter.pos.index;
            flush_temp(tokens, tagged_iter.contents, *span);

            skip_block_comment(tagged_iter)?;

            span.start = tagged_iter.pos.index;
            continue;
        }
        break;
    }

    Ok(())
}

fn skip_block_comment(tagged_iter: &mut TaggedIter) -> Result<(), Error> {
    let pos = tagged_iter.pos;
    tagged_iter.advance();
    tagged_iter.advance();

    while tagged_iter.peek2().is_some()
        && !(tagged_iter.peek() == Some('*') && tagged_iter.peek2() == Some('/'))
    {
        if tagged_iter.peek() == Some('/') && tagged_iter.peek2() == Some('*') {
            skip_block_comment(tagged_iter)?;

            // if recursive comment is ends at eof this will trigger
            if tagged_iter.peek() == None {
                return Err(Error::UnterminatedBlockComment(pos));
            }
        }

        tagged_iter.advance();
    }

    if tagged_iter.peek2() == None {
        return Err(Error::UnterminatedBlockComment(pos));
    }
    tagged_iter.advance();
    tagged_iter.advance();
    Ok(())
}

fn flush_temp(tokens: &mut Vec<Token>, file_contents: &str, span: Span) {
    const SYMBOLS: [(&str, TokenValue); 16] = [
        ("else", TokenValue::Else),
        ("fn", TokenValue::Fn),
        ("if", TokenValue::If),
        ("let", TokenValue::Let),
        ("mod", TokenValue::Mod),
        ("while", TokenValue::While),
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

impl fmt::Display for Error {
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
    fn test_read_tokens_mod() {
        assert_eq!(
            read_tokens(0, "mod"),
            Ok((
                vec![Token {
                    value: TokenValue::Mod,
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
    fn test_read_tokens_while() {
        assert_eq!(
            read_tokens(0, "while"),
            Ok((
                vec![Token {
                    value: TokenValue::While,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 5
                    },
                }],
                Pos { file: 0, index: 5 }
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
    fn test_read_tokens_label_paren_label() {
        assert_eq!(
            read_tokens(0, "f(x"),
            Ok((
                vec![
                    Token {
                        value: TokenValue::Label,
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
                    Token {
                        value: TokenValue::Label,
                        span: Span {
                            file: 0,
                            start: 2,
                            end: 3
                        },
                    },
                ],
                Pos { file: 0, index: 3 }
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

    #[test]
    fn test_read_tokens_ignore_line_comment() {
        assert_eq!(
            read_tokens(0, "let// abcd \nx"),
            Ok((
                vec![
                    Token {
                        value: TokenValue::Let,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 3
                        },
                    },
                    Token {
                        value: TokenValue::Label,
                        span: Span {
                            file: 0,
                            start: 12,
                            end: 13
                        },
                    }
                ],
                Pos { file: 0, index: 13 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_ignore_block_comment() {
        assert_eq!(
            read_tokens(0, "let/* abc */x"),
            Ok((
                vec![
                    Token {
                        value: TokenValue::Let,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 3
                        },
                    },
                    Token {
                        value: TokenValue::Label,
                        span: Span {
                            file: 0,
                            start: 12,
                            end: 13
                        },
                    }
                ],
                Pos { file: 0, index: 13 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_recursive_block_comment() {
        assert_eq!(
            read_tokens(0, "let/* /* abc */ */x"),
            Ok((
                vec![
                    Token {
                        value: TokenValue::Let,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 3
                        },
                    },
                    Token {
                        value: TokenValue::Label,
                        span: Span {
                            file: 0,
                            start: 18,
                            end: 19
                        },
                    }
                ],
                Pos { file: 0, index: 19 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_sequential_line_comments() {
        assert_eq!(
            read_tokens(0, "let//abc\n//def\ndef"),
            Ok((
                vec![
                    Token {
                        value: TokenValue::Let,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 3
                        },
                    },
                    Token {
                        value: TokenValue::Label,
                        span: Span {
                            file: 0,
                            start: 15,
                            end: 18
                        },
                    }
                ],
                Pos { file: 0, index: 18 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_unterminated_block_comment_with_recursive_at_eof() {
        assert_eq!(
            read_tokens(0, "let /* abc \n def /* */"),
            Err(Error::UnterminatedBlockComment(Pos { file: 0, index: 4 })),
        );
    }

    #[test]
    fn test_read_tokens_unterminated_block_comment_with_whitespace_eof() {
        assert_eq!(
            read_tokens(0, "let /* abc \n def /* */ "),
            Err(Error::UnterminatedBlockComment(Pos { file: 0, index: 4 })),
        );
    }
}
