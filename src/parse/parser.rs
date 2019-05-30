use super::Error;
use crate::lex::*;

pub struct Parser<'a> {
    tokens: &'a [Token],
    pub index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, index: 0 }
    }

    pub fn expect_label(&mut self) -> Result<&'a str, Error> {
        if self.index >= self.tokens.len() {
            Err(Error::EOF)
        } else {
            match self.tokens[self.index].value {
                TokenValue::Label(ref label) => {
                    self.index += 1;
                    Ok(&label)
                }
                _ => Err(Error::ExpectedToken(
                    TokenValue::Label("".to_string()),
                    self.tokens[self.index].span.start,
                )),
            }
        }
    }

    pub fn expect_token(&mut self, expected: TokenValue) -> Result<(), Error> {
        if self.index >= self.tokens.len() {
            Err(Error::EOF)
        } else if self.tokens[self.index].value == expected {
            self.index += 1;
            Ok(())
        } else {
            Err(Error::ExpectedToken(
                expected,
                self.tokens[self.index].span.start,
            ))
        }
    }

    pub fn many<T, E>(
        &mut self,
        mut f: impl FnMut(&mut Self) -> Result<T, E>,
    ) -> Result<Vec<T>, E> {
        let mut xs = Vec::new();
        loop {
            let old_index = self.index;
            match f(self) {
                Ok(x) => xs.push(x),
                Err(_) if old_index == self.index => return Ok(xs),
                Err(e) => Err(e)?,
            }
        }
    }
}

#[cfg(test)]
pub fn make_tokens(values: Vec<TokenValue>) -> Vec<Token> {
    use crate::pos::*;
    values
        .into_iter()
        .map(|value| Token {
            value,
            span: Span {
                start: Pos::start(),
                end: Pos::start(),
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect_label_out_of_bounds() {
        let mut parser = Parser::new(&[]);
        assert!(parser.expect_label().is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_label_matches() {
        let tokens = make_tokens(vec![TokenValue::Label("abc".to_string())]);
        let mut parser = Parser::new(&tokens);
        assert_eq!(parser.expect_label().unwrap(), "abc");
        assert_eq!(parser.index, 1);
    }

    #[test]
    fn test_expect_label_no_match() {
        let tokens = make_tokens(vec![TokenValue::Fn]);
        let mut parser = Parser::new(&tokens);
        assert!(parser.expect_label().is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_token_out_of_bounds() {
        let mut parser = Parser::new(&[]);
        assert!(parser.expect_token(TokenValue::Fn).is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_token_matches() {
        let tokens = make_tokens(vec![TokenValue::Fn]);
        let mut parser = Parser::new(&tokens);
        assert!(parser.expect_token(TokenValue::Fn).is_ok());
        assert_eq!(parser.index, 1);
    }

    #[test]
    fn test_expect_token_no_match() {
        let tokens = make_tokens(vec![TokenValue::Fn]);
        let mut parser = Parser::new(&tokens);
        assert!(parser.expect_token(TokenValue::OpenParen).is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_many_ok_no_move_then_err_no_move() {
        let mut first = true;
        assert_eq!(
            Parser::new(&[]).many(|_| if first {
                first = false;
                Ok(())
            } else {
                Err(())
            }),
            Ok(vec![()])
        );
    }

    #[test]
    fn test_many_ok_move_then_err_move() {
        let mut first = true;
        assert_eq!(
            Parser::new(&[]).many(|parser| {
                parser.index += 1;
                if first {
                    first = false;
                    Ok(())
                } else {
                    Err(())
                }
            }),
            Err(())
        );
    }

    #[test]
    fn test_many_ok_move_then_err_no_move() {
        let mut first = true;
        assert_eq!(
            Parser::new(&[]).many(|parser| {
                if first {
                    first = false;
                    parser.index += 1;
                    Ok(())
                } else {
                    Err(())
                }
            }),
            Ok(vec![()])
        );
    }
}
