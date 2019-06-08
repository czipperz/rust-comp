use super::parser::Parser;

pub fn many<'a, T, E, F>(parser: &mut Parser<'a>, f: F) -> Result<Vec<T>, E>
where
    F: FnMut(&mut Parser<'a>) -> Result<T, E>,
    T: 'a,
{
    many_separator(parser, f, |_| Ok(()))
}

pub fn many_separator<'a, T, E, F, S>(
    parser: &mut Parser<'a>,
    mut f: F,
    mut separator: S,
) -> Result<Vec<T>, E>
where
    F: FnMut(&mut Parser<'a>) -> Result<T, E>,
    S: FnMut(&mut Parser<'a>) -> Result<(), E>,
    T: 'a,
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

pub fn one_of<'a, T, E, F>(parser: &mut Parser<'a>, fs: &mut [F], none_match: E) -> Result<T, E>
where
    F: FnMut(&mut Parser<'a>) -> Result<T, E>,
    T: 'a,
{
    let old_index = parser.index;
    for f in fs {
        match f(parser) {
            Ok(x) => return Ok(x),
            Err(_) if old_index == parser.index => (),
            Err(e) => return Err(e),
        }
    }
    Err(none_match)
}

pub fn maybe<'a, T, E, F>(parser: &mut Parser<'a>, mut f: F) -> Result<Option<T>, E>
where
    F: FnMut(&mut Parser<'a>) -> Result<T, E>,
    T: 'a,
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

    #[test]
    fn test_one_of_no_functions_should_return_error() {
        assert_eq!(
            one_of::<(), i32, fn(&mut Parser) -> Result<(), i32>>(
                &mut Parser::new("", &[], Pos { file: 0, index: 0 }),
                &mut [],
                1
            ),
            Err(1)
        )
    }

    #[test]
    fn test_one_of_two_functions_first_ok() {
        assert_eq!(
            one_of::<i32, i32, fn(&mut Parser) -> Result<i32, i32>>(
                &mut Parser::new("", &[], Pos { file: 0, index: 0 }),
                &mut [|_| Ok(1), |_| panic!()],
                2
            ),
            Ok(1)
        )
    }
}
