use super::Error;
use crate::arena::Allocator;
use crate::pos::*;
use crate::token::*;

pub struct Parser<'a> {
    file_contents: &'a str,
    tokens: &'a [Token],
    eofpos: Pos,
    pub index: usize,
    allocator: Allocator<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(
        file_contents: &'a str,
        tokens: &'a [Token],
        eofpos: Pos,
        allocator: Allocator<'a>,
    ) -> Self {
        Parser {
            file_contents,
            tokens,
            eofpos,
            index: 0,
            allocator,
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
        self.expect_token(TokenValue::Label)?;
        let span = self.tokens[self.index - 1].span;
        Ok(&self.file_contents[span])
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

    pub fn alloc<T>(&self, t: T) -> &'a T {
        &*self.allocator.alloc(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena::Arena;
    use crate::lex::read_tokens;

    #[test]
    fn test_span_in_bounds() {
        let mut arena = Arena::new();
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
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
        let mut arena = Arena::new();
        let contents = "  ";
        let parser = Parser::new(contents, &[], Pos { file: 0, index: 2 }, arena.allocator());
        assert_eq!(parser.span(), parser.eof());
    }

    #[test]
    fn test_eof() {
        let mut arena = Arena::new();
        let eofpos = Pos { file: 0, index: 3 };
        let contents = " \n ";
        let parser = Parser::new(contents, &[], eofpos, arena.allocator());
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
        let mut arena = Arena::new();
        let contents = "";
        let mut parser = Parser::new(contents, &[], Pos { file: 0, index: 0 }, arena.allocator());
        assert!(parser.expect_label().is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_label_matches() {
        let mut arena = Arena::new();
        let contents = "abc";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        assert_eq!(parser.expect_label().unwrap(), "abc");
        assert_eq!(parser.index, tokens.len());
    }

    #[test]
    fn test_expect_label_no_match() {
        let mut arena = Arena::new();
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        assert!(parser.expect_label().is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_token_out_of_bounds() {
        let mut arena = Arena::new();
        let contents = "";
        let mut parser = Parser::new(contents, &[], Pos { file: 0, index: 0 }, arena.allocator());
        assert!(parser.expect_token(TokenValue::Fn).is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_token_matches() {
        let mut arena = Arena::new();
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        assert!(parser.expect_token(TokenValue::Fn).is_ok());
        assert_eq!(parser.index, tokens.len());
    }

    #[test]
    fn test_expect_token_no_match() {
        let mut arena = Arena::new();
        let contents = "fn";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        assert!(parser.expect_token(TokenValue::OpenParen).is_err());
        assert_eq!(parser.index, 0);
    }
}
