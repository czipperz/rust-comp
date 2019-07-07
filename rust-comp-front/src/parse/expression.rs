use super::block::expect_block;
use super::combinator::*;
use super::match_::expect_match;
use super::parser::Parser;
use super::tree::*;
use super::Error;
use crate::token::*;

type Precedence = i8;

pub fn expect_expression(parser: &mut Parser) -> Result<Expression, Error> {
    let base = expect_expression_basic(parser)?;
    expression_chain(parser, base)
}

fn expect_expression_basic(parser: &mut Parser) -> Result<Expression, Error> {
    match parser.peek_kind() {
        Some(TokenKind::Label) => expect_variable_expression(parser),
        Some(TokenKind::OpenParen) => expect_paren_expression(parser),
        Some(TokenKind::OpenCurly) => expect_block_expression(parser),
        Some(TokenKind::If) => expect_if_expression(parser),
        Some(TokenKind::Loop) => expect_loop_expression(parser),
        Some(TokenKind::While) => expect_while_expression(parser),
        Some(TokenKind::Match) => expect_match_expression(parser),
        Some(TokenKind::True) => expect_true_expression(parser),
        Some(TokenKind::False) => expect_false_expression(parser),
        Some(TokenKind::Integer) => expect_integer_expression(parser),
        _ => Err(Error::Expected("expression", parser.span())),
    }
}

fn expression_chain(parser: &mut Parser, mut expr: Expression) -> Result<Expression, Error> {
    let mut stack: Vec<(Expression, Token, Precedence)> = Vec::new();
    let mut max_precedence = 20;

    while let Some(token) = parser.peek() {
        if is_bin_op(token.kind) {
            parser.index += 1;
            let next = expect_expression_basic(parser)?;
            if precedence(token.kind) <= max_precedence {
                max_precedence = continue_precedence(token.kind);
            } else {
                expr = consolidate_stack(
                    expr,
                    &mut max_precedence,
                    &mut stack,
                    precedence(token.kind),
                );
            }
            let cont = continue_precedence(token.kind);
            stack.push((expr, token, cont));
            expr = next;
        } else if token.kind == TokenKind::Dot {
            parser.index += 1;
            let member = parser.expect_token(TokenKind::Label)?;
            if 2 <= max_precedence {
                max_precedence = 1;
            } else {
                expr = consolidate_stack(expr, &mut max_precedence, &mut stack, 2);
            }
            expr = Expression::MemberAccess(MemberAccess {
                object: Box::new(expr),
                dot_span: token.span,
                member,
            });
        } else if token.kind == TokenKind::OpenParen {
            if max_precedence < 3 {
                expr = consolidate_stack(expr, &mut max_precedence, &mut stack, 3);
            }
            let open_paren_span = parser.expect_token(TokenKind::OpenParen).unwrap();
            let (arguments, comma_spans) = many_comma_separated(parser, expect_expression)?;
            let close_paren_span = parser.expect_token(TokenKind::CloseParen)?;
            expr = Expression::FunctionCall(FunctionCall {
                function: Box::new(expr),
                open_paren_span,
                arguments,
                comma_spans,
                close_paren_span,
            });
        } else {
            break;
        }
    }

    Ok(collapse_stack(expr, stack))
}

fn consolidate_stack(
    mut expr: Expression,
    max_precedence: &mut Precedence,
    stack: &mut Vec<(Expression, Token, Precedence)>,
    token_precedence: Precedence,
) -> Expression {
    // consolidate stack up to token_precedence
    while !stack.is_empty() && stack.last().unwrap().2 < token_precedence {
        let (left, op, precedence) = stack.pop().unwrap();
        *max_precedence = precedence;
        expr = Expression::Binary(Binary {
            left: Box::new(left),
            op,
            right: Box::new(expr),
        });
    }
    expr
}

fn collapse_stack(mut expr: Expression, stack: Vec<(Expression, Token, Precedence)>) -> Expression {
    for (left, op, _) in stack.into_iter().rev() {
        expr = Expression::Binary(Binary {
            left: Box::new(left),
            op,
            right: Box::new(expr),
        });
    }
    expr
}

fn is_bin_op(token: TokenKind) -> bool {
    match token {
        TokenKind::Star
        | TokenKind::ForwardSlash
        | TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Ampersand
        | TokenKind::Bar
        | TokenKind::Equals
        | TokenKind::NotEquals
        | TokenKind::Set
        | TokenKind::And
        | TokenKind::Or => true,
        _ => false,
    }
}

/// The precedence required to stop an active chain
fn precedence(token: TokenKind) -> Precedence {
    match token {
        TokenKind::Star | TokenKind::ForwardSlash => 7,
        TokenKind::Plus | TokenKind::Minus => 8,
        TokenKind::Ampersand => 10,
        TokenKind::Bar => 12,
        TokenKind::Equals | TokenKind::NotEquals => 13,
        TokenKind::And | TokenKind::Or => 14,
        TokenKind::Set => 17,
        _ => unreachable!("{:?}", token),
    }
}

/// The precedence required to continue
fn continue_precedence(token: TokenKind) -> Precedence {
    // continue_precedence = precedence - if ltr { 1 } else { 0 }
    match token {
        TokenKind::Star | TokenKind::ForwardSlash => 6,
        TokenKind::Plus | TokenKind::Minus => 7,
        TokenKind::Ampersand => 9,
        TokenKind::Bar => 11,
        TokenKind::Equals | TokenKind::NotEquals => 13,
        TokenKind::And | TokenKind::Or => 13,
        TokenKind::Set => 17,
        _ => unreachable!(),
    }
}

fn expect_variable_expression<'a>(parser: &mut Parser) -> Result<Expression, Error> {
    parser
        .expect_token(TokenKind::Label)
        .map(|name| Expression::Variable(Variable { name }))
}

fn expect_paren_expression<'a>(parser: &mut Parser) -> Result<Expression, Error> {
    let open_paren_span = parser.expect_token(TokenKind::OpenParen)?;
    if let Ok(close_paren_span) = parser.expect_token(TokenKind::CloseParen) {
        Ok(Expression::Tuple(Tuple {
            open_paren_span,
            expressions: vec![],
            comma_spans: vec![],
            close_paren_span,
        }))
    } else {
        let expression = expect_expression(parser)?;
        if parser.peek_kind() == Some(TokenKind::Comma) {
            let mut expressions = Vec::new();
            expressions.push(expression);
            let mut comma_spans = Vec::new();
            comma_spans.push(parser.expect_token(TokenKind::Comma).unwrap());
            while parser.peek_kind() != Some(TokenKind::CloseParen) {
                expressions.push(expect_expression(parser)?);
                match parser.expect_token(TokenKind::Comma) {
                    Ok(span) => comma_spans.push(span),
                    Err(_) => break,
                }
            }
            let close_paren_span = parser.expect_token(TokenKind::CloseParen)?;
            Ok(Expression::Tuple(Tuple {
                open_paren_span,
                expressions,
                comma_spans,
                close_paren_span,
            }))
        } else {
            let close_paren_span = parser.expect_token(TokenKind::CloseParen)?;
            Ok(Expression::Paren(ParenExpression {
                open_paren_span,
                expression: Box::new(expression),
                close_paren_span,
            }))
        }
    }
}

fn expect_block_expression<'a>(parser: &mut Parser) -> Result<Expression, Error> {
    expect_block(parser).map(Expression::Block)
}

fn expect_if_expression<'a>(parser: &mut Parser) -> Result<Expression, Error> {
    expect_if_expression_(parser).map(Expression::If)
}

fn expect_if_expression_<'a>(parser: &mut Parser) -> Result<If, Error> {
    let if_span = parser.expect_token(TokenKind::If)?;
    let condition = expect_expression(parser)?;
    let then = expect_block(parser)?;
    let else_ = if parser.peek_kind() == Some(TokenKind::Else) {
        Some(Box::new(expect_else_expression(parser)?))
    } else {
        None
    };
    Ok(If {
        if_span,
        condition: Box::new(condition),
        then,
        else_,
    })
}

fn expect_else_expression<'a>(parser: &mut Parser) -> Result<Else, Error> {
    let else_span = parser.expect_token(TokenKind::Else)?;
    match parser.peek_kind() {
        Some(TokenKind::If) => expect_if_expression_(parser).map(ElseKind::If),
        Some(TokenKind::OpenCurly) => expect_block(parser).map(ElseKind::Block),
        _ => Err(Error::Expected("else expression", parser.span())),
    }
    .map(|kind| Else { else_span, kind })
}

fn expect_while_expression<'a>(parser: &mut Parser) -> Result<Expression, Error> {
    let while_span = parser.expect_token(TokenKind::While)?;
    let condition = expect_expression(parser)?;
    let block = expect_block(parser)?;
    Ok(Expression::While(While {
        while_span,
        condition: Box::new(condition),
        block,
    }))
}

fn expect_loop_expression<'a>(parser: &mut Parser) -> Result<Expression, Error> {
    let loop_span = parser.expect_token(TokenKind::Loop)?;
    let block = expect_block(parser)?;
    Ok(Expression::Loop(Loop { loop_span, block }))
}

fn expect_match_expression<'a>(parser: &mut Parser) -> Result<Expression, Error> {
    expect_match(parser).map(Expression::Match)
}

fn expect_true_expression<'a>(parser: &mut Parser) -> Result<Expression, Error> {
    expect_bool_expression(parser, TokenKind::True)
}

fn expect_false_expression<'a>(parser: &mut Parser) -> Result<Expression, Error> {
    expect_bool_expression(parser, TokenKind::False)
}

fn expect_bool_expression(parser: &mut Parser, kind: TokenKind) -> Result<Expression, Error> {
    Ok(Expression::Bool(Token {
        span: parser.expect_token(kind)?,
        kind,
    }))
}

fn expect_integer_expression(parser: &mut Parser) -> Result<Expression, Error> {
    use std::str::FromStr;
    let span = parser.expect_token(TokenKind::Integer)?;
    Ok(Expression::Integer(Integer {
        span,
        value: u128::from_str(parser.file_span(span))
            .map_err(|_| Error::IntegerOutOfRange(span))?,
    }))
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;
    use crate::token::TokenKind;
    use assert_matches::assert_matches;

    #[test]
    fn test_expect_variable_expression() {
        let (index, len, expression) = parse(expect_variable_expression, "ab");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Variable(Variable {
                name: Span {
                    file: 0,
                    start: 0,
                    end: 2
                }
            })
        );
    }

    #[test]
    fn test_expect_bool_expression_true() {
        let (index, len, expression) = parse(expect_expression, "true");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Bool(Token {
                span: Span {
                    file: 0,
                    start: 0,
                    end: 4
                },
                kind: TokenKind::True,
            })
        );
    }

    #[test]
    fn test_expect_bool_expression_false() {
        let (index, len, expression) = parse(expect_expression, "false");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Bool(Token {
                span: Span {
                    file: 0,
                    start: 0,
                    end: 5
                },
                kind: TokenKind::False,
            })
        );
    }

    #[test]
    fn test_expect_paren_expression() {
        let (index, len, expression) = parse(expect_expression, "(ab)");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Paren(ParenExpression {
                open_paren_span: Span {
                    file: 0,
                    start: 0,
                    end: 1
                },
                expression: Box::new(Expression::Variable(Variable {
                    name: Span {
                        file: 0,
                        start: 1,
                        end: 3
                    }
                })),
                close_paren_span: Span {
                    file: 0,
                    start: 3,
                    end: 4
                },
            })
        );
    }

    #[test]
    fn test_expect_tuple_expression_0() {
        let (index, len, expression) = parse(expect_expression, "()");
        let expression = expression.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Expression::Tuple(Tuple {
                open_paren_span: Span {
                    file: 0,
                    start: 0,
                    end: 1
                },
                expressions: vec![],
                comma_spans: vec![],
                close_paren_span: Span {
                    file: 0,
                    start: 1,
                    end: 2
                }
            })
        );
    }

    #[test]
    fn test_expect_tuple_expression_1() {
        let (index, len, expression) = parse(expect_expression, "(ab,)");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Tuple(Tuple {
            expressions,
            comma_spans,
            ..
        })) =>
        {
            assert_eq!(
                expressions,
                [Expression::Variable(Variable {
                    name: Span {
                        file: 0,
                        start: 1,
                        end: 3
                    }
                })]
            );
            assert_eq!(
                comma_spans,
                [Span {
                    file: 0,
                    start: 3,
                    end: 4
                }]
            );
        });
    }

    #[test]
    fn test_expect_tuple_expression_2() {
        let (index, len, expression) = parse(expect_expression, "(ab, cd)");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Tuple(Tuple {
            expressions,
            comma_spans,
            ..
        })) =>
        {
            assert_eq!(
                expressions,
                [
                    Expression::Variable(Variable {
                        name: Span {
                            file: 0,
                            start: 1,
                            end: 3
                        }
                    }),
                    Expression::Variable(Variable {
                        name: Span {
                            file: 0,
                            start: 5,
                            end: 7
                        }
                    })
                ]
            );
            assert_eq!(
                comma_spans,
                [Span {
                    file: 0,
                    start: 3,
                    end: 4
                }]
            );
        });
    }

    #[test]
    fn test_expect_variable_expression_fn_should_error() {
        let (index, _, expression) = parse(expect_variable_expression, "fn");
        assert_eq!(index, 0);
        assert_eq!(
            expression,
            Err(Error::ExpectedToken(
                TokenKind::Label,
                Span {
                    file: 0,
                    start: 0,
                    end: 2
                },
            ))
        );
    }

    #[test]
    fn test_expect_block_expression() {
        let (index, len, expression) = parse(expect_block_expression, "{}");
        assert_eq!(index, len);
        assert_eq!(
            expression,
            Ok(Expression::Block(Block {
                open_curly_span: Span {
                    file: 0,
                    start: 0,
                    end: 1
                },
                statements: vec![],
                expression: None,
                close_curly_span: Span {
                    file: 0,
                    start: 1,
                    end: 2
                },
            }))
        );
    }

    #[test]
    fn test_expect_if_expression() {
        let (index, len, expression) = parse(expect_if_expression, "if b {}");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::If(If { else_: None, .. })));
    }

    #[test]
    fn test_expect_if_else_expression() {
        let (index, len, expression) = parse(expect_if_expression, "if b {} else {}");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::If(If { else_, .. })) => {
            assert!(else_.is_some());
            assert_matches!(else_.unwrap().kind, ElseKind::Block(_));
        });
    }

    #[test]
    fn test_expect_if_else_if_expression() {
        let (index, len, expression) = parse(expect_if_expression, "if b {} else if c {}");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::If(If { else_, .. })) => {
            assert!(else_.is_some());
            assert_matches!(else_.unwrap().kind, ElseKind::If(If { else_: None, .. }));
        });
    }

    #[test]
    fn test_expect_loop_expression() {
        let (index, len, expression) = parse(expect_loop_expression, "loop {}");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Loop(_)));
    }

    #[test]
    fn test_expect_while_expression() {
        let (index, len, expression) = parse(expect_while_expression, "while b {}");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::While(_)));
    }

    #[test]
    fn test_expect_expression_handles_plus_expressions() {
        let (index, len, expression) = parse(expect_expression, "a + b");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Binary(Binary { op, .. })) => {
            assert_eq!(
                op,
                Token {
                    span: Span {
                        file: 0,
                        start: 2,
                        end: 3,
                    },
                    kind: TokenKind::Plus,
                }
            )
        });
    }

    #[test]
    fn test_expect_expression_handles_minus_expressions() {
        let (index, len, expression) = parse(expect_expression, "a - b");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Binary(Binary { op, .. })) => {
            assert_eq!(
                op,
                Token {
                    span: Span {
                        file: 0,
                        start: 2,
                        end: 3
                    },
                    kind: TokenKind::Minus,
                }
            )
        });
    }

    #[test]
    fn test_expect_expression_handles_bin_and_expressions() {
        let (index, len, expression) = parse(expect_expression, "a & b");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Binary(Binary { op, .. })) => {
            assert_eq!(
                op,
                Token {
                    span: Span {
                        file: 0,
                        start: 2,
                        end: 3
                    },
                    kind: TokenKind::Ampersand,
                }
            )
        });
    }

    #[test]
    fn test_expect_expression_handles_bin_or_expressions() {
        let (index, len, expression) = parse(expect_expression, "a | b");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Binary(Binary { op, .. })) => {
            assert_eq!(
                op,
                Token {
                    span: Span {
                        file: 0,
                        start: 2,
                        end: 3
                    },
                    kind: TokenKind::Bar,
                }
            )
        });
    }

    #[test]
    fn test_expect_expression_handles_times_expressions() {
        let (index, len, expression) = parse(expect_expression, "a * b");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Binary(Binary { op, .. })) => {
            assert_eq!(
                op,
                Token {
                    span: Span {
                        file: 0,
                        start: 2,
                        end: 3
                    },
                    kind: TokenKind::Star,
                }
            )
        });
    }

    #[test]
    fn test_expect_expression_handles_divided_by_expressions() {
        let (index, len, expression) = parse(expect_expression, "a / b");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Binary(Binary { op, .. })) => {
            assert_eq!(
                op,
                Token {
                    span: Span {
                        file: 0,
                        start: 2,
                        end: 3
                    },
                    kind: TokenKind::ForwardSlash,
                }
            )
        });
    }

    #[test]
    fn test_expect_expression_left_to_right_precedence() {
        let (index, len, expression) = parse(expect_expression, "a + b - c");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Binary(Binary { left, op, .. })) => {
            assert_eq!(op.kind, TokenKind::Minus);
            assert_matches!(*left, Expression::Binary(Binary { op, .. }) => {
                assert_eq!(op.kind, TokenKind::Plus);
            });
        });
    }

    #[test]
    fn test_expect_expression_different_precedences() {
        let (index, len, expression) = parse(expect_expression, "a + b * c - d");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Binary(Binary { left, op, .. })) => {
            assert_eq!(op.kind, TokenKind::Minus);
            assert_matches!(*left, Expression::Binary(Binary { op, right, .. }) => {
                assert_eq!(op.kind, TokenKind::Plus);
                assert_matches!(*right, Expression::Binary(Binary { op, .. }) => {
                    assert_eq!(op.kind, TokenKind::Star);
                });
            });
        });
    }

    #[test]
    fn test_expect_expression_function_call_no_args() {
        let (index, len, expression) = parse(expect_expression, "f()");
        assert_eq!(index, len);
        assert_matches!(
            expression,
            Ok(Expression::FunctionCall(FunctionCall {
                arguments, ..
            })) => {
                assert_eq!(arguments.len(), 0);
            }
        );
    }

    #[test]
    fn test_expect_expression_function_call_one_arg() {
        let (index, len, expression) = parse(expect_expression, "f(x + y)");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::FunctionCall(FunctionCall { arguments, .. })) => {
            assert_eq!(arguments.len(), 1);
            assert_matches!(arguments[0], Expression::Binary(Binary { .. }));
        });
    }

    #[test]
    fn test_expect_expression_function_call_two_args() {
        let (index, len, expression) = parse(expect_expression, "f(x + y, z && a)");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::FunctionCall(FunctionCall { arguments, .. })) => {
            assert_eq!(arguments.len(), 2);
            assert_matches!(arguments[0], Expression::Binary(Binary { .. }));
            assert_matches!(arguments[1], Expression::Binary(Binary { .. }));
        });
    }

    #[test]
    fn test_expect_expression_function_call_tighter_than_normal_ops() {
        let (index, len, expression) = parse(expect_expression, "x * f(y) + z");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Binary(Binary { left, .. })) => {
            assert_matches!(*left, Expression::Binary(Binary { right, .. }) => {
                assert_matches!(*right, Expression::FunctionCall(FunctionCall { .. }));
            });
        });
    }

    #[test]
    fn test_expect_expression_mult_then_set() {
        let (index, len, expression) = parse(expect_expression, "x * f(y) = z");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::Binary(Binary { left, op, .. })) => {
            assert_eq!(op.kind, TokenKind::Set);
            assert_matches!(*left, Expression::Binary(Binary { left, .. }) => {
                assert_matches!(*left, Expression::Variable(_));
            });
        });
    }

    #[test]
    fn test_expect_expression_field_access() {
        let (index, len, expression) = parse(expect_expression, "a.b");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::MemberAccess(MemberAccess {
            object,
            dot_span,
            member,
        })) => {
            assert_eq!(*object, Expression::Variable(Variable {
                name: Span { file: 0, start: 0, end: 1 }
            }));
            assert_eq!(dot_span, Span { file: 0, start: 1, end: 2 });
            assert_eq!(member, Span { file: 0, start: 2, end: 3 });
        });
    }

    #[test]
    fn test_expect_expression_method_call() {
        let (index, len, expression) = parse(expect_expression, "a.b()");
        assert_eq!(index, len);
        assert_matches!(expression, Ok(Expression::FunctionCall(FunctionCall {function,..})) => {
            assert_matches!(*function, Expression::MemberAccess(_));
        });
    }
}
