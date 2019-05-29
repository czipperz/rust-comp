use super::Error;
use crate::lex::*;

pub fn expect_label<'a>(tokens: &'a [Token], index: &mut usize) -> Result<&'a str, Error> {
    if *index >= tokens.len() {
        Err(Error::EOF)
    } else {
        match tokens[*index].token_type {
            TokenType::Label(ref label) => {
                *index += 1;
                Ok(&label)
            }
            _ => Err(Error::ExpectedToken(
                TokenType::Label("".to_string()),
                tokens[*index].span.start,
            )),
        }
    }
}

pub fn expect_token(tokens: &[Token], index: &mut usize, expected: TokenType) -> Result<(), Error> {
    if *index >= tokens.len() {
        Err(Error::EOF)
    } else if tokens[*index].token_type == expected {
        *index += 1;
        Ok(())
    } else {
        Err(Error::ExpectedToken(expected, tokens[*index].span.start))
    }
}

pub fn many<T, E>(
    mut f: impl FnMut(&[Token], &mut usize) -> Result<T, E>,
    tokens: &[Token],
    index: &mut usize,
) -> Result<Vec<T>, E> {
    let mut xs = Vec::new();
    loop {
        let old_index = *index;
        match f(tokens, index) {
            Ok(x) => xs.push(x),
            Err(_) if old_index == *index => return Ok(xs),
            Err(e) => Err(e)?,
        }
    }
}

#[cfg(test)]
pub fn make_token(token_type: TokenType) -> Token {
    use crate::pos::*;
    Token {
        token_type,
        span: Span {
            start: Pos::start(),
            end: Pos::start(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect_label_out_of_bounds() {
        let mut index = 0;
        assert!(expect_label(&[], &mut index).is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_expect_label_matches() {
        let mut index = 0;
        assert_eq!(
            expect_label(
                &[make_token(TokenType::Label("abc".to_string()))],
                &mut index
            )
            .unwrap(),
            "abc"
        );
        assert_eq!(index, 1);
    }

    #[test]
    fn test_expect_label_no_match() {
        let mut index = 0;
        assert!(expect_label(&[make_token(TokenType::Fn)], &mut index).is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_expect_token_out_of_bounds() {
        let mut index = 0;
        assert!(expect_token(&[], &mut index, TokenType::Fn).is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_expect_token_matches() {
        let mut index = 0;
        assert!(expect_token(&[make_token(TokenType::Fn)], &mut index, TokenType::Fn).is_ok());
        assert_eq!(index, 1);
    }

    #[test]
    fn test_expect_token_no_match() {
        let mut index = 0;
        assert!(expect_token(
            &[make_token(TokenType::Fn)],
            &mut index,
            TokenType::OpenParen
        )
        .is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_many_ok_no_move_then_err_no_move() {
        let mut first = true;
        assert_eq!(
            many(
                |_, _| if first {
                    first = false;
                    Ok(())
                } else {
                    Err(())
                },
                &[],
                &mut 0
            ),
            Ok(vec![()])
        );
    }

    #[test]
    fn test_many_ok_move_then_err_move() {
        let mut first = true;
        assert_eq!(
            many(
                |_, index| {
                    *index += 1;
                    if first {
                        first = false;
                        Ok(())
                    } else {
                        Err(())
                    }
                },
                &[],
                &mut 0
            ),
            Err(())
        );
    }

    #[test]
    fn test_many_ok_move_then_err_no_move() {
        let mut first = true;
        assert_eq!(
            many(
                |_, index| {
                    if first {
                        first = false;
                        *index += 1;
                        Ok(())
                    } else {
                        Err(())
                    }
                },
                &[],
                &mut 0
            ),
            Ok(vec![()])
        );
    }
}
