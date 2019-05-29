use super::general::*;
use crate::ast::*;
use crate::lex::*;

pub fn expect_block(tokens: &[Token], index: &mut usize) -> Result<Vec<Statement>, ()> {
    expect_token(tokens, index, &TokenType::OpenCurly)?;
    let statements = many(expect_statement, tokens, index)?;
    expect_token(tokens, index, &TokenType::CloseCurly)?;
    Ok(statements)
}

fn expect_statement(tokens: &[Token], index: &mut usize) -> Result<Statement, ()> {
    if expect_token(tokens, index, &TokenType::Semicolon).is_ok() {
        Ok(Statement::Empty)
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect_block_no_statements() {
        let mut index = 0;
        let statements = expect_block(
            &[
                make_token(TokenType::OpenCurly),
                make_token(TokenType::CloseCurly),
            ],
            &mut index,
        )
        .unwrap();
        assert_eq!(index, 2);
        assert_eq!(statements.len(), 0);
    }

    #[test]
    fn test_expect_block_with_statements() {
        let mut index = 0;
        let statements = expect_block(
            &[
                make_token(TokenType::OpenCurly),
                make_token(TokenType::Semicolon),
                make_token(TokenType::Semicolon),
                make_token(TokenType::CloseCurly),
            ],
            &mut index,
        )
        .unwrap();
        assert_eq!(index, 4);
        assert_eq!(statements, [Statement::Empty, Statement::Empty]);
    }

    #[test]
    fn test_expect_statement_empty() {
        let mut index = 0;
        assert!(expect_statement(&[], &mut index).is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_expect_statement_semicolon() {
        let mut index = 0;
        let statement = expect_statement(&[make_token(TokenType::Semicolon)], &mut index).unwrap();
        assert_eq!(index, 1);
        assert_eq!(statement, Statement::Empty);
    }
}
