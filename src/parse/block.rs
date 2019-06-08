use super::combinator::*;
use super::expression::expect_expression;
use super::parser::Parser;
use super::statement::expect_statement;
use super::Error;
use crate::ast::*;
use crate::token::TokenValue;

pub fn expect_block<'a>(parser: &mut Parser<'a>) -> Result<Block<'a>, Error> {
    parser.expect_token(TokenValue::OpenCurly)?;

    let mut statements = Vec::new();
    let expression;
    loop {
        let old_index = parser.index;
        match expect_statement(parser) {
            Ok(x) => statements.push(x),
            Err(_e) => {
                // TODO: The error above should be used here I don't know how
                // right now
                parser.index = old_index;
                expression = maybe(parser, expect_expression)?;
                break;
            }
        }
    }
    parser.expect_token(TokenValue::CloseCurly)?;
    Ok(Block {
        statements,
        expression: expression.map(Box::new),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::read_tokens;

    #[test]
    fn test_expect_block_no_statements() {
        let contents = "{}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let block = expect_block(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(block.statements.len(), 0);
        assert_eq!(block.expression, None);
    }

    #[test]
    fn test_expect_block_with_empty_statements() {
        let contents = "{;;}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let block = expect_block(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(block.statements, [Statement::Empty, Statement::Empty]);
        assert_eq!(block.expression, None);
    }

    #[test]
    fn test_expect_block_with_statement_then_expression() {
        let contents = "{x;y}";
        let (tokens, eofpos) = read_tokens(0, contents).unwrap();
        let mut parser = Parser::new(contents, &tokens, eofpos);
        let block = expect_block(&mut parser).unwrap();
        assert_eq!(parser.index, tokens.len());
        assert_eq!(
            block.statements,
            [Statement::Expression(Expression::Variable(Variable {
                name: "x"
            }))]
        );
        assert_eq!(
            block.expression,
            Some(Box::new(Expression::Variable(Variable { name: "y" })))
        );
    }
}
