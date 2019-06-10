use super::Error;
use crate::pos::*;
use crate::token::*;

pub struct Parser<'a, 't> {
    file_contents: &'a str,
    tokens: &'t [Token],
    eofpos: Pos,
    pub index: usize,
}

impl<'a, 't> Parser<'a, 't> {
    pub fn new(file_contents: &'a str, tokens: &'t [Token], eofpos: Pos) -> Self {
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
        Span {
            file: self.eofpos.file,
            start: self.eofpos.index,
            end: self.eofpos.index + 1,
        }
    }

    pub fn expect_label(&mut self) -> Result<&'a str, Error> {
        self.expect_token(TokenKind::Label)?;
        let span = self.tokens[self.index - 1].span;
        Ok(&self.file_contents[span])
    }

    pub fn expect_token(&mut self, expected: TokenKind) -> Result<(), Error> {
        if self.index < self.tokens.len() && self.tokens[self.index].kind == expected {
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

    pub fn peek(&self) -> Option<TokenKind> {
        self.tokens.get(self.index).map(|t| t.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::read_tokens;

    #[test]
    fn test_span_in_bounds() {
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let parser = Parser::new(contents, &tokens, eofpos);
        assert_eq!(
            parser.span(),
            Span {
                file: 0,
                start: 0,
                end: 2
            }
        );
    }

    #[test]
    fn test_span_out_of_bounds() {
        let contents = "  ";
        let parser = Parser::new(contents, &[], Pos { file: 0, index: 2 });
        assert_eq!(parser.span(), parser.eof());
    }

    #[test]
    fn test_eof() {
        let eofpos = Pos { file: 0, index: 3 };
        let contents = " \n ";
        let parser = Parser::new(contents, &[], eofpos);
        assert_eq!(
            parser.eof(),
            Span {
                file: eofpos.file,
                start: eofpos.index,
                end: eofpos.index + 1
            }
        );
    }

    #[test]
    fn test_expect_label_out_of_bounds() {
        let contents = "";
        let mut parser = Parser::new(contents, &[], Pos { file: 0, index: 0 });
        assert!(parser.expect_label().is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_label_matches() {
        let contents = "abc";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        assert_eq!(parser.expect_label().unwrap(), "abc");
        assert_eq!(parser.index, tokens.len());
    }

    #[test]
    fn test_expect_label_no_match() {
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        assert!(parser.expect_label().is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_token_out_of_bounds() {
        let contents = "";
        let mut parser = Parser::new(contents, &[], Pos { file: 0, index: 0 });
        assert!(parser.expect_token(TokenKind::Fn).is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_token_matches() {
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        assert!(parser.expect_token(TokenKind::Fn).is_ok());
        assert_eq!(parser.index, tokens.len());
    }

    #[test]
    fn test_expect_token_no_match() {
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        assert!(parser.expect_token(TokenKind::OpenParen).is_err());
        assert_eq!(parser.index, 0);
    }
}
