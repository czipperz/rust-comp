use super::Error;
use crate::token::*;
use crate::pos::*;

pub struct Parser<'a> {
    file_contents: &'a str,
    tokens: &'a [Token],
    eofpos: Pos,
    pub index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(file_contents: &'a str, tokens: &'a [Token], eofpos: Pos) -> Self {
        Parser {
            file_contents,
            tokens,
            eofpos,
            index: 0,
        }
    }

    pub fn span(&self) -> Span {
        if self.index < self.tokens.len() {
            self.tokens[self.index].span
        } else {
            self.eof()
        }
    }

    pub fn eof(&self) -> Span {
        let mut end = self.eofpos;
        end.increment(' ');
        Span {
            start: self.eofpos,
            end,
        }
    }

    pub fn expect_label(&mut self) -> Result<&'a str, Error> {
        self.expect_token(TokenValue::Label)?;
        Ok(&self.file_contents[self.tokens[self.index - 1].span])
    }

    pub fn expect_token(&mut self, expected: TokenValue) -> Result<(), Error> {
        if self.index < self.tokens.len() && self.tokens[self.index].value == expected {
            self.index += 1;
            Ok(())
        } else {
            Err(Error::ExpectedToken(
                expected,
                if self.index < self.tokens.len() {
                    self.tokens[self.index].span
                } else {
                    self.eof()
                },
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_in_bounds() {
        let tokens = [Token {
            value: TokenValue::Fn,
            span: Span {
                start: Pos::start(),
                end: Pos {
                    line: 0,
                    column: 2,
                    index: 2,
                },
            },
        }];
        let parser = Parser::new(
            "fn",
            &tokens,
            Pos {
                line: 0,
                column: 2,
                index: 2,
            },
        );
        assert_eq!(
            parser.span(),
            Span {
                start: Pos {
                    line: 0,
                    column: 0,
                    index: 0,
                },
                end: Pos {
                    line: 0,
                    column: 2,
                    index: 2,
                }
            }
        );
    }

    #[test]
    fn test_span_out_of_bounds() {
        let parser = Parser::new(
            "  ",
            &[],
            Pos {
                line: 0,
                column: 2,
                index: 2,
            },
        );
        assert_eq!(parser.span(), parser.eof());
    }

    #[test]
    fn test_eof() {
        let parser = Parser::new(
            "",
            &[],
            Pos {
                line: 1,
                column: 1,
                index: 2,
            },
        );
        assert_eq!(
            parser.eof(),
            Span {
                start: Pos {
                    line: 1,
                    column: 1,
                    index: 2,
                },
                end: Pos {
                    line: 1,
                    column: 2,
                    index: 3,
                }
            }
        );
    }

    #[test]
    fn test_expect_label_out_of_bounds() {
        let mut parser = Parser::new("", &[], Pos::start());
        assert!(parser.expect_label().is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_label_matches() {
        use crate::pos::*;
        let tokens = [Token {
            value: TokenValue::Label,
            span: Span {
                start: Pos {
                    line: 0,
                    column: 0,
                    index: 0,
                },
                end: Pos {
                    line: 0,
                    column: 3,
                    index: 3,
                },
            },
        }];
        let mut parser = Parser::new("abc", &tokens, Pos::start());
        assert_eq!(parser.expect_label().unwrap(), "abc");
        assert_eq!(parser.index, 1);
    }

    #[test]
    fn test_expect_label_no_match() {
        let tokens = make_tokens(vec![TokenValue::Fn]);
        let mut parser = Parser::new("fn", &tokens, Pos::start());
        assert!(parser.expect_label().is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_token_out_of_bounds() {
        let mut parser = Parser::new("", &[], Pos::start());
        assert!(parser.expect_token(TokenValue::Fn).is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_token_matches() {
        let tokens = make_tokens(vec![TokenValue::Fn]);
        let mut parser = Parser::new("fn", &tokens, Pos::start());
        assert!(parser.expect_token(TokenValue::Fn).is_ok());
        assert_eq!(parser.index, 1);
    }

    #[test]
    fn test_expect_token_no_match() {
        let tokens = make_tokens(vec![TokenValue::Fn]);
        let mut parser = Parser::new("", &tokens, Pos::start());
        assert!(parser.expect_token(TokenValue::OpenParen).is_err());
        assert_eq!(parser.index, 0);
    }
}
