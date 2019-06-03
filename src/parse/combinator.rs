use super::parser::Parser;

pub fn many<T, E, F>(parser: &mut Parser, mut f: F) -> Result<Vec<T>, E>
where
    F: FnMut(&mut Parser) -> Result<T, E>,
{
    let mut xs = Vec::new();
    loop {
        let old_index = parser.index;
        match f(parser) {
            Ok(x) => xs.push(x),
            Err(_) if old_index == parser.index => return Ok(xs),
            Err(e) => return Err(e),
        }
    }
}

pub fn one_of<T, E, F>(parser: &mut Parser, fs: &mut [F], none_match: E) -> Result<T, E>
where
    F: FnMut(&mut Parser) -> Result<T, E>,
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
