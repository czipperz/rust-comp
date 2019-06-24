use super::parser::Parser;
use super::Error;
use crate::pos::Span;

pub fn many<T, E, F>(parser: &mut Parser, f: F) -> Result<Vec<T>, E>
where
    F: FnMut(&mut Parser) -> Result<T, E>,
{
    many_separator(parser, f, |_| Ok(()))
}

pub fn many_comma_separated<T, F>(parser: &mut Parser, f: F) -> Result<(Vec<T>, Vec<Span>), Error>
where
    F: FnMut(&mut Parser) -> Result<T, Error>,
{
    use crate::token::TokenKind;
    let mut comma_spans = Vec::new();
    let spans = many_separator(parser, f, |p| {
        p.expect_token(TokenKind::Comma)
            .map(|s| comma_spans.push(s))
    })?;
    Ok((spans, comma_spans))
}

fn many_separator<T, E, F, S>(parser: &mut Parser, mut f: F, mut separator: S) -> Result<Vec<T>, E>
where
    F: FnMut(&mut Parser) -> Result<T, E>,
    S: FnMut(&mut Parser) -> Result<(), E>,
{
    let mut xs = Vec::new();
    loop {
        let old_index = parser.index;
        match f(parser) {
            Ok(x) => xs.push(x),
            Err(_) if old_index == parser.index => return Ok(xs),
            Err(e) => return Err(e),
        }

        let old_index = parser.index;
        match separator(parser) {
            Ok(()) => (),
            Err(_) if old_index == parser.index => return Ok(xs),
            Err(e) => return Err(e),
        }
    }
}

pub fn maybe<T, E, F>(parser: &mut Parser, mut f: F) -> Result<Option<T>, E>
where
    F: FnMut(&mut Parser) -> Result<T, E>,
{
    let old_index = parser.index;
    match f(parser) {
        Ok(x) => Ok(Some(x)),
        Err(_) if old_index == parser.index => Ok(None),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pos::Pos;

    #[test]
    fn test_many_ok_no_move_then_err_no_move() {
        let mut first = true;
        assert_eq!(
            many(&mut Parser::new("", &[], Pos { file: 0, index: 0 }), |_| {
                if first {
                    first = false;
                    Ok(())
                } else {
                    Err(())
                }
            }),
            Ok(vec![()])
        );
    }

    #[test]
    fn test_many_ok_move_then_err_move() {
        let mut first = true;
        assert_eq!(
            many(
                &mut Parser::new("", &[], Pos { file: 0, index: 0 }),
                |parser| {
                    parser.index += 1;
                    if first {
                        first = false;
                        Ok(())
                    } else {
                        Err(())
                    }
                }
            ),
            Err(())
        );
    }

    #[test]
    fn test_many_ok_move_then_err_no_move() {
        let mut first = true;
        assert_eq!(
            many(
                &mut Parser::new("", &[], Pos { file: 0, index: 0 }),
                |parser| {
                    if first {
                        first = false;
                        parser.index += 1;
                        Ok(())
                    } else {
                        Err(())
                    }
                }
            ),
            Ok(vec![()])
        );
    }

    #[test]
    fn test_cant_parse_separator() {
        assert_eq!(
            many_separator(
                &mut Parser::new("", &[], Pos { file: 0, index: 0 }),
                |_| Ok(()),
                |_| Err(())
            ),
            Ok(vec![()])
        );
    }

    #[test]
    fn test_cant_parse_separator_but_advance() {
        assert_eq!(
            many_separator(
                &mut Parser::new("", &[], Pos { file: 0, index: 0 }),
                |_| Ok(()),
                |parser| {
                    parser.index += 1;
                    Err(())
                }
            ),
            Err(())
        );
    }

    #[test]
    fn test_successful_parse_separator() {
        let mut first = true;
        assert_eq!(
            many_separator(
                &mut Parser::new("", &[], Pos { file: 0, index: 0 }),
                |_| {
                    if first {
                        first = false;
                        Ok(())
                    } else {
                        Err(())
                    }
                },
                |_| Ok(())
            ),
            Ok(vec![()])
        );
    }
}
