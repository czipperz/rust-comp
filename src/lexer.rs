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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenizerError {}

pub fn read_tokens(mut tagged_iter: TaggedIter) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens = Vec::new();
    while !tagged_iter.eof() {
        if tagged_iter.looking_at("fn") {
            let start = tagged_iter.pos();
            tagged_iter.nth("fn".len());
            let end = tagged_iter.pos();
            tokens.push(Token {
                token_type: TokenType::TFn,
                span: Span { start, end },
            });
        } else {
            tagged_iter.next().unwrap();
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
    fn test_read_tokens_fn_token() {
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
}
