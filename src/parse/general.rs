use crate::lex::*;

pub fn expect_label<'a>(tokens: &'a [Token], index: &mut usize) -> Result<&'a str, ()> {
    if *index >= tokens.len() {
        Err(())
    } else {
        match tokens[*index].token_type {
            TokenType::TLabel(ref label) => {
                *index += 1;
                Ok(&label)
            }
            _ => Err(()),
        }
    }
}

pub fn expect_token(tokens: &[Token], index: &mut usize, expected: &TokenType) -> Result<(), ()> {
    if *index >= tokens.len() {
        Err(())
    } else if tokens[*index].token_type == *expected {
        *index += 1;
        Ok(())
    } else {
        Err(())
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
                &[make_token(TokenType::TLabel("abc".to_string()))],
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
        assert!(expect_label(&[make_token(TokenType::TFn)], &mut index).is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_expect_token_out_of_bounds() {
        let mut index = 0;
        assert!(expect_token(&[], &mut index, &TokenType::TFn).is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_expect_token_matches() {
        let mut index = 0;
        assert!(expect_token(&[make_token(TokenType::TFn)], &mut index, &TokenType::TFn).is_ok());
        assert_eq!(index, 1);
    }

    #[test]
    fn test_expect_token_no_match() {
        let mut index = 0;
        assert!(expect_token(
            &[make_token(TokenType::TFn)],
            &mut index,
            &TokenType::TOpenParen
        )
        .is_err());
        assert_eq!(index, 0);
    }
}
