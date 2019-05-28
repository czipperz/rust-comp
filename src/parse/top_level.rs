use crate::ast::*;
use crate::lex::{Token, TokenType};
use super::general::*;
use super::body::parse_block;

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
    let parameters = parse_parameters(tokens, index)?;
    let body = parse_block(tokens, index)?;
    Ok(Function {
        name: name.to_string(),
        parameters,
        body,
    })
}

fn parse_parameters(tokens: &[Token], index: &mut usize) -> Result<Vec<Parameter>, ()> {
    expect_token(tokens, index, &TokenType::TOpenParen)?;
    expect_token(tokens, index, &TokenType::TCloseParen)?;
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::*;

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
        for i in 0..tokens.len() - 1 {
            dbg!(i);
            let mut index = 0;
            assert!(parse_fn(&tokens[0..i], &mut index).is_err());
            assert_eq!(index, i);
        }
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
        assert_eq!(f.parameters.len(), 0);
        assert_eq!(f.body.len(), 0);
    }
}
