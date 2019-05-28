use crate::lex::*;
use crate::ast::*;
use super::general::*;

pub fn parse_block(tokens: &[Token], index: &mut usize) -> Result<Vec<Statement>, ()> {
    expect_token(tokens, index, &TokenType::TOpenCurly)?;
    expect_token(tokens, index, &TokenType::TCloseCurly)?;
    Ok(Vec::new())
}
