use super::combinator::*;
use super::parser::Parser;
use super::statement::expect_statement;
use super::Error;
use crate::ast::*;
use crate::token::TokenValue;

pub fn expect_block<'a>(parser: &mut Parser<'a>) -> Result<&'a Block<'a>, Error> {
    parser.expect_token(TokenValue::OpenCurly)?;
    let statements = many(parser, expect_statement)?;
    parser.expect_token(TokenValue::CloseCurly)?;
    Ok(parser.alloc(Block { statements }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena::Arena;
    use crate::lex::read_tokens;

    #[test]
    fn test_expect_block_no_statements() {
        let mut arena = Arena::new();
        let contents = "{}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let block = expect_block(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(block.statements.len(), 0);
    }

    #[test]
    fn test_expect_block_with_empty_statements() {
        let mut arena = Arena::new();
        let contents = "{;;}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos, arena.allocator());
        let block = expect_block(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(block.statements, [&Statement::Empty, &Statement::Empty]);
    }
}
