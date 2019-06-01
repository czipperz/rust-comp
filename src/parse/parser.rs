use super::Error;
use crate::pos::*;
use crate::token::*;

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
        Span::range(self.eofpos, " ")
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
    use crate::lex::read_tokens;

    #[test]
    fn test_span_in_bounds() {
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let parser = Parser::new(contents, &tokens, eofpos);
        assert_eq!(parser.span(), Span::range(Pos::start(), "fn"));
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
        let eofpos = Pos {
            line: 1,
            column: 1,
            index: 2,
        };
        let parser = Parser::new("", &[], eofpos);
        assert_eq!(parser.eof(), Span::range(eofpos, " "));
    }

    #[test]
    fn test_expect_label_out_of_bounds() {
        let mut parser = Parser::new("", &[], Pos::start());
        assert!(parser.expect_label().is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_label_matches() {
        let contents = "abc";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        assert_eq!(parser.expect_label().unwrap(), "abc");
        assert_eq!(parser.index, 1);
    }

    #[test]
    fn test_expect_label_no_match() {
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
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
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        assert!(parser.expect_token(TokenValue::Fn).is_ok());
        assert_eq!(parser.index, 1);
    }

    #[test]
    fn test_expect_token_no_match() {
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        assert!(parser.expect_token(TokenValue::OpenParen).is_err());
        assert_eq!(parser.index, 0);
    }
}
