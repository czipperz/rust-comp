use super::block::expect_block;
use super::combinator::*;
use super::parser::Parser;
use super::Error;
use crate::ast::*;
use crate::token::TokenKind;

type Precedence = i8;

pub fn expect_expression<'a>(parser: &mut Parser<'a, '_>) -> Result<Expression<'a>, Error> {
    let base = expect_expression_basic(parser)?;
    expression_chain(parser, base)
}

fn expect_expression_basic<'a>(parser: &mut Parser<'a, '_>) -> Result<Expression<'a>, Error> {
    one_of(
        parser,
        &mut [
            expect_variable_expression,
            expect_paren_expression,
            expect_block_expression,
            expect_if_expression,
            expect_while_expression,
        ][..],
        Error::Expected("expression", parser.span()),
    )
}

fn expression_chain<'a>(
    parser: &mut Parser<'a, '_>,
    mut expr: Expression<'a>,
) -> Result<Expression<'a>, Error> {
    let mut stack = Vec::new();
    let mut max_precedence = 20;

    while let Some(op) = parser.peek().and_then(BinOp::from_token) {
        parser.index += 1;
        let next = expect_expression_basic(parser)?;
        if op.precedence() <= max_precedence {
            max_precedence = op.max_precedence();
            stack.push((expr, op));
        } else {
            // consolidate stack up to op.max_precedence()
            while !stack.is_empty() && stack.last().unwrap().1.max_precedence() < op.precedence() {
                let (left, lop) = stack.pop().unwrap();
                max_precedence = lop.max_precedence();
                expr = Expression::Binary(Binary {
                    left: Box::new(left),
                    op: lop,
                    right: Box::new(expr),
                });
            }
            stack.push((expr, op));
        }
        expr = next;
    }

    Ok(collapse_stack(expr, stack))
}

fn collapse_stack<'a>(
    mut expr: Expression<'a>,
    stack: Vec<(Expression<'a>, BinOp)>,
) -> Expression<'a> {
    for (left, op) in stack.into_iter().rev() {
        expr = Expression::Binary(Binary {
            left: Box::new(left),
            op,
            right: Box::new(expr),
        });
    }
    expr
}

impl BinOp {
    fn from_token(token: TokenKind) -> Option<BinOp> {
        match token {
            TokenKind::Plus => Some(BinOp::Plus),
            TokenKind::Minus => Some(BinOp::Minus),
            TokenKind::Star => Some(BinOp::Times),
            TokenKind::ForwardSlash => Some(BinOp::DividedBy),
            TokenKind::Equals => Some(BinOp::EqualTo),
            TokenKind::NotEquals => Some(BinOp::NotEqualTo),
            TokenKind::Set => Some(BinOp::SetTo),
            TokenKind::And => Some(BinOp::And),
            TokenKind::Or => Some(BinOp::Or),
            _ => None,
        }
    }

    /// The precedence required to stop an active chain
    fn precedence(self) -> Precedence {
        match self {
            BinOp::Times | BinOp::DividedBy => 7,
            BinOp::Plus | BinOp::Minus => 8,
            BinOp::EqualTo | BinOp::NotEqualTo => 13,
            BinOp::And | BinOp::Or => 14,
            BinOp::SetTo => 17,
        }
    }

    /// The precedence required to continue
    fn max_precedence(self) -> Precedence {
        // max_precedence = precedence - if ltr { 1 } else { 0 }
        match self {
            BinOp::Times | BinOp::DividedBy => 6,
            BinOp::Plus | BinOp::Minus => 7,
            BinOp::EqualTo | BinOp::NotEqualTo => 13,
            BinOp::And | BinOp::Or => 13,
            BinOp::SetTo => 17,
        }
    }
}

fn expect_variable_expression<'a>(parser: &mut Parser<'a, '_>) -> Result<Expression<'a>, Error> {
    parser
        .expect_label()
        .map(|name| Expression::Variable(Variable { name }))
}

fn expect_paren_expression<'a>(parser: &mut Parser<'a, '_>) -> Result<Expression<'a>, Error> {
    parser.expect_token(TokenKind::OpenParen)?;
    let expression = expect_expression(parser)?;
    parser.expect_token(TokenKind::CloseParen)?;
    Ok(Expression::Paren(Box::new(expression)))
}

fn expect_block_expression<'a>(parser: &mut Parser<'a, '_>) -> Result<Expression<'a>, Error> {
    expect_block(parser).map(Expression::Block)
}

fn expect_if_expression<'a>(parser: &mut Parser<'a, '_>) -> Result<Expression<'a>, Error> {
    expect_if_expression_(parser).map(Expression::If)
}

fn expect_if_expression_<'a>(parser: &mut Parser<'a, '_>) -> Result<If<'a>, Error> {
    parser.expect_token(TokenKind::If)?;
    let condition = expect_expression(parser)?;
    let then = expect_block(parser)?;
    let else_ = if parser.expect_token(TokenKind::Else).is_ok() {
        Some(Box::new(expect_else_expression(parser)?))
    } else {
        None
    };
    Ok(If {
        condition: Box::new(condition),
        then,
        else_,
    })
}

fn expect_else_expression<'a>(parser: &mut Parser<'a, '_>) -> Result<Else<'a>, Error> {
    fn else_expression_if<'a>(parser: &mut Parser<'a, '_>) -> Result<Else<'a>, Error> {
        expect_if_expression_(parser).map(Else::If)
    }
    fn else_expression_block<'a>(parser: &mut Parser<'a, '_>) -> Result<Else<'a>, Error> {
        expect_block(parser).map(Else::Block)
    }

    one_of(
        parser,
        &mut [else_expression_if, else_expression_block][..],
        Error::Expected("else expression", parser.span()),
    )
}

fn expect_while_expression<'a>(parser: &mut Parser<'a, '_>) -> Result<Expression<'a>, Error> {
    parser.expect_token(TokenKind::While)?;
    let condition = expect_expression(parser)?;
    let block = expect_block(parser)?;
    Ok(Expression::While(While {
        condition: Box::new(condition),
        block,
    }))
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;
    use crate::token::TokenKind;

    #[test]
    fn test_expect_variable_expression() {
        let (index, len, expression) = parse(expect_variable_expression, "ab");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(expression, Expression::Variable(Variable { name: "ab" }));
    }

    #[test]
    fn test_expect_paren_expression() {
        let (index, len, expression) = parse(expect_expression, "(ab)");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Paren(Box::new(Expression::Variable(Variable { name: "ab" })))
        );
    }

    #[test]
    fn test_expect_variable_expression_fn_should_error() {
        let (index, _, expression) = parse(expect_variable_expression, "fn");
        let error = expression.unwrap_err();
        assert_eq!(index, 0);
        assert_eq!(
            error,
            Error::ExpectedToken(
                TokenKind::Label,
                Span {
                    file: 0,
                    start: 0,
                    end: 2
                },
            )
        );
    }

    #[test]
    fn test_expect_block_expression() {
        let (index, len, expression) = parse(expect_block_expression, "{}");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Block(Block {
                statements: vec![],
                expression: None,
            })
        );
    }

    #[test]
    fn test_expect_if_expression() {
        let (index, len, expression) = parse(expect_if_expression, "if b {}");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::If(If {
                condition: Box::new(Expression::Variable(Variable { name: "b" })),
                then: Block {
                    statements: vec![],
                    expression: None,
                },
                else_: None,
            })
        );
    }

    #[test]
    fn test_expect_if_else_expression() {
        let (index, len, expression) = parse(expect_if_expression, "if b {} else {}");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::If(If {
                condition: Box::new(Expression::Variable(Variable { name: "b" })),
                then: Block {
                    statements: vec![],
                    expression: None,
                },
                else_: Some(Box::new(Else::Block(Block {
                    statements: vec![],
                    expression: None,
                })))
            })
        );
    }

    #[test]
    fn test_expect_if_else_if_expression() {
        let (index, len, expression) = parse(expect_if_expression, "if b {} else if c {}");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::If(If {
                condition: Box::new(Expression::Variable(Variable { name: "b" })),
                then: Block {
                    statements: vec![],
                    expression: None,
                },
                else_: Some(Box::new(Else::If(If {
                    condition: Box::new(Expression::Variable(Variable { name: "c" })),
                    then: Block {
                        statements: vec![],
                        expression: None,
                    },
                    else_: None,
                }))),
            })
        );
    }

    #[test]
    fn test_expect_while_expression() {
        let (index, len, expression) = parse(expect_while_expression, "while b {}");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::While(While {
                condition: Box::new(Expression::Variable(Variable { name: "b" })),
                block: Block {
                    statements: vec![],
                    expression: None,
                },
            })
        );
    }

    #[test]
    fn test_expect_expression_handles_plus_expressions() {
        let (index, len, expression) = parse(expect_expression, "a + b");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Binary(Binary {
                left: Box::new(Expression::Variable(Variable { name: "a" })),
                op: BinOp::Plus,
                right: Box::new(Expression::Variable(Variable { name: "b" }))
            }),
        );
    }

    #[test]
    fn test_expect_expression_handles_minus_expressions() {
        let (index, len, expression) = parse(expect_expression, "a - b");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Binary(Binary {
                left: Box::new(Expression::Variable(Variable { name: "a" })),
                op: BinOp::Minus,
                right: Box::new(Expression::Variable(Variable { name: "b" }))
            }),
        );
    }

    #[test]
    fn test_expect_expression_handles_times_expressions() {
        let (index, len, expression) = parse(expect_expression, "a * b");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Binary(Binary {
                left: Box::new(Expression::Variable(Variable { name: "a" })),
                op: BinOp::Times,
                right: Box::new(Expression::Variable(Variable { name: "b" }))
            }),
        );
    }

    #[test]
    fn test_expect_expression_handles_divided_by_expressions() {
        let (index, len, expression) = parse(expect_expression, "a / b");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Binary(Binary {
                left: Box::new(Expression::Variable(Variable { name: "a" })),
                op: BinOp::DividedBy,
                right: Box::new(Expression::Variable(Variable { name: "b" }))
            }),
        );
    }

    #[test]
    fn test_expect_expression_left_to_right_precedence() {
        let (index, len, expression) = parse(expect_expression, "a + b - c");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Binary(Binary {
                left: Box::new(Expression::Binary(Binary {
                    left: Box::new(Expression::Variable(Variable { name: "a" })),
                    op: BinOp::Plus,
                    right: Box::new(Expression::Variable(Variable { name: "b" })),
                })),
                op: BinOp::Minus,
                right: Box::new(Expression::Variable(Variable { name: "c" })),
            }),
        );
    }

    #[test]
    fn test_expect_expression_different_precedences() {
        let (index, len, expression) = parse(expect_expression, "a + b * c - d");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Binary(Binary {
                left: Box::new(Expression::Binary(Binary {
                    left: Box::new(Expression::Variable(Variable { name: "a" })),
                    op: BinOp::Plus,
                    right: Box::new(Expression::Binary(Binary {
                        left: Box::new(Expression::Variable(Variable { name: "b" })),
                        op: BinOp::Times,
                        right: Box::new(Expression::Variable(Variable { name: "c" })),
                    }))
                })),
                op: BinOp::Minus,
                right: Box::new(Expression::Variable(Variable { name: "d" })),
            }),
        );
    }
}
