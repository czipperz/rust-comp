mod tagged_iter;
use self::tagged_iter::TaggedIter;

use crate::pos::*;
use crate::token::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    UnterminatedBlockComment(Pos),
    UnrecognizedControlChar(Pos),
}

pub fn read_tokens<'a>(file: usize, contents: &str) -> Result<(Vec<Token>, Pos), Error> {
    let mut keywords = HashMap::new();
    keywords.insert("!=", TokenKind::NotEquals);
    keywords.insert("&", TokenKind::Ampersand);
    keywords.insert("&&", TokenKind::And);
    keywords.insert("(", TokenKind::OpenParen);
    keywords.insert(")", TokenKind::CloseParen);
    keywords.insert("*", TokenKind::Star);
    keywords.insert("+", TokenKind::Plus);
    keywords.insert(",", TokenKind::Comma);
    keywords.insert("-", TokenKind::Minus);
    keywords.insert("->", TokenKind::ThinArrow);
    keywords.insert(".", TokenKind::Dot);
    keywords.insert("/", TokenKind::ForwardSlash);
    keywords.insert(":", TokenKind::Colon);
    keywords.insert("::", TokenKind::ColonColon);
    keywords.insert(";", TokenKind::Semicolon);
    keywords.insert("=", TokenKind::Set);
    keywords.insert("==", TokenKind::Equals);
    keywords.insert("=>", TokenKind::FatArrow);
    keywords.insert("_", TokenKind::Underscore);
    keywords.insert("const", TokenKind::Const);
    keywords.insert("else", TokenKind::Else);
    keywords.insert("enum", TokenKind::Enum);
    keywords.insert("false", TokenKind::False);
    keywords.insert("fn", TokenKind::Fn);
    keywords.insert("if", TokenKind::If);
    keywords.insert("let", TokenKind::Let);
    keywords.insert("match", TokenKind::Match);
    keywords.insert("mod", TokenKind::Mod);
    keywords.insert("mut", TokenKind::Mut);
    keywords.insert("pub", TokenKind::Pub);
    keywords.insert("struct", TokenKind::Struct);
    keywords.insert("true", TokenKind::True);
    keywords.insert("use", TokenKind::Use);
    keywords.insert("while", TokenKind::While);
    keywords.insert("{", TokenKind::OpenCurly);
    keywords.insert("||", TokenKind::Or);
    keywords.insert("}", TokenKind::CloseCurly);

    let mut tagged_iter = TaggedIter::new(file, contents);
    let mut tokens = Vec::new();
    let mut span = Span {
        file: tagged_iter.pos().file,
        start: tagged_iter.pos().index,
        end: tagged_iter.pos().index,
    };

    loop {
        skip_comments(&keywords, &mut tokens, &mut tagged_iter, &mut span)?;

        span.end = tagged_iter.pos().index;

        match tagged_iter.peek() {
            None => {
                flush_temp(&keywords, &mut tokens, tagged_iter.contents(), span);
                break;
            }

            Some(ch) if ch.is_whitespace() => {
                // end the current token
                flush_temp(&keywords, &mut tokens, tagged_iter.contents(), span);

                // eat all whitespace
                tagged_iter.advance();
                loop {
                    match tagged_iter.peek() {
                        Some(ch) if ch.is_whitespace() => {
                            tagged_iter.advance();
                        }
                        _ => break,
                    }
                }
                span.start = tagged_iter.pos().index;
            }

            Some(ch) if is_symbol(ch) => {
                if span.start != span.end {
                    // there is still a previous token, flush it
                    flush_temp_nonempty(&keywords, &mut tokens, tagged_iter.contents(), span);
                    span.start = span.end;
                }

                // start a new symbol token
                tagged_iter.advance();
                if "-=".contains(ch) {
                    if tagged_iter.peek() == Some('>') {
                        tagged_iter.advance();
                    }
                }
                if "!=".contains(ch) {
                    if tagged_iter.peek() == Some('=') {
                        tagged_iter.advance();
                    }
                }
                if ":&|".contains(ch) && tagged_iter.peek() == Some(ch) {
                    tagged_iter.advance();
                }
                span.end = tagged_iter.pos().index;

                flush_temp_nonempty(&keywords, &mut tokens, tagged_iter.contents(), span);
                span.start = span.end;
            }

            Some(ch) if ch.is_control() => {
                return Err(Error::UnrecognizedControlChar(tagged_iter.pos()))
            }

            Some(_) => tagged_iter.advance(),
        }
    }

    Ok((tokens, tagged_iter.pos()))
}

fn is_symbol(ch: char) -> bool {
    let symbols = "!&()*+,-./:;=>{|}";
    ch.is_ascii() && symbols.as_bytes().binary_search(&(ch as u8)).is_ok()
}

fn skip_comments(
    keywords: &HashMap<&str, TokenKind>,
    tokens: &mut Vec<Token>,
    tagged_iter: &mut TaggedIter,
    span: &mut Span,
) -> Result<(), Error> {
    loop {
        if tagged_iter.peek() == Some('/') && tagged_iter.peek2() == Some('/') {
            span.end = tagged_iter.pos().index;
            flush_temp(keywords, tokens, tagged_iter.contents(), *span);

            tagged_iter.advance();
            tagged_iter.advance();
            while tagged_iter.peek().is_some() && tagged_iter.peek() != Some('\n') {
                tagged_iter.advance();
            }
            tagged_iter.advance();

            span.start = tagged_iter.pos().index;
            continue;
        }

        if tagged_iter.peek() == Some('/') && tagged_iter.peek2() == Some('*') {
            span.end = tagged_iter.pos().index;
            flush_temp(keywords, tokens, tagged_iter.contents(), *span);

            skip_block_comment(tagged_iter)?;

            span.start = tagged_iter.pos().index;
            continue;
        }
        break;
    }

    Ok(())
}

fn skip_block_comment(tagged_iter: &mut TaggedIter) -> Result<(), Error> {
    let pos = tagged_iter.pos();
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

fn flush_temp(
    keywords: &HashMap<&str, TokenKind>,
    tokens: &mut Vec<Token>,
    file_contents: &str,
    span: Span,
) {
    if span.start != span.end {
        flush_temp_nonempty(&keywords, tokens, file_contents, span)
    }
}

fn flush_temp_nonempty(
    keywords: &HashMap<&str, TokenKind>,
    tokens: &mut Vec<Token>,
    file_contents: &str,
    span: Span,
) {
    tokens.push(Token {
        kind: if let Some(kind) = keywords.get(&file_contents[span]) {
            *kind
        } else {
            TokenKind::Label
        },
        span,
    });
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
    fn test_read_tokens_const() {
        assert_eq!(
            read_tokens(0, "const"),
            Ok((
                vec![Token {
                    kind: TokenKind::Const,
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
    fn test_read_tokens_else() {
        assert_eq!(
            read_tokens(0, "else"),
            Ok((
                vec![Token {
                    kind: TokenKind::Else,
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
    fn test_read_tokens_enum() {
        assert_eq!(
            read_tokens(0, "enum"),
            Ok((
                vec![Token {
                    kind: TokenKind::Enum,
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
    fn test_read_tokens_false() {
        assert_eq!(
            read_tokens(0, "false"),
            Ok((
                vec![Token {
                    kind: TokenKind::False,
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
    fn test_read_tokens_fn_eof() {
        assert_eq!(
            read_tokens(0, "fn"),
            Ok((
                vec![Token {
                    kind: TokenKind::Fn,
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
                    kind: TokenKind::Fn,
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
                    kind: TokenKind::Label,
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
                    kind: TokenKind::If,
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
                    kind: TokenKind::Let,
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
    fn test_read_tokens_match() {
        assert_eq!(
            read_tokens(0, "match"),
            Ok((
                vec![Token {
                    kind: TokenKind::Match,
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
    fn test_read_tokens_mod() {
        assert_eq!(
            read_tokens(0, "mod"),
            Ok((
                vec![Token {
                    kind: TokenKind::Mod,
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
    fn test_read_tokens_mut() {
        assert_eq!(
            read_tokens(0, "mut"),
            Ok((
                vec![Token {
                    kind: TokenKind::Mut,
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
    fn test_read_tokens_pub() {
        assert_eq!(
            read_tokens(0, "pub"),
            Ok((
                vec![Token {
                    kind: TokenKind::Pub,
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
    fn test_read_tokens_true() {
        assert_eq!(
            read_tokens(0, "true"),
            Ok((
                vec![Token {
                    kind: TokenKind::True,
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
    fn test_read_tokens_use() {
        assert_eq!(
            read_tokens(0, "use"),
            Ok((
                vec![Token {
                    kind: TokenKind::Use,
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
                    kind: TokenKind::While,
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
    fn test_read_tokens_struct() {
        assert_eq!(
            read_tokens(0, "struct"),
            Ok((
                vec![Token {
                    kind: TokenKind::Struct,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 6
                    },
                }],
                Pos { file: 0, index: 6 }
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
                        kind: TokenKind::OpenParen,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 1
                        },
                    },
                    Token {
                        kind: TokenKind::CloseParen,
                        span: Span {
                            file: 0,
                            start: 1,
                            end: 2
                        },
                    },
                    Token {
                        kind: TokenKind::OpenCurly,
                        span: Span {
                            file: 0,
                            start: 2,
                            end: 3
                        },
                    },
                    Token {
                        kind: TokenKind::CloseCurly,
                        span: Span {
                            file: 0,
                            start: 3,
                            end: 4
                        },
                    },
                    Token {
                        kind: TokenKind::Colon,
                        span: Span {
                            file: 0,
                            start: 4,
                            end: 5
                        },
                    },
                    Token {
                        kind: TokenKind::Comma,
                        span: Span {
                            file: 0,
                            start: 5,
                            end: 6
                        },
                    },
                    Token {
                        kind: TokenKind::Semicolon,
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
                        kind: TokenKind::Set,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 1
                        },
                    },
                    Token {
                        kind: TokenKind::OpenParen,
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
                        kind: TokenKind::Label,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 1
                        },
                    },
                    Token {
                        kind: TokenKind::OpenParen,
                        span: Span {
                            file: 0,
                            start: 1,
                            end: 2
                        },
                    },
                    Token {
                        kind: TokenKind::Label,
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
                    kind: TokenKind::FatArrow,
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
                    kind: TokenKind::ThinArrow,
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
    fn test_read_tokens_colon_colon() {
        assert_eq!(
            read_tokens(0, "::"),
            Ok((
                vec![Token {
                    kind: TokenKind::ColonColon,
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
    fn test_read_tokens_dot() {
        assert_eq!(
            read_tokens(0, "."),
            Ok((
                vec![Token {
                    kind: TokenKind::Dot,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 1,
                    },
                }],
                Pos { file: 0, index: 1 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_plus() {
        assert_eq!(
            read_tokens(0, "+"),
            Ok((
                vec![Token {
                    kind: TokenKind::Plus,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 1,
                    },
                }],
                Pos { file: 0, index: 1 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_minus() {
        assert_eq!(
            read_tokens(0, "-"),
            Ok((
                vec![Token {
                    kind: TokenKind::Minus,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 1,
                    },
                }],
                Pos { file: 0, index: 1 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_star() {
        assert_eq!(
            read_tokens(0, "*"),
            Ok((
                vec![Token {
                    kind: TokenKind::Star,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 1,
                    },
                }],
                Pos { file: 0, index: 1 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_ampersand() {
        assert_eq!(
            read_tokens(0, "&"),
            Ok((
                vec![Token {
                    kind: TokenKind::Ampersand,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 1,
                    },
                }],
                Pos { file: 0, index: 1 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_forward_slash() {
        assert_eq!(
            read_tokens(0, "/"),
            Ok((
                vec![Token {
                    kind: TokenKind::ForwardSlash,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 1,
                    },
                }],
                Pos { file: 0, index: 1 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_equals() {
        assert_eq!(
            read_tokens(0, "=="),
            Ok((
                vec![Token {
                    kind: TokenKind::Equals,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 2,
                    },
                }],
                Pos { file: 0, index: 2 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_not_equals() {
        assert_eq!(
            read_tokens(0, "!="),
            Ok((
                vec![Token {
                    kind: TokenKind::NotEquals,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 2,
                    },
                }],
                Pos { file: 0, index: 2 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_and() {
        assert_eq!(
            read_tokens(0, "&&"),
            Ok((
                vec![Token {
                    kind: TokenKind::And,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 2,
                    },
                }],
                Pos { file: 0, index: 2 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_or() {
        assert_eq!(
            read_tokens(0, "||"),
            Ok((
                vec![Token {
                    kind: TokenKind::Or,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 2,
                    },
                }],
                Pos { file: 0, index: 2 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_underscore() {
        assert_eq!(
            read_tokens(0, "_"),
            Ok((
                vec![Token {
                    kind: TokenKind::Underscore,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 1,
                    },
                }],
                Pos { file: 0, index: 1 }
            ))
        );
    }

    #[test]
    fn test_read_tokens_underscore_letter_is_label() {
        assert_eq!(
            read_tokens(0, "_a"),
            Ok((
                vec![Token {
                    kind: TokenKind::Label,
                    span: Span {
                        file: 0,
                        start: 0,
                        end: 2,
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
                        kind: TokenKind::Let,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 3
                        },
                    },
                    Token {
                        kind: TokenKind::Label,
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
                        kind: TokenKind::Let,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 3
                        },
                    },
                    Token {
                        kind: TokenKind::Label,
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
                        kind: TokenKind::Let,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 3
                        },
                    },
                    Token {
                        kind: TokenKind::Label,
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
                        kind: TokenKind::Let,
                        span: Span {
                            file: 0,
                            start: 0,
                            end: 3
                        },
                    },
                    Token {
                        kind: TokenKind::Label,
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

    #[test]
    fn test_read_tokens_unrecognized_control_char() {
        assert_eq!(
            read_tokens(0, "\u{0002}"),
            Err(Error::UnrecognizedControlChar(Pos { file: 0, index: 0 }))
        );
    }
}
