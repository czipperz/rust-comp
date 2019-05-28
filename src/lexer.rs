use crate::pos::*;
use crate::tagged_iter::TaggedIter;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    token_type: TokenType,
    span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    TFn,
    TLabel(String),
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
            if !temp.is_empty() {
                tokens.push(Token {
                    token_type: if temp == "fn" {
                        TokenType::TFn
                    } else {
                        TokenType::TLabel(temp.clone())
                    },
                    span: Span { start, end: pos },
                });
                temp.clear();
            }

            if ch.is_none() {
                break;
            }
            start = tagged_iter.pos();
        } else {
            let ch = ch.unwrap();
            temp.push(ch);
        }
    }
    Ok(tokens)
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
            read_tokens(TaggedIter::new("".to_string(), "file".to_string())),
            Ok(vec![])
        );
    }

    #[test]
    fn test_read_tokens_whitespace_file() {
        assert_eq!(
            read_tokens(TaggedIter::new("  ".to_string(), "file".to_string())),
            Ok(vec![])
        );
    }

    #[test]
    fn test_read_tokens_fn_eof() {
        assert_eq!(
            read_tokens(TaggedIter::new("fn".to_string(), "file".to_string())),
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
            read_tokens(TaggedIter::new("fn ".to_string(), "file".to_string())),
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
            read_tokens(TaggedIter::new("fnx".to_string(), "file".to_string())),
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
}
