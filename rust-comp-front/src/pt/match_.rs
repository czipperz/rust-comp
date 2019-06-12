use super::statement::needs_semicolon;
use super::{Error, Parse, Parser};
use crate::ast::*;
use crate::token::TokenKind;

impl<'a> Parse<'a> for Match<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<Match<'a>, Error> {
        parser.expect_token(TokenKind::Match)?;
        let value = Expression::parse(parser)?;
        parser.expect_token(TokenKind::OpenCurly)?;
        let mut matches = Vec::new();
        while parser.expect_token(TokenKind::CloseCurly).is_err() {
            matches.push(MatchItem::parse(parser)?);
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
}

impl<'a> Parse<'a> for MatchItem<'a> {
    fn parse(parser: &mut Parser<'a, '_>) -> Result<MatchItem<'a>, Error> {
        let pattern = Pattern::parse(parser)?;
        parser.expect_token(TokenKind::FatArrow)?;
        let value = Expression::parse(parser)?;
        Ok(MatchItem { pattern, value })
    }
}

#[cfg(test)]
mod tests {
    use super::super::parse;
    use super::*;

    #[test]
    fn test_parse_match_0() {
        let (index, len, match_) = parse("match x {}");
        assert_eq!(index, len);
        assert_eq!(
            match_,
            Ok(Match {
                value: Box::new(Expression::Variable(Variable { name: "x" })),
                matches: vec![]
            })
        );
    }

    #[test]
    fn test_parse_match_1() {
        let (index, len, match_) = parse("match x { a => () }");
        assert_eq!(index, len);
        assert_eq!(
            match_,
            Ok(Match {
                value: Box::new(Expression::Variable(Variable { name: "x" })),
                matches: vec![MatchItem {
                    pattern: Pattern::Named("a"),
                    value: Expression::Tuple(vec![]),
                }]
            })
        );
    }

    #[test]
    fn test_parse_match_1_trailing_comma() {
        let (index, len, match_) = parse("match x { a => (), }");
        assert_eq!(index, len);
        assert_eq!(
            match_,
            Ok(Match {
                value: Box::new(Expression::Variable(Variable { name: "x" })),
                matches: vec![MatchItem {
                    pattern: Pattern::Named("a"),
                    value: Expression::Tuple(vec![]),
                }]
            })
        );
    }

    #[test]
    fn test_parse_match_2_no_comma_because_braces() {
        let (index, len, match_) = parse("match x { a => {} b => () }");
        assert_eq!(index, len);
        assert_eq!(
            match_,
            Ok(Match {
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
            })
        );
    }
}
