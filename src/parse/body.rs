use super::parser::*;
use super::Error;
use crate::ast::*;
use crate::lex::*;

pub fn expect_block(parser: &mut Parser) -> Result<Vec<Statement>, Error> {
    parser.expect_token(TokenType::OpenCurly)?;
    let statements = parser.many(expect_statement)?;
    parser.expect_token(TokenType::CloseCurly)?;
    Ok(statements)
}

fn expect_statement(parser: &mut Parser) -> Result<Statement, Error> {
    if parser.expect_token(TokenType::Semicolon).is_ok() {
        Ok(Statement::Empty)
    } else {
        Err(Error::EOF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect_block_no_statements() {
        let tokens = make_tokens(vec![TokenType::OpenCurly, TokenType::CloseCurly]);
        let mut parser = Parser::new(&tokens);
        let statements = expect_block(&mut parser).unwrap();
        assert_eq!(parser.index, 2);
        assert_eq!(statements.len(), 0);
    }

    #[test]
    fn test_expect_block_with_statements() {
        let tokens = make_tokens(vec![
            TokenType::OpenCurly,
            TokenType::Semicolon,
            TokenType::Semicolon,
            TokenType::CloseCurly,
        ]);
        let mut parser = Parser::new(&tokens);
        let statements = expect_block(&mut parser).unwrap();
        assert_eq!(parser.index, 4);
        assert_eq!(statements, [Statement::Empty, Statement::Empty]);
    }

    #[test]
    fn test_expect_statement_empty() {
        let mut parser = Parser::new(&[]);
        assert!(expect_statement(&mut parser).is_err());
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_expect_statement_semicolon() {
        let tokens = make_tokens(vec![TokenType::Semicolon]);
        let mut parser = Parser::new(&tokens);
        let statement = expect_statement(&mut parser).unwrap();
        assert_eq!(parser.index, 1);
        assert_eq!(statement, Statement::Empty);
    }
}
