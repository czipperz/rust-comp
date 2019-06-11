use super::error::Error;
use super::expression::expect_expression;
use super::parser::Parser;
use super::pattern::expect_pattern;
use super::statement::needs_semicolon;
use crate::ast::*;
use crate::token::TokenKind;

pub fn expect_match<'a>(parser: &mut Parser<'a, '_>) -> Result<Match<'a>, Error> {
    parser.expect_token(TokenKind::Match)?;
    let value = expect_expression(parser)?;
    parser.expect_token(TokenKind::OpenCurly)?;
    let mut matches = Vec::new();
    while parser.expect_token(TokenKind::CloseCurly).is_err() {
        matches.push(expect_pair(parser)?);
        if needs_semicolon(&matches.last().unwrap().value)
            && parser.expect_token(TokenKind::Comma).is_err()
        {
            parser.expect_token(TokenKind::CloseCurly)?;
            break;
        }
    }
    Ok(Match {
        value: Box::new(value),
        matches,
    })
}

fn expect_pair<'a>(parser: &mut Parser<'a, '_>) -> Result<MatchItem<'a>, Error> {
    let pattern = expect_pattern(parser)?;
    parser.expect_token(TokenKind::FatArrow)?;
    let value = expect_expression(parser)?;
    Ok(MatchItem { pattern, value })
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;

    #[test]
    fn test_expect_match_0() {
        let (index, len, match_) = parse(expect_match, "match x {}");
        let match_ = match_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            match_,
            Match {
                value: Box::new(Expression::Variable(Variable { name: "x" })),
                matches: vec![]
            }
        );
    }

    #[test]
    fn test_expect_match_1() {
        let (index, len, match_) = parse(expect_match, "match x { a => () }");
        let match_ = match_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            match_,
            Match {
                value: Box::new(Expression::Variable(Variable { name: "x" })),
                matches: vec![MatchItem {
                    pattern: Pattern::Named("a"),
                    value: Expression::Tuple(vec![]),
                }]
            }
        );
    }

    #[test]
    fn test_expect_match_1_trailing_comma() {
        let (index, len, match_) = parse(expect_match, "match x { a => (), }");
        let match_ = match_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            match_,
            Match {
                value: Box::new(Expression::Variable(Variable { name: "x" })),
                matches: vec![MatchItem {
                    pattern: Pattern::Named("a"),
                    value: Expression::Tuple(vec![]),
                }]
            }
        );
    }

    #[test]
    fn test_expect_match_2_no_comma_because_braces() {
        let (index, len, match_) = parse(expect_match, "match x { a => {} b => () }");
        let match_ = match_.unwrap();
        assert_eq!(index, len);
        assert_eq!(
            match_,
            Match {
                value: Box::new(Expression::Variable(Variable { name: "x" })),
                matches: vec![
                    MatchItem {
                        pattern: Pattern::Named("a"),
                        value: Expression::Block(Block {
                            statements: Vec::new(),
                            expression: None
                        }),
                    },
                    MatchItem {
                        pattern: Pattern::Named("b"),
                        value: Expression::Tuple(vec![]),
                    }
                ]
            }
        );
    }
}
