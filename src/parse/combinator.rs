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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pos::Pos;

    #[test]
    fn test_many_ok_no_move_then_err_no_move() {
        let mut first = true;
        assert_eq!(
            many(&mut Parser::new("", &[], Pos::start()), |_| if first {
                first = false;
                Ok(())
            } else {
                Err(())
            }),
            Ok(vec![()])
        );
    }

    #[test]
    fn test_many_ok_move_then_err_move() {
        let mut first = true;
        assert_eq!(
            many(&mut Parser::new("", &[], Pos::start()), |parser| {
                parser.index += 1;
                if first {
                    first = false;
                    Ok(())
                } else {
                    Err(())
                }
            }),
            Err(())
        );
    }

    #[test]
    fn test_many_ok_move_then_err_no_move() {
        let mut first = true;
        assert_eq!(
            many(&mut Parser::new("", &[], Pos::start()), |parser| {
                if first {
                    first = false;
                    parser.index += 1;
                    Ok(())
                } else {
                    Err(())
                }
            }),
            Ok(vec![()])
        );
    }
}
