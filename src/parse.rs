use crate::ast::*;
use crate::lex::{Token, TokenType};
use crate::pos::*;

pub fn parse(tokens: &[Token]) -> Result<(), ()> {
    let mut index = 0;
    parse_top_level(tokens, &mut index)?;
    Ok(())
}

fn parse_top_level(tokens: &[Token], index: &mut usize) -> Result<TopLevel, ()> {
    parse_fn(tokens, index).map(TopLevel::Function)
}

fn parse_fn(tokens: &[Token], index: &mut usize) -> Result<Function, ()> {
    expect_token(tokens, index, &TokenType::TFn)?;
    let name = expect_label(tokens, index)?;
    expect_token(tokens, index, &TokenType::TOpenParen)?;
    expect_token(tokens, index, &TokenType::TCloseParen)?;
    expect_token(tokens, index, &TokenType::TOpenCurly)?;
    expect_token(tokens, index, &TokenType::TCloseCurly)?;
    Ok(Function {
        name: name.to_string(),
    })
}

fn expect_label<'a>(tokens: &'a [Token], index: &mut usize) -> Result<&'a str, ()> {
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

fn expect_token(tokens: &[Token], index: &mut usize, expected: &TokenType) -> Result<(), ()> {
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
mod tests {
    use super::*;
    use TokenType::*;

    fn make_token(token_type: TokenType) -> Token {
        Token {
            token_type,
            span: Span {
                start: Pos::start(),
                end: Pos::start(),
            },
        }
    }

    #[test]
    fn test_expect_token_out_of_bounds() {
        let mut index = 0;
        assert!(expect_token(&[], &mut index, &TokenType::TFn).is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_expect_matches() {
        let mut index = 0;
        assert!(expect_token(&[make_token(TokenType::TFn)], &mut index, &TokenType::TFn).is_ok());
        assert_eq!(index, 1);
    }

    #[test]
    fn test_expect_no_match() {
        let mut index = 0;
        assert!(expect_token(
            &[make_token(TokenType::TFn)],
            &mut index,
            &TokenType::TOpenParen
        )
        .is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_parse_fn_invalid() {
        let tokens = &vec![
            TFn,
            TLabel("f".to_string()),
            TOpenParen,
            TCloseParen,
            TOpenCurly,
            TCloseCurly,
        ]
        .into_iter()
        .map(make_token)
        .collect::<Vec<_>>();
        for i in 0..tokens.len() {
            dbg!(i);
            let mut index = 0;
            assert!(parse_fn(&tokens[0..i], &mut index).is_err());
            assert_eq!(index, i);
        }
    }

    #[test]
    fn test_parse_fn_eof() {
        let mut index = 0;
        assert!(parse_fn(&[make_token(TFn)], &mut index).is_err());
        assert_eq!(index, 1);
    }

    #[test]
    fn test_parse_fn_matching() {
        let mut index = 0;
        let f = parse_fn(
            &vec![
                TFn,
                TLabel("f".to_string()),
                TOpenParen,
                TCloseParen,
                TOpenCurly,
                TCloseCurly,
            ]
            .into_iter()
            .map(make_token)
            .collect::<Vec<_>>(),
            &mut index,
        )
        .unwrap();
        assert_eq!(index, 6);
        assert_eq!(f.name, "f");
    }
}
