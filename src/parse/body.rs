use super::general::*;
use crate::ast::*;
use crate::lex::*;

pub fn parse_block(tokens: &[Token], index: &mut usize) -> Result<Vec<Statement>, ()> {
    let mut statements = Vec::new();
    expect_token(tokens, index, &TokenType::OpenCurly)?;
    loop {
        match parse_statement(tokens, index) {
            Ok(statement) => statements.push(statement),
            Err(_) => break,
        }
    }
    expect_token(tokens, index, &TokenType::CloseCurly)?;
    Ok(statements)
}

fn parse_statement(tokens: &[Token], index: &mut usize) -> Result<Statement, ()> {
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
    fn test_parse_block_no_statements() {
        let mut index = 0;
        let statements = parse_block(
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
    fn test_parse_block_with_statements() {
        let mut index = 0;
        let statements = parse_block(
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
    fn test_parse_statement_empty() {
        let mut index = 0;
        assert!(parse_statement(&[], &mut index).is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_parse_statement_semicolon() {
        let mut index = 0;
        let statement = parse_statement(&[make_token(TokenType::Semicolon)], &mut index).unwrap();
        assert_eq!(index, 1);
        assert_eq!(statement, Statement::Empty);
    }
}
